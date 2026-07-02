//! Domain models for the spike screens.

use std::fmt;

/// State of a survey session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SessionState {
    Draft,
    Live,
    Archived,
}

impl SessionState {
    /// Human-readable label for state badge.
    pub fn label(self) -> &'static str {
        match self {
            SessionState::Draft => "Draft",
            SessionState::Live => "Live",
            SessionState::Archived => "Archived",
        }
    }

    /// CSS class for styling (reflects state visually).
    pub fn badge_class(self) -> &'static str {
        match self {
            SessionState::Draft => "badge-draft",
            SessionState::Live => "badge-live",
            SessionState::Archived => "badge-archived",
        }
    }
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Metadata for a survey session.
#[derive(Clone, Debug)]
pub struct SessionData {
    pub id: String,
    pub title: String,
    pub state: SessionState,
    pub participant_count: usize,
}

impl SessionData {
    /// Mock data: session list for screen 1.
    pub fn mock_list() -> Vec<Self> {
        vec![
            SessionData {
                id: "sess-001".to_string(),
                title: "Q1 2026 Product Roadmap".to_string(),
                state: SessionState::Live,
                participant_count: 23,
            },
            SessionData {
                id: "sess-002".to_string(),
                title: "Rust Adoption Sentiment".to_string(),
                state: SessionState::Archived,
                participant_count: 18,
            },
            SessionData {
                id: "sess-003".to_string(),
                title: "Feature Priority Vote".to_string(),
                state: SessionState::Draft,
                participant_count: 0,
            },
        ]
    }

    /// Mock data: current session for screen 2 (live session).
    pub fn mock_live() -> Self {
        SessionData {
            id: "sess-001".to_string(),
            title: "Q1 2026 Product Roadmap".to_string(),
            state: SessionState::Live,
            participant_count: 23,
        }
    }

    /// Mock data: recap for screen 3 (result export).
    pub fn mock_recap() -> Self {
        SessionData {
            id: "sess-001".to_string(),
            title: "Q1 2026 Product Roadmap".to_string(),
            state: SessionState::Archived,
            participant_count: 23,
        }
    }
}

/// A response option and its aggregated count.
#[derive(Clone, Debug)]
pub struct AnswerData {
    pub text: String,
    pub count: usize,
    pub percentage: f32,
}

impl AnswerData {
    /// Mock aggregated responses for the live session.
    pub fn mock_responses() -> Vec<Self> {
        let total: f32 = 23.0;
        vec![
            AnswerData {
                text: "Prioritize API stability".to_string(),
                count: 12,
                percentage: (12.0_f32 / total * 100.0_f32).round(),
            },
            AnswerData {
                text: "Prioritize new features".to_string(),
                count: 7,
                percentage: (7.0_f32 / total * 100.0_f32).round(),
            },
            AnswerData {
                text: "Split equally".to_string(),
                count: 4,
                percentage: (4.0_f32 / total * 100.0_f32).round(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_state_label_draft() {
        assert_eq!(SessionState::Draft.label(), "Draft");
    }

    #[test]
    fn session_state_label_live() {
        assert_eq!(SessionState::Live.label(), "Live");
    }

    #[test]
    fn session_state_label_archived() {
        assert_eq!(SessionState::Archived.label(), "Archived");
    }

    #[test]
    fn session_state_badge_class_reflects_state() {
        assert_eq!(SessionState::Draft.badge_class(), "badge-draft");
        assert_eq!(SessionState::Live.badge_class(), "badge-live");
        assert_eq!(SessionState::Archived.badge_class(), "badge-archived");
    }

    #[test]
    fn session_data_mock_list_has_three_items() {
        let sessions = SessionData::mock_list();
        assert_eq!(sessions.len(), 3);
        assert_eq!(sessions[0].state, SessionState::Live);
        assert_eq!(sessions[1].state, SessionState::Archived);
        assert_eq!(sessions[2].state, SessionState::Draft);
    }

    #[test]
    fn session_data_mock_list_participant_counts_correct() {
        let sessions = SessionData::mock_list();
        assert_eq!(sessions[0].participant_count, 23);
        assert_eq!(sessions[1].participant_count, 18);
        assert_eq!(sessions[2].participant_count, 0);
    }

    #[test]
    fn answer_data_percentages_sum_to_100() {
        let answers = AnswerData::mock_responses();
        let total: f32 = answers.iter().map(|a| a.percentage).sum();
        assert!(
            (total - 100.0).abs() < 2.0,
            "Percentages do not sum to ~100: {}",
            total
        );
    }

    #[test]
    fn answer_data_counts_match_session_participant_count() {
        let answers = AnswerData::mock_responses();
        let total_responses: usize = answers.iter().map(|a| a.count).sum();
        assert_eq!(total_responses, 23);
    }
}
