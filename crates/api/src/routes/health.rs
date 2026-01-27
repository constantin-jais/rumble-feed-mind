//! Health check routes

use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::state::AppState;

/// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

/// Health check handler
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Build health routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/health", get(health_check))
}
