//! Ingestion errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse feed: {0}")]
    FeedParse(String),

    #[error("Failed to fetch feed: {0}")]
    FeedFetch(#[from] reqwest::Error),

    #[error("HTTP error: status {0}")]
    HttpStatus(u16),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;
