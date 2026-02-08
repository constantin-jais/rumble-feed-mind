//! OPML parser using quick-xml for proper XML handling

use super::models::{OpmlDocument, OpmlOutline};
use crate::error::{Error, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use tracing::warn;

/// OPML parser with proper XML handling
pub struct OpmlParser;

impl OpmlParser {
    /// Parse OPML content from string
    pub fn parse(content: &str) -> Result<OpmlDocument> {
        let mut reader = Reader::from_str(content);
        reader.config_mut().trim_text(true);

        let mut title = None;
        let mut date_created = None;
        let mut owner_email = None;
        let mut outlines = Vec::new();
        let mut in_head = false;
        let mut in_body = false;
        let mut outline_stack: Vec<OpmlOutline> = Vec::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();

                    match name.as_str() {
                        "head" => in_head = true,
                        "body" => in_body = true,
                        "title" if in_head => {
                            title = Self::read_text_content(&mut reader);
                        }
                        "datecreated" if in_head => {
                            date_created = Self::read_text_content(&mut reader);
                        }
                        "owneremail" if in_head => {
                            owner_email = Self::read_text_content(&mut reader);
                        }
                        "outline" if in_body => {
                            if let Some(outline) = Self::parse_outline_element(e) {
                                outline_stack.push(outline);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();

                    if name == "outline" && in_body {
                        if let Some(outline) = Self::parse_outline_element(e) {
                            // Self-closing outline - add directly to parent or root
                            if let Some(parent) = outline_stack.last_mut() {
                                parent.children.push(outline);
                            } else {
                                outlines.push(outline);
                            }
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_lowercase();

                    match name.as_str() {
                        "head" => in_head = false,
                        "body" => in_body = false,
                        "outline" => {
                            if let Some(outline) = outline_stack.pop() {
                                if let Some(parent) = outline_stack.last_mut() {
                                    parent.children.push(outline);
                                } else {
                                    outlines.push(outline);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    warn!("XML parse error: {}", e);
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        // Handle any remaining outlines in stack (malformed XML)
        while let Some(outline) = outline_stack.pop() {
            if let Some(parent) = outline_stack.last_mut() {
                parent.children.push(outline);
            } else {
                outlines.push(outline);
            }
        }

        if outlines.is_empty() {
            return Err(Error::OpmlParse("No outlines found in OPML".to_string()));
        }

        Ok(OpmlDocument {
            title,
            date_created,
            owner_email,
            outlines,
        })
    }

    /// Read text content until the next end tag
    fn read_text_content(reader: &mut Reader<&[u8]>) -> Option<String> {
        let mut content = String::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Text(e)) => {
                    if let Ok(text) = e.unescape() {
                        content.push_str(&text);
                    }
                }
                Ok(Event::End(_)) | Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }

    /// Parse attributes from an outline element
    fn parse_outline_element(e: &BytesStart) -> Option<OpmlOutline> {
        let mut text = None;
        let mut title = None;
        let mut outline_type = None;
        let mut xml_url = None;
        let mut html_url = None;
        let mut description = None;

        for attr in e.attributes().flatten() {
            let key = String::from_utf8_lossy(attr.key.as_ref()).to_lowercase();
            let value = String::from_utf8_lossy(&attr.value).to_string();

            match key.as_str() {
                "text" => text = Some(value),
                "title" => title = Some(value),
                "type" => outline_type = Some(value),
                "xmlurl" => xml_url = Some(value),
                "htmlurl" => html_url = Some(value),
                "description" => description = Some(value),
                "url" if xml_url.is_none() => xml_url = Some(value),
                "name" if text.is_none() => text = Some(value),
                _ => {}
            }
        }

        // text is required, but fall back to title if not present
        let text = text.or_else(|| title.clone())?;

        // Validate URL if present
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
            children: Vec::new(),
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

    #[test]
    fn test_parse_simple_opml() {
        let opml = r#"<?xml version="1.0" encoding="UTF-8"?>
<opml version="2.0">
  <head>
    <title>Test Feeds</title>
  </head>
  <body>
    <outline type="rss" text="Hacker News" xmlUrl="https://news.ycombinator.com/rss" htmlUrl="https://news.ycombinator.com"/>
  </body>
</opml>"#;

        let doc = OpmlParser::parse(opml).unwrap();
        assert_eq!(doc.title, Some("Test Feeds".to_string()));
        assert_eq!(doc.outlines.len(), 1);
        assert_eq!(doc.outlines[0].text, "Hacker News");
        assert_eq!(
            doc.outlines[0].xml_url,
            Some("https://news.ycombinator.com/rss".to_string())
        );
    }

    #[test]
    fn test_parse_nested_opml() {
        let opml = r#"<?xml version="1.0" encoding="UTF-8"?>
<opml version="2.0">
  <head><title>Nested Test</title></head>
  <body>
    <outline text="Tech">
      <outline type="rss" text="Feed 1" xmlUrl="https://example.com/1"/>
      <outline type="rss" text="Feed 2" xmlUrl="https://example.com/2"/>
    </outline>
    <outline type="rss" text="Root Feed" xmlUrl="https://example.com/root"/>
  </body>
</opml>"#;

        let doc = OpmlParser::parse(opml).unwrap();
        assert_eq!(doc.outlines.len(), 2);

        // First outline is a folder
        let folder = &doc.outlines[0];
        assert_eq!(folder.text, "Tech");
        assert!(folder.xml_url.is_none());
        assert_eq!(folder.children.len(), 2);

        // Second outline is a root feed
        let root_feed = &doc.outlines[1];
        assert_eq!(root_feed.text, "Root Feed");
        assert!(root_feed.xml_url.is_some());
    }

    #[test]
    fn test_opml_outline_struct() {
        let outline = OpmlOutline {
            text: "Test Feed".to_string(),
            title: Some("Test Feed".to_string()),
            outline_type: Some("rss".to_string()),
            xml_url: Some("https://example.com/feed".to_string()),
            html_url: Some("https://example.com".to_string()),
            description: None,
            children: vec![],
        };

        assert!(outline.is_feed());
        assert!(!outline.is_folder());
        assert_eq!(outline.text, "Test Feed");
    }

    #[test]
    fn test_opml_folder_struct() {
        let folder = OpmlOutline {
            text: "Tech".to_string(),
            title: Some("Tech".to_string()),
            outline_type: None,
            xml_url: None,
            html_url: None,
            description: None,
            children: vec![OpmlOutline {
                text: "Feed 1".to_string(),
                title: None,
                outline_type: Some("rss".to_string()),
                xml_url: Some("https://example.com/1".to_string()),
                html_url: None,
                description: None,
                children: vec![],
            }],
        };

        assert!(folder.is_folder());
        assert!(!folder.is_feed());
        assert_eq!(folder.children.len(), 1);
    }
}
