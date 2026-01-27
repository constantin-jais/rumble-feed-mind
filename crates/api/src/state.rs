//! Application state

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use feedmind_core::crypto::KeyEncryption;

use crate::config::AppConfig;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    /// Database connection pool
    pub db: PgPool,
    /// Redis connection manager
    pub redis: ConnectionManager,
    /// Key encryption handler
    pub encryption: KeyEncryption,
    /// JWT secret
    pub jwt_secret: String,
    /// JWT expiration (seconds)
    pub jwt_expiration: u64,
    /// Is production environment
    pub is_production: bool,
}

impl AppState {
    /// Create new app state from config
    pub async fn new(config: &AppConfig) -> anyhow::Result<Self> {
        // Connect to PostgreSQL
        let db = PgPoolOptions::new()
            .max_connections(20)
            .connect(&config.database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&db)
            .await?;

        // Connect to Redis
        let redis_client = redis::Client::open(config.redis_url.as_str())?;
        let redis = ConnectionManager::new(redis_client).await?;

        // Setup encryption
        let encryption = KeyEncryption::from_base64(
            &config.master_key,
            config.master_key_version,
        )?;

        Ok(Self {
            inner: Arc::new(AppStateInner {
                db,
                redis,
                encryption,
                jwt_secret: config.jwt_secret.clone(),
                jwt_expiration: config.jwt_expiration,
                is_production: config.is_production(),
            }),
        })
    }

    /// Get database pool
    pub fn db(&self) -> &PgPool {
        &self.inner.db
    }

    /// Get Redis connection
    pub fn redis(&self) -> ConnectionManager {
        self.inner.redis.clone()
    }

    /// Get encryption handler
    pub fn encryption(&self) -> &KeyEncryption {
        &self.inner.encryption
    }

    /// Get JWT secret
    pub fn jwt_secret(&self) -> &str {
        &self.inner.jwt_secret
    }

    /// Get JWT expiration
    pub fn jwt_expiration(&self) -> u64 {
        self.inner.jwt_expiration
    }

    /// Check if production
    pub fn is_production(&self) -> bool {
        self.inner.is_production
    }
}
