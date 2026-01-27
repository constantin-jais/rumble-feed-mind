//! Article routes

use axum::{
    extract::{Path, Query, State},
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

/// List articles query
#[derive(Debug, Deserialize)]
pub struct ListArticlesQuery {
    pub feed_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub status: Option<String>, // "unread", "read", "starred", "hidden"
    pub search: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

/// Update article request
#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub read: Option<bool>,
    pub starred: Option<bool>,
    pub tags: Option<Vec<String>>,
}

/// Article response
#[derive(Serialize)]
pub struct ArticleResponse {
    pub data: ArticleData,
}

#[derive(Serialize)]
pub struct ArticleData {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub feed_title: String,
    pub title: String,
    pub url: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<String>,
    pub read: bool,
    pub starred: bool,
    pub hidden: bool,
    pub hidden_reason: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
}

/// List articles response
#[derive(Serialize)]
pub struct ListArticlesResponse {
    pub data: Vec<ArticleData>,
    pub meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub cursor: Option<String>,
    pub has_more: bool,
}

/// List articles
async fn list_articles(
    State(_state): State<AppState>,
    Query(_query): Query<ListArticlesQuery>,
) -> ApiResult<Json<ListArticlesResponse>> {
    // TODO: Implement
    // 1. Get user from auth
    // 2. Query articles with filters
    // 3. Join with article_states for user-specific data
    // 4. Return paginated results

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Get a single article
async fn get_article(
    State(_state): State<AppState>,
    Path(_article_id): Path<Uuid>,
) -> ApiResult<Json<ArticleResponse>> {
    // TODO: Implement
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Update article state (read, starred, tags)
async fn update_article(
    State(_state): State<AppState>,
    Path(_article_id): Path<Uuid>,
    Json(_req): Json<UpdateArticleRequest>,
) -> ApiResult<Json<ArticleResponse>> {
    // TODO: Implement
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Mark all articles as read
async fn mark_all_read(
    State(_state): State<AppState>,
    Query(_query): Query<ListArticlesQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: Mark all matching articles as read
    Ok(Json(serde_json::json!({ "data": { "updated": 0 } })))
}

/// Restore a hidden article
async fn restore_article(
    State(_state): State<AppState>,
    Path(_article_id): Path<Uuid>,
) -> ApiResult<Json<ArticleResponse>> {
    // TODO: Unhide the article
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Build article routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/articles", get(list_articles))
        .route("/api/v1/articles/mark-read", patch(mark_all_read))
        .route("/api/v1/articles/:article_id", get(get_article).patch(update_article))
        .route("/api/v1/articles/:article_id/restore", patch(restore_article))
}
