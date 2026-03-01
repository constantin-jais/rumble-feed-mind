//! Rules engine compatibility exports.
//!
//! New code should prefer `feedmind-domain` for rule models and
//! `feedmind-rules` for evaluation.

pub use feedmind_domain::rules::{Rule, RuleAction, RuleMatch, RuleType};
pub use feedmind_rules::{EvaluationResult, RegexRule, RuleEvaluator};
