//! OPML data models.

use serde::{Deserialize, Serialize};

/// OPML document structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpmlDocument {
    pub title: Option<String>,
    pub date_created: Option<String>,
    pub owner_email: Option<String>,
    pub outlines: Vec<OpmlOutline>,
}

/// OPML outline, either a folder or a feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpmlOutline {
    pub text: String,
    pub title: Option<String>,
    pub outline_type: Option<String>,
    pub xml_url: Option<String>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub children: Vec<OpmlOutline>,
}

impl OpmlDocument {
    /// Create a new empty document.
    pub fn new(title: Option<String>) -> Self {
        Self {
            title,
            date_created: Some(chrono::Utc::now().to_rfc2822()),
            owner_email: None,
            outlines: Vec::new(),
        }
    }

    /// Count total feeds recursively.
    pub fn feed_count(&self) -> usize {
        fn count_feeds(outlines: &[OpmlOutline]) -> usize {
            outlines
                .iter()
                .map(|o| {
                    if o.xml_url.is_some() {
                        1
                    } else {
                        count_feeds(&o.children)
                    }
                })
                .sum()
        }
        count_feeds(&self.outlines)
    }

    /// Count total folders recursively.
    pub fn folder_count(&self) -> usize {
        fn count_folders(outlines: &[OpmlOutline]) -> usize {
            outlines
                .iter()
                .map(|o| {
                    if o.xml_url.is_none() && !o.children.is_empty() {
                        1 + count_folders(&o.children)
                    } else {
                        count_folders(&o.children)
                    }
                })
                .sum()
        }
        count_folders(&self.outlines)
    }
}

impl OpmlOutline {
    /// Create a new feed outline.
    pub fn feed(text: String, xml_url: String, html_url: Option<String>) -> Self {
        Self {
            text: text.clone(),
            title: Some(text),
            outline_type: Some("rss".to_string()),
            xml_url: Some(xml_url),
            html_url,
            description: None,
            children: Vec::new(),
        }
    }

    /// Create a new folder outline.
    pub fn folder(text: String) -> Self {
        Self {
            text,
            title: None,
            outline_type: None,
            xml_url: None,
            html_url: None,
            description: None,
            children: Vec::new(),
        }
    }

    /// Check if this outline is a feed.
    pub fn is_feed(&self) -> bool {
        self.xml_url.is_some()
    }

    /// Check if this outline is a folder.
    pub fn is_folder(&self) -> bool {
        !self.children.is_empty() && self.xml_url.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opml_feed_count() {
        let mut doc = OpmlDocument::new(Some("Test".to_string()));

        let mut folder = OpmlOutline::folder("Tech".to_string());
        folder.children.push(OpmlOutline::feed(
            "HN".to_string(),
            "https://news.ycombinator.com/rss".to_string(),
            None,
        ));
        folder.children.push(OpmlOutline::feed(
            "TC".to_string(),
            "https://techcrunch.com/feed".to_string(),
            None,
        ));
        doc.outlines.push(folder);

        doc.outlines.push(OpmlOutline::feed(
            "Example".to_string(),
            "https://example.com/feed".to_string(),
            None,
        ));

        assert_eq!(doc.feed_count(), 3);
        assert_eq!(doc.folder_count(), 1);
    }
}
