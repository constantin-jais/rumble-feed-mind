//! Authentication routes

use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

/// Register request
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

/// Login request
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

/// Auth response
#[derive(Serialize)]
pub struct AuthResponse {
    pub data: AuthData,
}

#[derive(Serialize)]
pub struct AuthData {
    pub token: String,
    pub user: UserData,
}

#[derive(Serialize)]
pub struct UserData {
    pub id: String,
    pub email: String,
    pub tier: String,
}

/// Register new user
async fn register(
    State(_state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<Json<AuthResponse>> {
    // Validate request
    req.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // TODO: Implement user registration
    // 1. Check if email exists
    // 2. Hash password with argon2
    // 3. Create user in database
    // 4. Generate JWT token

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Login user
async fn login(
    State(_state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    // Validate request
    req.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // TODO: Implement login
    // 1. Find user by email
    // 2. Verify password with argon2
    // 3. Generate JWT token

    Err(ApiError::Internal("Not implemented".to_string()))
}

/// Logout (invalidate token)
async fn logout() -> ApiResult<Json<serde_json::Value>> {
    // TODO: Add token to blacklist in Redis
    Ok(Json(serde_json::json!({ "data": { "success": true } })))
}

/// Build auth routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/logout", post(logout))
}
