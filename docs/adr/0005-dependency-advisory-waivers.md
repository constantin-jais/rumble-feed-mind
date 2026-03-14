# ADR 0005 — Temporary dependency advisory waivers

## Status

Accepted temporary waiver. Expires: **2026-09-30**.

## Context

`cargo deny check advisories` reports advisories through transitive dependencies:

| Advisory | Path | Reason for temporary waiver |
| --- | --- | --- |
| `RUSTSEC-2025-0057` | `fxhash` via `scraper` | Feed parsing/extraction path; replacement requires ingestion dependency evaluation. |
| `RUSTSEC-2026-0174` | `http-types` via `async-stripe` | Stripe is being isolated as optional adapter; removal/feature isolation pending. |
| `RUSTSEC-2024-0384` | `instant` via `async-stripe` path | Same Stripe isolation path. |
| `RUSTSEC-2024-0436` | `paste` via UI dependencies | UI dependency stack is migration/target evaluation; no new product UI authorized yet. |
| `RUSTSEC-2026-0173` | `proc-macro-error2` via UI/validator deps | Needs dependency upgrade/replacement evaluation. |

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
