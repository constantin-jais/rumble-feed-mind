//! Regex-based rule evaluation

use super::models::{Rule, RuleMatch};
use crate::article::Article;
use crate::error::Result;
use regex::Regex;

/// Compiled regex rule for fast evaluation
pub struct RegexRule {
    rule: Rule,
    compiled: Regex,
}

impl RegexRule {
    /// Compile a rule's pattern into a regex
    pub fn compile(rule: Rule) -> Result<Self> {
        let compiled = Regex::new(&rule.pattern)?;
        Ok(Self { rule, compiled })
    }

    /// Evaluate the rule against an article
    pub fn evaluate(&self, article: &Article) -> RuleMatch {
        // Check title
        if self.compiled.is_match(&article.title) {
            return RuleMatch::matched(
                &self.rule,
                1.0,
                format!("Title matches pattern: {}", self.rule.pattern),
            );
        }

        // Check content if available
        if let Some(ref content) = article.content {
            if self.compiled.is_match(content) {
                return RuleMatch::matched(
                    &self.rule,
                    1.0,
                    format!("Content matches pattern: {}", self.rule.pattern),
                );
            }
        }

        // Check summary if available
        if let Some(ref summary) = article.summary {
            if self.compiled.is_match(summary) {
                return RuleMatch::matched(
                    &self.rule,
                    1.0,
                    format!("Summary matches pattern: {}", self.rule.pattern),
                );
            }
        }

        // Check author if available
        if let Some(ref author) = article.author {
            if self.compiled.is_match(author) {
                return RuleMatch::matched(
                    &self.rule,
                    1.0,
                    format!("Author matches pattern: {}", self.rule.pattern),
                );
            }
        }

        // Check categories
        for category in &article.categories {
            if self.compiled.is_match(category) {
                return RuleMatch::matched(
                    &self.rule,
                    1.0,
                    format!(
                        "Category '{}' matches pattern: {}",
                        category, self.rule.pattern
                    ),
                );
            }
        }

        RuleMatch::no_match(&self.rule)
    }

    /// Get the underlying rule
    pub fn rule(&self) -> &Rule {
        &self.rule
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RuleAction;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_article(title: &str, content: Option<&str>) -> Article {
        Article {
            id: Uuid::new_v4(),
            feed_id: Uuid::new_v4(),
            guid: "test-guid".to_string(),
            title: title.to_string(),
            url: Some("https://example.com".to_string()),
            content: content.map(|s| s.to_string()),
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
    fn test_regex_rule_title_match() {
        let rule = Rule::new_regex(
            Uuid::new_v4(),
            "Crypto filter".to_string(),
            r"(?i)crypto|bitcoin".to_string(),
            RuleAction::Hide,
        );

        let regex_rule = RegexRule::compile(rule).unwrap();

        let article = create_test_article("Bitcoin price surges", None);
        let result = regex_rule.evaluate(&article);

        assert!(result.matched);
        assert_eq!(result.confidence, 1.0);
        assert!(result.reason.contains("Title matches"));
    }

    #[test]
    fn test_regex_rule_no_match() {
        let rule = Rule::new_regex(
            Uuid::new_v4(),
            "Crypto filter".to_string(),
            r"(?i)crypto|bitcoin".to_string(),
            RuleAction::Hide,
        );

        let regex_rule = RegexRule::compile(rule).unwrap();

        let article = create_test_article("New JavaScript framework released", None);
        let result = regex_rule.evaluate(&article);

        assert!(!result.matched);
    }

    #[test]
    fn test_regex_rule_content_match() {
        let rule = Rule::new_regex(
            Uuid::new_v4(),
            "Sponsored filter".to_string(),
            r"(?i)sponsored|advertisement".to_string(),
            RuleAction::Hide,
        );

        let regex_rule = RegexRule::compile(rule).unwrap();

        let article = create_test_article(
            "Great product review",
            Some("This is a sponsored post about..."),
        );
        let result = regex_rule.evaluate(&article);

        assert!(result.matched);
        assert!(result.reason.contains("Content matches"));
    }

    #[test]
    fn test_invalid_regex() {
        let rule = Rule::new_regex(
            Uuid::new_v4(),
            "Invalid".to_string(),
            r"[invalid(".to_string(),
            RuleAction::Hide,
        );

        let result = RegexRule::compile(rule);
        assert!(result.is_err());
    }
}
