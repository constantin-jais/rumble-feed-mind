//! Rules engine for filtering articles

mod evaluator;
mod models;
mod regex_rule;

pub use evaluator::RuleEvaluator;
pub use models::{Rule, RuleAction, RuleMatch, RuleType};
pub use regex_rule::RegexRule;
