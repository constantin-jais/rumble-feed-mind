//! Application state

use std::{path::Path, sync::Arc};

use feedmind_crypto::KeyEncryption;
use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
#[cfg(feature = "stripe")]
use stripe::Client as StripeClient;

use crate::config::{AppConfig, StripeConfig};

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
    /// Stripe client (None if billing disabled)
    #[cfg(feature = "stripe")]
    pub stripe: Option<StripeClient>,
    /// Stripe configuration
    pub stripe_config: StripeConfig,
}

impl AppState {
    /// Create new app state from config
    pub async fn new(config: &AppConfig) -> anyhow::Result<Self> {
        // Connect to PostgreSQL
        let db = PgPoolOptions::new()
            .max_connections(20)
            .connect(&config.database_url)
            .await?;

        // Run migrations from the workspace migrations directory at runtime.
        let migrations = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
        sqlx::migrate::Migrator::new(migrations)
            .await?
            .run(&db)
            .await?;

        // Connect to Redis
        let redis_client = redis::Client::open(config.redis_url.as_str())?;
        let redis = ConnectionManager::new(redis_client).await?;

        // Setup encryption
        let encryption = KeyEncryption::from_base64(&config.master_key, config.master_key_version)?;

        // Setup Stripe client if configured
        #[cfg(feature = "stripe")]
        let stripe = if config.stripe.is_configured() {
            Some(StripeClient::new(config.stripe.secret_key()))
        } else {
            None
        };

        Ok(Self {
            inner: Arc::new(AppStateInner {
                db,
                redis,
                encryption,
                jwt_secret: config.jwt_secret.clone(),
                jwt_expiration: config.jwt_expiration,
                is_production: config.is_production(),
                #[cfg(feature = "stripe")]
                stripe,
                stripe_config: config.stripe.clone(),
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

    /// Get Stripe client (returns None if billing is disabled)
    #[cfg(feature = "stripe")]
    pub fn stripe(&self) -> Option<&StripeClient> {
        self.inner.stripe.as_ref()
    }

    /// Get Stripe config
    pub fn stripe_config(&self) -> &StripeConfig {
        &self.inner.stripe_config
    }

    /// Check if billing is enabled
    #[cfg(feature = "stripe")]
    pub fn billing_enabled(&self) -> bool {
        self.inner.stripe.is_some()
    }

    /// Check if billing is enabled
    #[cfg(not(feature = "stripe"))]
    pub fn billing_enabled(&self) -> bool {
        false
    }
}
