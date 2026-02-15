//! Article routes

use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::{ApiError, ApiResult};
use crate::extractors::auth::CurrentUser;
use crate::state::AppState;

/// List articles query
#[derive(Debug, Deserialize)]
pub struct ListArticlesQuery {
    pub feed_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub categories: Option<String>, // Comma-separated list of categories
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

/// Update article request
#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub is_read: Option<bool>,
    pub is_starred: Option<bool>,
    pub tags: Option<Vec<String>>,
}

/// Batch update request
#[derive(Debug, Deserialize)]
pub struct BatchUpdateRequest {
    pub article_ids: Vec<Uuid>,
    pub is_read: Option<bool>,
    pub is_starred: Option<bool>,
}

/// Article row
#[derive(Serialize, FromRow)]
pub struct ArticleRow {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub url: Option<String>,
    pub title: String,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_read: bool,
    pub is_starred: bool,
    pub is_hidden: bool,
    pub word_count: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Article list item (without full content)
#[derive(Serialize, FromRow)]
pub struct ArticleListItem {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub feed_title: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_read: bool,
    pub is_starred: bool,
    pub word_count: Option<i32>,
    pub categories: sqlx::types::Json<Vec<String>>,
}

#[derive(Serialize)]
pub struct ArticleResponse {
    pub data: ArticleRow,
}

#[derive(Serialize)]
pub struct ArticlesListResponse {
    pub data: Vec<ArticleListItem>,
    pub meta: ListMeta,
}

#[derive(Serialize)]
pub struct ListMeta {
    pub total: i64,
    pub cursor: Option<String>,
    pub has_more: bool,
}

/// List articles
async fn list_articles(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<ListArticlesQuery>,
) -> ApiResult<Json<ArticlesListResponse>> {
    let limit = query.limit.unwrap_or(50).min(100);

    // Build status filter
    let status_filter = match query.status.as_deref() {
        Some("unread") => "AND a.is_read = FALSE AND a.is_hidden = FALSE",
        Some("read") => "AND a.is_read = TRUE",
        Some("starred") => "AND a.is_starred = TRUE",
        Some("hidden") => "AND a.is_hidden = TRUE",
        _ => "AND a.is_hidden = FALSE",
    };

    // Build category filter (using ?| operator for "any of" matching)
    let category_filter = if let Some(ref cats) = query.categories {
        let categories: Vec<&str> = cats
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if !categories.is_empty() {
            format!(
                "AND a.categories ?| ARRAY[{}]",
                categories
                    .iter()
                    .map(|c| format!("'{}'", c.replace('\'', "''")))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Build query based on filters
    let articles: Vec<ArticleListItem> = match (query.feed_id, query.folder_id) {
        (Some(feed_id), _) => {
            // Filter by feed
            sqlx::query_as(&format!(
                r#"
                SELECT a.id, a.feed_id, f.title as feed_title, a.url, a.title, a.author,
                       a.summary, a.image_url, a.published_at, a.is_read, a.is_starred, a.word_count,
                       a.categories
                FROM articles a
                JOIN feeds f ON f.id = a.feed_id
                WHERE a.user_id = $1 AND a.feed_id = $2 {} {}
                ORDER BY a.published_at DESC NULLS LAST
                LIMIT $3
                "#,
                status_filter, category_filter
            ))
            .bind(user.id)
            .bind(feed_id)
            .bind(limit)
            .fetch_all(state.db())
            .await
        }
        (_, Some(folder_id)) => {
            // Filter by folder
            sqlx::query_as(&format!(
                r#"
                SELECT a.id, a.feed_id, f.title as feed_title, a.url, a.title, a.author,
                       a.summary, a.image_url, a.published_at, a.is_read, a.is_starred, a.word_count,
                       a.categories
                FROM articles a
                JOIN feeds f ON f.id = a.feed_id
                WHERE a.user_id = $1 AND f.folder_id = $2 {} {}
                ORDER BY a.published_at DESC NULLS LAST
                LIMIT $3
                "#,
                status_filter, category_filter
            ))
            .bind(user.id)
            .bind(folder_id)
            .bind(limit)
            .fetch_all(state.db())
            .await
        }
        _ => {
            // All articles
            sqlx::query_as(&format!(
                r#"
                SELECT a.id, a.feed_id, f.title as feed_title, a.url, a.title, a.author,
                       a.summary, a.image_url, a.published_at, a.is_read, a.is_starred, a.word_count,
                       a.categories
                FROM articles a
                JOIN feeds f ON f.id = a.feed_id
                WHERE a.user_id = $1 {} {}
                ORDER BY a.published_at DESC NULLS LAST
                LIMIT $2
                "#,
                status_filter, category_filter
            ))
            .bind(user.id)
            .bind(limit)
            .fetch_all(state.db())
            .await
        }
    }
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    // Get total count
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM articles WHERE user_id = $1 AND is_hidden = FALSE",
    )
    .bind(user.id)
    .fetch_one(state.db())
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    let has_more = articles.len() as i64 == limit;

    Ok(Json(ArticlesListResponse {
        data: articles,
        meta: ListMeta {
            total,
            cursor: None,
            has_more,
        },
    }))
}

/// Get a single article
async fn get_article(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(article_id): Path<Uuid>,
) -> ApiResult<Json<ArticleResponse>> {
    let article: Option<ArticleRow> = sqlx::query_as(
        r#"
        SELECT id, feed_id, url, title, author, summary, content, image_url,
               published_at, is_read, is_starred, is_hidden, word_count, created_at
        FROM articles
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(article_id)
    .bind(user.id)
    .fetch_optional(state.db())
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    let article = article.ok_or_else(|| ApiError::NotFound("Article not found".to_string()))?;

    Ok(Json(ArticleResponse { data: article }))
}

/// Update article (read/star status)
async fn update_article(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(article_id): Path<Uuid>,
    Json(req): Json<UpdateArticleRequest>,
) -> ApiResult<Json<ArticleResponse>> {
    // Build update query dynamically
    let mut updates = Vec::new();
    let mut param_idx = 3;

    if req.is_read.is_some() {
        updates.push(format!("is_read = ${}", param_idx));
        param_idx += 1;
        if req.is_read == Some(true) {
            updates.push("read_at = NOW()".to_string());
        }
    }
    if req.is_starred.is_some() {
        updates.push(format!("is_starred = ${}", param_idx));
        if req.is_starred == Some(true) {
            updates.push("starred_at = NOW()".to_string());
        }
    }

    if updates.is_empty() {
        // Nothing to update, just return the article
        return get_article(State(state), user, Path(article_id)).await;
    }

    let query = format!(
        r#"
        UPDATE articles SET {}, updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        RETURNING id, feed_id, url, title, author, summary, content, image_url,
                  published_at, is_read, is_starred, is_hidden, word_count, created_at
        "#,
        updates.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, ArticleRow>(&query)
        .bind(article_id)
        .bind(user.id);

    if let Some(is_read) = req.is_read {
        query_builder = query_builder.bind(is_read);
    }
    if let Some(is_starred) = req.is_starred {
        query_builder = query_builder.bind(is_starred);
    }

    let article = query_builder
        .fetch_optional(state.db())
        .await
        .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::NotFound("Article not found".to_string()))?;

    Ok(Json(ArticleResponse { data: article }))
}

/// Batch update articles
async fn batch_update_articles(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<BatchUpdateRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    if req.article_ids.is_empty() {
        return Err(ApiError::Validation("No article IDs provided".to_string()));
    }

    if req.article_ids.len() > 100 {
        return Err(ApiError::Validation(
            "Maximum 100 articles per batch".to_string(),
        ));
    }

    let mut updated = 0;

    if let Some(is_read) = req.is_read {
        let result = if is_read {
            sqlx::query(
                "UPDATE articles SET is_read = TRUE, read_at = NOW(), updated_at = NOW() WHERE id = ANY($1) AND user_id = $2"
            )
        } else {
            sqlx::query(
                "UPDATE articles SET is_read = FALSE, read_at = NULL, updated_at = NOW() WHERE id = ANY($1) AND user_id = $2"
            )
        }
        .bind(&req.article_ids)
        .bind(user.id)
        .execute(state.db())
        .await
        .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

        updated = result.rows_affected() as i64;
    }

    if let Some(is_starred) = req.is_starred {
        let result = if is_starred {
            sqlx::query(
                "UPDATE articles SET is_starred = TRUE, starred_at = NOW(), updated_at = NOW() WHERE id = ANY($1) AND user_id = $2"
            )
        } else {
            sqlx::query(
                "UPDATE articles SET is_starred = FALSE, starred_at = NULL, updated_at = NOW() WHERE id = ANY($1) AND user_id = $2"
            )
        }
        .bind(&req.article_ids)
        .bind(user.id)
        .execute(state.db())
        .await
        .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

        updated = result.rows_affected() as i64;
    }

    Ok(Json(serde_json::json!({
        "data": {
            "updated": updated
        }
    })))
}

/// Mark all articles as read
async fn mark_all_read(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(query): Query<ListArticlesQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = if let Some(feed_id) = query.feed_id {
        sqlx::query(
            "UPDATE articles SET is_read = TRUE, read_at = NOW(), updated_at = NOW() WHERE user_id = $1 AND feed_id = $2 AND is_read = FALSE"
        )
        .bind(user.id)
        .bind(feed_id)
        .execute(state.db())
        .await
    } else if let Some(folder_id) = query.folder_id {
        sqlx::query(
            r#"
            UPDATE articles SET is_read = TRUE, read_at = NOW(), updated_at = NOW()
            WHERE user_id = $1 AND is_read = FALSE
            AND feed_id IN (SELECT id FROM feeds WHERE folder_id = $2)
            "#
        )
        .bind(user.id)
        .bind(folder_id)
        .execute(state.db())
        .await
    } else {
        sqlx::query(
            "UPDATE articles SET is_read = TRUE, read_at = NOW(), updated_at = NOW() WHERE user_id = $1 AND is_read = FALSE"
        )
        .bind(user.id)
        .execute(state.db())
        .await
    }
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(serde_json::json!({
        "data": {
            "updated": result.rows_affected()
        }
    })))
}

/// Build article routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/articles", get(list_articles))
        .route("/api/v1/articles/batch", put(batch_update_articles))
        .route(
            "/api/v1/articles/mark-all-read",
            axum::routing::post(mark_all_read),
        )
        .route(
            "/api/v1/articles/{id}",
            get(get_article).put(update_article),
        )
}
