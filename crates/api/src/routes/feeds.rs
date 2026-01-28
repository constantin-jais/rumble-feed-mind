//! Feed management routes

use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::error::{ApiError, ApiResult};
use crate::extractors::auth::CurrentUser;
use crate::state::AppState;

/// Create feed request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateFeedRequest {
    #[validate(url(message = "Invalid URL"))]
    #[validate(length(max = 2048))]
    pub url: String,
    pub folder_id: Option<Uuid>,
    pub title: Option<String>,
}

/// Update feed request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateFeedRequest {
    #[validate(length(max = 500))]
    pub title: Option<String>,
    pub folder_id: Option<Uuid>,
    pub priority: Option<String>,
}

/// List feeds query
#[derive(Debug, Deserialize)]
pub struct ListFeedsQuery {
    pub folder_id: Option<Uuid>,
    pub priority: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

/// Feed response
#[derive(Serialize, FromRow)]
pub struct FeedRow {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub site_url: Option<String>,
    pub icon_url: Option<String>,
    pub feed_type: Option<String>,
    pub priority: String,
    pub folder_id: Option<Uuid>,
    pub article_count: i32,
    pub unread_count: i32,
    pub error_count: i32,
    pub last_error: Option<String>,
    pub last_fetched_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct FeedResponse {
    pub data: FeedRow,
}

#[derive(Serialize)]
pub struct FeedsListResponse {
    pub data: Vec<FeedRow>,
    pub meta: ListMeta,
}

#[derive(Serialize)]
pub struct ListMeta {
    pub total: i64,
    pub cursor: Option<String>,
    pub has_more: bool,
}

/// Create a new feed
async fn create_feed(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<CreateFeedRequest>,
) -> ApiResult<Json<FeedResponse>> {
    req.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // Check feed limit
    let feed_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM feeds WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(state.db())
        .await
        .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    if feed_count >= user.tier.max_feeds() as i64 {
        return Err(ApiError::Forbidden(format!(
            "Feed limit reached ({}/{}). Upgrade to add more feeds.",
            feed_count,
            user.tier.max_feeds()
        )));
    }

    // Check if feed URL already exists for user
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM feeds WHERE user_id = $1 AND url = $2)")
            .bind(user.id)
            .bind(&req.url)
            .fetch_one(state.db())
            .await
            .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    if exists {
        return Err(ApiError::Conflict("Feed already exists".to_string()));
    }

    // Fetch and parse the feed to get metadata
    let fetcher = feedmind_core::feed::FeedFetcher::new()
        .map_err(|e| ApiError::Internal(format!("Fetcher error: {}", e)))?;

    let (feed_meta, _items) = fetcher
        .fetch(&req.url)
        .await
        .map_err(|e| ApiError::Validation(format!("Failed to fetch feed: {}", e)))?;

    // Use provided title or fetched title
    let title = req.title.unwrap_or(feed_meta.title);
    let feed_type_str = feed_meta.feed_type.to_string();

    // Insert feed
    let feed: FeedRow = sqlx::query_as(
        r#"
        INSERT INTO feeds (user_id, url, title, description, site_url, feed_type, folder_id, priority)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'warm')
        RETURNING id, url, title, description, site_url, icon_url, feed_type, priority,
                  folder_id, article_count, unread_count, error_count, last_error,
                  last_fetched_at, created_at
        "#
    )
    .bind(user.id)
    .bind(&req.url)
    .bind(&title)
    .bind(&feed_meta.description)
    .bind(&feed_meta.site_url)
    .bind(&feed_type_str)
    .bind(req.folder_id)
    .fetch_one(state.db())
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to create feed: {}", e)))?;

    // Queue initial fetch job
    let mut redis = state.redis();
    let job = serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "job_type": { "type": "FetchFeed", "data": { "feed_id": feed.id.to_string() } },
        "attempts": 0,
        "max_attempts": 3,
        "created_at": chrono::Utc::now().to_rfc3339()
    });
    let _: () = redis::cmd("RPUSH")
        .arg("feedmind:jobs")
        .arg(job.to_string())
        .query_async(&mut redis)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to queue job: {}", e)))?;

    Ok(Json(FeedResponse { data: feed }))
}

