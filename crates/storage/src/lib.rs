//! Storage ports for FeedMind.
//!
//! This crate defines persistence boundaries only. Concrete PostgreSQL, SQLite,
//! or in-memory adapters must live behind these traits and must not leak into
//! domain, ingest, rules, or sync crates.

use std::future::Future;

use feedmind_domain::{Article, Feed, FeedMindEvent, Rule};
use feedmind_sync::{EventBatch, SyncCursor, UserSnapshot};
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
