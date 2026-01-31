//! Application configuration

use config::{Config, Environment};
use serde::Deserialize;

/// Application configuration
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Server host
    #[serde(default = "default_host")]
    pub host: String,

    /// Server port
    #[serde(default = "default_port")]
    pub port: u16,

    /// Database URL
    pub database_url: String,

    /// Redis URL
    pub redis_url: String,

    /// JWT secret
    pub jwt_secret: String,

    /// JWT expiration in seconds (default: 7 days)
    #[serde(default = "default_jwt_expiration")]
    pub jwt_expiration: u64,

    /// Master encryption key (base64)
    pub master_key: String,

    /// Master key version
    #[serde(default = "default_key_version")]
    pub master_key_version: u32,

    /// Environment (development, production)
    #[serde(default = "default_environment")]
    pub environment: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3001
}

fn default_jwt_expiration() -> u64 {
    60 * 60 * 24 * 7 // 7 days
}

fn default_key_version() -> u32 {
    1
}

fn default_environment() -> String {
    "development".to_string()
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn load() -> anyhow::Result<Self> {
        let config = Config::builder()
            .add_source(Environment::default().separator("__"))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;

        // Validate required fields
        if app_config.database_url.is_empty() {
            anyhow::bail!("DATABASE_URL is required");
        }
        if app_config.redis_url.is_empty() {
            anyhow::bail!("REDIS_URL is required");
        }
        if app_config.jwt_secret.is_empty() {
            anyhow::bail!("JWT_SECRET is required");
        }
        if app_config.master_key.is_empty() {
            anyhow::bail!("MASTER_KEY is required");
        }

        Ok(app_config)
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
