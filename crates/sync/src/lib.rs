//! Sync primitives for replay, snapshots, and future offline replication.
//!
//! This crate is intentionally storage-agnostic. It defines event batches and
//! snapshots that can be persisted by `feedmind-storage` implementations.

pub mod curated;
pub mod local;

use chrono::{DateTime, Utc};
use feedmind_domain::{Article, Feed, FeedMindEvent, Rule};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Monotonic cursor used by sync clients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncCursor {
    pub last_event_id: Option<Uuid>,
    pub observed_at: DateTime<Utc>,
}

impl SyncCursor {
    pub fn empty() -> Self {
        Self {
            last_event_id: None,
            observed_at: Utc::now(),
        }
    }

    pub fn after(event: &FeedMindEvent) -> Self {
        Self {
            last_event_id: Some(event.id),
            observed_at: event.occurred_at,
        }
    }
}

/// Ordered batch of domain events for replay.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventBatch {
    pub user_id: Uuid,
    pub cursor_before: SyncCursor,
    pub cursor_after: SyncCursor,
    pub events: Vec<FeedMindEvent>,
}

impl EventBatch {
    pub fn new(user_id: Uuid, cursor_before: SyncCursor, events: Vec<FeedMindEvent>) -> Self {
        let cursor_after = events
            .last()
            .map(SyncCursor::after)
            .unwrap_or_else(|| cursor_before.clone());

        Self {
            user_id,
            cursor_before,
            cursor_after,
            events,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// Portable user snapshot for export/import and first sync.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSnapshot {
    pub user_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub feeds: Vec<Feed>,
    pub articles: Vec<Article>,
    pub rules: Vec<Rule>,
    pub events: Vec<FeedMindEvent>,
}

impl UserSnapshot {
    pub fn empty(user_id: Uuid) -> Self {
        Self {
            user_id,
            generated_at: Utc::now(),
            feeds: Vec::new(),
            articles: Vec::new(),
            rules: Vec::new(),
            events: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feedmind_domain::FeedMindEventPayload;

    #[test]
    fn batch_cursor_advances_to_last_event() {
        let user_id = Uuid::new_v4();
        let first = FeedMindEvent::new(
            user_id,
            FeedMindEventPayload::FeedAdded {
                feed_id: Uuid::new_v4(),
                url: "https://example.com/feed.xml".to_string(),
            },
        );
        let second = FeedMindEvent::new(
            user_id,
            FeedMindEventPayload::FeedFetched {
                feed_id: Uuid::new_v4(),
                item_count: 2,
            },
        );

        let batch = EventBatch::new(user_id, SyncCursor::empty(), vec![first, second.clone()]);

        assert_eq!(batch.cursor_after.last_event_id, Some(second.id));
        assert!(!batch.is_empty());
    }

    #[test]
    fn empty_snapshot_has_identity() {
        let user_id = Uuid::new_v4();
        let snapshot = UserSnapshot::empty(user_id);

        assert_eq!(snapshot.user_id, user_id);
        assert!(snapshot.feeds.is_empty());
    }
}
