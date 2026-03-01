//! Rule domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Regex,
    Ai,
}

/// Action to take when rule matches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleAction {
    Hide,
    Keep,
    Tag(String),
    Star,
    MarkRead,
}

/// Filtering rule definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub rule_type: RuleType,
    pub pattern: String,
    pub action: RuleAction,
    pub feed_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub active: bool,
    pub priority: i32,
    pub stop_on_match: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Legacy rule evaluation result. New code should prefer `decision::RuleDecision`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMatch {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub matched: bool,
    pub confidence: f32,
    pub reason: String,
}

impl Rule {
    /// Create a new regex rule.
    pub fn new_regex(user_id: Uuid, name: String, pattern: String, action: RuleAction) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            rule_type: RuleType::Regex,
            pattern,
            action,
            feed_id: None,
            folder_id: None,
            active: true,
            priority: 0,
            stop_on_match: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if rule applies to a specific feed.
    pub fn applies_to_feed(&self, feed_id: Uuid) -> bool {
        self.feed_id.is_none_or(|id| id == feed_id)
    }
}

impl RuleMatch {
    /// Create a match result.
    pub fn matched(rule: &Rule, confidence: f32, reason: String) -> Self {
        Self {
            rule_id: rule.id,
            rule_name: rule.name.clone(),
            matched: true,
            confidence,
            reason,
        }
    }

    /// Create a no-match result.
    pub fn no_match(rule: &Rule) -> Self {
        Self {
            rule_id: rule.id,
            rule_name: rule.name.clone(),
            matched: false,
            confidence: 0.0,
            reason: "No match".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_applies_to_feed() {
        let user_id = Uuid::new_v4();
        let feed_id = Uuid::new_v4();

        let global_rule = Rule::new_regex(
            user_id,
            "Global".to_string(),
            "test".to_string(),
            RuleAction::Hide,
        );
        assert!(global_rule.applies_to_feed(feed_id));

        let mut specific_rule = global_rule.clone();
        specific_rule.feed_id = Some(feed_id);
        assert!(specific_rule.applies_to_feed(feed_id));
        assert!(!specific_rule.applies_to_feed(Uuid::new_v4()));
    }
}
