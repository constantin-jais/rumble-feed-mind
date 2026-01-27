//! Auth extractor for getting current user from JWT

use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

/// Current authenticated user
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub tier: UserTier,
}

/// User subscription tier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserTier {
    Free,
    ProTrial,
    Pro,
    Team,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Get Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".to_string()))?;

        // Extract Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError::Unauthorized("Invalid Authorization header".to_string()))?;

        // TODO: Validate JWT and extract user
        // 1. Decode JWT
        // 2. Verify signature
        // 3. Check expiration
        // 4. Return user data

        let _ = token; // Suppress unused warning

        Err(ApiError::Unauthorized(
            "JWT validation not implemented".to_string(),
        ))
    }
}

impl UserTier {
    /// Get rate limit for this tier (requests per minute)
    pub fn rate_limit(&self) -> u32 {
        match self {
            UserTier::Free => 60,
            UserTier::ProTrial | UserTier::Pro => 300,
            UserTier::Team => 500,
        }
    }

    /// Get max feeds for this tier
    pub fn max_feeds(&self) -> u32 {
        match self {
            UserTier::Free => 100, // AMD-017: revised from 25
            UserTier::ProTrial | UserTier::Pro | UserTier::Team => 10_000,
        }
    }

    /// Get max rules for this tier
    pub fn max_rules(&self) -> u32 {
        match self {
            UserTier::Free => 10, // AMD-017: revised from 3
            UserTier::ProTrial | UserTier::Pro | UserTier::Team => 500,
        }
    }
}
