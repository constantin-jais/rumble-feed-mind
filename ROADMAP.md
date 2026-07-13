# Roadmap

This is a contribution map, not a startup roadmap or a delivery promise. It shows where help is useful while keeping scope explicit.

## Now

- make dogfooding evidence visible through commands, fixtures, CI checks, generated reports, or linked docs;
- stabilize runtime tests and feed fixtures;
- improve classification logs;
- maintain the separated-role, transaction-local tenant and forced-RLS guarantees from ADR 0006;
- document advisory waivers and known limits;
- keep the read-only Dioxus curated-review proof, its portable contract and browser gate green;
- maintain the bounded, exact-allowlist local synchronization path and its dated proof;
- keep Rust CI, security, and release checks green.

## Next

- review the bounded local-source contract and decide whether the next source is a sandboxed hosted fetcher or local-only storage;
- add multiple example curated-item exports without embedding third-party live content;
- improve ingest and rule errors;
- add contract tests around BYOK, export, and fail-closed behavior;
- prepare the first alpha-quality pipeline release.

## Later

- interactive OPML import and scheduled synchronization behind explicit auth and retention contracts;
- broader feed and source integrations;
- release provenance for curated outputs;
- hosted or multi-user usage only when privacy, SSRF isolation, retention and provider boundaries are explicit.
