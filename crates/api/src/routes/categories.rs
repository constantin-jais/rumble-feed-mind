//! Category routes

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::{ApiError, ApiResult};
use crate::extractors::auth::CurrentUser;
use crate::state::AppState;

/// Category with article count
#[derive(Serialize, FromRow)]
pub struct CategoryRow {
    pub category: String,
    pub article_count: i64,
    pub feed_count: i64,
}

/// Category for a specific feed
#[derive(Serialize, FromRow)]
pub struct FeedCategoryRow {
    pub category: String,
    pub article_count: i32,
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
    pub last_seen_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct CategoriesResponse {
    pub data: Vec<CategoryRow>,
}

#[derive(Serialize)]
pub struct FeedCategoriesResponse {
    pub data: Vec<FeedCategoryRow>,
}

/// List all categories for the user (aggregated across all feeds)
async fn list_categories(
    State(state): State<AppState>,
    user: CurrentUser,
) -> ApiResult<Json<CategoriesResponse>> {
    let categories: Vec<CategoryRow> = sqlx::query_as(
        r#"
        SELECT
            fc.category,
            SUM(fc.article_count)::bigint as article_count,
            COUNT(DISTINCT fc.feed_id)::bigint as feed_count
        FROM feed_categories fc
        JOIN feeds f ON f.id = fc.feed_id
        WHERE f.user_id = $1 AND fc.article_count > 0
        GROUP BY fc.category
        ORDER BY article_count DESC
        "#,
    )
    .bind(user.id)
    .fetch_all(state.db())
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(CategoriesResponse { data: categories }))
}

/// List categories for a specific feed
async fn list_feed_categories(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(feed_id): Path<Uuid>,
) -> ApiResult<Json<FeedCategoriesResponse>> {
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

    let categories: Vec<FeedCategoryRow> = sqlx::query_as(
        r#"
        SELECT category, article_count, first_seen_at, last_seen_at
        FROM feed_categories
        WHERE feed_id = $1 AND article_count > 0
        ORDER BY article_count DESC
        "#,
    )
    .bind(feed_id)
    .fetch_all(state.db())
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(FeedCategoriesResponse { data: categories }))
}

/// Build category routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/categories", get(list_categories))
        .route("/api/v1/feeds/{id}/categories", get(list_feed_categories))
}
