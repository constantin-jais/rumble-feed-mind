# Roadmap

This is a contribution map, not a startup roadmap or a delivery promise. Official maturity remains Dojo, and the canonical readiness cockpit lives in [`docs/product-readiness.md`](docs/product-readiness.md).

## Now

- keep the deterministic local/CI proofs green;
- keep the read-only Dioxus review and bounded live-sync proofs green;
- prepare a private pilot/staging path with real sources;
- tighten logging classification plus retention/BYOK/SSRF boundaries;
- maintain the separated-role tenant and forced-RLS guarantees from ADR 0006.

## Next

- run the private pilot on real sources with a staged runbook;
- add interactive OPML import and multi-item review;
- harden retention/BYOK/SSRF sandboxing and operational checks;
- keep release/provenance checks ready for a later alpha decision.

## Later

- alpha release only after the private pilot, review, sandbox, and operations gates are proven;
- broader feed and source integrations;
- hosted or multi-user usage only when privacy, SSRF isolation, retention, and provider boundaries are explicit.
