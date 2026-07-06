//! API routes

pub mod articles;
pub mod auth;
#[cfg(feature = "stripe")]
pub mod billing;
pub mod categories;
pub mod feeds;
pub mod folders;
pub mod health;
pub mod opml;
pub mod rules;
pub mod tags;
