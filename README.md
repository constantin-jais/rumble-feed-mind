# rumble-feed-mind

Lecteur de flux intelligent, Rust-core, multi-plateforme.

## Intention

`rumble-feed-mind` transforme les flux RSS/Atom/JSON Feed en veille exploitable : import OPML, lecture, règles de tri, enrichissement IA en BYOK et distribution sur plusieurs surfaces utilisateur.

Le produit appartient à la couche **Rumble** : il porte l'expérience de lecture et de veille. Il ne doit pas devenir l'ingestion générique, l'orchestrateur agentique, ni le registre d'artefacts.

## Architecture

```text
crates/core    logique métier Rust : feeds, OPML, articles, règles, crypto
crates/api     API Axum : auth, REST, enveloppes, accès PostgreSQL/Redis
crates/worker  jobs async : fetch, rules, billing/dunning, planification
crates/cli     diagnostics et opérations locales
apps/web       frontend web/PWA
migrations     schéma PostgreSQL
```

Principe directeur : **Rust at core, adapters at edges**.

## Cibles de distribution

| Cible | Statut | Rôle |
| --- | --- | --- |
| Web/PWA | existant à consolider | surface principale de lecture |
| API self-hosted | existant à durcir | backend souverain PostgreSQL/Redis |
| Worker | existant à durcir | fetch et traitements asynchrones |
| CLI | squelette | import/export, diagnostics, admin local |
| Desktop Tauri | cible | shell multi-OS sans logique métier dupliquée |
| Mobile | cible | client mobile API-first ; offline natif à décider plus tard |

## Démarrage local

```bash
cargo check
cargo test --workspace
cd apps/web && npm install && npm run lint
```

Services locaux :

```bash
docker compose up -d postgres redis
```

## Gates de refonte

```bash
cargo fmt --all --check
cargo check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
agentic-harness goals report --config goals.toml
```

`cargo check` passe actuellement. Le passage de `clippy -D warnings` est un jalon de refonte : la base contient encore des API préparées mais non branchées.

## Documentation

- `AGENTS.md` — doctrine locale pour agents et contributeurs.
- `goals.toml` — suivi agentic-harness.
- `docs/refactor-plan.md` — plan de refonte Rust-core/multi-plateforme.
- `docs/adr/` — décisions structurantes.
