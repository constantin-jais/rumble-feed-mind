//! Feed parsing using feed-rs.

use chrono::Utc;
use feed_rs::parser;
use feedmind_domain::feed::{Feed, FeedItem, FeedType};
use uuid::Uuid;

use crate::error::{Error, Result};

/// Feed parser that handles RSS, Atom, and JSON Feed formats.
pub struct FeedParser;

impl FeedParser {
    /// Parse feed content from bytes.
    pub fn parse(content: &[u8], url: &str) -> Result<(Feed, Vec<FeedItem>)> {
        let parsed = parser::parse(content).map_err(|e| Error::FeedParse(e.to_string()))?;

        let feed_type = match parsed.feed_type {
            feed_rs::model::FeedType::Atom => FeedType::Atom,
            feed_rs::model::FeedType::JSON => FeedType::JsonFeed,
            _ => FeedType::Rss,
        };

        let title = parsed
            .title
            .map(|t| t.content)
            .unwrap_or_else(|| "Untitled Feed".to_string());

        let description = parsed.description.map(|d| d.content);
        let site_url = parsed.links.first().map(|l| l.href.clone());
        let icon_url = parsed
            .icon
            .map(|i| i.uri)
            .or_else(|| parsed.logo.map(|l| l.uri));

        let now = Utc::now();
        let feed = Feed {
            id: Uuid::new_v4(),
            url: url.to_string(),
            title,
            description,
            site_url,
            feed_type,
            icon_url,
            last_fetched_at: Some(now),
            last_error: None,
            error_count: 0,
            created_at: now,
            updated_at: now,
        };

        let items = parsed
            .entries
            .into_iter()
            .take(500)
            .map(Self::parse_entry)
            .collect();

        Ok((feed, items))
    }

    fn parse_entry(entry: feed_rs::model::Entry) -> FeedItem {
        let title = entry
            .title
            .map(|t| t.content)
            .unwrap_or_else(|| "Untitled".to_string());

        let title = if title.len() > 500 {
            format!("{}...", &title[..497])
        } else {
            title
        };

        let url = entry.links.first().map(|l| l.href.clone());

        let content = entry
            .content
            .and_then(|c| c.body)
            .or_else(|| entry.summary.clone().map(|s| s.content));

        let content = content.map(|c| {
            if c.len() > 100_000 {
                format!("{}...", &c[..99_997])
            } else {
                c
            }
        });

        let summary = entry.summary.map(|s| s.content);
        let author = entry.authors.first().map(|a| a.name.clone());
        let published_at = entry.published;
        let updated_at = entry.updated;
        let categories = entry.categories.into_iter().map(|c| c.term).collect();

        let (enclosure_url, enclosure_type) = entry
            .media
            .first()
            .and_then(|m| m.content.first())
            .map(|c| {
                (
                    c.url.clone().map(|u| u.to_string()),
                    c.content_type.clone().map(|t| t.to_string()),
                )
            })
            .unwrap_or((None, None));

        let guid = if entry.id.is_empty() {
            FeedItem::generate_guid(&title, url.as_deref())
        } else {
            entry.id
        };

        FeedItem {
            guid,
            title,
            url,
            content,
            summary,
            author,
            published_at,
            updated_at,
            categories,
            enclosure_url,
            enclosure_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RSS_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Test Feed</title>
    <link>https://example.com</link>
    <description>A test feed</description>
    <item>
      <title>Test Article</title>
      <link>https://example.com/article1</link>
      <description>Article content</description>
      <guid>article-1</guid>
    </item>
  </channel>
</rss>"#;

    #[test]
    fn test_parse_rss() {
        let (feed, items) =
            FeedParser::parse(RSS_SAMPLE.as_bytes(), "https://example.com/feed.xml")
                .expect("Failed to parse RSS");

        assert_eq!(feed.title, "Test Feed");
        assert_eq!(feed.feed_type, FeedType::Rss);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Test Article");
        assert_eq!(items[0].guid, "article-1");
    }

    const ATOM_SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Test Atom Feed</title>
  <link href="https://example.com"/>
  <entry>
    <title>Atom Article</title>
    <id>atom-entry-1</id>
    <link href="https://example.com/atom1"/>
    <summary>Atom summary</summary>
  </entry>
</feed>"#;

    #[test]
    fn test_parse_atom() {
        let (feed, items) =
            FeedParser::parse(ATOM_SAMPLE.as_bytes(), "https://example.com/atom.xml")
                .expect("Failed to parse Atom");

        assert_eq!(feed.title, "Test Atom Feed");
        assert_eq!(feed.feed_type, FeedType::Atom);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Atom Article");
    }
}
