//! Regex-based rule evaluation.

use feedmind_domain::article::Article;
use feedmind_domain::decision::{DecisionEvidence, RuleDecision};
use feedmind_domain::rules::{Rule, RuleMatch};
use regex::Regex;

use crate::error::Result;

/// Compiled regex rule for fast evaluation.
pub struct RegexRule {
    rule: Rule,
    compiled: Regex,
}

impl RegexRule {
    /// Compile a rule's pattern into a regex.
    pub fn compile(rule: Rule) -> Result<Self> {
        let compiled = Regex::new(&rule.pattern)?;
        Ok(Self { rule, compiled })
    }

    /// Evaluate the rule against an article and return the legacy match shape.
    pub fn evaluate(&self, article: &Article) -> RuleMatch {
        self.evaluate_decision(article).map_or_else(
            || RuleMatch::no_match(&self.rule),
            |decision| {
                RuleMatch::matched(
                    &self.rule,
                    decision.confidence,
                    decision.explanation.clone(),
                )
            },
        )
    }

    /// Evaluate the rule and return the explainable decision shape.
    pub fn evaluate_decision(&self, article: &Article) -> Option<RuleDecision> {
        let evidence = self.matching_evidence(article)?;
        Some(RuleDecision::matched(
            self.rule.id,
            Some(article.id),
            vec![self.rule.action.clone()],
            1.0,
            format!(
                "{} matches pattern: {}",
                evidence.field_label(),
                self.rule.pattern
            ),
            vec![evidence],
        ))
    }

    /// Get the underlying rule.
    pub fn rule(&self) -> &Rule {
        &self.rule
    }

    fn matching_evidence(&self, article: &Article) -> Option<DecisionEvidence> {
        self.match_field("title", &article.title)
            .or_else(|| {
                article
                    .content
                    .as_deref()
                    .and_then(|v| self.match_field("content", v))
            })
            .or_else(|| {
                article
                    .summary
                    .as_deref()
                    .and_then(|v| self.match_field("summary", v))
            })
            .or_else(|| {
                article
                    .author
                    .as_deref()
                    .and_then(|v| self.match_field("author", v))
            })
            .or_else(|| {
                article
                    .categories
                    .iter()
                    .find_map(|category| self.match_field("category", category))
            })
    }

    fn match_field(&self, field: &str, value: &str) -> Option<DecisionEvidence> {
        self.compiled.find(value).map(|matched| DecisionEvidence {
            field: field.to_string(),
            excerpt: matched.as_str().to_string(),
            pattern: Some(self.rule.pattern.clone()),
        })
    }
}

trait EvidenceLabel {
    fn field_label(&self) -> &'static str;
}

impl EvidenceLabel for DecisionEvidence {
    fn field_label(&self) -> &'static str {
        match self.field.as_str() {
            "title" => "Title",
            "content" => "Content",
            "summary" => "Summary",
            "author" => "Author",
            "category" => "Category",
            _ => "Field",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use feedmind_domain::rules::RuleAction;
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
    fn test_regex_rule_decision_has_evidence() {
        let rule = Rule::new_regex(
            Uuid::new_v4(),
            "Crypto filter".to_string(),
            r"(?i)bitcoin".to_string(),
            RuleAction::Hide,
        );

        let regex_rule = RegexRule::compile(rule).unwrap();
        let article = create_test_article("Bitcoin price surges", None);
        let decision = regex_rule.evaluate_decision(&article).unwrap();

        assert_eq!(decision.evidence[0].field, "title");
        assert_eq!(decision.evidence[0].excerpt, "Bitcoin");
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
