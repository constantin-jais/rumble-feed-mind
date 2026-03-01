# ADR 0002 — Stack produit Rust-first / Dioxus

## Statut

Acceptée.

## Contexte

L'écosystème Rumble converge vers une règle simple : les produits interactifs doivent partager une trajectoire Rust-first afin d'éviter la fragmentation frontend, la duplication des invariants métier et les dépendances implicites à des stacks différentes.

`rumble-feed-mind` avait une trajectoire historique web/mobile orientée TypeScript. Cette trajectoire reste utile comme référence fonctionnelle, mais elle ne doit pas devenir la destination durable du produit.

## Décision

`rumble-feed-mind` adopte la trajectoire commune des Rumble interactifs :

- cœur métier en crates Rust spécialisées ;
- API serveur Rust/Axum ;
- worker Rust/Tokio ;
- CLI comme consommateur de référence ;
- UI cible : **Dioxus** ;
- distribution desktop/mobile à évaluer via la trajectoire Dioxus/native/web du reste de l'écosystème ;
- surfaces TypeScript existantes : legacy/reference de migration, pas cible durable ;
- aucun invariant métier durable ne doit vivre uniquement côté TypeScript.

## Stack cible

| Couche | Choix | Raison |
| --- | --- | --- |
| Domaine | Rust crates pures (`domain`, `ingest`, `rules`, puis `ai`, `sync`, `storage`) | testabilité, portabilité, déterminisme |
| API | Axum + Tokio | existant, robuste, self-hostable |
| Jobs | Tokio worker + Redis Streams | existant, scalable, souverain si self-hosté |
| DB serveur | PostgreSQL + SQLx | existant, requêtes typées |
| Local cache futur | SQLite + SQLx | offline desktop/mobile possible |
| UI interactive | Dioxus | convergence Rumble, Rust-first, multi-target |
| Release | gear-cable à évaluer | artefacts reproductibles et distribution souveraine |

## Conséquences

- Les extractions de crates Rust priment sur les nouvelles fonctionnalités UI.
- La surface web existante reste une référence temporaire et une source de tests de comportement.
- Les nouveaux flux produit doivent être exprimés en domaine Rust avant UI.
- Les règles, décisions, explications et exports doivent rester testables sans frontend.
- Le produit reste compatible avec le harness : specs → packages/exports → validation → planification.

## Garde-fous

- Pas de big bang : la surface existante peut rester tant que Dioxus ne couvre pas les parcours critiques.
- Toute nouvelle dépendance frontend majeure nécessite ADR.
- Chaque extraction de crate doit garder `cargo test --workspace` vert.
- Aucun secret/BYOK ne doit être exposé au frontend durablement.
- Les fonctionnalités réutilisables d'ingestion doivent être candidates Wrench plutôt que rester cachées dans le produit.

## Alternatives rejetées

- **Expo durable** : rapide mobile-first, mais trop éloigné de la convergence Rust-first.
- **Next.js durable + Rust backend** : risque de duplication des invariants métier côté client.
- **Leptos/Tauri comme cible spécifique FeedMind** : cohérent Rust, mais crée une exception de stack par rapport aux autres Rumble interactifs.

## Notes licence

Le projet s'aligne sur la doctrine permissive de l'écosystème et utilise MIT au niveau workspace. Toute exception future doit être documentée par ADR/waiver.
