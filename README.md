> [!WARNING]
> **Frozen on 2026-07-16 — reserved as the future home of Radar ([monorepo ADR-0008](https://github.com/libre-ai/libre-ai/blob/main/docs/adr/0008-multi-repo-target-topology-and-brand.md)).**
> Radar is being rebuilt from locked contracts in the canonical base repository [`libre-ai/libre-ai`](https://github.com/libre-ai/libre-ai) (target: `apps/radar`). This repository will reopen as the real product repository when the owner activates it. Everything below describes the pre-freeze state and no longer reflects the current architecture or roadmap.

<p align="center">
  <img src=".github/assets/repository-card.svg" alt="Libre AI Feed Radar, represented by a radar selecting explainable feed items." width="100%">
</p>

# Libre AI Feed Radar

A local-first feed intelligence pipeline: OPML, RSS, Atom and JSON Feed into explainable curated exports.

[![Rust CI](https://github.com/libre-ai/feed-radar/actions/workflows/rust-ci.yml/badge.svg?branch=main)](https://github.com/libre-ai/feed-radar/actions/workflows/rust-ci.yml)
[![Security](https://github.com/libre-ai/feed-radar/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/libre-ai/feed-radar/actions/workflows/security.yml)
[![Contracts](https://github.com/libre-ai/feed-radar/actions/workflows/contracts.yml/badge.svg?branch=main)](https://github.com/libre-ai/feed-radar/actions/workflows/contracts.yml)
[![Release](https://github.com/libre-ai/feed-radar/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/libre-ai/feed-radar/actions/workflows/release.yml)

## Status

| | |
| --- | --- |
| Maturity | **Dojo** — usable CLI/API proofs, incomplete product workflow |
| Works today | local OPML parsing, bounded allowlisted HTTPS synchronization, rule evaluation, payload-minimized replay state, worker scheduler code with validated refresh-interval bounds and one-shot start guard, client-safe exports and one read-only Dioxus curated-review proof |
| Not scale-ready | interactive import, hosted operations, multi-user guarantees, and production observability; hosted operations remain unproved |
| Product UI proof | runnable Dioxus web slice over either the deterministic golden or a validated live-sync export, with pinned Client Kit provenance and Chromium/Firefox/WebKit mobile-width checks; not an API or release |
| Historical IDs | `rumble-feed-mind` and `feedmind-*` remain technical package/contract IDs |

Canonical readiness cockpit: [`docs/product-readiness.md`](docs/product-readiness.md).

Temporary RustSec waivers documented in the repository must be removed or renewed before their stated expiry; they are not a production-readiness claim.

## Quickstart without secrets

```bash
cargo test --workspace
cargo run -p feedmind-cli -- opml-summary --file examples/demo.opml
cargo run -p feedmind-cli -- evaluate-rule \
  --article examples/demo-article.json \
  --rule examples/demo-rule.json
cargo run -p feedmind-cli -- demo-curate \
  --opml examples/demo.opml \
  --article examples/demo-article.json \
  --rule examples/demo-rule.json \
  --output out/curated.json
cargo run -p feedmind-cli -- validate-curated-export --file out/curated.json
diff -u examples/expected-curated-export.json out/curated.json
```

The optional `demo-curate-live` command uses the network and is intentionally excluded from deterministic CI.

### Bounded live synchronization

`sync-curated` imports an OPML set, validates every fetch and redirect against exact HTTPS hosts, caps sources/items/body size, deduplicates through a hash-only local state and removes stale output when no new signal exists:

```bash
cargo run -p feedmind-cli -- sync-curated \
  --opml examples/demo.opml \
  --rule examples/demo-rule.json \
  --output target/live/curated.json \
  --state target/live/state.json \
  --allow-host www.clever-cloud.com \
  --allow-host clever.cloud \
  --allow-host www.clever.cloud \
  --allow-host blog.rust-lang.org
```

Redirect hosts are intentionally not inferred. The command is local and does not make the product publicly available.

## Local product journey

The Dioxus slice renders a validated `CuratedItemExport` through the portable `feedmind-sync` contract. Deterministic builds use the golden; a live proof can inject the output of `sync-curated` at build time. The browser displays the selected item, decision, reason, explanation, confidence and technical evidence without database, account, runtime network request or browser storage.

```bash
python3 scripts/verify-design-system.py
cargo test -p feedmind-sync -p feedmind-app
cargo check -p feedmind-domain -p feedmind-sync -p feedmind-app \
  --target wasm32-unknown-unknown --features feedmind-app/web
./scripts/build-feedmind-app.sh
npm ci --prefix e2e
npm --prefix e2e test
```

The bundle carries the verified Libre IA Design System 2.0 CSS from the pinned Client Kit builder revision and rejects provenance drift, remote assets and failed contrast checks. Generate the dated networked traversal with `scripts/generate-live-radar-proof.sh` and explicit `--allow-host` arguments. This remains a local, read-only product proof: it does not establish interactive import, PWA, hosted, desktop, native mobile or multi-user availability.

## PostgreSQL tenant isolation

[ADR 0006](docs/adr/0006-tenant-context-and-row-level-security.md) is enforced below the adapters: 18 tenant tables enable and force RLS, API/CLI user work uses transaction-local `app.user_id`, authentication is restricted to fixed-`search_path` functions, and worker access is granted per table without ownership or `BYPASSRLS`.

Local development provisions separate group and login roles on a fresh volume:

```bash
docker compose down -v # required once when upgrading the former single-role volume
docker compose up -d

export MIGRATION_DATABASE_URL='postgresql://feed_radar_migrator_dev:feed_radar_migrator_dev@localhost:5434/feedmind?options=-c%20role%3Dfeed_radar_owner'
export DATABASE_URL='postgresql://feed_radar_app_dev:feed_radar_app_dev@localhost:5434/feedmind?options=-c%20role%3Dfeed_radar_app'
export AUTH_DATABASE_URL='postgresql://feed_radar_auth_dev:feed_radar_auth_dev@localhost:5434/feedmind?options=-c%20role%3Dfeed_radar_auth'
export WORKER_DATABASE_URL='postgresql://feed_radar_worker_dev:feed_radar_worker_dev@localhost:5434/feedmind?options=-c%20role%3Dfeed_radar_worker'

cargo run -p feedmind-cli -- migrate
```

These credentials are local fixtures only. Production creates independent login principals outside product migrations and grants each exactly one NOLOGIN group role from [`scripts/postgres/provision-roles.sql`](scripts/postgres/provision-roles.sql). Existing single-role databases first run the explicit, product-object-only [`transfer-legacy-ownership.sql`](scripts/postgres/transfer-legacy-ownership.sql) as an administrator. API and worker configuration never receives `MIGRATION_DATABASE_URL`.

## Database inspection gate

[`db-security-manifest.json`](db-security-manifest.json) classifies every PostgreSQL table and records tenant derivation without granting waivers. Run the same ordered inspection used in CI:

```bash
mkdir -p target/db-inspect
for migration in migrations/*.sql; do
  cat "$migration"
  printf '\n'
done > target/db-inspect/schema.sql

wrench-db-inspect run \
  --manifest db-security-manifest.json \
  --schema-dump target/db-inspect/schema.sql \
  --profile protected_branch \
  --report-json target/db-inspect/report.json
```

The protected-branch profile passes with zero parser errors, zero unknown state, complete enabled/forced RLS coverage and no waiver. CI downloads the immutable `wrench-db-inspect` Linux archive from the consolidated Proof Kit release `db-inspect-v0.1.0-alpha.7` and verifies SHA-256 before producing JSON and Markdown evidence.

## Boundaries

Feed Radar owns subscriptions, user-visible rules, selection decisions and explainable exports. It does not own generic ingestion infrastructure, durable memory, orchestration or shared client primitives. Integrations cross those boundaries through explicit contracts.

## Architecture

The Rust workspace separates domain, ingestion, rules, provider traits, sync, storage ports, API, workers and CLI. UI targets remain targets until backed by runnable evidence. ADR 0002 makes Dioxus the durable target; the removed Leptos spike remains historical evidence in [`docs/spikes/leptos-web-shell.md`](docs/spikes/leptos-web-shell.md), and desktop packaging requires a later, evidence-backed decision.

- [`docs/refactor-plan.md`](docs/refactor-plan.md)
- [`docs/launch-target.md`](docs/launch-target.md)
- [`docs/adr/0002-rust-first-product-stack.md`](docs/adr/0002-rust-first-product-stack.md)

## Contributing

- [Contribution guide](CONTRIBUTING.md)
- [Roadmap](ROADMAP.md)
- [Security policy](SECURITY.md)
- [Agent guidance](AGENTS.md)

## License

[MIT](LICENSE).
