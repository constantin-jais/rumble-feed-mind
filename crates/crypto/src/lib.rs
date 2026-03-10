//! Cryptographic utilities for FeedMind.

mod encryption;
mod error;

pub use encryption::{generate_master_key, EncryptedData, KeyEncryption};
pub use error::{Error, Result};
