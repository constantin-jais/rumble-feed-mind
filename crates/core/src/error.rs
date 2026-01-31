//! Error types for FeedMind Core

use thiserror::Error;

/// Core error types
#[derive(Error, Debug)]
pub enum Error {
    /// Feed parsing failed
    #[error("Failed to parse feed: {0}")]
    FeedParse(String),

    /// Feed fetch failed
    #[error("Failed to fetch feed: {0}")]
    FeedFetch(#[from] reqwest::Error),

    /// HTTP error status
    #[error("HTTP error: status {0}")]
    HttpStatus(u16),

    /// OPML parsing failed
    #[error("Failed to parse OPML: {0}")]
    OpmlParse(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Rule evaluation failed
    #[error("Rule evaluation failed: {0}")]
    RuleEvaluation(String),

    /// Regex compilation failed
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),

    /// Encryption error
    #[error("Encryption error: {0}")]
    Encryption(String),

    /// Decryption error
    #[error("Decryption error: {0}")]
    Decryption(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found
    #[error("{0} not found")]
    NotFound(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for Core operations
pub type Result<T> = std::result::Result<T, Error>;
