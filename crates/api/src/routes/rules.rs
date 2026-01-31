//! Rules management routes

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

/// Create rule request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateRuleRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub rule_type: String, // "regex" (V1), "ai" (V1.1)
    #[validate(length(min = 1, max = 1000))]
    pub pattern: String,
    pub action: RuleActionRequest,
    pub feed_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub priority: Option<i32>,
    pub stop_on_match: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuleActionRequest {
    Hide,
    Keep,
    Tag { tag: String },
    Star,
    MarkRead,
}

/// Update rule request
#[derive(Debug, Deserialize)]
pub struct UpdateRuleRequest {
    pub name: Option<String>,
    pub pattern: Option<String>,
    pub action: Option<RuleActionRequest>,
    pub feed_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub active: Option<bool>,
    pub priority: Option<i32>,
    pub stop_on_match: Option<bool>,
}

/// List rules query
#[derive(Debug, Deserialize)]
pub struct ListRulesQuery {
    pub feed_id: Option<Uuid>,
    pub active: Option<bool>,
}

/// Rule response
#[derive(Serialize)]
pub struct RuleResponse {
    pub data: RuleData,
}

#[derive(Serialize)]
pub struct RuleData {
    pub id: Uuid,
    pub name: String,
    pub rule_type: String,
    pub pattern: String,
    pub action: RuleActionRequest,
    pub feed_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub active: bool,
    pub priority: i32,
    pub stop_on_match: bool,
    pub match_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// List rules response
#[derive(Serialize)]
pub struct ListRulesResponse {
    pub data: Vec<RuleData>,
}

/// Preview rule result
#[derive(Serialize)]
pub struct PreviewResponse {
    pub data: PreviewData,
}

#[derive(Serialize)]
pub struct PreviewData {
    pub would_match: i64,
    pub would_keep: i64,
    pub sample_matches: Vec<PreviewMatch>,
}

#[derive(Serialize)]
pub struct PreviewMatch {
    pub article_id: Uuid,
    pub title: String,
    pub reason: String,
}

/// Create a new rule
async fn create_rule(
    State(_state): State<AppState>,
    Json(req): Json<CreateRuleRequest>,
) -> ApiResult<Json<RuleResponse>> {
    req.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // Validate regex pattern
    if req.rule_type == "regex" {
        regex::Regex::new(&req.pattern)
            .map_err(|e| ApiError::Validation(format!("Invalid regex: {}", e)))?;
    }

    // TODO: Implement
    // 1. Create rule in database
    // 2. Compile and cache regex

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// List user's rules
async fn list_rules(
    State(_state): State<AppState>,
    Query(_query): Query<ListRulesQuery>,
) -> ApiResult<Json<ListRulesResponse>> {
    // TODO: Implement
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Get a single rule
async fn get_rule(
    State(_state): State<AppState>,
    Path(_rule_id): Path<Uuid>,
) -> ApiResult<Json<RuleResponse>> {
    // TODO: Implement
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Update a rule
async fn update_rule(
    State(_state): State<AppState>,
    Path(_rule_id): Path<Uuid>,
    Json(_req): Json<UpdateRuleRequest>,
) -> ApiResult<Json<RuleResponse>> {
    // TODO: Implement
    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Delete a rule
async fn delete_rule(
    State(_state): State<AppState>,
    Path(_rule_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: Implement
    Ok(Json(serde_json::json!({ "data": { "deleted": true } })))
}

/// Preview rule effect on recent articles
async fn preview_rule(
    State(_state): State<AppState>,
    Json(req): Json<CreateRuleRequest>,
) -> ApiResult<Json<PreviewResponse>> {
    req.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // TODO: Implement
    // 1. Get last 7 days of articles
    // 2. Apply rule to each
    // 3. Return summary + samples

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Build rules routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/rules", get(list_rules).post(create_rule))
        .route("/api/v1/rules/preview", post(preview_rule))
        .route("/api/v1/rules/:rule_id", get(get_rule).put(update_rule).delete(delete_rule))
}
