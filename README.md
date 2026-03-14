# rumble-feed-mind

Moteur personnel de veille souveraine, **Rust-first**, multi-plateforme.

## Rôle dans la stack

- **Layer :** Rumble — dojo produit.
- **Maturité :** `dojo`, prêt pour planification d'implémentation cadrée.
- **Incrément courant :** P0 contrats + P1 preuve Rust/API.
- **Apprentissage :** veille, ingestion, règles explicables, BYOK, export/handoff.
- **Prochaine étape qualité :** tests runtime `CuratedItemExport`, classification logs, retrait des waivers advisories.

Voir le cockpit écosystème : [`constantin-jais/ecosystem/status.md`](https://github.com/constantin-jais/constantin-jais/blob/main/ecosystem/status.md).

## Intention

`rumble-feed-mind` transforme les flux RSS/Atom/JSON Feed en veille exploitable : import OPML, lecture, règles de tri, enrichissement IA en BYOK, explicabilité et distribution sur plusieurs surfaces utilisateur.

Le projet ne vise pas seulement un lecteur RSS avec IA. Il vise un système de décision personnel : ingestion, normalisation, qualification, explication, synchronisation et export.

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
