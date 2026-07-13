//! Portable, payload-minimized state for bounded local feed synchronization.

use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const LOCAL_SYNC_SNAPSHOT_FORMAT: &str = "feedmind.local_sync_snapshot.v0.1";
pub const HARD_MAX_SOURCES: usize = 32;
pub const HARD_MAX_ITEMS_PER_SOURCE: usize = 100;
pub const HARD_MAX_TOTAL_ITEMS: usize = 500;
pub const HARD_MAX_SEEN_ITEMS: usize = 2_048;

/// Operator-selected limits, capped by immutable product safety ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedSyncLimits {
    pub max_sources: usize,
    pub max_items_per_source: usize,
    pub max_total_items: usize,
}

impl BoundedSyncLimits {
    pub fn new(
        max_sources: usize,
        max_items_per_source: usize,
        max_total_items: usize,
    ) -> Result<Self, LocalSyncError> {
        if max_sources == 0
            || max_sources > HARD_MAX_SOURCES
            || max_items_per_source == 0
            || max_items_per_source > HARD_MAX_ITEMS_PER_SOURCE
            || max_total_items == 0
            || max_total_items > HARD_MAX_TOTAL_ITEMS
            || max_total_items < max_items_per_source
        {
            return Err(LocalSyncError::InvalidLimits);
        }
        Ok(Self {
            max_sources,
            max_items_per_source,
            max_total_items,
        })
    }
}

impl Default for BoundedSyncLimits {
    fn default() -> Self {
        Self {
            max_sources: 8,
            max_items_per_source: 50,
            max_total_items: 200,
        }
    }
}

/// Local replay state. Only hashes cross this boundary: source URLs, GUIDs and
/// article content remain in the fetch process and are never persisted here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalSyncSnapshot {
    pub format: String,
    pub generated_at: DateTime<Utc>,
    pub source_hashes: Vec<String>,
    pub seen_item_hashes: Vec<String>,
    pub selected_export_hash: Option<String>,
}

impl LocalSyncSnapshot {
    pub fn empty() -> Self {
        Self {
            format: LOCAL_SYNC_SNAPSHOT_FORMAT.to_string(),
            generated_at: Utc::now(),
            source_hashes: Vec::new(),
            seen_item_hashes: Vec::new(),
            selected_export_hash: None,
        }
    }

    pub fn validate(&self) -> Result<(), LocalSyncError> {
        if self.format != LOCAL_SYNC_SNAPSHOT_FORMAT {
            return Err(LocalSyncError::UnsupportedFormat);
        }
        if self.source_hashes.len() > HARD_MAX_SOURCES
            || self.seen_item_hashes.len() > HARD_MAX_SEEN_ITEMS
        {
            return Err(LocalSyncError::SnapshotTooLarge);
        }
        if self
            .source_hashes
            .iter()
            .chain(self.seen_item_hashes.iter())
            .chain(self.selected_export_hash.iter())
            .any(|hash| !is_sha256(hash))
        {
            return Err(LocalSyncError::InvalidHash);
        }
        let source_count = self.source_hashes.iter().collect::<HashSet<_>>().len();
        let item_count = self.seen_item_hashes.iter().collect::<HashSet<_>>().len();
        if source_count != self.source_hashes.len() || item_count != self.seen_item_hashes.len() {
            return Err(LocalSyncError::DuplicateHash);
        }
        Ok(())
    }

    pub fn has_seen(&self, item_hash: &str) -> bool {
        self.seen_item_hashes
            .iter()
            .any(|existing| existing == item_hash)
    }

    pub fn advance(
        &mut self,
        source_hashes: Vec<String>,
        inspected_item_hashes: impl IntoIterator<Item = String>,
        selected_export_hash: Option<String>,
    ) -> Result<(), LocalSyncError> {
        if source_hashes.len() > HARD_MAX_SOURCES {
            return Err(LocalSyncError::SnapshotTooLarge);
        }
        let mut seen = self
            .seen_item_hashes
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        for hash in inspected_item_hashes {
            if seen.insert(hash.clone()) {
                self.seen_item_hashes.push(hash);
            }
        }
        if self.seen_item_hashes.len() > HARD_MAX_SEEN_ITEMS {
            let overflow = self.seen_item_hashes.len() - HARD_MAX_SEEN_ITEMS;
            self.seen_item_hashes.drain(0..overflow);
        }
        self.source_hashes = source_hashes;
        self.selected_export_hash = selected_export_hash;
        self.generated_at = Utc::now();
        self.validate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LocalSyncError {
    #[error("local sync limits are invalid")]
    InvalidLimits,
    #[error("local sync snapshot format is unsupported")]
    UnsupportedFormat,
    #[error("local sync snapshot exceeds a hard size limit")]
    SnapshotTooLarge,
    #[error("local sync snapshot contains an invalid hash")]
    InvalidHash,
    #[error("local sync snapshot contains a duplicate hash")]
    DuplicateHash,
}

fn is_sha256(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64 && hex.chars().all(|character| character.is_ascii_hexdigit())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(value: usize) -> String {
        format!("sha256:{value:064x}")
    }

    #[test]
    fn limits_are_nonzero_and_hard_capped() {
        assert!(BoundedSyncLimits::new(8, 50, 200).is_ok());
        assert_eq!(
            BoundedSyncLimits::new(HARD_MAX_SOURCES + 1, 50, 200),
            Err(LocalSyncError::InvalidLimits)
        );
        assert_eq!(
            BoundedSyncLimits::new(8, 50, 20),
            Err(LocalSyncError::InvalidLimits)
        );
    }

    #[test]
    fn state_contains_only_bounded_hashes_and_deduplicates_updates() {
        let mut state = LocalSyncSnapshot::empty();
        state
            .advance(
                vec![hash(1)],
                vec![hash(2), hash(2), hash(3)],
                Some(hash(4)),
            )
            .unwrap();
        assert_eq!(state.seen_item_hashes, vec![hash(2), hash(3)]);
        assert!(state.has_seen(&hash(2)));
        let serialized = serde_json::to_string(&state).unwrap();
        assert!(!serialized.contains("https://"));
        assert!(state.validate().is_ok());
    }

    #[test]
    fn unknown_fields_and_invalid_hashes_fail_closed() {
        let mut value = serde_json::to_value(LocalSyncSnapshot::empty()).unwrap();
        value["source_hashes"] = serde_json::json!(["https://private.example/feed"]);
        value["unexpected"] = serde_json::json!(true);
        assert!(serde_json::from_value::<LocalSyncSnapshot>(value).is_err());

        let mut state = LocalSyncSnapshot::empty();
        state.source_hashes.push("not-a-hash".to_string());
        assert_eq!(state.validate(), Err(LocalSyncError::InvalidHash));
    }
}
