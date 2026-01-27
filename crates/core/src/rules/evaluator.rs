//! Rule evaluator that processes multiple rules against articles

use super::models::{Rule, RuleAction, RuleMatch, RuleType};
use super::regex_rule::RegexRule;
use crate::article::Article;
use crate::error::Result;
use tracing::debug;

/// Evaluates rules against articles
pub struct RuleEvaluator {
    regex_rules: Vec<RegexRule>,
}

/// Result of evaluating all rules against an article
#[derive(Debug)]
pub struct EvaluationResult {
    /// All rule matches (for debugging/explainability)
    pub matches: Vec<RuleMatch>,
    /// Final action to apply (first matching rule with stop_on_match or highest priority)
    pub action: Option<RuleAction>,
    /// Rule that determined the action
    pub deciding_rule: Option<String>,
}

impl RuleEvaluator {
    /// Create a new evaluator from a list of rules
    pub fn new(rules: Vec<Rule>) -> Result<Self> {
        // Sort by priority (descending) then by creation date
        let mut sorted_rules = rules;
        sorted_rules.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.created_at.cmp(&b.created_at))
        });

        // Compile regex rules
        let regex_rules: Vec<RegexRule> = sorted_rules
            .into_iter()
            .filter(|r| r.active && r.rule_type == RuleType::Regex)
            .map(RegexRule::compile)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { regex_rules })
    }

    /// Evaluate all rules against an article
    pub fn evaluate(&self, article: &Article, feed_id: uuid::Uuid) -> EvaluationResult {
        let mut matches = Vec::new();
        let mut final_action: Option<RuleAction> = None;
        let mut deciding_rule: Option<String> = None;

        for regex_rule in &self.regex_rules {
            // Skip rules that don't apply to this feed
            if !regex_rule.rule().applies_to_feed(feed_id) {
                continue;
            }

            let rule_match = regex_rule.evaluate(article);

            if rule_match.matched {
                debug!(
                    rule = %rule_match.rule_name,
                    article = %article.title,
                    "Rule matched"
                );

                // Set action if not already set
                if final_action.is_none() {
                    final_action = Some(regex_rule.rule().action.clone());
                    deciding_rule = Some(rule_match.rule_name.clone());
                }

                matches.push(rule_match);

                // Stop if this rule has stop_on_match
                if regex_rule.rule().stop_on_match {
                    break;
                }
            } else {
                matches.push(rule_match);
            }
        }

        EvaluationResult {
            matches,
            action: final_action,
            deciding_rule,
        }
    }

    /// Evaluate rules against multiple articles (batch)
    pub fn evaluate_batch(&self, articles: &[(Article, uuid::Uuid)]) -> Vec<EvaluationResult> {
        articles
            .iter()
            .map(|(article, feed_id)| self.evaluate(article, *feed_id))
            .collect()
    }

    /// Get count of active rules
    pub fn rule_count(&self) -> usize {
        self.regex_rules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_article(title: &str) -> Article {
        Article {
            id: Uuid::new_v4(),
            feed_id: Uuid::new_v4(),
            guid: "test-guid".to_string(),
            title: title.to_string(),
            url: None,
            content: None,
            summary: None,
            author: None,
            published_at: None,
            categories: vec![],
            enclosure_url: None,
            enclosure_type: None,
            fetched_at: Utc::now(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_evaluator_single_rule() {
        let user_id = Uuid::new_v4();
        let feed_id = Uuid::new_v4();

        let rule = Rule::new_regex(
            user_id,
            "Crypto filter".to_string(),
            r"(?i)bitcoin".to_string(),
            RuleAction::Hide,
        );

        let evaluator = RuleEvaluator::new(vec![rule]).unwrap();

        let article = create_test_article("Bitcoin hits new high");
        let result = evaluator.evaluate(&article, feed_id);

        assert!(result.action.is_some());
        assert_eq!(result.action, Some(RuleAction::Hide));
        assert_eq!(result.deciding_rule, Some("Crypto filter".to_string()));
    }

    #[test]
    fn test_evaluator_priority() {
        let user_id = Uuid::new_v4();
        let feed_id = Uuid::new_v4();

        let mut low_priority = Rule::new_regex(
            user_id,
            "Low priority".to_string(),
            r"news".to_string(),
            RuleAction::Hide,
        );
        low_priority.priority = 1;

        let mut high_priority = Rule::new_regex(
            user_id,
            "High priority".to_string(),
            r"news".to_string(),
            RuleAction::Keep,
        );
        high_priority.priority = 10;

        let evaluator = RuleEvaluator::new(vec![low_priority, high_priority]).unwrap();

        let article = create_test_article("Breaking news today");
        let result = evaluator.evaluate(&article, feed_id);

        // High priority rule should win
        assert_eq!(result.action, Some(RuleAction::Keep));
        assert_eq!(result.deciding_rule, Some("High priority".to_string()));
    }

    #[test]
    fn test_evaluator_no_match() {
        let user_id = Uuid::new_v4();
        let feed_id = Uuid::new_v4();

        let rule = Rule::new_regex(
            user_id,
            "Crypto filter".to_string(),
            r"(?i)bitcoin".to_string(),
            RuleAction::Hide,
        );

        let evaluator = RuleEvaluator::new(vec![rule]).unwrap();

        let article = create_test_article("New JavaScript framework");
        let result = evaluator.evaluate(&article, feed_id);

        assert!(result.action.is_none());
        assert!(result.deciding_rule.is_none());
    }
}
