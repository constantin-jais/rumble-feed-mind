//! Feed fetching with HTTP client

use super::models::{Feed, FeedItem};
use super::parser::FeedParser;
use crate::error::{Error, Result};
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};

/// Feed fetcher configuration
#[derive(Debug, Clone)]
pub struct FetcherConfig {
    /// Request timeout (default: 30s per AMD-003)
    pub timeout: Duration,
    /// Max redirects (default: 5 per AMD-003)
    pub max_redirects: usize,
    /// Max response size (default: 10MB per AMD-003)
    pub max_size: usize,
    /// User agent
    pub user_agent: String,
}

impl Default for FetcherConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_redirects: 5,
            max_size: 10 * 1024 * 1024, // 10MB
            user_agent: format!(
                "FeedMind/{} (+https://feedmind.ai)",
                env!("CARGO_PKG_VERSION")
            ),
        }
    }
}

/// HTTP feed fetcher
pub struct FeedFetcher {
    client: Client,
    config: FetcherConfig,
}

impl FeedFetcher {
    /// Create a new fetcher with default config
    pub fn new() -> Result<Self> {
        Self::with_config(FetcherConfig::default())
    }

    /// Create a new fetcher with custom config
    pub fn with_config(config: FetcherConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .redirect(reqwest::redirect::Policy::limited(config.max_redirects))
            .user_agent(&config.user_agent)
            .gzip(true)
            .build()
            .map_err(|e| Error::Internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Fetch and parse a feed from URL
    pub async fn fetch(&self, url: &str) -> Result<(Feed, Vec<FeedItem>)> {
        info!(url = %url, "Fetching feed");

        let response = self.client
            .get(url)
            .header("Accept", "application/rss+xml, application/atom+xml, application/xml, text/xml, application/json")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            warn!(url = %url, status = %status, "Feed fetch failed");
            return Err(Error::HttpStatus(status.as_u16()));
        }

        // Check content length
        if let Some(len) = response.content_length() {
            if len as usize > self.config.max_size {
                return Err(Error::FeedParse(format!(
                    "Feed too large: {} bytes (max: {})",
                    len, self.config.max_size
                )));
            }
        }

        let bytes = response.bytes().await?;

        if bytes.len() > self.config.max_size {
            return Err(Error::FeedParse(format!(
                "Feed too large: {} bytes (max: {})",
                bytes.len(),
                self.config.max_size
            )));
        }

        FeedParser::parse(&bytes, url)
    }

    /// Fetch only headers to check if feed has been modified
    pub async fn check_modified(
        &self,
        url: &str,
        etag: Option<&str>,
        last_modified: Option<&str>,
    ) -> Result<bool> {
        let mut request = self.client.head(url);

        if let Some(etag) = etag {
            request = request.header("If-None-Match", etag);
        }
        if let Some(lm) = last_modified {
            request = request.header("If-Modified-Since", lm);
        }

        let response = request.send().await?;

        // 304 Not Modified = feed hasn't changed
        Ok(response.status() != reqwest::StatusCode::NOT_MODIFIED)
    }
}

impl Default for FeedFetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create default FeedFetcher")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FetcherConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_redirects, 5);
        assert_eq!(config.max_size, 10 * 1024 * 1024);
    }
}
