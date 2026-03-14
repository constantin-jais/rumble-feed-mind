# ADR 0004 — Auth boundary: JWT sessions, Biscuit delegation

## Status

Accepted as a readiness boundary; implementation enforcement pending.

## Context

The active FeedMind repo currently uses JWT for product session tokens. The ecosystem shared authorization direction is Biscuit for delegated rights across Rumble/Bolt/Wrench/Gear boundaries.

FeedMind needs both local product sessions and safe export/harness delegation. Reusing JWT for delegation would duplicate security-sensitive logic and weaken attenuation/revocation semantics.

## Decision

JWT may remain a temporary product session mechanism for local API sessions only.

Biscuit, or the accepted shared delegated-authorization contract, is required for:

- cross-service delegation;
- export submission to other Rumbles;
- harness handoff submission;
- provider calls delegated through Bolt or another service;
- any future action that needs attenuation, caveats, revocation refs, or scoped authorization evidence.

## Rules

- JWTs must never appear in exports, logs, Wrench reports, Gear metadata, or handoff payloads.
- `CuratedItemExport` carries actor refs and approval refs, not session tokens.
- FeedMind must not invent product-specific delegation tokens.
- A future migration may replace JWT sessions, but that is not required for specs readiness.

## Consequences

- FeedMind remains not ready for harness package until this boundary is covered by tests or implementation checks.
- Session auth and delegated auth must be documented separately in API/security specs.

## Rejected alternatives

- JWT for all delegation: rejected; insufficient attenuation and shared verification semantics.
- Full auth rewrite before specs readiness: rejected; too broad and not required before contract proof.
