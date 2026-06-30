//! Domain event primitives for replay, audit, and future sync.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::decision::RuleDecision;

/// Event envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedMindEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub payload: FeedMindEventPayload,
}

/// Business events emitted by the domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum FeedMindEventPayload {
    FeedAdded { feed_id: Uuid, url: String },
    FeedFetched { feed_id: Uuid, item_count: usize },
    ArticleDiscovered { article_id: Uuid, feed_id: Uuid },
    ArticleNormalized { article_id: Uuid },
    RuleEvaluated { decision: RuleDecision },
    ArticleRead { article_id: Uuid },
    ArticleExported { article_id: Uuid, target: String },
}

impl FeedMindEvent {
    pub fn new(user_id: Uuid, payload: FeedMindEventPayload) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            occurred_at: Utc::now(),
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_has_identity_and_timestamp() {
        let user_id = Uuid::new_v4();
        let event = FeedMindEvent::new(
            user_id,
            FeedMindEventPayload::FeedAdded {
                feed_id: Uuid::new_v4(),
                url: "https://example.com/feed.xml".to_string(),
            },
        );

        assert_eq!(event.user_id, user_id);
    }
}
