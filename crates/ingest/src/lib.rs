//! Feed ingestion: HTTP fetch, parse, normalize.

mod error;
mod fetcher;
mod parser;

pub use error::{Error, Result};
pub use fetcher::{FeedFetcher, FetcherConfig};
pub use parser::FeedParser;
