//! Storage ports for FeedMind.
//!
//! This crate defines persistence boundaries only. Concrete PostgreSQL, SQLite,
//! or in-memory adapters must live behind these traits and must not leak into
//! domain, ingest, rules, or sync crates.

use std::future::Future;
use std::ops::{Deref, DerefMut};

use feedmind_domain::{Article, Feed, FeedMindEvent, Rule};
use feedmind_sync::{EventBatch, SyncCursor, UserSnapshot};
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("backend error: {0}")]
    Backend(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;

/// PostgreSQL transaction carrying one transaction-local tenant identity.
///
/// The identity is installed with `set_config(..., true)`, PostgreSQL's
/// transaction-local form. Dropping or committing the transaction therefore
/// cannot leave tenant state on a pooled connection.
pub struct TenantTransaction {
    inner: Transaction<'static, Postgres>,
    tenant_id: Uuid,
}

impl Deref for TenantTransaction {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TenantTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TenantTransaction {
    /// Begin a transaction and install its tenant identity before any product query.
    pub async fn begin(pool: &PgPool, tenant_id: Uuid) -> sqlx::Result<Self> {
        let mut inner = pool.begin().await?;
        let configured: String = sqlx::query_scalar("SELECT set_config('app.user_id', $1, true)")
            .bind(tenant_id.to_string())
            .fetch_one(&mut *inner)
            .await?;

        if configured != tenant_id.to_string() {
            return Err(sqlx::Error::Protocol(
                "PostgreSQL did not install the requested transaction-local tenant context"
                    .to_string(),
            ));
        }

        Ok(Self { inner, tenant_id })
    }

    /// Tenant identity bound to this transaction.
    pub fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }

    /// SQLx executor for queries that belong to this tenant transaction.
    pub fn connection(&mut self) -> &mut PgConnection {
        &mut self.inner
    }

    /// Commit all work and clear transaction-local tenant state.
    pub async fn commit(self) -> sqlx::Result<()> {
        self.inner.commit().await
    }

    /// Roll back all work and clear transaction-local tenant state.
    pub async fn rollback(self) -> sqlx::Result<()> {
        self.inner.rollback().await
    }
}

/// Feed persistence port.
pub trait FeedStore: Send + Sync {
    fn get_feed(
        &self,
        user_id: Uuid,
        feed_id: Uuid,
    ) -> impl Future<Output = Result<Option<Feed>>> + Send;

    fn list_feeds(&self, user_id: Uuid) -> impl Future<Output = Result<Vec<Feed>>> + Send;

    fn save_feed(&self, user_id: Uuid, feed: Feed) -> impl Future<Output = Result<Feed>> + Send;
}

/// Article persistence port.
pub trait ArticleStore: Send + Sync {
    fn get_article(
        &self,
        user_id: Uuid,
        article_id: Uuid,
    ) -> impl Future<Output = Result<Option<Article>>> + Send;

    fn list_articles_for_feed(
        &self,
        user_id: Uuid,
        feed_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Article>>> + Send;

    fn save_article(
        &self,
        user_id: Uuid,
        article: Article,
    ) -> impl Future<Output = Result<Article>> + Send;
}

/// Rule persistence port.
pub trait RuleStore: Send + Sync {
    fn list_rules(&self, user_id: Uuid) -> impl Future<Output = Result<Vec<Rule>>> + Send;

    fn save_rule(&self, user_id: Uuid, rule: Rule) -> impl Future<Output = Result<Rule>> + Send;
}

/// Event persistence port for replay and sync.
pub trait EventStore: Send + Sync {
    fn append_event(
        &self,
        user_id: Uuid,
        event: FeedMindEvent,
    ) -> impl Future<Output = Result<FeedMindEvent>> + Send;

    fn events_after(
        &self,
        user_id: Uuid,
        cursor: SyncCursor,
        limit: usize,
    ) -> impl Future<Output = Result<EventBatch>> + Send;
}

/// Snapshot persistence/export port.
pub trait SnapshotStore: Send + Sync {
    fn export_snapshot(&self, user_id: Uuid) -> impl Future<Output = Result<UserSnapshot>> + Send;

    fn import_snapshot(
        &self,
        snapshot: UserSnapshot,
    ) -> impl Future<Output = Result<UserSnapshot>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_error_is_displayable() {
        let err = StorageError::NotFound("feed".to_string());
        assert_eq!(err.to_string(), "not found: feed");
    }
}
