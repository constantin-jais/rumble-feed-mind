//! Regex-backed rule evaluation.

use feedmind_domain::article::Article;
use feedmind_domain::rules::{Rule, RuleMatch};
use regex::Regex;

use crate::Result;

/// Compiled regex rule.
pub struct RegexRule {
    rule: Rule,
    regex: Regex,
}

impl RegexRule {
    pub fn compile(rule: Rule) -> Result<Self> {
        let regex = Regex::new(&rule.pattern)?;
        Ok(Self { rule, regex })
    }

    pub fn evaluate(&self, article: &Article) -> RuleMatch {
        let haystack = format!(
            "{}\n{}\n{}",
            article.title,
            article.summary.as_deref().unwrap_or_default(),
            article.content.as_deref().unwrap_or_default()
        );

        if self.regex.is_match(&haystack) {
            RuleMatch::matched(
                &self.rule,
                1.0,
                format!("regex `{}` matched article text", self.rule.pattern),
            )
        } else {
            RuleMatch::no_match(&self.rule)
        }
    }
}
