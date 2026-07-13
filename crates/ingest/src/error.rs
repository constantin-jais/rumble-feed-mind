//! Ingestion errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse feed: {0}")]
    FeedParse(String),

    #[error("feed request failed")]
    FeedFetch,

    #[error("invalid feed URL")]
    InvalidUrl,

    #[error("feed URL is outside the configured public-source policy")]
    UrlNotAllowed,

    #[error("feed URL resolves to a non-public network")]
    NonPublicNetwork,

    #[error("feed URL could not be resolved")]
    DnsResolution,

    #[error("feed redirect is invalid")]
    InvalidRedirect,

    #[error("feed exceeded the redirect limit")]
    TooManyRedirects,

    #[error("feed body exceeds the configured {max_bytes}-byte limit")]
    BodyTooLarge { max_bytes: usize },

    #[error("HTTP error: status {0}")]
    HttpStatus(u16),

    #[error("internal feed client error")]
    Internal,
}

pub type Result<T> = std::result::Result<T, Error>;
