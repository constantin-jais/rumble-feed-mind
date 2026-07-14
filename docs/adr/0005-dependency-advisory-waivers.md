# ADR 0005 — Temporary dependency advisory waivers

## Status

Accepted temporary waiver. Expires: **2026-09-30**. Remaining waivers: async-stripe only.

Resolution update (2026-07-12): `validator_derive 0.20.1` replaces `proc-macro-error2` with maintained `proc-macro-error3`; the `RUSTSEC-2026-0173` waiver is removed. The original decision table remains as the historical acceptance record.
Resolution update (2026-07-14): `scraper 0.27.0` no longer depends on `fxhash`; the `RUSTSEC-2025-0057` waiver is removed after PR #79. `cargo tree -i fxhash` returns no package, and `cargo deny` / `cargo audit` are green.

## Context

`cargo deny check advisories` reports advisories through transitive dependencies:

| Advisory | Path | Reason for temporary waiver | Current impact | Removal plan |
| --- | --- | --- | --- | --- |
| `RUSTSEC-2025-0057` | `fxhash` via `scraper` | Resolved by upgrading `scraper 0.27.0` in PR #79; no `fxhash` remains in the graph. | No current impact; `cargo tree -i fxhash` returns no package. | Completed via the `scraper 0.27.0` upgrade and waiver removal in PR #79. |
| `RUSTSEC-2026-0174` | `http-types` via optional `async-stripe` feature | Stripe is now isolated as an optional adapter, but all-features supply-chain audit still sees the dependency. | Payment adapter is optional and not part of the no-secret quickstart. | Replace/remove `async-stripe` or move to a safer payment adapter before waiver expiry (I4 follow-up). |
| `RUSTSEC-2024-0384` | `instant` via optional `async-stripe` path | Same optional Stripe adapter path. | Same optional Stripe path. | Replace/remove `async-stripe` or move to a safer payment adapter before waiver expiry (I4 follow-up). |
| `RUSTSEC-2026-0173` | `proc-macro-error2` via UI/validator deps | Needs dependency upgrade/replacement evaluation. | Build-time/proc-macro path; no scale-ready claim. | Upgrade or replace affected dependencies (I6). |
| `RUSTSEC-2026-0194` | `quick-xml` NsReader via `feed-rs 2.3` | Ingestion dependency; feed-rs upstream fix pending. No published patch as of 2026-07-03. | Article ingestion path; no multi-tenant trust boundary crossed. | Resolve via upstream feed-rs patch (Option A) or force safe quick-xml constraint (Option B) by 2026-09-30 (I7). |
| `RUSTSEC-2026-0195` | `quick-xml` general via `feed-rs 2.3` | Same ingestion path; same feed-rs upstream constraint. | Same as 2026-0194; related to the same transitive chain. | Resolve via upstream feed-rs patch (Option A) or force safe quick-xml constraint (Option B) by 2026-09-30 (I7). |

## Decision

The advisories are temporarily ignored in `deny.toml` to unblock readiness planning, not product expansion.

This waiver does **not** authorize:

- new product UI expansion;
- mandatory Stripe dependency;
- using affected paths for sensitive provider/BYOK material without tests;
- implementation planning without the harness gates.

## Required follow-up before expiry

1. Replace/remove the optional `async-stripe` adapter or move to a safer payment adapter (covers RUSTSEC-2026-0174, 2024-0384, 2026-0097). Default/core build isolation is complete, but all-features audit still requires the waiver.
2. ~~Evaluate replacing `scraper` or its affected transitive path (RUSTSEC-2025-0057).~~ Resolved by upgrading `scraper 0.27.0` in PR #79; `cargo tree -i fxhash` returns no package.
3. ~~Upgrade or replace UI/validator dependencies pulling unmaintained proc-macro crates (RUSTSEC-2026-0173).~~ Resolved by `validator_derive 0.20.1` on 2026-07-12.
4. Resolve `quick-xml` advisories via upstream patch or safe version constraint (RUSTSEC-2026-0194, 2026-0195); deadline 2026-09-30.
5. Remove advisory ignores when fixed.

## Acceptance impact

With this ADR and `deny.toml`, advisory risk is explicit and time-bounded. FeedMind may be considered ready for planning-only harness packaging only if all other gates pass and the proof records the waiver reference.
