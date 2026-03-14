# rumble-feed-mind — Readiness Audit

Date: 2026-06-30  
Decision: **READY_FOR_IMPLEMENTATION_PLANNING** for scoped `CuratedItemExport` / Provider-BYOK hardening package.  
Allowed work: planning tasks, Rust core implementation, and MVP Rust-first UI screens within scope.  
Forbidden work: Rumble-triggered execution, non-MVP UI expansion, mandatory Stripe dependency, or provider-backed AI beyond tested Provider/BYOK gates.

## Executive decision

`rumble-feed-mind` now has complete ecosystem specs for the scoped package, deterministic export/handoff proof, explicit Stripe/auth/advisory ADRs, and green Rust/advisory/license gates. It may enter planning-only implementation and limited MVP UI work under these constraints:

- JWT remains local session auth only; ADR 0004 and tests enforce Biscuit as the harness/delegation boundary.
- Stripe remains optional payment adapter only; ADR 0003 and tests enforce explicit opt-in configuration.
- dependency advisories are time-bound by ADR 0005 and must be removed before 2026-09-30 or release.
- `CuratedItemExport` and Provider/BYOK policy are drafted, instantiated in specs, and covered by deterministic smoke proof.

## PASS / WARN / FAIL by axis

| Axis | Status | Evidence | Required action |
| --- | --- | --- | --- |
| Security | **PASS with waiver** | JWT/Biscuit boundary tests pass; BYOK Debug redaction tests pass; Provider/BYOK policy drafted; advisory waivers accepted by ADR 0005. | Remove advisory waivers before expiry/release. |
| Quality | **PASS** | `cargo fmt`, `cargo clippy`, `cargo test`, `cargo test --no-run` pass. Rust-first ADR accepted. Ecosystem specs cover required readiness categories. | Keep gates green during implementation. |
| Performance | **WARN** | PostgreSQL + Redis + worker architecture is plausible; no load/perf acceptance yet. | Define ingestion throughput targets during implementation planning. |
| Complétude | **PASS** | `CuratedItemExport` and Provider/BYOK are drafted/instantiated; export fixture/smoke and handoff planning proof pass. | Implement only scoped package tasks. |
| Sovereignty | **PASS with constraints** | PostgreSQL/Redis self-hostable; MIT workspace license; Stripe optionality tests pass; ADR 0003 documents risk; local/EU provider defaults specified. | Keep Stripe optional and remove waivers before release. |

## Stack / sovereignty findings

### Rust/Dioxus or Rust-first target

PASS. Evidence:

- workspace Rust crates in `Cargo.toml`;
- `README.md` says Rust-first product stack;
- `docs/adr/0002-rust-first-product-stack.md` accepts Dioxus as target UI;
- `apps/web-rs` exists as Rust UI target.

### Next.js legacy

WARN. Evidence: `apps/web/package.json` uses Next.js 16. It is documented as legacy/migration reference in `README.md` and ADR 0002. This is acceptable only if no new durable product invariants are added there.

### Stripe

PASS with constraints. Evidence: ADR 0003 documents Stripe as optional payment adapter only; tests prove Stripe is disabled unless explicitly configured. Follow-up before release: isolate/remove advisory-bearing Stripe path or renew waiver by formal review.

### JWT vs Biscuit

PASS for planning. Evidence: ADR 0004 documents JWT as local session only and Biscuit for delegation/harness flows; tests prove JWT cannot authorize harness delegation while Biscuit boundary can.

### BYOK

PASS for planning. Evidence: `crates/crypto` encryption tests pass; `EncryptedData` Debug redacts ciphertext/nonce; Provider/BYOK policy is instantiated in specs. Full storage UI/API lifecycle remains implementation scope.

### Redis/PostgreSQL

PASS with constraints. PostgreSQL and Redis are self-hostable and compatible with EU/Clever Cloud deployment. Production policy must require EU residency and no managed US hyperscaler.

### Feed ingestion

WARN. Current feed ingestion is product-local (`crates/ingest`). It may remain local for MVP only if `CuratedItemExport` is product-owned and no other product consumes feed parsing. If reused by Note/LM/COS, extraction should become Wrench Loader capability.

### CuratedItemExport

PASS for spec smoke, WARN for runtime. Shared draft exists at `specs/shared/contracts/curated-item-export.v0.1.md`, is instantiated in FeedMind specs, and has deterministic fixture/smoke at `specs/rumble-feed-mind/verify_curated_item_export.py`. Missing: product runtime tests and real Wrench/Gear integration.

### PII/secrets in logs

WARN. No real secret found in committed config; placeholders are explicit. Logs include user IDs and model/usage counts in API/worker tracing. Before product continuation, define logging classification: no raw feed private content, no BYOK keys, no tokens, no payment details, no emails in unredacted logs.

### License

PASS at workspace level: MIT in `Cargo.toml`.  
PASS with waiver. `cargo deny check licenses` and `cargo deny check advisories` are green; advisory ignores are documented in ADR 0005 and expire 2026-09-30.

## Remaining constraints before release

| Constraint | Owner layer | Reason |
| --- | --- | --- |
| Remove or renew advisory waivers before 2026-09-30 | Product / Security | ADR 0005 is time-bound and not a permanent release exception. |
| Keep Stripe optional | Product / Sovereignty | Payment adapter must not become core runtime dependency. |
| Keep JWT out of delegation/harness | Shared Auth / Product | Biscuit/shared delegated auth remains required for cross-boundary rights. |
| Replace smoke with real Wrench/Gear integration when available | Wrench / Gear | Current smoke is deterministic contract proof, not deployed integration. |

## Commands executed

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo test --workspace --no-run
cargo deny check licenses
cargo deny check advisories
```

Results:

- `cargo fmt --all --check`: PASS
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS
- `cargo test --workspace`: PASS
- `cargo test --workspace --no-run`: PASS
- `cargo deny check licenses`: PASS after adding `deny.toml` and making `feedmind-cli` inherit workspace MIT license
- `cargo deny check advisories`: PASS with ADR 0005 temporary advisory waivers
- `cargo deny check`: PASS after internal workspace path dependencies were pinned to version `0.1.0`; duplicate dependency findings remain warnings

Notable audit findings from `cargo deny check advisories` and full `cargo deny check`:

- `fxhash` unmaintained via `scraper`;
- `http-types` advisory via `async-stripe`;
- `instant` unmaintained via `async-stripe` path;
- `paste` and `proc-macro-error2` unmaintained via Leptos/Dioxus-adjacent UI dependency path;
- duplicate dependency warnings.

## Final decision

**READY_FOR_IMPLEMENTATION_PLANNING**.

UI product is authorized only for the MVP Rust-first scope:

1. feeds/inbox read and triage surfaces;
2. deterministic rules first;
3. curated items;
4. export preview/validation;
5. provider policy surfaces that do not expose keys;
6. no mandatory Stripe UI path;
7. no Rumble-triggered execution.

Implementation must remain behind planning tasks and human approval. Advisory waivers must be removed or reviewed before 2026-09-30/release.
