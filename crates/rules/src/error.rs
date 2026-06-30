//! Rules engine errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