/// List all feeds
async fn list_feeds(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<ListFeedsQuery>,
) -> ApiResult<Json<FeedsListResponse>> {
    let limit = query.limit.unwrap_or(50).min(100);

    let feeds: Vec<FeedRow> = if let Some(folder_id) = query.folder_id {
        sqlx::query_as(
            r#"
            SELECT id, url, title, description, site_url, icon_url, feed_type, priority,
                   folder_id, article_count, unread_count, error_count, last_error,
                   last_fetched_at, created_at
            FROM feeds
            WHERE user_id = $1 AND folder_id = $2
            ORDER BY position ASC, title ASC
            LIMIT $3
            "#,
        )
        .bind(user.id)
        .bind(folder_id)
        .bind(limit)
        .fetch_all(state.db())
        .await
    } else {
        sqlx::query_as(
            r#"
            SELECT id, url, title, description, site_url, icon_url, feed_type, priority,
                   folder_id, article_count, unread_count, error_count, last_error,
                   last_fetched_at, created_at
            FROM feeds
            WHERE user_id = $1
            ORDER BY position ASC, title ASC
            LIMIT $2
            "#,
        )
        .bind(user.id)
        .bind(limit)
        .fetch_all(state.db())
        .await
    }
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM feeds WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(state.db())
        .await
        .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    let has_more = feeds.len() as i64 == limit;

    Ok(Json(FeedsListResponse {
        data: feeds,
        meta: ListMeta {
            total,
            cursor: None,
            has_more,
        },
    }))
}

/// Get a single feed
async fn get_feed(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(feed_id): Path<Uuid>,
) -> ApiResult<Json<FeedResponse>> {
    let feed: Option<FeedRow> = sqlx::query_as(
        r#"
        SELECT id, url, title, description, site_url, icon_url, feed_type, priority,
               folder_id, article_count, unread_count, error_count, last_error,
               last_fetched_at, created_at
        FROM feeds
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(feed_id)
    .bind(user.id)
    .fetch_optional(state.db())
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    let feed = feed.ok_or_else(|| ApiError::NotFound("Feed not found".to_string()))?;

    Ok(Json(FeedResponse { data: feed }))
}

/// Update a feed
async fn update_feed(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(feed_id): Path<Uuid>,
    Json(req): Json<UpdateFeedRequest>,
) -> ApiResult<Json<FeedResponse>> {
    req.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // Verify ownership
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM feeds WHERE id = $1 AND user_id = $2)")
            .bind(feed_id)
            .bind(user.id)
            .fetch_one(state.db())
            .await
            .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    if !exists {
        return Err(ApiError::NotFound("Feed not found".to_string()));
    }

    // Validate priority if provided
    if let Some(ref priority) = req.priority {
        if !["hot", "warm", "cold"].contains(&priority.as_str()) {
            return Err(ApiError::Validation(
                "Invalid priority. Use: hot, warm, cold".to_string(),
            ));
        }
    }

    // Update feed
    let feed: FeedRow = sqlx::query_as(
        r#"
        UPDATE feeds SET
            title = COALESCE($3, title),
            folder_id = COALESCE($4, folder_id),
            priority = COALESCE($5, priority),
            updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        RETURNING id, url, title, description, site_url, icon_url, feed_type, priority,
                  folder_id, article_count, unread_count, error_count, last_error,
                  last_fetched_at, created_at
        "#,
    )
    .bind(feed_id)
    .bind(user.id)
    .bind(&req.title)
    .bind(req.folder_id)
    .bind(&req.priority)
    .fetch_one(state.db())
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to update feed: {}", e)))?;

    Ok(Json(FeedResponse { data: feed }))
}

/// Delete a feed
async fn delete_feed(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(feed_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM feeds WHERE id = $1 AND user_id = $2")
        .bind(feed_id)
        .bind(user.id)
        .execute(state.db())
        .await
        .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Feed not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "data": { "success": true } })))
}

/// Refresh a feed manually
async fn refresh_feed(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(feed_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    // Verify ownership
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM feeds WHERE id = $1 AND user_id = $2)")
            .bind(feed_id)
            .bind(user.id)
            .fetch_one(state.db())
            .await
            .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    if !exists {
        return Err(ApiError::NotFound("Feed not found".to_string()));
    }

    // Queue fetch job
    let mut redis = state.redis();
    let job = serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "job_type": { "type": "FetchFeed", "data": { "feed_id": feed_id.to_string() } },
        "attempts": 0,
        "max_attempts": 3,
        "created_at": chrono::Utc::now().to_rfc3339()
    });
    let _: () = redis::cmd("RPUSH")
        .arg("feedmind:jobs")
        .arg(job.to_string())
        .query_async(&mut redis)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to queue job: {}", e)))?;

    Ok(Json(serde_json::json!({ "data": { "queued": true } })))
}

/// Build feed routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/feeds", get(list_feeds).post(create_feed))
        .route(
            "/api/v1/feeds/:id",
            get(get_feed).put(update_feed).delete(delete_feed),
        )
        .route("/api/v1/feeds/:id/refresh", post(refresh_feed))
}
