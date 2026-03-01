//! Job definitions for background processing

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A background job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// Unique job identifier
    pub id: Uuid,
    /// Type of job with associated data
    pub job_type: JobType,
    /// Number of retry attempts
    pub attempts: u32,
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// When the job was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Job {
    /// Create a new job
    pub fn new(job_type: JobType) -> Self {
        Self {
            id: Uuid::new_v4(),
            job_type,
            attempts: 0,
            max_attempts: 3,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create a job with custom max attempts
    pub fn with_max_attempts(job_type: JobType, max_attempts: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            job_type,
            attempts: 0,
            max_attempts,
            created_at: chrono::Utc::now(),
        }
    }
}

/// Types of background jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum JobType {
    /// Fetch and parse a single feed
    FetchFeed { feed_id: Uuid },

    /// Evaluate rules for a batch of articles
    EvaluateRules { article_ids: Vec<Uuid> },

    /// Refresh all feeds that need updating (scheduled job)
    RefreshAllFeeds,

    /// Clean up old read articles (scheduled job)
    CleanupOldArticles {
        /// Days to keep read articles
        retention_days: u32,
    },

    /// Export user data (GDPR compliance)
    ExportUserData { user_id: Uuid },

    /// Delete user data (GDPR compliance)
    DeleteUserData { user_id: Uuid },

    /// Send email notification
    SendEmail { to: String, template: EmailTemplate },

    // ========================================================================
    // BILLING JOBS
    // ========================================================================
    /// Check dunning status for all accounts in grace period (scheduled job)
    /// Runs daily to downgrade or suspend accounts
    CheckDunningStatus,

    /// Sync usage records to Stripe for metered billing
    SyncUsageToStripe { user_id: Uuid },

    /// Clean up old webhook events (> 30 days)
    CleanupWebhookEvents,
}

/// Email templates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "template", content = "data")]
pub enum EmailTemplate {
    /// Welcome email for new users
    Welcome { username: String },

    /// Weekly digest
    WeeklyDigest { user_id: Uuid, article_count: u32 },

    /// Export ready notification
    ExportReady { download_url: String },

    // ========================================================================
    // DUNNING EMAIL TEMPLATES
    // ========================================================================
    /// Payment failed - first notification (day 1)
    PaymentFailedDay1 {
        user_id: Uuid,
        amount: i64,
        currency: String,
    },

    /// Payment failed - reminder (day 3)
    PaymentFailedDay3 {
        user_id: Uuid,
        amount: i64,
        currency: String,
    },

    /// Payment failed - final warning before downgrade (day 7)
    PaymentFailedDay7 {
        user_id: Uuid,
        amount: i64,
        currency: String,
    },

    /// Account downgraded to free due to non-payment
    AccountDowngraded {
        user_id: Uuid,
        previous_plan: String,
    },

    /// Account suspended due to extended non-payment (day 30)
    AccountSuspended { user_id: Uuid },

    /// Payment recovered - account restored
    PaymentRecovered { user_id: Uuid, plan: String },
}

/// Job priority levels
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JobPriority {
    /// Low priority (cleanup, exports)
    Low = 0,
    /// Normal priority (feed fetching)
    #[default]
    Normal = 1,
    /// High priority (rule evaluation, user-triggered actions)
    High = 2,
}

impl JobType {
    /// Get the default priority for this job type
    pub fn default_priority(&self) -> JobPriority {
        match self {
            JobType::FetchFeed { .. } => JobPriority::Normal,
            JobType::EvaluateRules { .. } => JobPriority::High,
            JobType::RefreshAllFeeds => JobPriority::Low,
            JobType::CleanupOldArticles { .. } => JobPriority::Low,
            JobType::ExportUserData { .. } => JobPriority::Normal,
            JobType::DeleteUserData { .. } => JobPriority::High,
            JobType::SendEmail { .. } => JobPriority::Normal,
            JobType::CheckDunningStatus => JobPriority::Normal,
            JobType::SyncUsageToStripe { .. } => JobPriority::Low,
            JobType::CleanupWebhookEvents => JobPriority::Low,
        }
    }

    /// Get the queue name for this job type
    pub fn queue_name(&self) -> &'static str {
        match self {
            JobType::FetchFeed { .. } => "feedmind:jobs:feeds",
            JobType::EvaluateRules { .. } => "feedmind:jobs:rules",
            JobType::RefreshAllFeeds => "feedmind:jobs:scheduled",
            JobType::CleanupOldArticles { .. } => "feedmind:jobs:scheduled",
            JobType::ExportUserData { .. } => "feedmind:jobs:exports",
            JobType::DeleteUserData { .. } => "feedmind:jobs:exports",
            JobType::SendEmail { .. } => "feedmind:jobs:notifications",
            JobType::CheckDunningStatus => "feedmind:jobs:billing",
            JobType::SyncUsageToStripe { .. } => "feedmind:jobs:billing",
            JobType::CleanupWebhookEvents => "feedmind:jobs:scheduled",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_creation() {
        let job = Job::new(JobType::FetchFeed {
            feed_id: Uuid::new_v4(),
        });

        assert_eq!(job.attempts, 0);
        assert_eq!(job.max_attempts, 3);
    }

    #[test]
    fn test_job_serialization() {
        let job = Job::new(JobType::EvaluateRules {
            article_ids: vec![Uuid::new_v4(), Uuid::new_v4()],
        });

        let json = serde_json::to_string(&job).unwrap();
        let parsed: Job = serde_json::from_str(&json).unwrap();

        assert_eq!(job.id, parsed.id);
    }

    #[test]
    fn test_job_priority() {
        assert!(JobPriority::High > JobPriority::Normal);
        assert!(JobPriority::Normal > JobPriority::Low);
    }
}
