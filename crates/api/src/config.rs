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

    /// Stripe configuration
    #[serde(flatten)]
    pub stripe: StripeConfig,
}

/// Stripe configuration
#[derive(Debug, Deserialize, Clone)]
pub struct StripeConfig {
    /// Stripe secret key
    #[serde(default)]
    pub stripe_secret_key: Option<String>,

    /// Stripe publishable key (for frontend)
    #[serde(default)]
    pub stripe_publishable_key: Option<String>,

    /// Stripe webhook signing secret
    #[serde(default)]
    pub stripe_webhook_secret: Option<String>,

    /// Price IDs for subscription plans
    #[serde(default)]
    pub stripe_price_pro_monthly: Option<String>,
    #[serde(default)]
    pub stripe_price_pro_annual: Option<String>,
    #[serde(default)]
    pub stripe_price_team_monthly: Option<String>,
    #[serde(default)]
    pub stripe_price_team_annual: Option<String>,

    /// Price IDs for usage-based billing
    #[serde(default)]
    pub stripe_price_ai_tokens: Option<String>,
    #[serde(default)]
    pub stripe_price_api_calls: Option<String>,
}

impl StripeConfig {
    /// Check if Stripe is configured
    pub fn is_configured(&self) -> bool {
        self.stripe_secret_key.is_some()
    }

    /// Get the Stripe secret key (panics if not configured)
    pub fn secret_key(&self) -> &str {
        self.stripe_secret_key
            .as_deref()
            .expect("Stripe secret key not configured")
    }

    /// Get the webhook secret (panics if not configured)
    pub fn webhook_secret(&self) -> &str {
        self.stripe_webhook_secret
            .as_deref()
            .expect("Stripe webhook secret not configured")
    }
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

        // Log Stripe configuration status
        if app_config.stripe.is_configured() {
            tracing::info!("Stripe billing is configured");
        } else {
            tracing::warn!("Stripe billing is not configured - billing features will be disabled");
        }

        Ok(app_config)
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Check if Stripe billing is enabled
    pub fn billing_enabled(&self) -> bool {
        self.stripe.is_configured()
    }
}
