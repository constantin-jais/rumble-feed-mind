//! OPML errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse OPML: {0}")]
    OpmlParse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
