//! Explainable rules engine.

mod error;
mod evaluator;
mod regex_rule;

pub use error::{Error, Result};
pub use evaluator::{EvaluationResult, RuleEvaluator};
pub use feedmind_domain::rules::{Rule, RuleAction, RuleMatch, RuleType};
pub use regex_rule::RegexRule;
