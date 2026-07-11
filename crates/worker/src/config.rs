//! Worker configuration

use config::{Config, Environment};
use serde::Deserialize;

/// Worker configuration
#[derive(Debug, Deserialize)]
pub struct WorkerConfig {
    /// Dedicated worker-role database URL.
    pub worker_database_url: String,

    /// Redis URL
    pub redis_url: String,

    /// Number of concurrent feed fetches
    #[serde(default = "default_concurrent_fetches")]
    pub concurrent_fetches: usize,

    /// Feed refresh interval in seconds
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval: u64,

    /// Master encryption key (base64)
    pub master_key: String,

    /// Master key version
    #[serde(default = "default_key_version")]
    pub master_key_version: u32,
}

fn default_concurrent_fetches() -> usize {
    50 // AMD-003: Max concurrent fetches
}

fn default_refresh_interval() -> u64 {
    900 // 15 minutes
}

fn default_key_version() -> u32 {
    1
}

impl WorkerConfig {
    /// Load configuration from environment
    pub fn load() -> anyhow::Result<Self> {
        let config = Config::builder()
            .add_source(Environment::default().separator("__"))
            .build()?;

        let worker_config: WorkerConfig = config.try_deserialize()?;

        if worker_config.worker_database_url.is_empty() {
            anyhow::bail!("WORKER_DATABASE_URL is required for the dedicated worker role");
        }
        if worker_config.redis_url.is_empty() {
            anyhow::bail!("REDIS_URL is required");
        }

        Ok(worker_config)
    }
}
