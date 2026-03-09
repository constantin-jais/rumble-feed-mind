//! Rust UI shell for FeedMind.
//!
//! This crate is the durable Rust-first web surface. The existing Next.js app
//! remains a legacy reference until this UI covers the critical launch flows.

use feedmind_domain::FeedType;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="feedmind-shell">
            <Hero />
            <LaunchChecklist />
        </main>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let supported = [FeedType::Rss, FeedType::Atom, FeedType::JsonFeed]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" / ");

    view! {
        <section>
            <p>"Rumble Feed Mind"</p>
            <h1>"Veille souveraine, Rust-first."</h1>
            <p>"Formats supportés : " {supported}</p>
        </section>
    }
}

#[component]
fn LaunchChecklist() -> impl IntoView {
    let items = [
        "Importer OPML",
        "Fetcher les flux",
        "Évaluer les règles avec evidence",
        "Exporter un snapshot",
    ];

    view! {
        <section>
            <h2>"Parcours critique lancement"</h2>
            <ul>
                {items.into_iter().map(|item| view! { <li>{item}</li> }).collect_view()}
            </ul>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_feed_types_are_visible() {
        let labels = [FeedType::Rss, FeedType::Atom, FeedType::JsonFeed]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        assert_eq!(labels, ["rss", "atom", "json"]);
    }
}
