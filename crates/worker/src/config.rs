//! Worker configuration

use config::{Config, Environment};
use serde::Deserialize;
use std::time::Duration;

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

    /// Feed refresh interval in seconds (inclusive 300..=86400)
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

const MIN_REFRESH_INTERVAL_SECONDS: u64 = 300;
const MAX_REFRESH_INTERVAL_SECONDS: u64 = 86_400;
const DEFAULT_REFRESH_INTERVAL_SECONDS: u64 = MIN_REFRESH_INTERVAL_SECONDS;

fn default_refresh_interval() -> u64 {
    DEFAULT_REFRESH_INTERVAL_SECONDS
}

pub(crate) fn refresh_interval_duration(refresh_interval: u64) -> anyhow::Result<Duration> {
    if !(MIN_REFRESH_INTERVAL_SECONDS..=MAX_REFRESH_INTERVAL_SECONDS).contains(&refresh_interval) {
        anyhow::bail!(
            "REFRESH_INTERVAL must be between {MIN_REFRESH_INTERVAL_SECONDS} and {MAX_REFRESH_INTERVAL_SECONDS} seconds (inclusive)"
        );
    }

    Ok(Duration::from_secs(refresh_interval))
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

        refresh_interval_duration(worker_config.refresh_interval)?;

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
    fn worker_config_defaults_refresh_interval_to_300_seconds() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("CONCURRENT_FETCHES", Some("7")),
            ("MASTER_KEY_VERSION", Some("9")),
            ("REFRESH_INTERVAL", None),
        ]);

        let config = WorkerConfig::load().expect("worker config should load from environment");

        assert_eq!(config.concurrent_fetches, 7);
        assert_eq!(config.refresh_interval, 300);
        assert_eq!(config.master_key_version, 9);
        assert_eq!(
            config.worker_database_url,
            "postgres://worker:worker@localhost/worker"
        );
        assert_eq!(config.redis_url, "redis://localhost:6379/1");
    }

    #[test]
    fn worker_config_loads_explicit_refresh_interval_of_600_seconds() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("REFRESH_INTERVAL", Some("600")),
        ]);

        let config = WorkerConfig::load().expect("explicit refresh interval should load");

        assert_eq!(config.refresh_interval, 600);
    }

    #[test]
    fn worker_config_accepts_minimum_refresh_interval() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("REFRESH_INTERVAL", Some("300")),
        ]);

        let config = WorkerConfig::load().expect("minimum refresh interval should load");

        assert_eq!(config.refresh_interval, 300);
    }

    #[test]
    fn worker_config_accepts_maximum_refresh_interval() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("REFRESH_INTERVAL", Some("86400")),
        ]);

        let config = WorkerConfig::load().expect("maximum refresh interval should load");

        assert_eq!(config.refresh_interval, 86400);
    }

    #[test]
    fn worker_config_rejects_refresh_interval_below_minimum() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("REFRESH_INTERVAL", Some("299")),
        ]);

        assert!(WorkerConfig::load().is_err());
    }

    #[test]
    fn worker_config_rejects_refresh_interval_above_maximum() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("REFRESH_INTERVAL", Some("86401")),
        ]);

        assert!(WorkerConfig::load().is_err());
    }

    #[test]
    fn worker_config_rejects_invalid_refresh_interval_values() {
        let _env = EnvGuard::set(&[
            (
                "WORKER_DATABASE_URL",
                Some("postgres://worker:worker@localhost/worker"),
            ),
            ("REDIS_URL", Some("redis://localhost:6379/1")),
            ("MASTER_KEY", Some("base64-fixture")),
            ("REFRESH_INTERVAL", Some("not-a-number")),
        ]);

        assert!(WorkerConfig::load().is_err());
    }
}
