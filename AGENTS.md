# rumble-feed-mind — consignes agents

## Identité

`rumble-feed-mind` est un lecteur de flux intelligent multi-plateforme. Le cœur produit doit rester en Rust : parsing RSS/Atom/JSON Feed, OPML, règles, crypto, synchronisation, file de jobs et invariants métier.

## Doctrine

- **Rust at core** : toute règle métier réutilisable vit dans `crates/core` ou dans un crate Rust dédié.
- **Adapters minces** : `api`, `worker`, `cli`, web/desktop/mobile ne portent pas la logique métier durable.
- **Distribution multi-plateforme** : web/PWA d'abord, desktop via Tauri ensuite, mobile via client API ; UniFFI seulement si un vrai besoin offline natif apparaît.
- **Souveraineté** : self-hostable, PostgreSQL/Redis, Clever Cloud comme cible EU, pas de dépendance obligatoire à un hyperscaler US.
- **BYOK** : clés IA utilisateur chiffrées, jamais loggées, jamais commitées.
- **Preuve > promesse** : chaque incrément de refonte doit laisser une commande de vérification reproductible.

## Architecture cible

```text
crates/
  core/      domaine pur : feeds, articles, règles, OPML, crypto, contrats IA
  api/       adapter HTTP Axum + auth + pagination + enveloppes API
  worker/    adapter jobs Redis/scheduler/fetch/evaluation
  cli/       diagnostics, import/export, opérations locales
apps/
  web/       client web/PWA
  desktop/   cible future Tauri, shell fin autour du web/core via API locale si besoin
  mobile/    cible future Expo ou client natif, sans logique métier dupliquée
```

## Quality gates locaux

À exécuter avant tout commit de refonte :

```bash
cargo fmt --all --check
cargo check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
agentic-harness goals report --config goals.toml
```

État actuel : `cargo check` passe, mais `clippy -D warnings` est un objectif de refonte, pas encore un invariant acquis.

## Règles de modification

- Lire les fichiers concernés avant édition.
- Préférer les petits changements réversibles.
- Ne pas introduire de `unwrap()` hors tests sans justification.
- Ne pas masquer une erreur par un `allow` global sans ADR ou commentaire local.
- Garder `Cargo.lock` versionné pour la reproductibilité des applications.
- Documenter toute décision structurante dans `docs/adr/`.

## Priorité de refonte

1. Stabiliser le domaine `crates/core` sous tests.
2. Réduire les warnings puis activer `clippy -D warnings` en CI.
3. Découpler API/worker des détails métier.
4. Ajouter smoke tests API/worker et scripts de release multi-target.
5. Préparer les shells web/desktop/mobile sans dupliquer la logique.
