# ADR 0001 — Rust-core et distribution multi-plateforme

## Statut

Supersédée par [ADR 0002 — Pivot stack Rust-first produit](0002-rust-first-product-stack.md).

## Note

Cette ADR documente la trajectoire prudente initiale. Elle est conservée pour l'historique, mais le cap actuel est l'option C : Rust-first produit, voir ADR 0002.

## Contexte

Le projet doit devenir distribuable sur plusieurs plateformes sans dupliquer les invariants métier. Le repo contient déjà un backend Rust, un worker, une CLI et un frontend web. La priorité est de fiabiliser le cœur plutôt que d'ajouter immédiatement des shells de distribution.

## Décision

- `crates/core` est la source de vérité métier.
- `crates/api`, `crates/worker`, `crates/cli` sont des adapters.
- La première surface multi-plateforme reste web/PWA.
- Le desktop visé est Tauri, ajouté après stabilisation des gates Rust.
- Le mobile reste API-first dans un premier temps.
- UniFFI ou bindings natifs ne seront introduits qu'après preuve d'un besoin offline/local fort.

## Conséquences

- Les nouvelles règles métier doivent être testées dans Rust avant intégration UI.
- Les clients ne doivent pas réimplémenter parsing, règles ou crypto.
- Les releases devront couvrir au minimum les binaires Rust et la web app.
- La complexité native mobile est différée, ce qui réduit le risque de refonte initiale.

## Alternatives rejetées

- **Frontend-first** : rapide visuellement, mais laisse la dette métier se multiplier.
- **Tout partager en natif via UniFFI immédiatement** : puissant mais trop coûteux avant validation du besoin offline.
- **Backend SaaS uniquement** : incompatible avec l'objectif self-hostable/souverain.
