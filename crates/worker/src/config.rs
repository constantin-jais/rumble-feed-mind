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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::{Mutex, MutexGuard};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        _lock: MutexGuard<'static, ()>,
        vars: Vec<(String, Option<String>)>,
    }

    impl EnvGuard {
        fn set(pairs: &[(&str, Option<&str>)]) -> Self {
            let lock = ENV_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let vars = pairs
                .iter()
                .map(|(key, value)| {
                    let key = (*key).to_string();
                    let previous = env::var(&key).ok();
                    match value {
                        Some(value) => env::set_var(&key, value),
                        None => env::remove_var(&key),
                    }
                    (key, previous)
                })
                .collect();

            Self { _lock: lock, vars }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, previous) in self.vars.drain(..).rev() {
                match previous {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
        }
    }

    #[test]
    fn worker_config_loads_deterministic_values_from_environment() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("CONCURRENT_FETCHES", Some("7")),
            ("REFRESH_INTERVAL", Some("1")),
            ("MASTER_KEY_VERSION", Some("9")),
        ]);

        let config = WorkerConfig::load().expect("worker config should load from environment");

        assert_eq!(config.concurrent_fetches, 7);
        assert_eq!(config.refresh_interval, 1);
        assert_eq!(config.master_key_version, 9);
        assert_eq!(
            config.worker_database_url,
            "postgres://worker:worker@localhost/worker"
        );
        assert_eq!(config.redis_url, "redis://localhost:6379/1");
    }
}
