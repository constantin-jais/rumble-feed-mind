//! Local, read-only Feed Radar product slice over the real curated export contract.

use dioxus::prelude::*;
use feedmind_sync::curated::{CuratedItemExport, CuratedItemValidationError};
use std::fmt;

const GOLDEN_EXPORT: &str = include_str!("../../../examples/expected-curated-export.json");
const DESIGN_TOKENS: Asset = asset!("/assets/libre-ia/tokens.css");
const DESIGN_THEMES: Asset = asset!("/assets/libre-ia/themes.css");
const DESIGN_COMPONENTS: Asset = asset!("/assets/libre-ia/components.css");
const STYLES: Asset = asset!("/assets/styles.css");

#[derive(Debug, Clone, PartialEq)]
pub enum ReviewState {
    Ready(Box<CuratedItemExport>),
    Empty,
    Error(ReviewLoadError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewLoadError {
    InvalidJson,
    UnsafeContract(CuratedItemValidationError),
}

impl fmt::Display for ReviewLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson => formatter.write_str("L’export local est illisible."),
            Self::UnsafeContract(_) => formatter
                .write_str("L’export local ne respecte pas les contraintes de sécurité attendues."),
        }
    }
}

pub fn load_review(raw: &str) -> ReviewState {
    if raw.trim().is_empty() {
        return ReviewState::Empty;
    }
    match serde_json::from_str::<CuratedItemExport>(raw) {
        Ok(export) => match export.validate_client_safe() {
            Ok(()) => ReviewState::Ready(Box::new(export)),
            Err(error) => ReviewState::Error(ReviewLoadError::UnsafeContract(error)),
        },
        Err(_) => ReviewState::Error(ReviewLoadError::InvalidJson),
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DESIGN_TOKENS }
        document::Link { rel: "stylesheet", href: DESIGN_THEMES }
        document::Link { rel: "stylesheet", href: DESIGN_COMPONENTS }
        document::Link { rel: "stylesheet", href: STYLES }
        {render_state(load_review(GOLDEN_EXPORT))}
    }
}

pub fn render_state(state: ReviewState) -> Element {
    match state {
        ReviewState::Ready(export) => render_ready(export),
        ReviewState::Empty => rsx! {
            main { class: "shell",
                section { class: "state-card", aria_live: "polite",
                    p { class: "eyebrow", "Démonstration locale" }
                    h1 { "Aucun export local disponible" }
                    p { "Générez le contrat CuratedItemExport avec la CLI avant de relancer cette preuve." }
                }
            }
        },
        ReviewState::Error(error) => rsx! {
            main { class: "shell",
                section { class: "state-card error", role: "alert",
                    p { class: "eyebrow", "Export refusé" }
                    h1 { "La revue ne peut pas afficher cet élément" }
                    p { "{error}" }
                    p { class: "muted", "Aucun contenu partiel ni payload brut n’est rendu." }
                }
            }
        },
    }
}

fn render_ready(export: Box<CuratedItemExport>) -> Element {
    let evidence = &export.rule_evidence[0];
    let confidence = format!("{:.0} %", evidence.confidence * 100.0);
    let source_title = export
        .source_ref
        .first_feed_title
        .as_deref()
        .unwrap_or("Source locale");
    rsx! {
        main { class: "shell",
            header { class: "hero",
                div {
                    p { class: "eyebrow", "Feed Radar · démonstration locale" }
                    h1 { "Une information retenue pour une raison visible." }
                    p { class: "lede", "Cette page relit un export déterministe produit par le pipeline Rust. Elle n’appelle ni API, ni fournisseur IA, ni base de données." }
                }
                div { class: "status", aria_label: "Décision de curation",
                    span { class: "status-mark", aria_hidden: "true", "✓" }
                    span { "{export.curation.decision}" }
                }
            }

            article { class: "review-card", aria_labelledby: "item-title",
                div { class: "item-head",
                    p { class: "source", "{source_title}" }
                    h2 { id: "item-title", "{export.item.title}" }
                    p { "{export.item.content_excerpt}" }
                }

                section { class: "decision", aria_labelledby: "decision-title",
                    p { class: "eyebrow", "Raison décisive" }
                    h3 { id: "decision-title", "{export.curation.reason}" }
                    p { "{evidence.explanation}" }
                    dl { class: "metrics",
                        div { dt { "Décision de règle" } dd { "{evidence.decision}" } }
                        div { dt { "Confiance déclarée" } dd { "{confidence}" } }
                        div { dt { "Éléments du flux" } dd { "{export.source_ref.opml_feed_count}" } }
                    }
                }

                if !export.item.tags.is_empty() {
                    ul { class: "tags", aria_label: "Thèmes",
                        for tag in &export.item.tags {
                            li { "{tag}" }
                        }
                    }
                }

                details { class: "proof",
                    summary { "Examiner la preuve technique" }
                    dl {
                        div { dt { "Hash de l’evidence" } dd { code { "{evidence.evidence_hash}" } } }
                        div { dt { "Hash du contenu" } dd { code { "{export.item.content_hash}" } } }
                        div { dt { "Provenance" } dd { code { "{export.provenance_ref.provenance_id}" } } }
                        div { dt { "Classification" } dd { "{export.privacy_classification}" } }
                    }
                }
            }

            aside { class: "limits", aria_label: "Limites de la démonstration",
                h2 { "Ce que cette preuve ne démontre pas" }
                p { "Pas d’import interactif, de fetch réseau, de compte, de stockage navigateur, de synchronisation ou de support mobile natif." }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_declares_the_pinned_design_assets() {
        assert!(DESIGN_TOKENS.to_string().contains("tokens.css"));
        assert!(DESIGN_THEMES.to_string().contains("themes.css"));
        assert!(DESIGN_COMPONENTS.to_string().contains("components.css"));
        assert!(STYLES.to_string().contains("styles.css"));
    }

    #[test]
    fn golden_fixture_renders_the_complete_decision() {
        let html = dioxus_ssr::render_element(render_state(load_review(GOLDEN_EXPORT)));
        assert!(html.contains("Rust-first local feed curation"));
        assert!(html.contains("Keep Rust sovereignty articles"));
        assert!(html.contains("Title matches pattern"));
        assert!(html.contains("100 %"));
        assert!(html.contains("Examiner la preuve technique"));
    }

    #[test]
    fn unsafe_fixture_renders_no_partial_content() {
        let mut export: CuratedItemExport = serde_json::from_str(GOLDEN_EXPORT).unwrap();
        export.constraints.contains_secrets = true;
        let raw = serde_json::to_string(&export).unwrap();
        let html = dioxus_ssr::render_element(render_state(load_review(&raw)));
        assert!(html.contains("Export refusé"));
        assert!(!html.contains("Rust-first local feed curation"));
        assert!(!html.contains(&export.item.content_hash));
    }

    #[test]
    fn empty_fixture_is_honest() {
        let html = dioxus_ssr::render_element(render_state(load_review("  ")));
        assert!(html.contains("Aucun export local disponible"));
    }
}
