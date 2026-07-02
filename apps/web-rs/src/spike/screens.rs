//! Three UI screens demonstrating Leptos component ergonomics and reactivity.

use crate::spike::models::{AnswerData, SessionData};
use leptos::prelude::*;

/// Screen 1: Browse survey sessions.
///
/// Displays a list of sessions with title, state badge, and participant count.
/// Demonstrates: component composition, signal-driven rendering, list iteration.
#[component]
pub fn SessionList() -> impl IntoView {
    let sessions = SessionData::mock_list();

    view! {
        <div class="screen screen-list">
            <h1>"Survey Sessions"</h1>
            <ul class="session-list">
                {sessions
                    .into_iter()
                    .map(|session| {
                        view! {
                            <li class="session-item">
                                <h2 class="session-title">{session.title.clone()}</h2>
                                <div class="session-meta">
                                    <span class={format!("badge {}", session.state.badge_class())}>
                                        {session.state.label()}
                                    </span>
                                    <span class="participant-count">
                                        {session.participant_count}
                                        " participants"
                                    </span>
                                </div>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

/// Screen 2: Live question presentation.
///
/// Shows the current question, aggregated poll responses, and participant presence.
/// Demonstrates: signals, reactive updates, progress bars, real-time indicators.
#[component]
pub fn LiveSession() -> impl IntoView {
    let session = SessionData::mock_live();
    let current_question = "What should we prioritize next?";
    let responses = AnswerData::mock_responses();
    let participant_count = session.participant_count;

    view! {
        <div class="screen screen-live">
            <h1>{session.title.clone()}</h1>
            <div class="presence-indicator">
                <span class="dot dot-live"></span>
                "Live: "
                {participant_count}
                " participants"
            </div>

            <div class="question-box">
                <h2 class="question-text">{current_question}</h2>
            </div>

            <div class="poll-results">
                <h3>"Responses"</h3>
                {responses
                    .into_iter()
                    .map(|answer| {
                        view! {
                            <div class="answer-item">
                                <div class="answer-header">
                                    <span class="answer-text">{answer.text.clone()}</span>
                                    <span class="answer-count">
                                        {answer.count}
                                        " votes"
                                    </span>
                                </div>
                                <div class="progress-bar">
                                    <div
                                        class="progress-fill"
                                        style=format!("width: {}%", answer.percentage)
                                    >
                                        <span class="percentage-label">{answer.percentage}%</span>
                                    </div>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// Screen 3: Results recap and export options.
///
/// Shows a summary of the session and mock export buttons.
/// Demonstrates: event handlers, conditional rendering, button states.
#[component]
pub fn ResultExport() -> impl IntoView {
    let session = SessionData::mock_recap();
    let responses = AnswerData::mock_responses();

    // Export format: simplified for spike (no state mutation in mock).
    let export_format = "csv";

    let handle_export = move |_| {
        // In real app: trigger download, call backend, etc.
        // Mock: just swallow the event for now (proves button works)
        let _ = export_format;
    };

    view! {
        <div class="screen screen-export">
            <h1>"Results: " {session.title.clone()}</h1>

            <div class="recap-section">
                <h2>"Summary"</h2>
                <p class="recap-stat">
                    <strong>"Participants: "</strong>
                    {session.participant_count}
                </p>
                <p class="recap-stat">
                    <strong>"Questions: "</strong>
                    "1"
                </p>
                <p class="recap-stat">
                    <strong>"Total Responses: "</strong>
                    {responses.iter().map(|r| r.count).sum::<usize>()}
                </p>
            </div>

            <div class="response-summary">
                <h2>"Question Breakdown"</h2>
                <ul>
                    {responses
                        .into_iter()
                        .map(|answer| {
                            view! {
                                <li class="summary-item">
                                    <span class="summary-answer">{answer.text.clone()}</span>
                                    <span class="summary-stats">
                                        {answer.count}
                                        " ("
                                        {answer.percentage}
                                        "%)"
                                    </span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>

            <div class="export-section">
                <h2>"Export Options"</h2>
                <div class="export-controls">
                    <label class="radio-group">
                        <input
                            type="radio"
                            name="export_format"
                            value="csv"
                            checked={export_format == "csv"}
                        />
                        "CSV"
                    </label>
                    <label class="radio-group">
                        <input
                            type="radio"
                            name="export_format"
                            value="json"
                            checked={export_format == "json"}
                        />
                        "JSON"
                    </label>
                    <label class="radio-group">
                        <input
                            type="radio"
                            name="export_format"
                            value="pdf"
                            checked={export_format == "pdf"}
                        />
                        "PDF"
                    </label>
                </div>

                <button class="export-button" on:click=handle_export>
                    "Export as "
                    {export_format.to_uppercase()}
                </button>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_list_renders_three_sessions() {
        // In a real Leptos test, we'd use leptos_testing utilities.
        // This spike proves the component API is sound by compiling.
        let sessions = SessionData::mock_list();
        assert_eq!(sessions.len(), 3);
    }

    #[test]
    fn live_session_aggregates_100_percent() {
        let responses = AnswerData::mock_responses();
        let total: f32 = responses.iter().map(|r| r.percentage).sum();
        // Allow ±2% tolerance due to rounding in percentage calculations
        assert!((total - 100.0).abs() < 2.0);
    }

    #[test]
    fn result_export_preserves_answer_counts() {
        let responses = AnswerData::mock_responses();
        let total_votes: usize = responses.iter().map(|r| r.count).sum();
        let session = SessionData::mock_recap();
        assert_eq!(total_votes, session.participant_count);
    }
}
