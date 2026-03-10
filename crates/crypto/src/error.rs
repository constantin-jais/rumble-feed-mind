//! Crypto errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),
}

pub type Result<T> = std::result::Result<T, Error>;
