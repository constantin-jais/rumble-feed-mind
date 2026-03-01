//! Decision primitives for explainable feed handling.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::rules::RuleAction;

pub type DecisionId = Uuid;

/// Outcome of a rule or ranking decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionOutcome {
    Matched,
    NotMatched,
    Skipped,
}

/// Evidence used to explain a decision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionEvidence {
    pub field: String,
    pub excerpt: String,
    pub pattern: Option<String>,
}

/// Rule decision with explicit actions and evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleDecision {
    pub id: DecisionId,
    pub rule_id: Uuid,
    pub article_id: Option<Uuid>,
    pub outcome: DecisionOutcome,
    pub actions: Vec<RuleAction>,
    pub confidence: f32,
    pub explanation: String,
    pub evidence: Vec<DecisionEvidence>,
}

impl RuleDecision {
    pub fn matched(
        rule_id: Uuid,
        article_id: Option<Uuid>,
        actions: Vec<RuleAction>,
        confidence: f32,
        explanation: String,
        evidence: Vec<DecisionEvidence>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id,
            article_id,
            outcome: DecisionOutcome::Matched,
            actions,
            confidence,
            explanation,
            evidence,
        }
    }

    pub fn not_matched(rule_id: Uuid, article_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            rule_id,
            article_id,
            outcome: DecisionOutcome::NotMatched,
            actions: Vec::new(),
            confidence: 0.0,
            explanation: "No match".to_string(),
            evidence: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matched_decision_carries_evidence() {
        let rule_id = Uuid::new_v4();
        let decision = RuleDecision::matched(
            rule_id,
            None,
            vec![RuleAction::Hide],
            1.0,
            "Title matched".to_string(),
            vec![DecisionEvidence {
                field: "title".to_string(),
                excerpt: "bitcoin rally".to_string(),
                pattern: Some("bitcoin".to_string()),
            }],
        );

        assert_eq!(decision.rule_id, rule_id);
        assert_eq!(decision.outcome, DecisionOutcome::Matched);
        assert_eq!(decision.evidence.len(), 1);
    }
}
