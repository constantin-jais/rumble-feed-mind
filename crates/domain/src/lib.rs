//! FeedMind domain primitives.
//!
//! This crate owns pure product concepts and invariants. It must stay free of
//! network, database, queue, UI, and provider-specific dependencies.

pub mod article;
pub mod decision;
pub mod event;
pub mod feed;
pub mod opml;
pub mod rules;

pub use article::{Article, ArticleState, ReadStatus};
pub use decision::{DecisionEvidence, DecisionId, DecisionOutcome, RuleDecision};
pub use event::{FeedMindEvent, FeedMindEventPayload};
pub use feed::{Feed, FeedItem, FeedType};
pub use opml::{OpmlDocument, OpmlOutline};
pub use rules::{Rule, RuleAction, RuleMatch, RuleType};
