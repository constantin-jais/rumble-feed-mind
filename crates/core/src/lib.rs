//! FeedMind Core - Business logic for RSS feed management
//!
//! This crate contains the core domain logic for FeedMind:
//! - Feed parsing (RSS, Atom, JSON Feed)
//! - OPML import/export
//! - Rules engine (regex + AI)
//! - Article management
//! - AI provider abstraction

pub mod article;
pub mod crypto;
pub mod domain;
pub mod error;
pub mod feed;
pub mod opml;
pub mod rules;

pub use error::{Error, Result};
