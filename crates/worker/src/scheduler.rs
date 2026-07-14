//! Job scheduler for periodic tasks

use redis::aio::ConnectionManager;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio_cron_scheduler::{Job as CronJob, JobScheduler};
use tracing::{error, info};

use crate::config::{refresh_interval_duration, WorkerConfig};
use crate::jobs::{Job, JobType};
use crate::queue::enqueue_job;

/// Scheduler for periodic background tasks
pub struct Scheduler {
    cron: JobScheduler,
    redis_url: String,
    refresh_interval: Duration,
    started: AtomicBool,
}

impl Scheduler {
    /// Create a new scheduler
    pub async fn new(config: &WorkerConfig) -> anyhow::Result<Self> {
        let refresh_interval = refresh_interval_duration(config.refresh_interval)?;
        let cron = JobScheduler::new().await?;

        Ok(Self {
            cron,
            redis_url: config.redis_url.clone(),
            refresh_interval,
            started: AtomicBool::new(false),
        })
    }

    /// Start the scheduler with all configured jobs
    pub async fn start(&self) -> anyhow::Result<()> {
        self.started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| {
                anyhow::anyhow!("scheduler already started; create a new Scheduler instance")
            })?;

        info!("Starting job scheduler");

        self.schedule_feed_refresh().await?;
        self.schedule_cleanup().await?;
        self.schedule_dunning_check().await?;
        self.schedule_webhook_cleanup().await?;

        self.cron.start().await?;

        info!("Job scheduler started");
        Ok(())
    }

    /// Schedule periodic feed refresh
    async fn schedule_feed_refresh(&self) -> anyhow::Result<()> {
        let redis_url = self.redis_url.clone();
        let refresh_interval = self.refresh_interval;
        let job = CronJob::new_repeated_async(refresh_interval, move |_uuid, _lock| {
            let redis_url = redis_url.clone();
            Box::pin(async move {
                if let Err(e) = enqueue_refresh_all(&redis_url).await {
                    error!(error = %e, "Failed to enqueue feed refresh");
                }
            })
        })?;

        self.cron.add(job).await?;
        info!(
            refresh_interval_seconds = self.refresh_interval.as_secs(),
            "Scheduled feed refresh"
        );

        Ok(())
    }

    /// Schedule daily cleanup of old articles
    async fn schedule_cleanup(&self) -> anyhow::Result<()> {
        let redis_url = self.redis_url.clone();

        // Every day at 3:00 AM
        let job = CronJob::new_async("0 0 3 * * *", move |_uuid, _lock| {
            let redis_url = redis_url.clone();
            Box::pin(async move {
                if let Err(e) = enqueue_cleanup(&redis_url, 90).await {
                    error!(error = %e, "Failed to enqueue cleanup");
                }
            })
        })?;

        self.cron.add(job).await?;
        info!("Scheduled cleanup daily at 3:00 AM");

        Ok(())
    }

    /// Schedule daily dunning check
    async fn schedule_dunning_check(&self) -> anyhow::Result<()> {
        let redis_url = self.redis_url.clone();

        // Every day at 6:00 AM
        let job = CronJob::new_async("0 0 6 * * *", move |_uuid, _lock| {
            let redis_url = redis_url.clone();
            Box::pin(async move {
                if let Err(e) = enqueue_dunning_check(&redis_url).await {
                    error!(error = %e, "Failed to enqueue dunning check");
                }
            })
        })?;

        self.cron.add(job).await?;
        info!("Scheduled dunning check daily at 6:00 AM");

        Ok(())
    }

    /// Schedule weekly webhook cleanup
    async fn schedule_webhook_cleanup(&self) -> anyhow::Result<()> {
        let redis_url = self.redis_url.clone();

        // Every Sunday at 4:00 AM
        let job = CronJob::new_async("0 0 4 * * 0", move |_uuid, _lock| {
            let redis_url = redis_url.clone();
            Box::pin(async move {
                if let Err(e) = enqueue_webhook_cleanup(&redis_url).await {
                    error!(error = %e, "Failed to enqueue webhook cleanup");
                }
            })
        })?;

        self.cron.add(job).await?;
        info!("Scheduled webhook cleanup weekly on Sunday at 4:00 AM");

        Ok(())
    }

    /// Shutdown the scheduler gracefully
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        info!("Shutting down scheduler");
        self.cron.shutdown().await?;
        Ok(())
    }
}

/// Enqueue a refresh all feeds job
async fn enqueue_refresh_all(redis_url: &str) -> anyhow::Result<()> {
    let mut redis = connect_redis(redis_url).await?;
    let job = Job::new(JobType::RefreshAllFeeds);
    enqueue_job(&mut redis, &job).await?;
    info!("Enqueued RefreshAllFeeds job");
    Ok(())
}

