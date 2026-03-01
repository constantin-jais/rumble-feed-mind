//! Feed parsing and management compatibility exports.
//!
//! New code should prefer `feedmind-domain` for models and `feedmind-ingest`
//! for parsing/fetching. This module keeps existing `feedmind-core` consumers
//! stable during the crate split.

pub use feedmind_domain::feed::{Feed, FeedItem, FeedType};
pub use feedmind_ingest::{FeedFetcher, FeedParser, FetcherConfig};
