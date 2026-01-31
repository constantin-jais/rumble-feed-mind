//! OPML import/export routes

use axum::{
    extract::State,
    http::header,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

/// Import OPML request
#[derive(Debug, Deserialize)]
pub struct ImportOpmlRequest {
    /// Base64-encoded OPML content
    pub content: String,
}

/// Import result
#[derive(Serialize)]
pub struct ImportOpmlResponse {
    pub data: ImportResult,
}

#[derive(Serialize)]
pub struct ImportResult {
    pub feeds_imported: i64,
    pub feeds_skipped: i64,
    pub folders_created: i64,
    pub errors: Vec<ImportError>,
}

#[derive(Serialize)]
pub struct ImportError {
    pub url: String,
    pub reason: String,
}

/// Export response headers
const OPML_CONTENT_TYPE: &str = "application/xml";

/// Import OPML file
async fn import_opml(
    State(_state): State<AppState>,
    Json(req): Json<ImportOpmlRequest>,
) -> ApiResult<Json<ImportOpmlResponse>> {
    // Decode base64
    let content = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &req.content,
    ).map_err(|e| ApiError::BadRequest(format!("Invalid base64: {}", e)))?;

    // Parse OPML
    let _doc = feedmind_core::opml::OpmlParser::parse_bytes(&content)
        .map_err(|e| ApiError::BadRequest(format!("Invalid OPML: {}", e)))?;

    // TODO: Implement
    // 1. Create folders
    // 2. Create feeds (respecting user limits)
    // 3. Queue feed fetches
    // 4. Return import summary

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Export feeds as OPML
async fn export_opml(
    State(_state): State<AppState>,
) -> ApiResult<([(header::HeaderName, &'static str); 2], String)> {
    // TODO: Implement
    // 1. Get all user's feeds
    // 2. Build OPML document
    // 3. Export as XML

    let _doc = feedmind_core::opml::OpmlDocument::new(Some("FeedMind Export".to_string()));

    // Return OPML with proper headers
    let headers = [
        (header::CONTENT_TYPE, OPML_CONTENT_TYPE),
        (header::CONTENT_DISPOSITION, "attachment; filename=\"feedmind-export.opml\""),
    ];

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Build OPML routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/opml/import", post(import_opml))
        .route("/api/v1/opml/export", get(export_opml))
}
