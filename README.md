# rumble-feed-mind

[![Rust CI](https://github.com/constantin-jais/rumble-feed-mind/actions/workflows/rust-ci.yml/badge.svg?branch=main)](https://github.com/constantin-jais/rumble-feed-mind/actions/workflows/rust-ci.yml)
[![Security](https://github.com/constantin-jais/rumble-feed-mind/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/constantin-jais/rumble-feed-mind/actions/workflows/security.yml)
[![Release](https://github.com/constantin-jais/rumble-feed-mind/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/constantin-jais/rumble-feed-mind/actions/workflows/release.yml)

Moteur personnel de veille souveraine, **Rust-first**, multi-plateforme.

## What it does

`rumble-feed-mind` explore un workflow de veille personnel : charger des abonnements, inspecter des articles, appliquer des règles explicables, puis préparer des exports auditables. Le besoin utilisateur est simple : réduire le bruit d'une veille sans confier ses lectures, ses règles ou ses clés à une plateforme fermée.

## Stack role

- **Layer :** Rumble — Product.
- **Role :** pipeline produit de veille et curation de flux.
- **Mission :** transformer des flux RSS/Atom/JSON Feed en veille explicable, exportable et réutilisable.
- **Maturity :** `dojo`.
- **Scale-ready :** no — la preuve Rust/API existe, mais le workflow utilisateur complet, les logs classifiés et les waivers d'advisories restent à durcir.
- **Current increment :** P0 contrats + P1 preuve Rust/API.
- **Learning value :** veille, curation, règles déterministes, OPML/feed smokes, export/handoff contracts.
- **Next quality step :** tests runtime `CuratedItemExport`, classification logs, retrait des waivers advisories.

Voir le cockpit écosystème : [`constantin-jais/ecosystem/status.md`](https://github.com/constantin-jais/constantin-jais/blob/main/ecosystem/status.md).

## Dogfooding

This repository is part of the forge dogfooding loop: the ecosystem should use its own tools to make specs, maturity, contracts, releases, and product documentation observable.

Current visible evidence:

- Rust CI, security, and release workflows exercise the feed pipeline surface;
- contracts and fixtures frame ingest, curation, BYOK, and export behavior;
- README maturity notes keep advisories and runtime hardening limits explicit.

Expected next evidence:

- publish example curated-item exports and classification logs;
- make fail-closed feed cases visible as fixtures.

Dogfooding claims should stay backed by visible commands, fixtures, CI workflows, generated reports, or linked docs.

## Usable today

Ce qui fonctionne localement sans base de données ni secret :

- parser un fichier OPML et afficher un résumé JSON ;
- évaluer une règle regex sur un article JSON ;
- afficher l'aide CLI ;
- lancer les tests Rust du workspace ;
- produire des artefacts CLI tag/manual via le workflow release.

Les commandes `import`, `export`, `create-user` et `stats` existent aussi, mais elles demandent `DATABASE_URL` et ne font pas partie du quickstart sans secret.

## Quickstart

```bash
cargo test --workspace
cargo run -p feedmind-cli -- --help
cargo run -p feedmind-cli -- opml-summary --file examples/demo.opml
cargo run -p feedmind-cli -- evaluate-rule \
  --article examples/demo-article.json \
  --rule examples/demo-rule.json
```

## Example output

Résumé OPML :

```txt
{
  "title": "FeedMind demo subscriptions",
  "feed_count": 2,
  "folder_count": 1
}
```

Évaluation de règle :

```txt
{
  "matched": true,
  "action": "star",
  "deciding_rule": "Keep Rust sovereignty articles"
}
```

## Target demo

La démo produit cible n'est pas encore complète : `OPML → fetch feeds → rules → curated export JSON`. Aujourd'hui, seules les briques locales sans base (`opml-summary`, `fetch-feed`, `evaluate-rule`) et les contrats d'export sont prouvés séparément.

## Not scale-ready yet

- Pas encore de workflow utilisateur complet OPML → export curaté en une commande.
- Pas d'observabilité ni de runbook opérationnel.
- Pas de déploiement self-hosted documenté de bout en bout.
- Pas de contraintes multi-utilisateur/load testées.
- Waivers RustSec temporaires à retirer ou renouveler avant release.
- Les surfaces UI Rust/desktop/mobile restent des cibles, pas des garanties actuelles.

## Next product milestone

Fournir une démo locale unique `examples/demo.opml` + fixtures article/règle → `out/curated.json`, sans base de données et avec sortie lisible par un humain.

## Intention

`rumble-feed-mind` vise à transformer les flux RSS/Atom/JSON Feed en veille exploitable : import OPML, lecture, règles de tri, explicabilité et export. Aujourd'hui, les preuves locales couvrent surtout l'OPML, les règles et des contrats d'export ; l'enrichissement fournisseur/BYOK et les surfaces utilisateur restent cadrés mais non vendus comme prêts.

Le projet ne vise pas seulement un lecteur RSS. Il explore un système de décision personnel : ingestion, normalisation, qualification, explication, synchronisation et export, livré par incréments vérifiables.

Le produit appartient à la couche **Rumble** : il porte l'expérience de lecture et de veille. Il ne doit pas devenir l'ingestion générique, l'orchestrateur agentique, ni le registre d'artefacts.

## Cap stack

Principe directeur : **Rust-first product stack**.

- Les invariants métier vivent en Rust.
- Les adapters sont minces.
- Les surfaces utilisateur durables migrent vers Rust.
- Next.js reste une référence transitoire, pas la cible long terme.

## Architecture cible

```text
crates/domain   types purs : Feed, Article, Rule, Decision, Opml, UserScope
crates/ingest   fetch, parse, normalize, dedup
crates/rules    règles déterministes, scoring, explications, evidence
crates/ai       traits providers, BYOK contracts, prompts auditables
crates/sync     event log, snapshots, import/export, offline-ready contracts
crates/storage  ports de persistance + impl PostgreSQL/SQLite si besoin
crates/api      adapter HTTP Axum
crates/worker   adapter jobs Tokio/Redis
crates/cli      diagnostics et opérations locales
apps/web-rs     cible UI Rust Leptos/WASM
apps/desktop    cible Tauri 2
apps/mobile     cible Tauri mobile ou shell Rust-first à valider
apps/web        legacy Next.js, référence de migration
migrations      schéma PostgreSQL serveur
```

## Cibles de distribution

| Cible | Statut | Rôle |
| --- | --- | --- |
| CLI Rust | priorité immédiate | prouver le core sans UI |
| API self-hosted | existant à durcir | backend souverain PostgreSQL/Redis |
| Worker Rust | existant à durcir | fetch et traitements asynchrones |
| Web Rust/Leptos | cible | surface web durable |
| Desktop Tauri 2 | cible | app Linux/macOS/Windows |
| Mobile Rust-first | cible expérimentale | distribution mobile sans dupliquer le métier |
| Next.js | legacy | référence fonctionnelle pendant migration |

## Gates de refonte

```bash
cargo fmt --all --check
cargo check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
agentic-harness goals report --config goals.toml
```

Frontend legacy :

```bash
cd apps/web && npm run lint
```

## Documentation

- `AGENTS.md` — doctrine locale pour agents et contributeurs.
- `goals.toml` — suivi agentic-harness.
- `docs/refactor-plan.md` — plan de refonte Rust-first.
- `docs/launch-target.md` — cible lancement produit et release train.
- `docs/adr/0001-rust-core-multiplatform.md` — décision initiale prudente.
- `docs/adr/0002-rust-first-product-stack.md` — pivot stack option C.
