# Product Readiness Cockpit

Observation: 2026-07-14  
Last verified: `9ab78a8` on `main`  
Official maturity: Dojo

This is a status snapshot, not a delivery promise. An empty issue list does not mean the product is complete.

## Legend

- `local+CI` — implemented in repo and proven by deterministic local/CI checks.
- `implemented-unhosted` — implemented and proven in a local or otherwise unhosted real-environment proof.
- `partial` — code or proof exists, but the end-to-end gate is incomplete.
- `blocked` — deliberately not claimed yet; waiting on an explicit gate.
- `later` — out of current readiness scope.

Not claimed here: interactive import, account/browser runtime workflow, hosted scheduler, real staging, source sandbox production, observability, backup, PWA/native/desktop, multi-user, or an alpha release pipeline.

## Ingest / sync

| Capability | implementation | evidence | real-environment proof | remaining gate |
| --- | --- | --- | --- | --- |
| OPML / RSS / Atom / JSON Feed pipeline | local+CI | `crates/opml`, `crates/ingest`, `crates/cli` (`opml-summary`, `fetch-feed`, `demo-curate`) | deterministic fixtures plus the dated local live-sync proof | none for the local pipeline |
| Allowlisted HTTPS sync with redirect and body-size bounds | implemented-unhosted | `docs/adr/0007-bounded-public-feed-sync.md`, `crates/cli/src/main.rs` (`sync-curated`), `docs/runbook.md` | `docs/evidence/live-sync-2026-07-13.md` on public feeds with explicit `--allow-host` | hosted fetcher, source sandbox, and scheduler proof |
| Hash-only replay, deduplication, stale-output pruning | local+CI | `crates/sync/src/local.rs`, `docs/adr/0007-bounded-public-feed-sync.md` | local tests plus the same live-sync proof | durable hosted replay state later |

## Rules / curation / export

| Capability | implementation | evidence | real-environment proof | remaining gate |
| --- | --- | --- | --- | --- |
| Explainable rules and decisions | local+CI | `crates/rules`, `crates/cli` (`evaluate-rule`, `demo-curate`), `examples/demo-rule.json` | deterministic demo curate and golden diff | none for explainability |
| `CuratedItemExport` client-safe contract | local+CI | `crates/sync/src/curated.rs`, `crates/cli/src/main.rs` (`validate-curated-export`) | validated export fixture and contract smoke | shared hosted API/worker contract later |
| Read-only Dioxus review over a golden or validated live export | implemented-unhosted | `docs/adr/0007-bounded-public-feed-sync.md`, `docs/plans/2026-07-first-dioxus-product-slice-v1.md`, `docs/evidence/live-sync-2026-07-13.md` | Chromium / Firefox / WebKit mobile-width proof on a validated live export | interactive import, browser runtime workflow, and remote fetch stay out of scope |
| Client Kit provenance and multi-browser proof | implemented-unhosted | `scripts/verify-design-system.py`, `e2e/playwright.config.ts` | pinned Client Kit revision, local design-system verification, 3-browser traversal | provenance drift and remote assets remain hard failures |

## Dioxus UI

| Capability | implementation | evidence | real-environment proof | remaining gate |
| --- | --- | --- | --- | --- |
| Read-only curated-review surface | implemented-unhosted | `docs/adr/0007-bounded-public-feed-sync.md`, `docs/evidence/live-sync-2026-07-13.md` | local bundle rendered with validated export, no DB/account/runtime fetch | interactive import and multi-item review |
| Interactive OPML import and multi-item review | later | `docs/readiness-audit.md`, `docs/launch-target.md` | none | private pilot with real sources and a dedicated browser workflow decision |
| Browser storage, service worker, PWA, native, desktop | later | `docs/launch-target.md`, `docs/plans/2026-07-first-dioxus-product-slice-v1.md` | none | separate distribution decision after Dioxus proof |

## Data / security

| Capability | implementation | evidence | real-environment proof | remaining gate |
| --- | --- | --- | --- | --- |
| PostgreSQL forced RLS and dedicated roles | local+CI | `migrations/20260711000001_enforce_tenant_rls.sql`, `scripts/postgres/provision-roles.sql`, `crates/storage/tests/postgres_rls.rs` | local RLS test harness and manifest inspection gate | production role provisioning outside migrations still needs operator control |
| Logging classification and secret redaction | partial | `docs/logging-classification.md`, `docs/readiness-audit.md`, `crates/crypto/src/encryption.rs`, `crates/sync/src/curated.rs` | deterministic redaction and safe-summary checks | full api/worker adversarial audit before any trust/scale claim |
| BYOK material excluded from client export | local+CI | `crates/sync/src/curated.rs`, `crates/cli/src/main.rs`, `docs/readiness-audit.md` | export validation and fixture smoke | provider lifecycle, storage, and policy UI later |
| Allowlist-only public fetch boundary | implemented-unhosted | `docs/adr/0007-bounded-public-feed-sync.md`, `docs/runbook.md` | dated live traversal over public feeds | production sandbox / SSRF hardening for hosted service |

## Operations / release

| Capability | implementation | evidence | real-environment proof | remaining gate |
| --- | --- | --- | --- | --- |
| Worker refresh interval 300..=86400 and one-shot start | local+CI | `crates/worker/src/config.rs`, `crates/worker/src/scheduler.rs` | unit tests and scheduler start guard | hosted scheduler and HA/failover not proven |
| CLI smoke and deterministic checks | local+CI | `README.md`, `crates/cli/src/main.rs`, `docs/runbook.md` | reproducible local commands and golden diff | none for the CLI smoke |
| Private pilot / staging with real sources | blocked | `docs/readiness-audit.md`, `docs/launch-target.md` | none | a private pilot that reuses real sources, real operators, and a staged runbook |
| Retention, BYOK, SSRF sandbox, and operations hardening | later | `docs/logging-classification.md`, `docs/adr/0007-bounded-public-feed-sync.md` | local policy checks only | end-to-end hosted sandbox and operations proof |
| Alpha release pipeline | later | none | none | all P0/P1/P2 promotion gates green |

## Promotion gates

- **P0** — current state: deterministic local/CI proofs plus the dated unhosted live-sync and Dioxus review proof.
- **P1** — private pilot with real sources: interactive OPML plus multi-item review, operator-run staging, and explicit refusal paths.
- **P2** — alpha candidate: retention/BYOK/SSRF sandbox, operations proof, and release packaging/rollback verified before any alpha wording.

These are promotion gates, not dates. They do not turn the current Dojo status into a release claim.

## Status summary

Current claim: local pipeline, bounded sync, explainable rules, client-safe export, read-only Dioxus review, and tenant-isolated storage.  
Not yet claimed: interactive import, hosted scheduler, real staging, source sandbox production, observability, backup, multi-user, PWA/native/desktop, or alpha release.
