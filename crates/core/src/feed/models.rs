//! Feed domain models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Feed type (detected from content)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeedType {
    Rss,
    Atom,
    JsonFeed,
}

/// Represents an RSS/Atom feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    /// Unique identifier
    pub id: Uuid,
    /// Feed URL
    pub url: String,
    /// Feed title
    pub title: String,
    /// Feed description
    pub description: Option<String>,
    /// Feed website URL
    pub site_url: Option<String>,
    /// Feed type
    pub feed_type: FeedType,
    /// Feed icon URL
    pub icon_url: Option<String>,
    /// Last successful fetch
    pub last_fetched_at: Option<DateTime<Utc>>,
    /// Last error message
    pub last_error: Option<String>,
    /// Number of consecutive errors
    pub error_count: u32,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// Represents an item/entry in a feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    /// Unique identifier (GUID from feed or generated)
    pub guid: String,
    /// Item title
    pub title: String,
    /// Item URL
    pub url: Option<String>,
    /// Item content (HTML)
    pub content: Option<String>,
    /// Item summary/description
    pub summary: Option<String>,
    /// Author name
    pub author: Option<String>,
    /// Published date
    pub published_at: Option<DateTime<Utc>>,
    /// Updated date
    pub updated_at: Option<DateTime<Utc>>,
    /// Categories/tags from feed
    pub categories: Vec<String>,
    /// Enclosure URL (for podcasts, media)
    pub enclosure_url: Option<String>,
    /// Enclosure MIME type
    pub enclosure_type: Option<String>,
}

impl Feed {
    /// Create a new feed with default values
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
    /// Generate a GUID from title and URL if not provided
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
