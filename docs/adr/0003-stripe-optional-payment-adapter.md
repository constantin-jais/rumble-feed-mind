# ADR 0003 — Stripe as optional payment adapter only

## Status

Accepted as a constraint for readiness; implementation enforcement pending.

## Context

`rumble-feed-mind` contains Stripe billing code and database fields. Stripe is a proprietary US payment dependency. The ecosystem sovereignty doctrine allows documenting unavoidable payment risk, but forbids making US/proprietary services mandatory for core product truth or local/self-hosted use.

## Decision

Stripe is an optional payment adapter, not a core runtime dependency.

Core FeedMind capabilities must work without Stripe:

- feed subscriptions;
- polling/normalization;
- deterministic rules;
- curated items;
- `CuratedItemExport` preview/export;
- local/self-hosted operation.

Stripe may be enabled only for hosted/commercial billing deployments with explicit configuration and documented risk.

## Rules

- Stripe identifiers must not appear in `CuratedItemExport`.
- Stripe secrets must never be logged or exported.
- Stripe code paths must fail closed when not configured.
- Payment data retention/deletion must be documented separately from feed/curation data.
- Harness readiness must not depend on Stripe availability.

## Consequences

- FeedMind remains not ready for harness package until optionality is enforced or verified by tests.
- `cargo deny` advisories via `async-stripe` remain a readiness blocker unless Stripe is isolated, replaced, or formally waived with expiry.

## Rejected alternatives

- Mandatory Stripe for all deployments: rejected; breaks self-hosted/sovereign core use.
- Removing all billing scope immediately: not required for specs readiness, but acceptable if product strategy changes.
