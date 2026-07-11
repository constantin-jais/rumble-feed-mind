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

    /// Application-role database URL. This principal must not own tables.
    pub database_url: String,

    /// Authentication-role database URL, restricted to reviewed functions.
    pub auth_database_url: String,

    /// Worker-role database URL used only by trusted webhook paths.
    #[serde(default)]
    pub worker_database_url: Option<String>,

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
            anyhow::bail!("DATABASE_URL is required for the application role");
        }
        if app_config.auth_database_url.is_empty() {
            anyhow::bail!("AUTH_DATABASE_URL is required for the authentication role");
        }
        if app_config.billing_enabled()
            && app_config
                .worker_database_url
                .as_deref()
                .is_none_or(str::is_empty)
        {
            anyhow::bail!("WORKER_DATABASE_URL is required when Stripe webhooks are enabled");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_is_disabled_when_secret_key_is_absent() {
        let config = StripeConfig {
            stripe_secret_key: None,
            stripe_publishable_key: Some("pk_fixture".to_string()),
            stripe_webhook_secret: Some("whsec_fixture".to_string()),
            stripe_price_pro_monthly: None,
            stripe_price_pro_annual: None,
            stripe_price_team_monthly: None,
            stripe_price_team_annual: None,
            stripe_price_ai_tokens: None,
            stripe_price_api_calls: None,
        };

        assert!(!config.is_configured());
    }

    #[test]
    fn stripe_is_enabled_only_by_explicit_secret_key() {
        let config = StripeConfig {
            stripe_secret_key: Some("stripe-enabled-fixture".to_string()),
            stripe_publishable_key: None,
            stripe_webhook_secret: None,
            stripe_price_pro_monthly: None,
            stripe_price_pro_annual: None,
            stripe_price_team_monthly: None,
            stripe_price_team_annual: None,
            stripe_price_ai_tokens: None,
            stripe_price_api_calls: None,
        };

        assert!(config.is_configured());
    }
}
