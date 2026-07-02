//! Spike: evaluating Leptos via three mock screens matching Dioxus spike.
//!
//! This module contains three representative screens for a "live survey" flow:
//! - Session list: browse surveys (title, state badge, participant count)
//! - Live session: present a question, aggregate poll responses in real-time
//! - Result export: display recap and export options
//!
//! Data is hardcoded. No backend calls or state management beyond Leptos signals.

mod models;
mod screens;

pub use models::{AnswerData, SessionData, SessionState};
pub use screens::{LiveSession, ResultExport, SessionList};
