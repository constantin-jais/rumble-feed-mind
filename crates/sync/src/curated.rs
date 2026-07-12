//! Portable contract for client-safe curated item exports.
//!
//! Deserialization rejects unknown fields. Call [`CuratedItemExport::validate_client_safe`]
//! before exposing any export to a client surface.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

pub const CURATED_ITEM_EXPORT_FORMAT: &str = "feedmind.curated_item_export.v0.1";
pub const CURATED_ITEM_EXPORT_ORIGIN: &str = "rumble-feed-mind";
pub const CURATED_ITEM_EXPORT_PURPOSE: &str = "local_export";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedItemExport {
    pub format: String,
    pub export_id: String,
    pub origin_product: String,
    pub workspace_id: String,
    pub created_by: String,
    pub created_at: String,
    pub purpose: String,
    pub privacy_classification: String,
    pub item: CuratedExportItem,
    pub source_ref: CuratedSourceRef,
    pub curation: CuratedExportCuration,
    pub rule_evidence: Vec<CuratedRuleEvidence>,
    pub constraints: CuratedExportConstraints,
    pub artifact_ref: CuratedArtifactRef,
    pub provenance_ref: CuratedProvenanceRef,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedExportItem {
    pub item_id: String,
    pub title: String,
    pub content_excerpt: String,
    pub content_hash: String,
    pub source_url_hash: String,
    pub published_at: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedSourceRef {
    pub source_id: String,
    pub source_type: String,
    pub origin_product: String,
    pub content_hash: String,
    pub provenance_id: String,
    pub opml_title: Option<String>,
    pub opml_feed_count: usize,
    pub first_feed_title: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedExportCuration {
    pub decision: String,
    pub reason: String,
    pub curated_by: String,
    pub curated_at: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedRuleEvidence {
    pub rule_id: String,
    pub decision: String,
    pub explanation: String,
    pub evidence_hash: String,
    pub confidence: f32,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedExportConstraints {
    pub contains_raw_private_content: bool,
    pub contains_secrets: bool,
    pub contains_byok_material: bool,
    pub allow_downstream_execution: bool,
    pub data_residency: String,
    pub retention_policy_ref: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedArtifactRef {
    pub artifact_id: String,
    pub artifact_type: String,
    pub hash: String,
    pub manifest_ref: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedProvenanceRef {
    pub provenance_id: String,
    pub operation: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CuratedRequiredField {
    ExportId,
    WorkspaceId,
    CreatedBy,
    CreatedAt,
    ItemId,
    ItemTitle,
    SourceId,
    SourceProvenanceId,
    CurationReason,
    CuratedBy,
    CuratedAt,
    RuleId,
    RuleExplanation,
    ArtifactId,
    ArtifactManifestRef,
    ProvenanceId,
    ProvenanceTimestamp,
    DataResidency,
    RetentionPolicyRef,
}

impl fmt::Display for CuratedRequiredField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ExportId => "export_id",
            Self::WorkspaceId => "workspace_id",
            Self::CreatedBy => "created_by",
            Self::CreatedAt => "created_at",
            Self::ItemId => "item.item_id",
            Self::ItemTitle => "item.title",
            Self::SourceId => "source_ref.source_id",
            Self::SourceProvenanceId => "source_ref.provenance_id",
            Self::CurationReason => "curation.reason",
            Self::CuratedBy => "curation.curated_by",
            Self::CuratedAt => "curation.curated_at",
            Self::RuleId => "rule_evidence.rule_id",
            Self::RuleExplanation => "rule_evidence.explanation",
            Self::ArtifactId => "artifact_ref.artifact_id",
            Self::ArtifactManifestRef => "artifact_ref.manifest_ref",
            Self::ProvenanceId => "provenance_ref.provenance_id",
            Self::ProvenanceTimestamp => "provenance_ref.timestamp",
            Self::DataResidency => "constraints.data_residency",
            Self::RetentionPolicyRef => "constraints.retention_policy_ref",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CuratedHashField {
    ItemContent,
    ItemSourceUrl,
    SourceContent,
    RuleEvidence,
    Artifact,
}

impl fmt::Display for CuratedHashField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ItemContent => "item.content_hash",
            Self::ItemSourceUrl => "item.source_url_hash",
            Self::SourceContent => "source_ref.content_hash",
            Self::RuleEvidence => "rule_evidence.evidence_hash",
            Self::Artifact => "artifact_ref.hash",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CuratedValueField {
    SourceType,
    CurationDecision,
    RuleEvidenceDecision,
    ArtifactType,
    ProvenanceOperation,
}

impl fmt::Display for CuratedValueField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::SourceType => "source_ref.source_type",
            Self::CurationDecision => "curation.decision",
            Self::RuleEvidenceDecision => "rule_evidence.decision",
            Self::ArtifactType => "artifact_ref.artifact_type",
            Self::ProvenanceOperation => "provenance_ref.operation",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CuratedConstraint {
    RawPrivateContent,
    Secrets,
    ByokMaterial,
    DownstreamExecution,
}

impl fmt::Display for CuratedConstraint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::RawPrivateContent => "contains_raw_private_content",
            Self::Secrets => "contains_secrets",
            Self::ByokMaterial => "contains_byok_material",
            Self::DownstreamExecution => "allow_downstream_execution",
        })
    }
}

/// Stable, payload-free failures returned by client-safe validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum CuratedValidationError {
    #[error("unsupported curated export format")]
    UnsupportedFormat,
    #[error("unexpected curated export origin")]
    UnexpectedOrigin,
    #[error("unexpected curated source origin")]
    UnexpectedSourceOrigin,
    #[error("unsupported curated export purpose")]
    UnsupportedPurpose,
    #[error("unsafe curated export privacy classification")]
    UnsafePrivacyClassification,
    #[error("empty required curated export field: {field}")]
    EmptyRequiredField { field: CuratedRequiredField },
    #[error("curated export requires rule evidence")]
    MissingRuleEvidence,
    #[error("invalid curated export hash: {field}")]
    InvalidHash { field: CuratedHashField },
    #[error("unsupported curated export value: {field}")]
    UnsupportedValue { field: CuratedValueField },
    #[error("unsafe curated export constraint: {constraint}")]
    UnsafeConstraint { constraint: CuratedConstraint },
    #[error("invalid curated rule confidence")]
    InvalidConfidence,
}

impl CuratedValidationError {
    /// Stable code suitable for telemetry without including payload content.
    pub const fn code(self) -> &'static str {
        match self {
            Self::UnsupportedFormat => "curated_export.unsupported_format",
            Self::UnexpectedOrigin => "curated_export.unexpected_origin",
            Self::UnexpectedSourceOrigin => "curated_export.unexpected_source_origin",
            Self::UnsupportedPurpose => "curated_export.unsupported_purpose",
            Self::UnsafePrivacyClassification => "curated_export.unsafe_privacy_classification",
            Self::EmptyRequiredField { .. } => "curated_export.empty_required_field",
            Self::MissingRuleEvidence => "curated_export.missing_rule_evidence",
            Self::InvalidHash { .. } => "curated_export.invalid_hash",
            Self::UnsupportedValue { .. } => "curated_export.unsupported_value",
            Self::UnsafeConstraint { .. } => "curated_export.unsafe_constraint",
            Self::InvalidConfidence => "curated_export.invalid_confidence",
        }
    }
}

impl CuratedItemExport {
    /// Validates the complete export before any client projection is created.
    ///
    /// Errors contain field categories only, never values from the export.
    pub fn validate_client_safe(&self) -> Result<(), CuratedValidationError> {
        if self.format != CURATED_ITEM_EXPORT_FORMAT {
            return Err(CuratedValidationError::UnsupportedFormat);
        }
        if self.origin_product != CURATED_ITEM_EXPORT_ORIGIN {
            return Err(CuratedValidationError::UnexpectedOrigin);
        }
        if self.source_ref.origin_product != CURATED_ITEM_EXPORT_ORIGIN {
            return Err(CuratedValidationError::UnexpectedSourceOrigin);
        }
        if self.purpose != CURATED_ITEM_EXPORT_PURPOSE {
            return Err(CuratedValidationError::UnsupportedPurpose);
        }
        if !matches!(self.privacy_classification.as_str(), "public" | "normal") {
            return Err(CuratedValidationError::UnsafePrivacyClassification);
        }

        for (value, field) in [
            (&self.export_id, CuratedRequiredField::ExportId),
            (&self.workspace_id, CuratedRequiredField::WorkspaceId),
            (&self.created_by, CuratedRequiredField::CreatedBy),
            (&self.created_at, CuratedRequiredField::CreatedAt),
            (&self.item.item_id, CuratedRequiredField::ItemId),
            (&self.item.title, CuratedRequiredField::ItemTitle),
            (&self.source_ref.source_id, CuratedRequiredField::SourceId),
            (
                &self.source_ref.provenance_id,
                CuratedRequiredField::SourceProvenanceId,
            ),
            (&self.curation.reason, CuratedRequiredField::CurationReason),
            (&self.curation.curated_by, CuratedRequiredField::CuratedBy),
            (&self.curation.curated_at, CuratedRequiredField::CuratedAt),
            (
                &self.artifact_ref.artifact_id,
                CuratedRequiredField::ArtifactId,
            ),
            (
                &self.artifact_ref.manifest_ref,
                CuratedRequiredField::ArtifactManifestRef,
            ),
            (
                &self.provenance_ref.provenance_id,
                CuratedRequiredField::ProvenanceId,
            ),
            (
                &self.provenance_ref.timestamp,
                CuratedRequiredField::ProvenanceTimestamp,
            ),
            (
                &self.constraints.data_residency,
                CuratedRequiredField::DataResidency,
            ),
            (
                &self.constraints.retention_policy_ref,
                CuratedRequiredField::RetentionPolicyRef,
            ),
        ] {
            require_nonempty(value, field)?;
        }

        validate_hash(&self.item.content_hash, CuratedHashField::ItemContent)?;
        validate_hash(&self.item.source_url_hash, CuratedHashField::ItemSourceUrl)?;
        validate_hash(
            &self.source_ref.content_hash,
            CuratedHashField::SourceContent,
        )?;
        validate_hash(&self.artifact_ref.hash, CuratedHashField::Artifact)?;

        validate_known_value(
            &self.source_ref.source_type,
            &["feed_item", "url", "document", "artifact"],
            CuratedValueField::SourceType,
        )?;
        validate_known_value(
            &self.curation.decision,
            &["saved", "rejected", "tagged", "exported"],
            CuratedValueField::CurationDecision,
        )?;
        validate_known_value(
            &self.artifact_ref.artifact_type,
            &["curated_export"],
            CuratedValueField::ArtifactType,
        )?;
        validate_known_value(
            &self.provenance_ref.operation,
            &["created", "imported", "transformed", "exported", "indexed"],
            CuratedValueField::ProvenanceOperation,
        )?;

        if self.rule_evidence.is_empty() {
            return Err(CuratedValidationError::MissingRuleEvidence);
        }
        for evidence in &self.rule_evidence {
            require_nonempty(&evidence.rule_id, CuratedRequiredField::RuleId)?;
            require_nonempty(&evidence.explanation, CuratedRequiredField::RuleExplanation)?;
            validate_hash(&evidence.evidence_hash, CuratedHashField::RuleEvidence)?;
            validate_known_value(
                &evidence.decision,
                &["match", "no_match", "manual_override", "not_evaluated"],
                CuratedValueField::RuleEvidenceDecision,
            )?;
            if !evidence.confidence.is_finite() || !(0.0..=1.0).contains(&evidence.confidence) {
                return Err(CuratedValidationError::InvalidConfidence);
            }
        }

        for (unsafe_value, constraint) in [
            (
                self.constraints.contains_raw_private_content,
                CuratedConstraint::RawPrivateContent,
            ),
            (
                self.constraints.contains_secrets,
                CuratedConstraint::Secrets,
            ),
            (
                self.constraints.contains_byok_material,
                CuratedConstraint::ByokMaterial,
            ),
            (
                self.constraints.allow_downstream_execution,
                CuratedConstraint::DownstreamExecution,
            ),
        ] {
            if unsafe_value {
                return Err(CuratedValidationError::UnsafeConstraint { constraint });
            }
        }

        Ok(())
    }
}

fn require_nonempty(
    value: &str,
    field: CuratedRequiredField,
) -> Result<(), CuratedValidationError> {
    if value.trim().is_empty() {
        Err(CuratedValidationError::EmptyRequiredField { field })
    } else {
        Ok(())
    }
}

fn validate_hash(value: &str, field: CuratedHashField) -> Result<(), CuratedValidationError> {
    let valid = value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64 && hex.chars().all(|character| character.is_ascii_hexdigit())
    });
    if valid {
        Ok(())
    } else {
        Err(CuratedValidationError::InvalidHash { field })
    }
}

