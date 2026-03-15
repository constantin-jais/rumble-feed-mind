# ADR 0005 — Temporary dependency advisory waivers

## Status

Accepted temporary waiver. Expires: **2026-09-30**.

## Context

`cargo deny check advisories` reports advisories through transitive dependencies:

| Advisory | Path | Reason for temporary waiver | Current impact | Removal plan |
| --- | --- | --- | --- | --- |
| `RUSTSEC-2025-0057` | `fxhash` via `scraper` | Feed parsing/extraction path; replacement requires ingestion dependency evaluation. | Local parsing path; no trusted multi-tenant runtime claimed. | Evaluate replacing `scraper` or moving reusable extraction pressure to Wrench Loader. |
| `RUSTSEC-2026-0174` | `http-types` via `async-stripe` | Stripe is being isolated as optional adapter; removal/feature isolation pending. | Payment adapter is optional and not part of the no-secret quickstart. | Isolate or remove `async-stripe` from default/core builds. |
| `RUSTSEC-2024-0384` | `instant` via `async-stripe` path | Same Stripe isolation path. | Same optional Stripe path. | Isolate or remove `async-stripe` from default/core builds. |
| `RUSTSEC-2024-0436` | `paste` via UI dependencies | UI dependency stack is migration/target evaluation; no new product UI authorized yet. | UI target is not claimed usable. | Upgrade or replace UI dependency stack before product UI expansion. |
| `RUSTSEC-2026-0173` | `proc-macro-error2` via UI/validator deps | Needs dependency upgrade/replacement evaluation. | Build-time/proc-macro path; no scale-ready claim. | Upgrade or replace affected dependencies. |
| `RUSTSEC-2023-0071` | `rsa` via `sqlx-mysql` as reported by `cargo audit` | `cargo audit` reports it before `cargo-deny` in the current lock graph; no RSA private-key decryption boundary is used by the current Postgres-first quickstart. | Audit-only waiver; current documented quickstart does not touch MySQL or RSA private-key operations. | Remove by trimming unused SQLx backend features or upgrading the dependency path. |
| `RUSTSEC-2026-0097` | `rand 0.7` via `async-stripe` / `http-types` as reported by `cargo audit` | Stripe remains optional and outside the no-secret quickstart. | Optional payment adapter path, not current local demo. | Remove with Stripe isolation/removal or upstream upgrade. |

## Decision

The advisories are temporarily ignored in `deny.toml` to unblock readiness planning, not product expansion.

This waiver does **not** authorize:

- new product UI expansion;
- mandatory Stripe dependency;
- using affected paths for sensitive provider/BYOK material without tests;
- implementation planning without the harness gates.

## Required follow-up before expiry

1. Isolate or remove `async-stripe` from default/core builds.
2. Evaluate replacing `scraper` or its affected transitive path.
3. Upgrade or replace UI/validator dependencies pulling unmaintained proc-macro crates.
4. Remove advisory ignores when fixed.

## Acceptance impact

With this ADR and `deny.toml`, advisory risk is explicit and time-bounded. FeedMind may be considered ready for planning-only harness packaging only if all other gates pass and the proof records the waiver reference.
