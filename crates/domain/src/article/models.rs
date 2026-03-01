//! Article domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::feed::FeedItem;

/// Read status for an article.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReadStatus {
    #[default]
    Unread,
    Read,
    Skipped,
}

/// Stored normalized article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub guid: String,
    pub title: String,
    pub url: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub categories: Vec<String>,
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
    pub fetched_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// User-specific article state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleState {
    pub user_id: Uuid,
    pub article_id: Uuid,
    pub read_status: ReadStatus,
    pub starred: bool,
    pub hidden: bool,
    pub hidden_reason: Option<String>,
    pub tags: Vec<String>,
    pub read_at: Option<DateTime<Utc>>,
    pub starred_at: Option<DateTime<Utc>>,
    pub hidden_at: Option<DateTime<Utc>>,
}

impl Article {
    /// Create a new article from a normalized feed item.
    pub fn from_feed_item(feed_id: Uuid, item: FeedItem) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            feed_id,
            guid: item.guid,
            title: item.title,
            url: item.url,
            content: item.content,
            summary: item.summary,
            author: item.author,
            published_at: item.published_at,
            categories: item.categories,
            enclosure_url: item.enclosure_url,
            enclosure_type: item.enclosure_type,
            fetched_at: now,
            created_at: now,
        }
    }
}

impl ArticleState {
    /// Create a new default state for an article.
    pub fn new(user_id: Uuid, article_id: Uuid) -> Self {
        Self {
            user_id,
            article_id,
            read_status: ReadStatus::Unread,
            starred: false,
            hidden: false,
            hidden_reason: None,
            tags: Vec::new(),
            read_at: None,
            starred_at: None,
            hidden_at: None,
        }
    }

    pub fn mark_read(&mut self) {
        self.read_status = ReadStatus::Read;
        self.read_at = Some(Utc::now());
    }

    pub fn mark_unread(&mut self) {
        self.read_status = ReadStatus::Unread;
        self.read_at = None;
    }

    pub fn toggle_starred(&mut self) {
        self.starred = !self.starred;
        self.starred_at = if self.starred { Some(Utc::now()) } else { None };
    }

    pub fn hide(&mut self, reason: String) {
        self.hidden = true;
        self.hidden_reason = Some(reason);
        self.hidden_at = Some(Utc::now());
    }

    pub fn restore(&mut self) {
        self.hidden = false;
        self.hidden_reason = None;
        self.hidden_at = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_state_mark_read() {
        let mut state = ArticleState::new(Uuid::new_v4(), Uuid::new_v4());
        assert_eq!(state.read_status, ReadStatus::Unread);

        state.mark_read();
        assert_eq!(state.read_status, ReadStatus::Read);
        assert!(state.read_at.is_some());

        state.mark_unread();
        assert_eq!(state.read_status, ReadStatus::Unread);
        assert!(state.read_at.is_none());
    }

    #[test]
    fn test_article_state_hide_restore() {
        let mut state = ArticleState::new(Uuid::new_v4(), Uuid::new_v4());

        state.hide("Matched rule: clickbait".to_string());
        assert!(state.hidden);
        assert_eq!(
            state.hidden_reason,
            Some("Matched rule: clickbait".to_string())
        );

        state.restore();
        assert!(!state.hidden);
        assert!(state.hidden_reason.is_none());
    }
}
