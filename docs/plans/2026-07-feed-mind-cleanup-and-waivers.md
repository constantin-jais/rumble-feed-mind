# Plan — Feed-Mind Cleanup & RustSec Waivers (2026-07 wave)

```yaml
# forge.plan.v0.1 — bolt-handoff-compatible header (maps onto canvas.bolt_handoff.v0.1)
format: forge.plan.v0.1
kind: planning_request
source:
  product: rumble-feed-mind
  plan_id: plan-2026-07-feed-mind-cleanup-and-waivers
  created_at: "2026-07-03"
execution_policy:
  planning_only: true
  allow_execution: false
  requires_human_approval_for_execution: true
traceability:
  - "target-version 1.0.0 (ecosystem DA-1 Dioxus 0.7.9 ratified via 0032-web-shell-dioxus-ratified.md in ecosystem/specs/shared/adrs/)"
  - "ADR 0002 Rust-first product stack (declares UI target = Dioxus, local to feed-mind)"
  - "ADR 0005 temporary dependency advisory waivers (expires 2026-09-30, lists 7 waivers with removal plans)"
  - "ADR 0003 stripe-optional-payment-adapter (authorizes async-stripe isolation)"
  - "security.yml § Log privacy smoke check and CI gates (feed-mind local)"
  - "readiness-audit.md § Scale-ready constraints on logging classification and waiver removal"
depends_on:
  - "ecosystem ratification via 0032-web-shell-dioxus-ratified.md (ADR 0032 from ecosystem/specs/shared/adrs/, not feed-mind local)"
  - "ADR 0005 scope authorization (removal plans for all 7 waivers in deny.toml)"
blocks:
  - "product UI surface migration to Dioxus (gated on waiver removal complete)"
  - "release artifact matrix expansion (CLI all targets ready; UI awaits ecosystem UI decision resolution)"
open_questions:
  - "RECONCILIATION REQUIRED: deny.toml ignore list (7 waivers) vs ADR 0005 table (7 different waivers). deny.toml includes RUSTSEC-2026-0194 & 2026-0195 (quick-xml via feed-rs) not explicitly documented in ADR 0005 table. These two waivers require either: (a) ADR 0005 amendment to cover quick-xml cascade from feed-rs upstream, or (b) separate technical decision before I7 proceeds."
risks:
  - id: R1
    severity: high
    description: "DENY.TOML / ADR 0005 MISMATCH: deny.toml lines 15–16 ignore RUSTSEC-2026-0194 & 2026-0195 (quick-xml NsReader and general quick-xml via feed-rs 2.3). ADR 0005 table documents only 7 waivers (2025-0057, 2026-0174, 2024-0384, 2024-0436, 2026-0173, 2023-0071, 2026-0097) and does NOT explicitly authorize 2026-0194/2026-0195. Plan's I7 assumes removal authority that ADR 0005 does not grant."
    mitigation: "BLOCKING: Before executing I1–I6, confirm via ADR 0005 amendment or separate decision that RUSTSEC-2026-0194 & 2026-0195 are in-scope for removal. If no authorization exists, defer I7 to post-ADR-amendment wave or escalate as separate RFD."
  - id: R2
    severity: high
    description: "LOG CLASSIFICATION SCOPE UNDERESTIMATION: I3 exit gate 'no raw user_id in output' cites only crates/cli/src/main.rs:954 as audit target. Verification audit found 10+ additional raw user_id/email occurrences: crates/cli/src/main.rs:1104 (user_id = %user_id), crates/cli/src/main.rs:1172 (user_id = %user_id), crates/api/src/routes/billing/webhooks.rs:310 (user_id = %user_id), crates/worker/src/queue.rs:442, 448, 453, 498, 514 (5 occurrences, info!(%user_id, ...)), crates/worker/src/handlers/dunning.rs:324, 343 (user_id = %user_id). I3's exit gate cannot pass without auditing all crates, not CLI-only."
    mitigation: "I3 scope expansion: audit and fix ALL raw user_id/email logging across crates/cli, crates/api, crates/worker (minimum). Update exit gate commands to grep across all affected crates. Update security.yml log privacy smoke check to include crates/worker/src/handlers/ and verify patterns."
  - id: R3
    severity: medium
    description: "RUSTSEC-2026-0194 & RUSTSEC-2026-0195 (quick-xml via feed-rs 2.3) — upstream feed-rs 2.3 has no published fix as of 2026-07-03; these waivers may require escalation to feed-rs maintainers or forced constraint on quick-xml if no patch lands before deadline 2026-09-30."
    mitigation: "I7 includes investigation of feed-rs roadmap + option to pin quick-xml to safe version or propose upstream PR. If upstream unresponsive by 2026-08-31 (29-day window), escalate as forge coordination issue for alternative ingestion library evaluation and execute Option B (force quick-xml constraint). Do NOT extend waiver past 2026-09-30 without explicit re-authorization."
  - id: R4
    severity: low
    description: "I5 (scraper investigation) is authorized by ADR 0005 removal plan, not gold-plating."
    mitigation: "None; I5 is in-scope per ADR 0005 § Removal plans."
  - id: R5
    severity: low
    description: "I4 (async-stripe feature gate) may affect API routes if billing is not already behind an optional feature."
    mitigation: "Review crates/api/src/routes/billing/* for conditional compilation; ensure cargo test --workspace succeeds with --no-default-features."
```

