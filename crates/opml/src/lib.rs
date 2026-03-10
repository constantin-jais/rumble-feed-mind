//! OPML import/export.

mod error;
mod exporter;
mod parser;

pub use error::{Error, Result};
pub use exporter::OpmlExporter;
pub use feedmind_domain::opml::{OpmlDocument, OpmlOutline};
pub use parser::OpmlParser;
