# rumble-feed-mind — consignes agents

## Identité

`rumble-feed-mind` est un moteur personnel de veille souveraine, Rust-first, multi-plateforme. Il ingère des flux, normalise les articles, applique des règles explicables, prépare des décisions de lecture et distribue l'expérience sur CLI/API/web/desktop/mobile.

## Doctrine

- **Rust-first product stack + Portal** : domaine, règles, sync, contrats IA et adapters doivent tendre vers Rust ; les surfaces durables consomment Portal pour tokens, accessibilité, i18n UI et shells web/natifs.
- **Next.js legacy** : `apps/web` reste une référence fonctionnelle de migration, pas la cible long terme.
- **Adapters minces** : `api`, `worker`, `cli`, UI et shells de distribution ne portent pas la logique métier durable.
- **Explicabilité obligatoire** : une règle ou décision de tri doit produire une raison et, si possible, une evidence.
- **Event-minded** : préférer des événements métier rejouables (`FeedFetched`, `ArticleDiscovered`, `RuleEvaluated`) aux mutations opaques.
- **Souveraineté** : self-hostable, PostgreSQL/Redis, SQLite local si offline, Clever Cloud comme cible EU, pas de dépendance obligatoire à un hyperscaler US.
- **BYOK** : clés IA utilisateur chiffrées, jamais loggées, jamais commitées.
- **Preuve > promesse** : chaque incrément de refonte doit laisser une commande de vérification reproductible.

## Architecture cible

```text
crates/
  domain/   types purs et invariants transverses
  ingest/   fetch, parse, normalize, dedup
  rules/    règles, scoring, decisions, evidence
  ai/       traits providers, BYOK contracts, prompts
  sync/     event log, snapshots, import/export
  storage/  ports de persistance + impls
  api/      adapter HTTP Axum
  worker/   adapter jobs Redis/scheduler/fetch/evaluation
  cli/      diagnostics, import/export, opérations locales
apps/
  web-rs/   cible UI Rust/WASM consommant Portal
  desktop/  cible Tauri 2
  mobile/   cible SwiftUI/Compose via Portal ou Tauri mobile selon preuve produit
  web/      legacy Next.js, référence de migration
```

## Quality gates locaux

À exécuter avant tout commit de refonte Rust :

```bash
cargo fmt --all --check
cargo check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
agentic-harness goals report --config goals.toml
```

Pour le legacy web :

```bash
cd apps/web && npm run lint
```

## Règles de modification

- Lire les fichiers concernés avant édition.
- Préférer les petits changements réversibles.
- Ne pas introduire de `unwrap()` hors tests sans justification.
- Ne pas masquer une erreur par un `allow` global sans ADR ou commentaire local.
- Garder `Cargo.lock` versionné pour la reproductibilité des applications.
- Documenter toute décision structurante dans `docs/adr/`.
- Toute nouvelle dépendance majeure doit être justifiée par licence, souveraineté, maintenance et alternative rejetée.

## Priorité de refonte

1. Extraire `crates/domain` sans changement fonctionnel.
2. Faire consommer `domain` par le core existant.
3. Extraire `ingest` et `rules` avec tests de non-régression.
4. Modéliser `Decision`, `Evidence`, `Action` et les événements métier.
5. Faire de la CLI le premier client complet du core.
6. Préparer `apps/web-rs` seulement après stabilisation des contrats UI Portal.
7. Ajouter Tauri 2 après preuve web-rs/CLI/API.
