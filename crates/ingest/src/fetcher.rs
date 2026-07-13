//! Bounded feed fetching with an optional public-source policy.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use feedmind_domain::feed::{Feed, FeedItem};
use reqwest::header::{ACCEPT, IF_MODIFIED_SINCE, IF_NONE_MATCH, LOCATION};
use reqwest::{Client, Method, Response, Url};
use sha2::{Digest, Sha256};
use tracing::{debug, info, warn};

use crate::error::{Error, Result};
use crate::parser::FeedParser;

/// Feed fetcher configuration.
#[derive(Debug, Clone)]
pub struct FetcherConfig {
    pub timeout: Duration,
    pub max_redirects: usize,
    pub max_size: usize,
    pub user_agent: String,
    /// Exact, normalized host names allowed by the public-source policy.
    /// An empty list keeps the historical local/worker behavior.
    pub allowed_hosts: Vec<String>,
    /// Reject clear-text feed URLs and downgrade redirects.
    pub https_only: bool,
    /// Resolve targets before each request and reject local/reserved addresses.
    pub block_non_public_networks: bool,
}

impl Default for FetcherConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_redirects: 5,
            max_size: 10 * 1024 * 1024,
            user_agent: format!(
                "LibreAI-FeedRadar/{} (+https://libre-ai.fr/produits/feed-radar/)",
                env!("CARGO_PKG_VERSION")
            ),
            allowed_hosts: Vec::new(),
            https_only: false,
            block_non_public_networks: false,
        }
    }
}

impl FetcherConfig {
    /// Conservative policy for an explicit set of public feed hosts.
    pub fn for_public_sources(allowed_hosts: Vec<String>) -> Self {
        Self {
            timeout: Duration::from_secs(10),
            max_redirects: 3,
            max_size: 2 * 1024 * 1024,
            allowed_hosts,
            https_only: true,
            block_non_public_networks: true,
            ..Self::default()
        }
    }
}

/// HTTP feed fetcher.
pub struct FeedFetcher {
    client: Client,
    config: FetcherConfig,
}

impl FeedFetcher {
    /// Create a new fetcher with default config.
    pub fn new() -> Result<Self> {
        Self::with_config(FetcherConfig::default())
    }

    /// Create a new fetcher with custom config.
    pub fn with_config(mut config: FetcherConfig) -> Result<Self> {
        if config.max_size == 0 || config.timeout.is_zero() {
            return Err(Error::Internal);
        }
        config.allowed_hosts = config
            .allowed_hosts
            .into_iter()
            .map(|host| normalize_host(&host))
            .filter(|host| !host.is_empty())
            .collect();
        config.allowed_hosts.sort();
        config.allowed_hosts.dedup();

        let client = Client::builder()
            .timeout(config.timeout)
            // Redirects are followed manually so every target is revalidated.
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(&config.user_agent)
            .gzip(true)
            .build()
            .map_err(|_| Error::Internal)?;

        Ok(Self { client, config })
    }

    /// Fetch and parse a feed from URL.
    pub async fn fetch(&self, url: &str) -> Result<(Feed, Vec<FeedItem>)> {
        info!(url_hash = %safe_hash(url), "Fetching feed");

        let (mut response, final_url) = self
            .send_following_redirects(Method::GET, url, None, None)
            .await?;
        let status = response.status();
        if !status.is_success() {
            warn!(url_hash = %safe_hash(url), status = %status, "Feed fetch failed");
            return Err(Error::HttpStatus(status.as_u16()));
        }

        if response
            .content_length()
            .is_some_and(|length| length > self.config.max_size as u64)
        {
            return Err(Error::BodyTooLarge {
                max_bytes: self.config.max_size,
            });
        }

        let mut body = Vec::with_capacity(
            response
                .content_length()
                .unwrap_or_default()
                .min(self.config.max_size as u64) as usize,
        );
        while let Some(chunk) = response.chunk().await.map_err(|_| Error::FeedFetch)? {
            if body.len().saturating_add(chunk.len()) > self.config.max_size {
                return Err(Error::BodyTooLarge {
                    max_bytes: self.config.max_size,
                });
            }
            body.extend_from_slice(&chunk);
        }

        FeedParser::parse(&body, final_url.as_str())
    }

    /// Fetch only headers to check if feed has been modified.
    pub async fn check_modified(
        &self,
        url: &str,
        etag: Option<&str>,
        last_modified: Option<&str>,
    ) -> Result<bool> {
        let (response, _) = self
            .send_following_redirects(Method::HEAD, url, etag, last_modified)
            .await?;
        Ok(response.status() != reqwest::StatusCode::NOT_MODIFIED)
    }

    async fn send_following_redirects(
        &self,
        method: Method,
        url: &str,
        etag: Option<&str>,
        last_modified: Option<&str>,
    ) -> Result<(Response, Url)> {
        let mut current = Url::parse(url).map_err(|_| Error::InvalidUrl)?;

        for redirect_count in 0..=self.config.max_redirects {
            self.validate_target(&current).await?;
            let mut request = self.client.request(method.clone(), current.clone()).header(
                ACCEPT,
                "application/rss+xml, application/atom+xml, application/xml, text/xml, application/json",
            );
            if let Some(value) = etag {
                request = request.header(IF_NONE_MATCH, value);
            }
            if let Some(value) = last_modified {
                request = request.header(IF_MODIFIED_SINCE, value);
            }

            let response = request.send().await.map_err(|_| Error::FeedFetch)?;
            if !response.status().is_redirection() {
                return Ok((response, current));
            }
            if redirect_count == self.config.max_redirects {
                return Err(Error::TooManyRedirects);
            }
            let location = response
                .headers()
                .get(LOCATION)
                .and_then(|value| value.to_str().ok())
                .ok_or(Error::InvalidRedirect)?;
            current = current.join(location).map_err(|_| Error::InvalidRedirect)?;
        }

        Err(Error::TooManyRedirects)
    }

