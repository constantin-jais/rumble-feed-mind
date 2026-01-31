//! OPML parser

use super::models::{OpmlDocument, OpmlOutline};
use crate::error::{Error, Result};
use scraper::{Html, Selector};
use tracing::warn;

/// OPML parser with permissive handling of malformed files
pub struct OpmlParser;

impl OpmlParser {
    /// Parse OPML content from string
    pub fn parse(content: &str) -> Result<OpmlDocument> {
        let document = Html::parse_document(content);

        // Extract title from head
        let title_selector = Selector::parse("head > title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>());

        // Extract date created
        let date_selector = Selector::parse("head > dateCreated").unwrap();
        let date_created = document
            .select(&date_selector)
            .next()
            .map(|e| e.text().collect::<String>());

        // Parse body outlines
        let body_selector = Selector::parse("body").unwrap();
        let outline_selector = Selector::parse(":scope > outline").unwrap();

        let outlines: Vec<OpmlOutline> = document
            .select(&body_selector)
            .next()
            .map(|body| {
                body.select(&outline_selector)
                    .filter_map(|e| Self::parse_outline(&e))
                    .collect()
            })
            .unwrap_or_default();

        if outlines.is_empty() {
            // Try alternative structure (some OPML files don't have proper body)
            let alt_selector = Selector::parse("outline").unwrap();
            let alt_outlines: Vec<OpmlOutline> = document
                .select(&alt_selector)
                .filter_map(|e| {
                    // Only parse top-level outlines
                    if e.parent()
                        .map(|p| {
                            p.value()
                                .as_element()
                                .map(|el| el.name() == "body")
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
                    {
                        Self::parse_outline(&e)
                    } else {
                        None
                    }
                })
                .collect();

            if alt_outlines.is_empty() {
                return Err(Error::OpmlParse("No outlines found in OPML".to_string()));
            }

            return Ok(OpmlDocument {
                title,
                date_created,
                owner_email: None,
                outlines: alt_outlines,
            });
        }

        Ok(OpmlDocument {
            title,
            date_created,
            owner_email: None,
            outlines,
        })
    }

    /// Parse a single outline element recursively
    fn parse_outline(element: &scraper::ElementRef) -> Option<OpmlOutline> {
        let attrs = element.value();

        // Get text attribute (required) - try multiple attribute names
        let text = attrs
            .attr("text")
            .or_else(|| attrs.attr("title"))
            .or_else(|| attrs.attr("name"))?
            .to_string();

        let title = attrs.attr("title").map(|s| s.to_string());
        let outline_type = attrs.attr("type").map(|s| s.to_string());

        // Get feed URL - try multiple attribute names (case insensitive handling)
        let xml_url = attrs
            .attr("xmlUrl")
            .or_else(|| attrs.attr("xmlurl"))
            .or_else(|| attrs.attr("url"))
            .map(|s| s.to_string());

        let html_url = attrs
            .attr("htmlUrl")
            .or_else(|| attrs.attr("htmlurl"))
            .map(|s| s.to_string());

        let description = attrs.attr("description").map(|s| s.to_string());

        // Parse child outlines
        let outline_selector = Selector::parse(":scope > outline").unwrap();
        let children: Vec<OpmlOutline> = element
            .select(&outline_selector)
            .filter_map(|e| Self::parse_outline(&e))
            .collect();

        // Validate URL if it's a feed
        if let Some(ref url) = xml_url {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                warn!(url = %url, "Invalid feed URL in OPML, skipping");
                return None;
            }
        }

        Some(OpmlOutline {
            text,
            title,
            outline_type,
            xml_url,
            html_url,
            description,
            children,
        })
    }

    /// Parse OPML from bytes (handles encoding)
    pub fn parse_bytes(content: &[u8]) -> Result<OpmlDocument> {
        // Try UTF-8 first
        if let Ok(s) = std::str::from_utf8(content) {
            return Self::parse(s);
        }

        // Try ISO-8859-1
        let decoded: String = content.iter().map(|&b| b as char).collect();
        Self::parse(&decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_OPML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<opml version="2.0">
  <head>
    <title>My Feeds</title>
    <dateCreated>Mon, 27 Jan 2025 12:00:00 GMT</dateCreated>
  </head>
  <body>
    <outline text="Tech" title="Tech">
      <outline type="rss" text="Hacker News"
               xmlUrl="https://news.ycombinator.com/rss"
               htmlUrl="https://news.ycombinator.com"/>
      <outline type="rss" text="TechCrunch"
               xmlUrl="https://techcrunch.com/feed/"/>
    </outline>
    <outline type="rss" text="Example" xmlUrl="https://example.com/feed.xml"/>
  </body>
</opml>"#;

    #[test]
    fn test_parse_opml() {
        let doc = OpmlParser::parse(SAMPLE_OPML).expect("Failed to parse OPML");

        assert_eq!(doc.title, Some("My Feeds".to_string()));
        assert_eq!(doc.feed_count(), 3);
        assert_eq!(doc.folder_count(), 1);

        // Check folder
        let folder = &doc.outlines[0];
        assert_eq!(folder.text, "Tech");
        assert!(folder.is_folder());
        assert_eq!(folder.children.len(), 2);

        // Check feed in folder
        let hn = &folder.children[0];
        assert_eq!(hn.text, "Hacker News");
        assert!(hn.is_feed());
        assert_eq!(
            hn.xml_url,
            Some("https://news.ycombinator.com/rss".to_string())
        );

        // Check root-level feed
        let example = &doc.outlines[1];
        assert_eq!(example.text, "Example");
        assert!(example.is_feed());
    }

    #[test]
    fn test_parse_opml_case_insensitive() {
        let opml = r#"<?xml version="1.0"?>
<opml version="1.0">
  <body>
    <outline text="Test" xmlurl="https://example.com/feed"/>
  </body>
</opml>"#;

        let doc = OpmlParser::parse(opml).expect("Failed to parse");
        assert_eq!(doc.feed_count(), 1);
        assert_eq!(
            doc.outlines[0].xml_url,
            Some("https://example.com/feed".to_string())
        );
    }
}