/// Enqueue a cleanup job
async fn enqueue_cleanup(redis_url: &str, retention_days: u32) -> anyhow::Result<()> {
    let mut redis = connect_redis(redis_url).await?;
    let job = Job::new(JobType::CleanupOldArticles { retention_days });
    enqueue_job(&mut redis, &job).await?;
    info!(retention_days, "Enqueued CleanupOldArticles job");
    Ok(())
}

/// Connect to Redis
async fn connect_redis(redis_url: &str) -> anyhow::Result<ConnectionManager> {
    let client = redis::Client::open(redis_url)?;
    let conn = ConnectionManager::new(client).await?;
    Ok(conn)
}

/// Enqueue a dunning check job
async fn enqueue_dunning_check(redis_url: &str) -> anyhow::Result<()> {
    let mut redis = connect_redis(redis_url).await?;
    let job = Job::new(JobType::CheckDunningStatus);
    enqueue_job(&mut redis, &job).await?;
    info!("Enqueued CheckDunningStatus job");
    Ok(())
}

/// Enqueue a webhook cleanup job
async fn enqueue_webhook_cleanup(redis_url: &str) -> anyhow::Result<()> {
    let mut redis = connect_redis(redis_url).await?;
    let job = Job::new(JobType::CleanupWebhookEvents);
    enqueue_job(&mut redis, &job).await?;
    info!("Enqueued CleanupWebhookEvents job");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    async fn scheduled_job_count(scheduler: &Scheduler) -> anyhow::Result<usize> {
        let metadata = scheduler.cron.context.metadata_storage.clone();
        let mut metadata = metadata.write().await;
        Ok(metadata.list_next_ticks().await?.len())
    }

    fn test_config(refresh_interval: u64) -> WorkerConfig {
        WorkerConfig {
            worker_database_url: "postgres://worker:worker@localhost/worker".to_string(),
            redis_url: "redis://localhost:6379/1".to_string(),
            concurrent_fetches: 50,
            refresh_interval,
            master_key: "base64-fixture".to_string(),
            master_key_version: 1,
        }
    }

    #[test]
    fn test_periodic_job_cron_expression_validity() {
        let job = CronJob::new_async("0 0 3 * * *", |_uuid, _lock| Box::pin(async move {}));
        assert!(job.is_ok(), "cleanup cron expression should parse");

        let job = CronJob::new_async("0 0 6 * * *", |_uuid, _lock| Box::pin(async move {}));
        assert!(job.is_ok(), "dunning cron expression should parse");

        let job = CronJob::new_async("0 0 4 * * 0", |_uuid, _lock| Box::pin(async move {}));
        assert!(job.is_ok(), "webhook cleanup cron expression should parse");
    }

    #[tokio::test]
    async fn scheduler_rejects_refresh_interval_below_minimum() {
        let config = test_config(299);

        assert!(Scheduler::new(&config).await.is_err());
    }

    #[tokio::test]
    async fn scheduler_rejects_refresh_interval_above_maximum() {
        let config = test_config(86_401);

        assert!(Scheduler::new(&config).await.is_err());
    }

    #[tokio::test]
    async fn scheduler_registers_feed_refresh_with_configured_interval() {
        let config = test_config(600);
        let scheduler = Scheduler::new(&config)
            .await
            .expect("scheduler should be created from worker config");

        let before = Utc::now();

        scheduler
            .schedule_feed_refresh()
            .await
            .expect("feed refresh should register");

        let metadata = scheduler.cron.context.metadata_storage.clone();
        let mut metadata = metadata.write().await;
        let jobs = metadata
            .list_next_ticks()
            .await
            .expect("next ticks should be readable");

        assert_eq!(jobs.len(), 1);

        let next_tick = Utc
            .timestamp_opt(jobs[0].next_tick as i64, 0)
            .single()
            .expect("next tick should be representable");
        let delta_seconds = next_tick.timestamp() - before.timestamp();
        assert!(
            (600..=601).contains(&delta_seconds),
            "expected next tick roughly 600 seconds in the future, got {delta_seconds}"
        );
    }

    #[tokio::test]
    async fn scheduler_start_registers_all_periodic_jobs_once() {
        let config = test_config(300);

        let mut scheduler = Scheduler::new(&config)
            .await
            .expect("scheduler should be created from worker config");

        assert_eq!(
            scheduled_job_count(&scheduler)
                .await
                .expect("jobs should be readable"),
            0
        );

        scheduler
            .start()
            .await
            .expect("first start should register all jobs");

        assert_eq!(
            scheduled_job_count(&scheduler)
                .await
                .expect("jobs should be readable after start"),
            4
        );

        let err = scheduler
            .start()
            .await
            .expect_err("second start should be rejected");
        assert!(
            err.to_string().contains("already started"),
            "unexpected scheduler start error: {err:#}"
        );

        assert_eq!(
            scheduled_job_count(&scheduler)
                .await
                .expect("jobs should remain registered after failed restart"),
            4
        );

        scheduler
            .shutdown()
            .await
            .expect("scheduler should shut down cleanly");
    }
}
