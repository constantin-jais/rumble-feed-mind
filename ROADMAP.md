# Roadmap

This is a contribution map, not a startup roadmap or a delivery promise. It shows where help is useful while keeping scope explicit.

## Now

- make dogfooding evidence visible through commands, fixtures, CI checks, generated reports, or linked docs;
- stabilize runtime tests and feed fixtures;
- improve classification logs;
- maintain the separated-role, transaction-local tenant and forced-RLS guarantees from ADR 0006;
- document advisory waivers and known limits;
- keep Rust CI, security, and release checks green.

## Next

- define and prove the first runnable Dioxus product slice after Portal UI contracts stabilize; this is the next UI milestone, not a desktop/Tauri shell;
- add example curated-item exports;
- improve ingest and rule errors;
- add contract tests around BYOK, export, and fail-closed behavior;
- prepare the first alpha-quality pipeline release.

## Later

- broader feed and source integrations;
- release provenance for curated outputs;
- hosted or multi-user usage only when privacy, retention, and provider boundaries are explicit.
