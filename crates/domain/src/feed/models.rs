//! Feed domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Feed type detected from source content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeedType {
    Rss,
    Atom,
    JsonFeed,
}

impl std::fmt::Display for FeedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedType::Rss => write!(f, "rss"),
            FeedType::Atom => write!(f, "atom"),
            FeedType::JsonFeed => write!(f, "json"),
        }
    }
}

/// RSS/Atom/JSON Feed metadata after normalization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub site_url: Option<String>,
    pub feed_type: FeedType,
    pub icon_url: Option<String>,
    pub last_fetched_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub error_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Normalized item/entry extracted from a feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    pub guid: String,
    pub title: String,
    pub url: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub categories: Vec<String>,
    pub enclosure_url: Option<String>,
    pub enclosure_type: Option<String>,
}

impl Feed {
    /// Create a new feed with default timestamps and counters.
    pub fn new(url: String, title: String, feed_type: FeedType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            url,
            title,
            description: None,
            site_url: None,
            feed_type,
            icon_url: None,
            last_fetched_at: None,
            last_error: None,
            error_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

impl FeedItem {
    /// Generate a stable GUID from title and optional URL when a feed omits it.
    pub fn generate_guid(title: &str, url: Option<&str>) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(title.as_bytes());
        if let Some(u) = url {
            hasher.update(u.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_new() {
        let feed = Feed::new(
            "https://example.com/feed.xml".to_string(),
            "Example Feed".to_string(),
            FeedType::Rss,
        );

        assert_eq!(feed.title, "Example Feed");
        assert_eq!(feed.feed_type, FeedType::Rss);
        assert!(feed.last_fetched_at.is_none());
    }

    #[test]
    fn test_generate_guid() {
        let guid1 = FeedItem::generate_guid("Title", Some("https://example.com"));
        let guid2 = FeedItem::generate_guid("Title", Some("https://example.com"));
        let guid3 = FeedItem::generate_guid("Different", Some("https://example.com"));

        assert_eq!(guid1, guid2);
        assert_ne!(guid1, guid3);
    }
}
