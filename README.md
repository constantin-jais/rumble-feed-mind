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
| Works today | local OPML parsing, rule evaluation, deterministic `CuratedItemExport` fixtures |
| Not scale-ready | hosted operations, multi-user guarantees, complete UI and production observability |
| Historical IDs | `rumble-feed-mind` and `feedmind-*` remain technical package/contract IDs |

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

## Boundaries

Feed Radar owns subscriptions, user-visible rules, selection decisions and explainable exports. It does not own generic ingestion infrastructure, durable memory, orchestration or shared client primitives. Integrations cross those boundaries through explicit contracts.

## Architecture

The Rust workspace separates domain, ingestion, rules, provider traits, sync, storage ports, API, workers and CLI. UI targets remain targets until backed by runnable evidence.

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
