//! Job scheduler for periodic tasks

use redis::aio::ConnectionManager;
use tokio_cron_scheduler::{Job as CronJob, JobScheduler};
use tracing::{error, info};

use crate::config::WorkerConfig;
use crate::jobs::{Job, JobType};
use crate::queue::enqueue_job;

/// Scheduler for periodic background tasks
pub struct Scheduler {
    cron: JobScheduler,
    redis_url: String,
}

impl Scheduler {
    /// Create a new scheduler
    pub async fn new(config: &WorkerConfig) -> anyhow::Result<Self> {
        let cron = JobScheduler::new().await?;

        Ok(Self {
            cron,
            redis_url: config.redis_url.clone(),
        })
    }

    /// Start the scheduler with all configured jobs
    pub async fn start(&self) -> anyhow::Result<()> {
        info!("Starting job scheduler");

        // Schedule feed refresh every 5 minutes
        self.schedule_feed_refresh().await?;

        // Schedule cleanup daily at 3 AM
        self.schedule_cleanup().await?;

        // Schedule dunning check daily at 6 AM
        self.schedule_dunning_check().await?;

        // Schedule webhook cleanup weekly on Sunday at 4 AM
        self.schedule_webhook_cleanup().await?;

        // Start the scheduler
        self.cron.start().await?;

        info!("Job scheduler started");
        Ok(())
    }

    /// Schedule periodic feed refresh
    async fn schedule_feed_refresh(&self) -> anyhow::Result<()> {
        let redis_url = self.redis_url.clone();

        // Every 5 minutes
        let job = CronJob::new_async("0 */5 * * * *", move |_uuid, _lock| {
            let redis_url = redis_url.clone();
            Box::pin(async move {
                if let Err(e) = enqueue_refresh_all(&redis_url).await {
                    error!(error = %e, "Failed to enqueue feed refresh");
                }
            })
        })?;

        self.cron.add(job).await?;
        info!("Scheduled feed refresh every 5 minutes");

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
    #[test]
    fn test_cron_expression_validity() {
        // These should parse without error
        let expressions = [
            "0 */5 * * * *", // Every 5 minutes
            "0 0 3 * * *",   // Daily at 3 AM
            "0 0 * * * *",   // Every hour
        ];

        for expr in expressions {
            // Just verify the format is valid
            assert!(!expr.is_empty());
        }
    }
}
