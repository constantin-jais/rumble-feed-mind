//! Rule evaluator.

use feedmind_domain::article::Article;
use feedmind_domain::rules::{Rule, RuleMatch, RuleType};

use crate::{RegexRule, Result};

/// Aggregate evaluation result for one article.
#[derive(Debug, Clone)]
pub struct EvaluationResult {
    pub matches: Vec<RuleMatch>,
}

impl EvaluationResult {
    pub fn any_matched(&self) -> bool {
        self.matches.iter().any(|m| m.matched)
    }
}

/// Deterministic rule evaluator.
pub struct RuleEvaluator;

impl RuleEvaluator {
    pub fn evaluate(rules: &[Rule], article: &Article) -> Result<EvaluationResult> {
        let mut matches = Vec::new();
        let mut sorted = rules
            .iter()
            .filter(|r| r.active)
            .cloned()
            .collect::<Vec<_>>();
        sorted.sort_by_key(|r| r.priority);

        for rule in sorted {
            let rule_match = match rule.rule_type {
                RuleType::Regex => RegexRule::compile(rule.clone())?.evaluate(article),
                RuleType::Ai => RuleMatch::no_match(&rule),
            };
            let stop = rule.stop_on_match && rule_match.matched;
            matches.push(rule_match);
            if stop {
                break;
            }
        }

        Ok(EvaluationResult { matches })
    }
}