    async fn validate_target(&self, url: &Url) -> Result<()> {
        if !matches!(url.scheme(), "http" | "https")
            || (self.config.https_only && url.scheme() != "https")
            || !url.username().is_empty()
            || url.password().is_some()
            || url.fragment().is_some()
            || (self.config.https_only && url.port().is_some_and(|port| port != 443))
        {
            debug!(target_hash = %safe_hash(url.as_str()), reason = "url_shape", "Feed target rejected");
            return Err(Error::UrlNotAllowed);
        }

        let host = url.host_str().ok_or(Error::InvalidUrl)?;
        let normalized_host = normalize_host(host);
        if !self.config.allowed_hosts.is_empty()
            && self
                .config
                .allowed_hosts
                .binary_search(&normalized_host)
                .is_err()
        {
            debug!(target_hash = %safe_hash(url.as_str()), host_hash = %safe_hash(&normalized_host), reason = "host_allowlist", "Feed target rejected");
            return Err(Error::UrlNotAllowed);
        }

        if !self.config.block_non_public_networks {
            return Ok(());
        }
        if let Ok(address) = normalized_host.parse::<IpAddr>() {
            return if is_public_address(address) {
                Ok(())
            } else {
                Err(Error::NonPublicNetwork)
            };
        }

        let port = url.port_or_known_default().ok_or(Error::InvalidUrl)?;
        let addresses = tokio::net::lookup_host((normalized_host.as_str(), port))
            .await
            .map_err(|_| Error::DnsResolution)?
            .map(|address| address.ip())
            .collect::<Vec<_>>();
        if addresses.is_empty() {
            return Err(Error::DnsResolution);
        }
        if addresses.into_iter().all(is_public_address) {
            Ok(())
        } else {
            Err(Error::NonPublicNetwork)
        }
    }
}

fn normalize_host(host: &str) -> String {
    host.trim().trim_end_matches('.').to_ascii_lowercase()
}

fn is_public_address(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => is_public_ipv4(address),
        IpAddr::V6(address) => is_public_ipv6(address),
    }
}

fn is_public_ipv4(address: Ipv4Addr) -> bool {
    let [first, second, ..] = address.octets();
    !(address.is_private()
        || address.is_loopback()
        || address.is_link_local()
        || address.is_unspecified()
        || address.is_multicast()
        || address.is_broadcast()
        || address.is_documentation()
        || first == 0
        || (first == 100 && (64..=127).contains(&second))
        || (first == 192 && second == 0)
        || (first == 198 && (18..=19).contains(&second))
        || first >= 240)
}

fn is_public_ipv6(address: Ipv6Addr) -> bool {
    if let Some(mapped) = address.to_ipv4_mapped() {
        return is_public_ipv4(mapped);
    }
    let segments = address.segments();
    !(address.is_loopback()
        || address.is_unspecified()
        || address.is_multicast()
        || (segments[0] & 0xfe00) == 0xfc00
        || (segments[0] & 0xffc0) == 0xfe80
        || (segments[0] == 0x2001 && segments[1] == 0x0db8))
}

fn safe_hash(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()[..16]
        .to_string()
}

impl Default for FeedFetcher {
    fn default() -> Self {
        Self::new().expect("default FeedFetcher configuration must be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[test]
    fn public_config_is_explicitly_bounded() {
        let config = FetcherConfig::for_public_sources(vec!["EXAMPLE.com.".to_string()]);
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.max_redirects, 3);
        assert_eq!(config.max_size, 2 * 1024 * 1024);
        assert!(config.https_only);
        assert!(config.block_non_public_networks);
    }

    #[tokio::test]
    async fn public_policy_rejects_cleartext_unlisted_and_private_targets() {
        let fetcher = FeedFetcher::with_config(FetcherConfig::for_public_sources(vec![
            "localhost".to_string(),
        ]))
        .unwrap();

        let cleartext = Url::parse("http://localhost/feed.xml").unwrap();
        assert!(matches!(
            fetcher.validate_target(&cleartext).await,
            Err(Error::UrlNotAllowed)
        ));

        let unlisted = Url::parse("https://example.com/feed.xml").unwrap();
        assert!(matches!(
            fetcher.validate_target(&unlisted).await,
            Err(Error::UrlNotAllowed)
        ));

        let alternate_port = Url::parse("https://localhost:8443/feed.xml").unwrap();
        assert!(matches!(
            fetcher.validate_target(&alternate_port).await,
            Err(Error::UrlNotAllowed)
        ));

        let private = Url::parse("https://localhost/feed.xml").unwrap();
        assert!(matches!(
            fetcher.validate_target(&private).await,
            Err(Error::NonPublicNetwork)
        ));
    }

    #[tokio::test]
    async fn body_limit_is_enforced_while_streaming_without_content_length() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0_u8; 1024];
            let _ = stream.read(&mut request).await;
            stream
                .write_all(
                    b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nContent-Type: application/xml\r\n\r\n6\r\nabcdef\r\n6\r\nghijkl\r\n0\r\n\r\n",
                )
                .await
                .unwrap();
        });

        let fetcher = FeedFetcher::with_config(FetcherConfig {
            timeout: Duration::from_secs(2),
            max_size: 8,
            ..FetcherConfig::default()
        })
        .unwrap();
        let error = fetcher
            .fetch(&format!("http://{address}/feed.xml"))
            .await
            .unwrap_err();
        assert!(matches!(error, Error::BodyTooLarge { max_bytes: 8 }));
    }
}
