# ADR 0002 — Pivot stack Rust-first produit

## Statut

Acceptée.

## Contexte

L'ADR 0001 posait une trajectoire prudente : Rust au cœur, web/PWA existante comme surface principale, desktop Tauri plus tard, mobile API-first. Cette trajectoire protège la livraison court terme, mais elle ne suffit pas pour l'ambition produit : distribution multi-plateforme souveraine avec un maximum d'invariants, d'UI state et de logique de synchronisation contrôlés en Rust.

Le projet pivote donc vers l'option C : **Rust-first sur le cœur, les adapters et les surfaces produit**, en gardant les composants existants comme référence de migration, pas comme cible durable.

## Décision

- Le cœur métier est éclaté progressivement en crates Rust spécialisées : `domain`, `ingest`, `rules`, `ai`, `sync`, `storage`.
- L'API serveur reste Rust/Axum.
- Le worker reste Rust/Tokio.
- La CLI devient le premier consommateur de référence du core.
- La nouvelle UI cible est Rust : **Leptos** pour l'interface web/WASM.
- La distribution desktop/mobile cible est **Tauri 2** autour de l'UI Rust/WebView.
- L'application Next.js existante devient `legacy web` : référence fonctionnelle et source de migration, mais pas destination long terme.
- Les clients ne doivent pas porter d'invariants métier en TypeScript.

## Stack cible

| Couche | Choix | Raison |
| --- | --- | --- |
| Domaine | Rust crates pures | testabilité, portabilité, déterminisme |
| API | Axum + Tokio | existant, robuste, souverain |
| Jobs | Tokio worker + Redis Streams | existant, scalable, self-hostable |
| DB serveur | PostgreSQL + SQLx | existant, requêtes typées |
| Local cache futur | SQLite + SQLx | offline desktop/mobile possible |
| Web UI | Leptos/WASM | UI Rust, SSR/CSR possible, bonne intégration web |
| Desktop/mobile shell | Tauri 2 | packaging multi-OS, surface système minimale |
| Release | cargo-dist/gear-cable à évaluer | artefacts reproductibles |

## Conséquences

- La priorité n'est plus de consolider Next.js, mais d'extraire les contrats UI et de préparer une UI Rust.
- Le TypeScript restant est toléré uniquement comme transition.
- Les nouveaux états UI complexes doivent être modélisés en Rust si réutilisables.
- Les tests de domaine précèdent toute migration de surface.
- Le risque UI augmente : Leptos/Tauri mobile sont moins standards qu'un couple Next/Expo. Ce risque est accepté pour maximiser souveraineté, cohérence et distribution Rust-first.

## Garde-fous

- Pas de big bang : Next.js reste disponible tant que la nouvelle UI ne couvre pas les parcours critiques.
- Pas d'ajout Tauri/mobile tant que les crates domaine et contrats UI ne sont pas stabilisés.
- Chaque extraction de crate doit garder `cargo test` et `clippy -D warnings` verts.
- Chaque nouveau choix de crate majeure doit passer par ADR et audit licence.

## Alternatives rejetées

- **Next.js durable + Rust backend** : trop proche d'un SaaS web classique, duplication probable des invariants côté client.
- **Expo mobile durable** : rapide, mais augmente la surface TypeScript et la dépendance à un écosystème non-Rust.
- **Dioxus unique pour toutes les surfaces** : intéressant mais moins aligné avec le besoin serveur/API existant et packaging Tauri déjà naturel pour desktop/mobile.
