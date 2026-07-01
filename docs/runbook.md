# FeedMind local runbook

Status: dojo / local proof. This runbook is for deterministic local demos and safe manual checks. It does not claim production readiness.

## Prerequisites

- Rust toolchain stable.
- No database or secret is required for the local demo path.
- Network is required only for `fetch-feed` and `demo-curate-live`.

## Deterministic demo

```bash
cargo test --workspace --all-features
cargo run -p feedmind-cli -- demo-curate \
  --opml examples/demo.opml \
  --article examples/demo-article.json \
  --rule examples/demo-rule.json \
  --output out/curated.json
cargo run -p feedmind-cli -- validate-curated-export --file out/curated.json
diff -u examples/expected-curated-export.json out/curated.json
```

Success means the local OPML/article/rule fixtures produce the expected `CuratedItemExport` and the safety constraints remain false:

- `contains_raw_private_content = false`
- `contains_secrets = false`
- `contains_byok_material = false`
- `allow_downstream_execution = false`

## Manual live demo

```bash
cargo run -p feedmind-cli -- demo-curate-live \
  --feed-url https://blog.rust-lang.org/feed.xml \
  --rule examples/demo-rule.json \
  --output out/live-curated.json
cargo run -p feedmind-cli -- validate-curated-export --file out/live-curated.json
```

This command is intentionally not a CI gate: public feeds can be slow, unavailable, or change content.

## Common failures

| Symptom | Likely cause | Action |
| --- | --- | --- |
| `Invalid rule JSON` | malformed fixture or unsupported action | validate `examples/demo-rule.json` |
| `Fetched feed does not contain items` | empty or incompatible live feed | try another feed URL |
| golden diff changes | contract output changed | review intentionally; update golden only with explanation |
| `DATABASE_URL must be set` | DB-backed command used | use the no-secret commands for local demo |

## Release precheck

Before a release candidate, run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo audit
cargo deny check advisories
```

Do not publish automatically from this runbook. Releases remain tag/manual only.
