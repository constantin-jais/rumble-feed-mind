//! Feed management routes

use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

/// Create feed request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateFeedRequest {
    #[validate(url(message = "Invalid URL"))]
    pub url: String,
    pub folder_id: Option<Uuid>,
}

/// Update feed request
#[derive(Debug, Deserialize)]
pub struct UpdateFeedRequest {
    pub title: Option<String>,
    pub folder_id: Option<Uuid>,
    pub priority: Option<String>, // "hot", "warm", "cold"
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
#[derive(Serialize)]
pub struct FeedResponse {
    pub data: FeedData,
}

#[derive(Serialize)]
pub struct FeedData {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub site_url: Option<String>,
    pub icon_url: Option<String>,
    pub folder_id: Option<Uuid>,
    pub priority: String,
    pub unread_count: i64,
    pub last_fetched_at: Option<String>,
    pub created_at: String,
}

/// List feeds response
#[derive(Serialize)]
pub struct ListFeedsResponse {
    pub data: Vec<FeedData>,
    pub meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub cursor: Option<String>,
    pub has_more: bool,
}

/// Create a new feed
async fn create_feed(
    State(_state): State<AppState>,
    Json(req): Json<CreateFeedRequest>,
) -> ApiResult<Json<FeedResponse>> {
    req.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // TODO: Implement
    // 1. Validate URL is a valid feed
    // 2. Fetch and parse feed
    // 3. Create feed in database
    // 4. Queue initial article fetch

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// List user's feeds
async fn list_feeds(
    State(_state): State<AppState>,
    Query(_query): Query<ListFeedsQuery>,
) -> ApiResult<Json<ListFeedsResponse>> {
    // TODO: Implement
    // 1. Get user from auth
    // 2. Query feeds with filters
    // 3. Return paginated results

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Get a single feed
async fn get_feed(
    State(_state): State<AppState>,
    Path(_feed_id): Path<Uuid>,
) -> ApiResult<Json<FeedResponse>> {
    // TODO: Implement
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Update a feed
async fn update_feed(
    State(_state): State<AppState>,
    Path(_feed_id): Path<Uuid>,
    Json(_req): Json<UpdateFeedRequest>,
) -> ApiResult<Json<FeedResponse>> {
    // TODO: Implement
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Delete a feed
async fn delete_feed(
    State(_state): State<AppState>,
    Path(_feed_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: Implement
    Ok(Json(serde_json::json!({ "data": { "deleted": true } })))
}

/// Refresh a feed manually
async fn refresh_feed(
    State(_state): State<AppState>,
    Path(_feed_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: Queue feed refresh job
    Ok(Json(serde_json::json!({ "data": { "queued": true } })))
}

/// Build feed routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/feeds", get(list_feeds).post(create_feed))
        .route("/api/v1/feeds/:feed_id", get(get_feed).put(update_feed).delete(delete_feed))
        .route("/api/v1/feeds/:feed_id/refresh", post(refresh_feed))
}
