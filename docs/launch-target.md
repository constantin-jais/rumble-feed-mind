# Cible lancement produit — Rust-first

## Définition du lancement

Le lancement produit cible n'est pas une simple démo web. C'est une distribution self-hostable et reproductible du moteur de veille : CLI, API, worker, base PostgreSQL/Redis, puis surface web Rust et desktop.

## Release train

### RC0 — Core proof

- `feedmind-domain` extrait.
- Décisions et événements métier disponibles.
- CLI capable d'exécuter un parcours local minimal.
- Smoke sans base : `feedmind-cli opml-summary --file <file.opml>`.
- Smoke réseau : `feedmind-cli fetch-feed --url <feed-url>`.
- Smoke règles : `feedmind-cli evaluate-rule --article <article.json> --rule <rule.json>`.
- Gates Rust strictes vertes.

### RC1 — Server self-hostable

- API Axum branchée sur les ports domaine/storage.
- Worker fetch/rules opérationnel.
- Docker Compose propre PostgreSQL/Redis.
- Smoke test local documenté.

### RC2 — Product surface Rust

- `apps/web-rs` Leptos couvre les parcours critiques : feeds, articles, règles.
- Next.js reste legacy uniquement.
- API contract stabilisé.

### RC3 — Multi-platform

- Desktop Tauri 2 Linux/macOS/Windows.
- Artefacts versionnés et checksums.
- Release notes et procédure rollback.

## Parcours critique launch

1. Démarrer PostgreSQL/Redis.
2. Créer un utilisateur local.
3. Importer OPML.
4. Fetcher les feeds.
5. Afficher les articles.
6. Créer une règle.
7. Voir une décision expliquée avec evidence.
8. Exporter les données utilisateur.

## Gates launch

```bash
cargo fmt --all --check
cargo check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
agentic-harness goals report --config goals.toml
```

À ajouter avant RC1 :

```bash
cargo deny check
cargo audit
```

Legacy web, tant qu'il existe :

```bash
cd apps/web && npm run lint
```

## Issues GitHub de pilotage

- #1 — Epic refonte Rust-first totale.
- #2 — Extraction `feedmind-domain`.
- #3 — Split `ingest/rules/sync/storage`.
- #4 — CLI client de référence.
- #5 — UI Rust Leptos puis Tauri 2.
