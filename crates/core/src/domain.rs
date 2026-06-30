//! Domain primitives re-exported during the core crate split.
//!
//! New code should depend on `feedmind-domain` directly. This module keeps
//! existing `feedmind-core` consumers stable during the migration.

pub use feedmind_domain::*;