## Context

### The decision

Post-ADR 0032 ecosystem ratification (ecosystem/specs/shared/adrs/0032-web-shell-dioxus-ratified.md, dated 2026-07-03), the feed-mind repo carries two unresolved deferred decisions and one hard deadline:

1. **Leptos spike vs Dioxus consensus** (ADR 0002 declares Dioxus as target; spike evaluation at `docs/spikes/leptos-web-shell.md` + `apps/web-rs` 496 LOC is completed but unintegrated — ecosystem DA-1 ratification via 0032 confirms Dioxus, making the spike evidence-only).

2. **Legacy Next.js surface lifecycle** (`apps/web` ~7,613 LOC, documented as "reference migration only" but still built/linted in CI via `web-security` job — needs archival and removal).

3. **RustSec advisory waivers expiry** (7 temporary waivers in `deny.toml:10-16` expire 2026-09-30; ADR 0005 documents removal plans for each; **RECONCILIATION PENDING**: deny.toml also includes RUSTSEC-2026-0194 & 2026-0195 which are not explicitly listed in ADR 0005's waiver table, requiring clarification before I7 executes).

4. **CI gate asymmetry** (apps/web-rs was workspace member but not explicitly gated; after retiral, verify no member escapes quality gates).

5. **Log classification gap** (readiness-audit.md flags user IDs in logs; security.yml log privacy smoke exists but needs comprehensive tightening across all crates; target = tracing IDs only, no raw user_id or email — **SCOPE EXPANSION**: audit found raw user_id logging in cli, api/billing, and worker crates, not just CLI).

### The source

All increments are rooted in:

- **ADR 0002** § Dioxus target decision (repo-local; reinforced by ecosystem DA-1).
- **ADR 0005** § RustSec waiver removal plans (each advisory tied to a concrete action: "isolate", "upgrade", "replace", or "evaluate").
- **ADR 0003** § Stripe optional adapter (authorizes async-stripe isolation).
- **readiness-audit.md** § constraints on MVP readiness (logs must pass privacy smoke; waivers must be gone before release claim).
- **Target-version 1.0.0 § Wave posture (DA-8):** big-bang (nothing is in service, blocking is acceptable, broken code gets rebuilt, every PR merges green under existing gates).

### The current state

| Item                   | Status                                                                                          | Evidence                                                                                |
| ---------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| workspace members      | 12 crates + 1 Rust app (web-rs)                                                                 | Cargo.toml lines 3–16                                                                   |
| legacy Next.js surface | ~7,613 LOC (excluding node_modules/.next), still in CI                                          | apps/web/, security.yml lines 53–68                                                     |
| Leptos spike           | 496 LOC Rust + tests passing                                                                    | apps/web-rs/, docs/spikes/leptos-web-shell.md                                           |
| RustSec waivers        | 7 items in deny.toml:10-16, but ADR 0005 table lists different 7 items; reconciliation required | deny.toml lines 10–16; ADR 0005 waiver table; **MISMATCH** at 2026-0194/2026-0195       |
| Logging                | tracing crate present; raw user_id logged in ≥10 locations across cli, api, worker crates       | security.yml line 40–45 smoke check (scoped to specific paths); source audit found gaps |
| CI gates               | rust-ci (license, supply-chain, deny/audit), web-security (legacy Next.js), release             | .github/workflows/*                                                                     |

## Target state

1. **workspace**: 11 crates only (apps/web-rs removed, apps/web archived off main).
2. **deny.toml**: All 7 waivers resolved (async-stripe isolated, scraper evaluated for replacement or refactored, UI deps upgraded, quick-xml forced to safe constraint or feed-rs patched) OR explicitly re-authorized with new timeline and linked GitHub issue.
3. **CI**: no web-security job (apps/web no longer in repo); all workspace members tested by `cargo test --workspace`; log privacy smoke check explicitly tests all crates.
4. **Logs**: no raw user_id or email in output across all crates; privacy smoke check tightens to enforce tracing IDs or hashed identifiers only; worker crate handlers included in audit.
5. **ADR 0002**: updated with notation that Leptos 0.7 spike was evaluated (docs/spikes/leptos-web-shell.md) but Dioxus remains the durable target per ecosystem convergence.
6. **ADR 0005**: amended to explicitly authorize removal of RUSTSEC-2026-0194 & 2026-0195 waivers (quick-xml via feed-rs) before I7 executes, OR separate RFD issued.

## Increments

### PREREQUISITE: Reconcile deny.toml & ADR 0005 (blocking gate)

**Pre-execution validation:**

Before merging I1, the following mismatch MUST be resolved:

| Waiver                       | In deny.toml | In ADR 0005 table | Status                                                            |
| ---------------------------- | ------------ | ----------------- | ----------------------------------------------------------------- |
| RUSTSEC-2025-0057            | ✓            | ✓                 | Aligned (scraper/fxhash)                                          |
| RUSTSEC-2026-0174            | ✓            | ✓                 | Aligned (http-types via async-stripe)                             |
| RUSTSEC-2024-0384            | ✓            | ✓                 | Aligned (instant via async-stripe)                                |
| RUSTSEC-2024-0436            | ✓            | ✓                 | Aligned (paste via UI)                                            |
| RUSTSEC-2026-0173            | ✓            | ✓                 | Aligned (proc-macro-error2 via UI)                                |
| RUSTSEC-2026-0194            | ✓            | ✗                 | **MISMATCH**: in deny.toml only; not documented in ADR 0005 table |
| RUSTSEC-2026-0195            | ✓            | ✗                 | **MISMATCH**: in deny.toml only; not documented in ADR 0005 table |
| RUSTSEC-2023-0071 (rsa)      | ✗            | ✓                 | In ADR 0005 but not in deny.toml ignore list                      |
| RUSTSEC-2026-0097 (rand 0.7) | ✗            | ✓                 | In ADR 0005 but not in deny.toml ignore list                      |

**Action:** Issue RFD or ADR 0005 amendment to clarify:

1. Are RUSTSEC-2026-0194 & 2026-0195 in-scope for removal by this wave? If yes, add explicit removal plan to ADR 0005. If no, defer I7 and document the constraint.
2. Why are RUSTSEC-2023-0071 & 2026-0097 missing from deny.toml if ADR 0005 authorizes them? Update deny.toml to match ADR 0005 table or clarify that they have already been resolved upstream.

**Exit gate:** ADR 0005 amendment or separate RFD is merged; deny.toml matches authorizations; I1 may proceed.

---

### I1 — Remove Leptos spike from workspace (PR: retire-leptos-spike)

**Pre-requisites:**

- Prerequisite reconciliation (deny.toml / ADR 0005) is complete.
- Decision DA-1 (Dioxus 0.7.9 ratified) is in effect.
- No code in the repo depends on apps/web-rs.

**Files:**

- `Cargo.toml` (workspace members list, line 15 delete).
- `docs/adr/0002-rust-first-product-stack.md` (add notation that Leptos 0.7 spike was completed as evidence).

**Work:**

1. Remove `"apps/web-rs",` from `Cargo.toml` workspace members list (line 15).
2. Append to ADR 0002 § Context a paragraph: "Leptos 0.7 spike evaluation (docs/spikes/leptos-web-shell.md) was completed 2026-07 and demonstrates Leptos viability for SSR+WASM; however, ecosystem convergence (DA-1 via ecosystem/specs/shared/adrs/0032-web-shell-dioxus-ratified.md, 2026-07-03) ratifies Dioxus 0.7.9 as the durable UI target for all Rumble products. The spike remains in this repository's history and spikes/ archive for future reference."

**Exit gates:**

- `cargo test --workspace` completes with 0 failures (should report fewer tests after web-rs removal).
- `cargo fmt --all --check` succeeds.
- `cargo check --workspace --all-targets --all-features` succeeds.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` succeeds.
- `cargo doc --workspace --all-features --no-deps` completes.
- GitHub Rust CI workflow passes end-to-end.
- `git grep "apps/web-rs" -- .github/workflows/` returns no matches (verify no CI references remain).

---

### I2 — Archive and remove legacy Next.js surface (PR: archive-legacy-nextjs)

**Pre-requisites:**

- I1 is merged (workspace is clean of Leptos spike).
- `apps/web` is documented as reference migration only; no production data lives there.

**Files:**

- Create orphan branch `archive-2026-07-legacy-nextjs` with content of `apps/web/` and this commit message header: "archive: legacy Next.js reference surface (~7.6k LOC) — retained for migration reference, removed from main per feed-mind ADR 0002".
- `apps/web/` (delete entire directory from main).
- `.github/workflows/security.yml` (remove `web-security` job, lines 53–68).

**Work:**

1. Create branch `archive-2026-07-legacy-nextjs` pointing to current commit; checkout that branch locally and ensure it compiles/passes all tests.
2. On main: delete `apps/web/` directory.
3. On main: edit `.github/workflows/security.yml` — remove the entire `web-security:` job block (lines 53–68). Verify the YAML syntax is still valid.
4. Commit to main with message: "chore: archive legacy Next.js surface (apps/web) to branch archive-2026-07-legacy-nextjs; remove from CI workflows".

**Exit gates:**

- `cargo test --workspace` passes (no web dependencies break Rust builds).
- `.github/workflows/security.yml` remains valid YAML (no dangling job references).
- GitHub Actions security workflow still runs on push to main (license-check and rust-supply-chain jobs execute; only web-security is removed).
- `git grep "apps/web/" -- .github/workflows/` returns no matches (verify no CI references remain).

---

### I3 — Correct CI gates and classify logs (PR: ci-gates-and-log-classification)

**Pre-requisites:**

- I1 and I2 are merged.

**Files:**

- `.github/workflows/security.yml` (log privacy smoke check, update patterns and scopes).
- `crates/cli/src/main.rs` (replace raw user_id logging at line 954 and structed-logging occurrences at 1104, 1172).
- `crates/api/src/routes/billing/webhooks.rs` (replace user_id = %user_id at line 310).
- `crates/worker/src/queue.rs` (replace info!(%user_id, ...) at lines 442, 448, 453, 498, 514).
- `crates/worker/src/handlers/dunning.rs` (replace user_id = %user_id at lines 324, 343).

**Work:**

1. **Comprehensive audit for raw user_id/email across all crates:**

   ```bash
   grep -rn 'user_id\s*=\s*%\|info!(%user_id\|debug!(%user_id\|info!(".*user' \
     crates/api/src crates/cli/src crates/worker/src crates/ingest/src --include="*.rs"
   ```

   Expected hits (locations to fix):
   - `crates/cli/src/main.rs:954` — `info!("Using user ID: {}", user_id);` (raw format string)
   - `crates/cli/src/main.rs:1104` — `user_id = %user_id` (structured field, displays raw value)
   - `crates/cli/src/main.rs:1172` — `user_id = %user_id` (structured field, displays raw value)
   - `crates/api/src/routes/billing/webhooks.rs:310` — `user_id = %user_id` (structured field)
   - `crates/worker/src/queue.rs:442, 448, 453, 498, 514` — `info!(%user_id, ...)` (structured shorthand, displays raw)
   - `crates/worker/src/handlers/dunning.rs:324, 343` — `user_id = %user_id` (structured field)

2. **Replace at all identified locations:**

   ```rust
   // Before (example from crates/cli/src/main.rs:954):
   info!("Using user ID: {}", user_id);

   // After (use tracing span context and hashing):
   info!(user_id_hash = %sha256_tag(user_id.as_bytes()), "User context initialized");

   // OR before (example from structured: crates/cli/src/main.rs:1104):
   info!(email_hash = %sha256_tag(email.as_bytes()), user_id = %user_id, "Created user");

   // After (remove raw user_id, keep hashed email):
   info!(email_hash = %sha256_tag(email.as_bytes()), user_id_hash = %sha256_tag(user_id.as_bytes()), "Created user");

   // OR (if tracing span preferred):
   info!(email_hash = %sha256_tag(email.as_bytes()), "Created user");
   // with span set separately: tracing::span!(tracing::Level::INFO, "user_context", user_id_hash = %sha256_tag(user_id.as_bytes()))
   ```

3. **Verify no other user_id/email logs exist in the specified crates** after fixes:

   ```bash
   grep -rn 'user_id\s*=\s*%\|info!(%user_id\|debug!(%user_id\|email\s*=\s*%' \
     crates/api/src crates/cli/src crates/worker/src crates/ingest/src --include="*.rs" \
     || echo "PASS: no raw PII fields found"
   ```

4. **Update `.github/workflows/security.yml` log privacy smoke check** (lines 40–45) to be explicit about expanded scope:

   ```yaml
   - name: Log privacy smoke
     run: |
       set -euo pipefail
       # Detect raw PII/payment/feed data in logs across all workspace crates.
       # Pattern: structured field assignment with % display (e.g., user_id = %user_id) or format macro with PII.
       if rg -n 'user_id\s*=\s*%|email\s*=\s*%|info!\(to,|customer_id\s*=\s*%|invoice_id\s*=\s*%|payment_method_id\s*=\s*%|subscription_id\s*=\s*%|event_id\s*=\s*%|url\s*=\s*%url|Failed to insert article \{\}' crates/api/src crates/worker/src crates/ingest/src crates/cli/src; then
         echo "::error::potential raw PII/payment/feed data in logs (user_id, email, customer_id, etc.)"
         exit 1
       fi
   ```

   (Ensures all crates/worker paths are checked, not just main.rs.)

5. **Verify all workspace members are tested:**
   ```bash
   # Confirm cargo test --workspace includes all crates
   cargo test --workspace --all-features 2>&1 | grep -E "^running|test result:" | head -20
   ```
   Expected: test suites for crypto, domain, ingest, opml, rules, sync, storage, core, api, worker, cli (11 crates). No apps/ should appear.

**Exit gates:**

- `cargo test --workspace --all-features` passes and lists tests from all 11 crates (no gaps, no apps/).
- `.github/workflows/security.yml` log privacy smoke check runs and passes (no PII patterns found across all crates).
- `grep -rn 'user_id\s*=\s*%\|info!(%user_id\|debug!(%user_id' crates/ --include="*.rs"` returns 0 matches (all raw user_id logging removed).
- `cargo fmt`, `cargo check`, `cargo clippy` all pass on the modified source files.
- Security and Rust CI workflows complete green on PR.

---

### I4 — Isolate and feature-gate async-stripe (PR: isolate-async-stripe)

**Pre-requisites:**

- I1, I2, I3 merged.
- Review crates/api/src/routes/billing/ to understand current async-stripe usage.
- ADR 0003 (stripe-optional-payment-adapter) authorizes this isolation.

**Files:**

- `Cargo.toml` (workspace, line 94: make stripe optional).
- `crates/api/Cargo.toml` (add stripe as optional feature dependency).
- `deny.toml` (remove RUSTSEC-2026-0174 and RUSTSEC-2024-0384 waivers).
- `crates/api/src/main.rs` or routes module (conditionally compile billing routes).

**Work:**

1. Edit `Cargo.toml` (workspace level) — make stripe optional (currently at line 94; may shift after I2):

   ```toml
   # OLD:
   stripe = { package = "async-stripe", version = "0.39", features = ["runtime-tokio-hyper"] }

   # NEW: remove from workspace.dependencies (move to crates/api)
   ```

2. Edit `crates/api/Cargo.toml` — add stripe as optional feature:

   ```toml
   [dependencies]
   # ... (existing)
   stripe = { package = "async-stripe", version = "0.39", features = ["runtime-tokio-hyper"], optional = true }

   [features]
   default = []
   stripe = ["dep:stripe"]
   ```

3. Edit `crates/api/src/main.rs` or routes module — conditionally compile billing routes:

   ```rust
   #[cfg(feature = "stripe")]
   pub mod routes {
       pub mod billing;
   }

   // Alternatively, if billing is already a submodule:
   #[cfg(feature = "stripe")]
   mod billing;
   ```

   (Exact placement depends on current structure; inspect crates/api/src/main.rs to determine the best location.)

4. Edit `deny.toml` — remove lines for RUSTSEC-2026-0174 and RUSTSEC-2024-0384 from the ignore list:

   ```toml
   # Before (lines 10-16):
   ignore = [
     "RUSTSEC-2025-0057",
     "RUSTSEC-2026-0174",   # DELETE: http-types via async-stripe (now optional)
     "RUSTSEC-2024-0384",   # DELETE: instant via async-stripe (now optional)
     "RUSTSEC-2024-0436",
     "RUSTSEC-2026-0173",
     "RUSTSEC-2026-0194",
     "RUSTSEC-2026-0195",
   ]

   # After:
   ignore = [
     "RUSTSEC-2025-0057",
     "RUSTSEC-2024-0436",
     "RUSTSEC-2026-0173",
     "RUSTSEC-2026-0194",
     "RUSTSEC-2026-0195",
   ]
   ```

5. **Verify builds with and without the feature:**

   ```bash
   cargo build --package feedmind-api --no-default-features
   cargo build --package feedmind-api --features stripe
   ```

6. **Detect duplicate/conflicting workspace dependency paths** after stripe isolation:
   ```bash
   cargo tree -d 2>&1 | grep -E "stripe|http-types|instant" || echo "PASS: no duplicate stripe dependencies"
   ```

**Exit gates:**

- `cargo test --workspace --all-features` passes (includes stripe feature).
- `cargo test --workspace --no-default-features` passes (stripe feature is optional; builds without it).
- `cargo deny check advisories` passes (RUSTSEC-2026-0174 and RUSTSEC-2024-0384 no longer appear in waiver list).
- `cargo tree -d` reports no duplicate stripe/http-types/instant paths.
- `cargo clippy`, `cargo fmt`, `cargo check` all pass.
- GitHub Rust CI and security workflows pass.

**Exit gate command (proof):**

```bash
cargo deny check advisories && echo "PASS: Stripe-related waivers removed" || echo "FAIL: waivers still present"
```

---

### I5 — Evaluate and plan scraper replacement (PR: investigate-scraper-replacement)

**Pre-requisites:**

- I1–I4 merged.
- ADR 0005 § "Evaluate replacing `scraper` or moving reusable extraction pressure to Gear Loader" (authorized removal plan).

**Files:**

- `docs/investigation/scraper-alternatives.md` (new file, document findings).
- `deny.toml` (conditionally: update RUSTSEC-2025-0057 waiver comment if decision is to defer).
- `docs/adr/0005-dependency-advisory-waivers.md` (amendment, if decision is to defer or revise timeline).

**Work:**

1. Research scraper library alternatives and impact:
   - `select.rs` (CSS selector, no HTML parsing) — would require paired HTML parser like html5ever; evaluate effort.
   - `ego-tree` (tree structure) — no built-in parsing; not a drop-in replacement.
   - `html5ever` (standards-based, unmaintained but stable) — heavier footprint; assess if feasible.
   - `nicer-html` or other lighter parsers — evaluate community alternatives.
   - Roll-in-house extractor via `feedmind-ingest` crate (given feed/article parsing is core) — assess reuse vs new code.

2. Document findings in `docs/investigation/scraper-alternatives.md`:
   - Pros/cons of each option (correctness, maintenance, performance, feature parity).
   - Impact on `crates/ingest/src/extractor.rs` (current scraper usage; LOC estimate for refactor if applicable).
   - **Recommendation**: replace with X, refactor over Y weeks, or defer to post-2026-09-30 wave.

3. **If replacement is in-scope for this wave (before 2026-09-30):**
   - Plan the refactor in a follow-up increment (I5b, separate PR).
   - Update `deny.toml` waiver comment to indicate timeline: "RUSTSEC-2025-0057, scraper/fxhash: Replacement planned Q3 2026 (docs/investigation/scraper-alternatives.md), removal target 2026-Q3."
   - Do NOT remove the waiver yet.

4. **If recommendation is to defer or evaluate further:**
   - Extend the RUSTSEC-2025-0057 waiver in `deny.toml` with explicit comment: "Under evaluation for replacement per docs/investigation/scraper-alternatives.md; removal target 2026-Q4; GitHub issue #XXX tracks follow-up."
   - Amend `docs/adr/0005-dependency-advisory-waivers.md` to reflect new timeline.
   - Create a GitHub issue in rumble-feed-mind to track the follow-up work and link in commit message.

**Exit gates:**

- Investigation document is merged and linked from ADR 0005.
- If replacement is in scope: refactor plan is documented as follow-up increment or distinct chantier (with separate PR).
- If deferred: `deny.toml` waiver comment is updated with explicit timeline and GitHub issue number; ADR 0005 is amended.
- No change to test results (investigation only).
- GitHub workflow runs green (doc changes only).

---

### I6 — Upgrade UI dependencies (PR: upgrade-ui-deps)

**Pre-requisites:**

- I1–I4 merged.
- Identify exact versions of paste and proc-macro-error2 causing RUSTSEC-2024-0436 and RUSTSEC-2026-0173.

**Files:**

- `Cargo.lock` (updated via cargo update).
- `Cargo.toml` (any crate depending on paste/proc-macro-error2 as direct dependency, if needed).
- `deny.toml` (remove RUSTSEC-2024-0436 and RUSTSEC-2026-0173 waivers).

**Work:**

1. Identify which crates use paste and proc-macro-error2:

   ```bash
   grep -r "^paste\|^proc-macro-error2" $DEV_ROOT/rumble-feed-mind/Cargo.lock | head -5
   cargo tree -i paste
   cargo tree -i proc-macro-error2
   ```

2. If they are transitive (pulled in by validator, syn, or other UI/proc-macro crates):

   ```bash
   # Upgrade the crate that pulls them in (likely validator or a UI-related proc-macro)
   cargo update -p validator
   cargo update -p <other-crate>
   ```

3. Verify the upgraded versions no longer carry the waivers:

   ```bash
   cargo deny check advisories 2>&1 | grep -E "RUSTSEC-2024-0436|RUSTSEC-2026-0173"
   ```

   Expected: no output (waivers gone).

4. Edit `deny.toml` — remove lines for RUSTSEC-2024-0436 and RUSTSEC-2026-0173 from the ignore list.

5. Run full test suite to ensure upgrades don't break API or behavior.

**Exit gates:**

- `cargo test --workspace --all-features` passes.
- `cargo deny check advisories` passes (RUSTSEC-2024-0436 and RUSTSEC-2026-0173 no longer in waiver list).
- `cargo deny check licenses` passes (upgraded deps don't introduce new licenses).
- `cargo clippy`, `cargo fmt`, `cargo check` all pass.
- GitHub Rust CI and security workflows pass.

**Exit gate command (proof):**

```bash
cargo deny check advisories | grep -E "RUSTSEC-2024-0436|RUSTSEC-2026-0173" && echo "FAIL: waivers still present" || echo "PASS: waivers removed"
```

---

### I7 — Address quick-xml advisories (PR: resolve-feed-rs-deps OR force-quickxml-constraint)

**Pre-requisites:**

- I1–I6 merged.
- **BLOCKING**: Prerequisite reconciliation confirmed that RUSTSEC-2026-0194 and RUSTSEC-2026-0195 are authorized for removal by this wave (either via ADR 0005 amendment or separate RFD).
- feed-rs 2.3 upstream status is assessed: no patch available as of 2026-07-03.
- **Hard deadline for decision**: 2026-08-31. If feed-rs upstream is unresponsive by this date, Option B (force safe quick-xml constraint) becomes mandatory; do not extend waivers past 2026-09-30 without re-authorization.

**Files:**

- `Cargo.toml` (workspace, conditional: add direct quick-xml constraint if forcing safe version).
- `deny.toml` (remove RUSTSEC-2026-0194 and RUSTSEC-2026-0195 waivers if resolved).
- `docs/investigation/feed-rs-quick-xml.md` (document the chosen path and escalation status).

**Work:**

**Option A: Upstream patch (if feed-rs releases fix by 2026-08-31):**

1. Await feed-rs 2.4 or patch release; upon availability, upgrade:
   ```bash
   cargo update -p feed-rs
   ```
2. Verify quick-xml is upgraded to a patched version:
   ```bash
   cargo tree | grep quick-xml
   ```
3. Confirm no RUSTSEC-2026-0194 or 2026-0195 appear in `cargo deny check advisories`.
4. Edit `deny.toml` — remove the two waivers from ignore list.

**Option B: Force safe quick-xml constraint (if upstream unresponsive by 2026-08-31):**

1. Determine the safe version of quick-xml (consult RustSec advisory dates and quick-xml release history).
   Suppose safe version is quick-xml ≥ 0.29 (example; adjust based on actual advisory data).
2. Add direct constraint to `Cargo.toml` (workspace.dependencies):
   ```toml
   # Constrain quick-xml to avoid RUSTSEC-2026-0194 & 2026-0195
   # Feed-rs 2.3 transitive dependency; safe version determined via RustSec advisory timeline
   quick-xml = { version = ">=0.29", features = ["serialize"] }
   ```
3. Run `cargo update`:
   ```bash
   cargo update -p feed-rs -p quick-xml
   ```
4. Verify feed-rs still compiles and feed parsing still works:
   ```bash
   cargo test -p feedmind-ingest --all-features
   ```
5. Confirm `cargo deny check advisories` no longer reports 2026-0194 or 2026-0195.
6. Edit `deny.toml` — remove the two waivers from ignore list.
7. Document in `docs/investigation/feed-rs-quick-xml.md` the constraint imposed and rationale.

**Option C: Open upstream escalation (if no safe constraint exists and feed-rs is unmaintained):**

1. **Only if Options A and B are not feasible.** Open issue on feed-rs GitHub: "quick-xml ≤0.28 has critical security advisories RUSTSEC-2026-0194/2026-0195; rumble-feed-mind cannot upgrade without upstream fix or quick-xml patch. Please advise timeline or alternative."
2. Document the escalation in `docs/investigation/feed-rs-quick-xml.md`.
3. **Do NOT extend the RUSTSEC-2026-0194 and RUSTSEC-2026-0195 waivers past 2026-09-30 without explicit re-authorization via ADR amendment or separate RFD.** If Option C is chosen, defer to post-2026-09-30 wave with separate decision.
4. Create GitHub issue in rumble-feed-mind to track the escalation, deadline, and fallback to alternative ingestion library.

**Exit gates (Option A or B only; if choosing C, do NOT merge I7, defer and open follow-up RFD):**

- `cargo test --workspace --all-features` passes.
- `cargo deny check advisories` passes (RUSTSEC-2026-0194 and RUSTSEC-2026-0195 removed from ignore list).
- `crates/ingest` tests pass (feed parsing behavior unchanged, no regression).
- GitHub Rust CI and security workflows pass.
- If Option B: Cargo.lock includes updated quick-xml ≥ safe version; constraint is documented in Cargo.toml comment.
- Investigation document (`docs/investigation/feed-rs-quick-xml.md`) is merged and linked from ADR 0005 amendment.

**Exit gate command (proof):**

```bash
cargo deny check advisories | grep -E "RUSTSEC-2026-0194|RUSTSEC-2026-0195" && echo "FAIL: waivers still present" || echo "PASS: waivers removed"
```

---

## Out of scope

The following are **not** part of this chantier:

1. **Product UI migration to Dioxus** — The future UI implementation is gated on:
   - Waivers purged (this chantier).
   - Dioxus patterns proven in a real product (e.g., rumble-lm, per target-version DoD).
   - Explicit demander/stakeholder request for UI work.

2. **Ingestion hardening beyond the current scope** — Loader contract validation, refusal codes, and adversarial ingestion testing are separate chantiers (tied to Gear Loader and Bolt integration).

3. **Stripe optional integration testing** — I4 feature-gates stripe; full end-to-end billing workflow testing is deferred to a future increment when Stripe is claimed as product-ready.

4. **Scraper replacement implementation (if I5 defers)** — I5 is investigation and decision only; actual refactor is a follow-up chantier.

5. **Feed-rs upstream coordination (if I7 Option C chosen)** — I7 opens the issue; managing the upstream lifecycle is a forge/ecosystem-level concern.

6. **ADR 0032 localization** — ADR 0032 (0032-web-shell-dioxus-ratified.md) is ecosystem-level; no local version exists in feed-mind/docs/adr/. It is referenced as decision authority but not duplicated into the local ADR archive.

## Verification

**End-to-end chantier success criteria:**

1. **Prerequisite reconciliation is complete:**
   - ADR 0005 amendment or separate RFD explicitly authorizes RUSTSEC-2026-0194 & 2026-0195 removal (or defers them with new timeline).
   - deny.toml matches authorizations in ADR 0005 table.

2. **All increments merge green:**
   - Each PR passes GitHub CI gates (Rust CI = license-check + rust-supply-chain; security.yml updated).
   - No workspace members are skipped by tests.
   - No CI job references removed artifacts (web-security job gone; license-check and rust-supply-chain remain).

3. **deny.toml is clean:**
   - All 7 authorized waivers are either removed (increments I4–I7 Options A/B) or explicitly re-documented with new timeline and GitHub issue link (I5/I7 Option C if deferred).
   - Running `cargo deny check advisories` reports 0 ignored advisories (if all resolved) or lists only documented deferrals with explicit deadline and tracking link.

4. **Logging is classified:**
   - No raw user_id/email appears in logs across crates/cli, crates/api, crates/worker, crates/ingest.
   - `cargo test --workspace` runs security.yml log privacy smoke check and passes.
   - Grep verification: `grep -rn 'user_id\s*=\s*%\|info!(%user_id' crates/ --include="*.rs"` returns 0 matches.

5. **Workspace is stable:**
   - Cargo.toml workspace members list contains only 11 crates (apps/web and apps/web-rs removed).
   - `cargo test --workspace --all-features` reports tests from all 11 crates; no gaps.
   - Release workflow still builds CLI multi-platform artifacts (no breakage).

6. **Documentation is updated:**
   - ADR 0002 notes Leptos spike as completed evidence (not a rejection; a durable design decision reinforced by ecosystem convergence).
   - ADR 0005 is amended to reflect waiver removal, new timelines, or deferred issues with GitHub tracking links.
   - Investigation documents (I5, I7) are merged and linked from the ADRs.
   - ADR 0032 reference is clarified as ecosystem-level decision (not duplicated into feed-mind local ADR archive).

**Verification commands (run after all PRs merged):**

```bash
# Workspace integrity
cargo test --workspace --all-features
cargo fmt --all --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps

# Security gates
cargo deny check advisories
cargo audit

# Log privacy (should pass with no PII patterns found)
grep -rn 'user_id\s*=\s*%\|info!(%user_id\|debug!(%user_id' crates/ --include="*.rs" && echo "FAIL: raw user_id still present" || echo "PASS: no raw user_id"

# Waiver removal confirmation
deny.toml ignore list contains 0 items (if all resolved) OR list shows only documented deferrals with explicit 2026-Q4 timeline, GitHub issue link, and ADR amendment.

# CLI artifact still builds
cargo run -p feedmind-cli -- --help

# No CI references to removed artifacts
git grep "apps/web/" -- .github/workflows/ || echo "PASS: no legacy app references"
git grep "web-security" -- .github/workflows/ || echo "PASS: web-security job removed"
```

---

**Plan created:** 2026-07-03  
**Status:** Completed  
**Completion note (2026-07-09):** Prerequisite (#19) and I1–I7 (#20–#26) all merged. Correction from the hygiene audit: the two async-stripe waivers (RUSTSEC-2026-0174, RUSTSEC-2024-0384) are still REQUIRED by the CI supply-chain gate — a removal attempt failed cargo-deny advisories and was reverted (#30); the earlier "advisory-not-detected" reading was a local feature-set artifact. Review points 2026-08-31 (scraper, quick-xml) and the external 2026-09-30 deadline unchanged.
