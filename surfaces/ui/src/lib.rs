//! Local, read-only Feed Radar product slice over the real curated export contract.

use dioxus::prelude::*;
use feedmind_sync::curated::{CuratedItemExport, CuratedValidationError};
use std::fmt;

const BUNDLED_EXPORT: &str = include_str!(concat!(env!("OUT_DIR"), "/review-export.json"));
const REVIEW_MODE: &str = include_str!(concat!(env!("OUT_DIR"), "/review-mode.txt"));
const DESIGN_TOKENS: Asset = asset!("/assets/libre-ia/tokens.css");
const DESIGN_THEMES: Asset = asset!("/assets/libre-ia/themes.css");
const DESIGN_COMPONENTS: Asset = asset!("/assets/libre-ia/components.css");
const STYLES: Asset = asset!("/assets/styles.css");

#[derive(Clone, PartialEq)]
pub enum ReviewState {
    Ready(Box<CuratedItemExport>),
    Empty,
    Error(ReviewLoadError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewLoadError {
    InvalidJson,
    UnsafeContract(CuratedValidationError),
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
        {render_state_with_mode(load_review(BUNDLED_EXPORT), REVIEW_MODE.trim() == "live-sync")}
    }
}

pub fn render_state(state: ReviewState) -> Element {
    render_state_with_mode(state, false)
}

fn render_state_with_mode(state: ReviewState, live_sync: bool) -> Element {
    match state {
        ReviewState::Ready(export) => render_ready(export, live_sync),
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

fn render_ready(export: Box<CuratedItemExport>, live_sync: bool) -> Element {
    let evidence = &export.rule_evidence[0];
    let confidence = format!("{:.0} %", evidence.confidence * 100.0);
    let source_title = export
        .source_ref
        .first_feed_title
        .as_deref()
        .unwrap_or("Source locale");
    let mode = if live_sync { "live-sync" } else { "fixture" };
    let eyebrow = if live_sync {
        "Feed Radar · synchronisation publique bornée"
    } else {
        "Feed Radar · démonstration locale"
    };
    let introduction = if live_sync {
        "Cette page relit un export produit depuis des flux publics explicitement autorisés. Le bundle reste statique : aucun fetch, compte ou fournisseur IA n’est appelé dans le navigateur."
    } else {
        "Cette page relit un export déterministe produit par le pipeline Rust. Elle n’appelle ni API, ni fournisseur IA, ni base de données."
    };
    rsx! {
        main { class: "shell", "data-review-mode": mode,
            header { class: "hero",
                div {
                    p { class: "eyebrow", "{eyebrow}" }
                    h1 { "Une information retenue pour une raison visible." }
                    p { class: "lede", "{introduction}" }
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
                if live_sync {
                    p { "L’import et la synchronisation ont lieu avant le build via la CLI locale. Pas encore d’import interactif, de compte, de stockage navigateur, d’actualisation en arrière-plan ou de support mobile natif." }
                } else {
                    p { "Pas d’import interactif, de fetch réseau, de compte, de stockage navigateur, de synchronisation ou de support mobile natif." }
                }
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
    fn bundled_export_renders_its_complete_decision() {
        let export: CuratedItemExport = serde_json::from_str(BUNDLED_EXPORT).unwrap();
        let html = dioxus_ssr::render_element(render_state(load_review(BUNDLED_EXPORT)));
        assert!(html.contains(&export.item.title));
        assert!(html.contains(&export.curation.reason));
        assert!(html.contains(&export.rule_evidence[0].explanation));
        assert!(html.contains("Examiner la preuve technique"));
    }

    #[test]
    fn live_sync_bundle_is_labelled_without_claiming_browser_fetch() {
        let html =
            dioxus_ssr::render_element(render_state_with_mode(load_review(BUNDLED_EXPORT), true));
        assert!(html.contains("data-review-mode=\"live-sync\""));
        assert!(html.contains("synchronisation publique bornée"));
        assert!(html.contains("avant le build via la CLI locale"));
    }

    #[test]
    fn unsafe_fixture_renders_no_partial_content() {
        let mut export: CuratedItemExport = serde_json::from_str(BUNDLED_EXPORT).unwrap();
        export.constraints.contains_secrets = true;
        let title = export.item.title.clone();
        let raw = serde_json::to_string(&export).unwrap();
        let html = dioxus_ssr::render_element(render_state(load_review(&raw)));
        assert!(html.contains("Export refusé"));
        assert!(!html.contains(&title));
        assert!(!html.contains(&export.item.content_hash));
    }

    #[cfg(feature = "web")]
    #[test]
    fn web_sys_location_and_window_features_are_available_for_all_features() {
        let compile_check = |window: &web_sys::Window| {
            let _ = window.location();
        };
        let _ = compile_check;
    }

    #[test]
    fn empty_fixture_is_honest() {
        let html = dioxus_ssr::render_element(render_state(load_review("  ")));
        assert!(html.contains("Aucun export local disponible"));
    }
}
