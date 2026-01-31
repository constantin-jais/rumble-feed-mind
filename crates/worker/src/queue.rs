//! Redis-based job queue consumer

use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{error, info};

use crate::config::WorkerConfig;
use crate::jobs::{Job, JobType};

/// Queue consumer for processing background jobs
pub struct QueueConsumer {
    redis: ConnectionManager,
    db: PgPool,
    concurrent_fetches: usize,
}

impl QueueConsumer {
    /// Create a new queue consumer
    pub async fn new(config: &WorkerConfig) -> anyhow::Result<Self> {
        // Connect to PostgreSQL
        let db = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await?;

        // Connect to Redis
        let redis_client = redis::Client::open(config.redis_url.as_str())?;
        let redis = ConnectionManager::new(redis_client).await?;

        Ok(Self {
            redis,
            db,
            concurrent_fetches: config.concurrent_fetches,
        })
    }

    /// Run the consumer loop
    pub async fn run(&mut self) -> anyhow::Result<()> {
        info!("Queue consumer running");

        loop {
            match self.process_next_job().await {
                Ok(Some(job)) => {
                    info!(job_type = ?job.job_type, job_id = %job.id, "Processed job");
                }
                Ok(None) => {
                    // No jobs, wait a bit
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                Err(e) => {
                    error!(error = %e, "Error processing job");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Process the next job from the queue
    async fn process_next_job(&mut self) -> anyhow::Result<Option<Job>> {
        // Pop job from Redis queue
        let job_data: Option<String> = redis::cmd("LPOP")
            .arg("feedmind:jobs")
            .query_async(&mut self.redis)
            .await?;

        let Some(data) = job_data else {
            return Ok(None);
        };

        let job: Job = serde_json::from_str(&data)?;

        // Process based on job type
        match &job.job_type {
            JobType::FetchFeed { feed_id } => {
                self.process_fetch_feed(*feed_id).await?;
            }
            JobType::EvaluateRules { article_ids } => {
                self.process_evaluate_rules(article_ids).await?;
            }
            JobType::RefreshAllFeeds => {
                self.process_refresh_all().await?;
            }
            JobType::CleanupOldArticles { retention_days } => {
                self.process_cleanup(*retention_days).await?;
            }
            JobType::ExportUserData { user_id } => {
                self.process_export_user_data(*user_id).await?;
            }
            JobType::DeleteUserData { user_id } => {
                self.process_delete_user_data(*user_id).await?;
            }
            JobType::SendEmail { to, template } => {
                self.process_send_email(to, template).await?;
            }
        }

        Ok(Some(job))
    }

    /// Fetch a single feed
    async fn process_fetch_feed(&self, feed_id: uuid::Uuid) -> anyhow::Result<()> {
        info!(%feed_id, "Fetching feed");

        // TODO: Implement
        // 1. Get feed URL from database
        // 2. Fetch and parse feed
        // 3. Deduplicate articles
        // 4. Insert new articles
        // 5. Queue rule evaluation for new articles
        // 6. Update feed last_fetched_at

        Ok(())
    }

    /// Evaluate rules for articles
    async fn process_evaluate_rules(&self, article_ids: &[uuid::Uuid]) -> anyhow::Result<()> {
        info!(count = article_ids.len(), "Evaluating rules for articles");

        // TODO: Implement
        // 1. Get user rules
        // 2. Get articles
        // 3. Evaluate each rule against each article
        // 4. Apply actions (hide, tag, etc.)
        // 5. Store evaluation results for explainability

        Ok(())
    }

    /// Refresh all feeds that need updating
    async fn process_refresh_all(&self) -> anyhow::Result<()> {
        info!("Refreshing all feeds");

        // TODO: Implement
        // 1. Get feeds that need refresh (based on smart polling)
        // 2. Queue FetchFeed jobs for each
        // 3. Respect concurrent_fetches limit

        let _ = self.concurrent_fetches; // Use field to suppress warning

        Ok(())
    }

    /// Clean up old articles
    async fn process_cleanup(&self, retention_days: u32) -> anyhow::Result<()> {
        info!(retention_days, "Cleaning up old articles");

        // TODO: Implement
        // 1. Delete read articles older than retention_days
        // 2. Delete orphaned data (tags, evaluations)

        Ok(())
    }

    /// Export user data (GDPR)
    async fn process_export_user_data(&self, user_id: uuid::Uuid) -> anyhow::Result<()> {
        info!(%user_id, "Exporting user data");

        // TODO: Implement
        // 1. Gather all user data (feeds, articles, rules, settings)
        // 2. Create ZIP archive
        // 3. Upload to temporary storage
        // 4. Send notification email with download link

        Ok(())
    }

    /// Delete user data (GDPR)
    async fn process_delete_user_data(&self, user_id: uuid::Uuid) -> anyhow::Result<()> {
        info!(%user_id, "Deleting user data");

        // TODO: Implement
        // 1. Delete all user data
        // 2. Anonymize logs
        // 3. Send confirmation email

        Ok(())
    }

    /// Send email notification
    async fn process_send_email(
        &self,
        to: &str,
        template: &crate::jobs::EmailTemplate,
    ) -> anyhow::Result<()> {
        info!(to, ?template, "Sending email");

        // TODO: Implement with Resend or similar provider
        // 1. Render template
        // 2. Send via email provider

        Ok(())
    }
}

/// Push a job to the queue
pub async fn enqueue_job(redis: &mut ConnectionManager, job: &Job) -> anyhow::Result<()> {
    let data = serde_json::to_string(job)?;
    let _: () = redis::cmd("RPUSH")
        .arg("feedmind:jobs")
        .arg(data)
        .query_async(redis)
        .await?;
    Ok(())
}