fn validate_known_value(
    value: &str,
    allowed: &[&str],
    field: CuratedValueField,
) -> Result<(), CuratedValidationError> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(CuratedValidationError::UnsupportedValue { field })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn fixture_value() -> Value {
        serde_json::from_str(include_str!(
            "../../../examples/expected-curated-export.json"
        ))
        .expect("curated export fixture must be valid JSON")
    }

    fn fixture_export() -> CuratedItemExport {
        serde_json::from_value(fixture_value()).expect("fixture must match portable contract")
    }

    fn validation_error(value: Value) -> CuratedValidationError {
        serde_json::from_value::<CuratedItemExport>(value)
            .expect("mutated fixture must remain structurally valid")
            .validate_client_safe()
            .expect_err("mutated fixture must be rejected")
    }

    #[test]
    fn golden_export_is_client_safe_and_round_trips_exactly() {
        let expected = fixture_value();
        let export = fixture_export();

        assert_eq!(export.validate_client_safe(), Ok(()));
        assert_eq!(
            serde_json::to_value(export).expect("portable contract must serialize"),
            expected
        );
    }

    #[test]
    fn rejects_unknown_fields_at_root_and_nested_levels() {
        let mut root = fixture_value();
        root["future_field"] = json!(true);
        assert!(serde_json::from_value::<CuratedItemExport>(root).is_err());

        let mut nested = fixture_value();
        nested["item"]["future_field"] = json!(true);
        assert!(serde_json::from_value::<CuratedItemExport>(nested).is_err());
    }

    #[test]
    fn rejects_unexpected_format_origin_and_purpose_without_payload_in_error() {
        for (field, replacement, expected, code) in [
            (
                "format",
                "feedmind.curated_item_export.v9",
                CuratedValidationError::UnsupportedFormat,
                "curated_export.unsupported_format",
            ),
            (
                "origin_product",
                "unknown-product",
                CuratedValidationError::UnexpectedOrigin,
                "curated_export.unexpected_origin",
            ),
            (
                "purpose",
                "agent_context",
                CuratedValidationError::UnsupportedPurpose,
                "curated_export.unsupported_purpose",
            ),
        ] {
            let mut value = fixture_value();
            value[field] = json!(replacement);
            let error = validation_error(value);
            assert_eq!(error, expected);
            assert_eq!(error.code(), code);
            assert!(!error.to_string().contains("unknown-product"));
        }
    }

    #[test]
    fn rejects_unexpected_source_origin() {
        let mut value = fixture_value();
        value["source_ref"]["origin_product"] = json!("unknown-product");

        assert_eq!(
            validation_error(value),
            CuratedValidationError::UnexpectedSourceOrigin
        );
    }

    #[test]
    fn rejects_unknown_enumerated_values() {
        let mut value = fixture_value();
        value["rule_evidence"][0]["decision"] = json!("future_decision");

        assert_eq!(
            validation_error(value),
            CuratedValidationError::UnsupportedValue {
                field: CuratedValueField::RuleEvidenceDecision,
            }
        );
    }

    #[test]
    fn rejects_invalid_hashes_in_every_hash_location() {
        let cases = [
            (&["item", "content_hash"][..], CuratedHashField::ItemContent),
            (
                &["item", "source_url_hash"][..],
                CuratedHashField::ItemSourceUrl,
            ),
            (
                &["source_ref", "content_hash"][..],
                CuratedHashField::SourceContent,
            ),
            (
                &["rule_evidence", "0", "evidence_hash"][..],
                CuratedHashField::RuleEvidence,
            ),
            (&["artifact_ref", "hash"][..], CuratedHashField::Artifact),
        ];

        for (path, field) in cases {
            let mut value = fixture_value();
            let target = path.iter().fold(&mut value, |current, segment| {
                if let Ok(index) = segment.parse::<usize>() {
                    &mut current[index]
                } else {
                    &mut current[*segment]
                }
            });
            *target = json!("sha256:not-a-valid-hash");
            assert_eq!(
                validation_error(value),
                CuratedValidationError::InvalidHash { field }
            );
        }
    }

    #[test]
    fn allows_public_and_normal_classifications() {
        for classification in ["public", "normal"] {
            let mut value = fixture_value();
            value["privacy_classification"] = json!(classification);
            let export: CuratedItemExport =
                serde_json::from_value(value).expect("classification must deserialize");
            assert_eq!(export.validate_client_safe(), Ok(()));
        }
    }

    #[test]
    fn rejects_private_classifications() {
        for classification in ["private", "sensitive", "no_handoff", "future"] {
            let mut value = fixture_value();
            value["privacy_classification"] = json!(classification);
            assert_eq!(
                validation_error(value),
                CuratedValidationError::UnsafePrivacyClassification
            );
        }
    }

    #[test]
    fn rejects_each_unsafe_constraint() {
        let cases = [
            (
                "contains_raw_private_content",
                CuratedConstraint::RawPrivateContent,
            ),
            ("contains_secrets", CuratedConstraint::Secrets),
            ("contains_byok_material", CuratedConstraint::ByokMaterial),
            (
                "allow_downstream_execution",
                CuratedConstraint::DownstreamExecution,
            ),
        ];

        for (field, constraint) in cases {
            let mut value = fixture_value();
            value["constraints"][field] = json!(true);
            assert_eq!(
                validation_error(value),
                CuratedValidationError::UnsafeConstraint { constraint }
            );
        }
    }

    #[test]
    fn rejects_missing_or_empty_reason_explanation_and_references() {
        let mut missing_evidence = fixture_value();
        missing_evidence["rule_evidence"] = json!([]);
        assert_eq!(
            validation_error(missing_evidence),
            CuratedValidationError::MissingRuleEvidence
        );

        for (path, field) in [
            (
                &["curation", "reason"][..],
                CuratedRequiredField::CurationReason,
            ),
            (
                &["rule_evidence", "0", "explanation"][..],
                CuratedRequiredField::RuleExplanation,
            ),
            (
                &["source_ref", "provenance_id"][..],
                CuratedRequiredField::SourceProvenanceId,
            ),
            (
                &["artifact_ref", "manifest_ref"][..],
                CuratedRequiredField::ArtifactManifestRef,
            ),
            (
                &["provenance_ref", "provenance_id"][..],
                CuratedRequiredField::ProvenanceId,
            ),
        ] {
            let mut value = fixture_value();
            let target = path.iter().fold(&mut value, |current, segment| {
                if let Ok(index) = segment.parse::<usize>() {
                    &mut current[index]
                } else {
                    &mut current[*segment]
                }
            });
            *target = json!("   ");
            assert_eq!(
                validation_error(value),
                CuratedValidationError::EmptyRequiredField { field }
            );
        }
    }
}
