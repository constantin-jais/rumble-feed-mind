//! Article domain models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Read status for an article
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReadStatus {
    #[default]
    Unread,
    Read,
    Skipped,
}

/// Represents a stored article
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Unique identifier
    pub id: Uuid,
    /// Feed ID
    pub feed_id: Uuid,
    /// Original GUID from feed
    pub guid: String,
    /// Article title
    pub title: String,
    /// Article URL
    pub url: Option<String>,
    /// Full content (HTML)
    pub content: Option<String>,
    /// Summary/excerpt
    pub summary: Option<String>,
    /// Author name
    pub author: Option<String>,
    /// Published date
    pub published_at: Option<DateTime<Utc>>,
    /// Categories from feed
    pub categories: Vec<String>,
    /// Enclosure URL (media)
    pub enclosure_url: Option<String>,
    /// Enclosure MIME type
    pub enclosure_type: Option<String>,
    /// When article was fetched
    pub fetched_at: DateTime<Utc>,
    /// Created at
    pub created_at: DateTime<Utc>,
}

/// User-specific article state (read, starred, tags, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleState {
    /// User ID
    pub user_id: Uuid,
    /// Article ID
    pub article_id: Uuid,
    /// Read status
    pub read_status: ReadStatus,
    /// Starred/favorited
    pub starred: bool,
    /// Hidden by rule
    pub hidden: bool,
    /// Hidden reason (rule name or AI explanation)
    pub hidden_reason: Option<String>,
    /// User-assigned tags
    pub tags: Vec<String>,
    /// Read at timestamp
    pub read_at: Option<DateTime<Utc>>,
    /// Starred at timestamp
    pub starred_at: Option<DateTime<Utc>>,
    /// Hidden at timestamp
    pub hidden_at: Option<DateTime<Utc>>,
}

impl Article {
    /// Create a new article from feed item
    pub fn from_feed_item(feed_id: Uuid, item: crate::feed::FeedItem) -> Self {
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
    /// Create a new default state for an article
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

    /// Mark as read
    pub fn mark_read(&mut self) {
        self.read_status = ReadStatus::Read;
        self.read_at = Some(Utc::now());
    }

    /// Mark as unread
    pub fn mark_unread(&mut self) {
        self.read_status = ReadStatus::Unread;
        self.read_at = None;
    }

    /// Toggle starred
    pub fn toggle_starred(&mut self) {
        self.starred = !self.starred;
        self.starred_at = if self.starred { Some(Utc::now()) } else { None };
    }

    /// Hide with reason
    pub fn hide(&mut self, reason: String) {
        self.hidden = true;
        self.hidden_reason = Some(reason);
        self.hidden_at = Some(Utc::now());
    }

    /// Restore (unhide)
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
