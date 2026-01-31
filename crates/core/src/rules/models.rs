//! Rule domain models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    /// Regular expression matching
    Regex,
    /// AI-powered natural language rule (V1.1)
    Ai,
}

/// Action to take when rule matches
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleAction {
    /// Hide the article
    Hide,
    /// Keep the article (whitelist)
    Keep,
    /// Add a tag
    Tag(String),
    /// Mark as starred
    Star,
    /// Mark as read
    MarkRead,
}

/// A filtering rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Unique identifier
    pub id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// Rule name
    pub name: String,
    /// Rule type
    pub rule_type: RuleType,
    /// Pattern (regex or natural language)
    pub pattern: String,
    /// Action to take on match
    pub action: RuleAction,
    /// Feed ID (None = global rule)
    pub feed_id: Option<Uuid>,
    /// Folder ID (None = all folders)
    pub folder_id: Option<Uuid>,
    /// Is rule active
    pub active: bool,
    /// Priority (higher = evaluated first)
    pub priority: i32,
    /// Stop processing after match
    pub stop_on_match: bool,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

/// Result of a rule evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMatch {
    /// Rule that matched
    pub rule_id: Uuid,
    /// Rule name
    pub rule_name: String,
    /// Did it match
    pub matched: bool,
    /// Confidence (0.0-1.0, always 1.0 for regex)
    pub confidence: f32,
    /// Human-readable explanation
    pub reason: String,
}

impl Rule {
    /// Create a new regex rule
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

    /// Check if rule applies to a specific feed
    pub fn applies_to_feed(&self, feed_id: Uuid) -> bool {
        self.feed_id.map_or(true, |id| id == feed_id)
    }
}

impl RuleMatch {
    /// Create a match result
    pub fn matched(rule: &Rule, confidence: f32, reason: String) -> Self {
        Self {
            rule_id: rule.id,
            rule_name: rule.name.clone(),
            matched: true,
            confidence,
            reason,
        }
    }

    /// Create a no-match result
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

        // Global rule applies to all feeds
        let global_rule = Rule::new_regex(
            user_id,
            "Global".to_string(),
            "test".to_string(),
            RuleAction::Hide,
        );
        assert!(global_rule.applies_to_feed(feed_id));

        // Feed-specific rule
        let mut specific_rule = global_rule.clone();
        specific_rule.feed_id = Some(feed_id);
        assert!(specific_rule.applies_to_feed(feed_id));
        assert!(!specific_rule.applies_to_feed(Uuid::new_v4()));
    }
}
