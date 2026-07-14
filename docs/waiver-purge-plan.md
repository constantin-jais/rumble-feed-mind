# Advisory Waiver Purge Plan

**Deadline:** 2026-09-30 (blocker for R2 maturity claim)

Two RustSec advisories remain waived in `deny.toml`. This plan preserves the original inventory and records completed removals with concrete evidence.

## Summary Table

| Advisory          | Dependency Chain                        | Vulnerability                                 | Action                                                                                                     | Status  | Target     |
| ----------------- | --------------------------------------- | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ------- | ---------- |
| RUSTSEC-2025-0057 | fxhash → scraper                        | Hash DoS / collision attack                   | Resolved by upgrading scraper to 0.27.0 in PR #79; no fxhash remains in the graph.                          | resolved | 2026-07-14 |
| RUSTSEC-2026-0174 | http-types → async-stripe               | Protocol or security issue in http-types      | Upgrade async-stripe ≥ 0.40 (if available) or migrate to reqwest native HTTP                               | pending | 2026-08-15 |
| RUSTSEC-2024-0384 | instant → async-stripe path dependency  | Timing or platform issue in instant crate     | Resolved by async-stripe upgrade or explicit instant patch                                                 | pending | 2026-08-15 |
| RUSTSEC-2024-0436 | paste → validator, or UI dep chain      | Macro expansion or compile-time vulnerability | Upgraded validator to 0.20; `paste` absent from the locked graph                                          | resolved | 2026-07-09 |
| RUSTSEC-2026-0173 | proc-macro-error2 → UI/validator derive | Unmaintained procedural macro                 | Updated `validator_derive` to 0.20.1, which uses maintained `proc-macro-error3`                            | resolved | 2026-07-12 |
| RUSTSEC-2026-0194 | quick-xml 0.37 → feed-rs 2.3 | NsReader unbounded allocation (quick-xml #970) | Updated to feed-rs 2.4.0 and quick-xml 0.41.0                                                             | resolved | 2026-07-09 |
| RUSTSEC-2026-0195 | quick-xml 0.37 → feed-rs 2.3 | Same dependency tree as 0194                  | Resolved with the same feed-rs/quick-xml update                                                            | resolved | 2026-07-09 |

## Detailed Resolution Plan

### 1. RUSTSEC-2025-0057: fxhash collision issue

**Affected dependency chain:** fxhash → scraper 0.22
**Impact:** HTML parsing used in `crates/ingest` for article normalization.

**Options:**

- **Option A (preferred):** Upgrade scraper to v0.23 or later if security patch available.
- **Option B (fallback):** Replace scraper with alternative parsers (e.g., html5ever, select.rs with unicode-security review).

**Action:** Check scraper changelog and crates.io for v0.23+; if available, upgrade Cargo.toml [workspace.dependencies] scraper entry. If not, evaluate alternative parsers by 2026-08-15.

**Resolution (2026-07-14):** Completed in PR #79 by upgrading `scraper` to 0.27.0. `cargo tree -i fxhash` now returns no package, and `cargo deny` / `cargo audit` are green.

**Owner:** ingest crate maintainer
**Milestone:** 2026-08-15

---

### 2. RUSTSEC-2026-0174: http-types protocol issue

**Affected dependency chain:** http-types → async-stripe 0.39
**Impact:** Payment integration via Stripe in `crates/api` and workers.

**Options:**

- **Option A (preferred):** Upgrade async-stripe to 0.40 or later if available.
- **Option B (fallback):** Replace async-stripe with reqwest native HTTP + manual Stripe API bindings.
- **Option C (isolation):** Move Stripe integration behind a feature gate (e.g., `[features] stripe = ["stripe"]`) to allow distribution without vulnerability exposure if upgrade not available.

**Action:** Monitor async-stripe releases; if 0.40+ available with http-types fix, upgrade [workspace.dependencies] stripe entry by 2026-08-15. If not, evaluate feature-gate isolation.

**Owner:** api crate maintainer
**Milestone:** 2026-08-15

---

### 3. RUSTSEC-2024-0384: instant platform timing

**Affected dependency chain:** instant → async-stripe 0.39
**Impact:** Transitive via Stripe, affects timing-sensitive operations.

**Options:**

- **Option A (preferred):** Resolve via async-stripe upgrade to 0.40+ (see RUSTSEC-2026-0174).
- **Option B (fallback):** Patch instant version explicitly if upstream async-stripe update not available.

**Action:** Upgrade async-stripe first; if instant remains unpatched after upgrade, add explicit instant patch to [workspace.dependencies] by 2026-08-30.

**Owner:** api crate maintainer (dependent on Stripe resolution)
**Milestone:** 2026-08-30

---

### 4. RUSTSEC-2024-0436: paste macro expansion

**Affected dependency chain:** paste → validator 0.19 (or UI dependencies)
**Impact:** Validation derive macros used in domain models and API request/response structs.

**Options:**

- **Option A (preferred):** Upgrade validator to 0.20 or later if patch available.
- **Option B (fallback):** Replace validator with alternative validation crates (e.g., garde, validify).
- **Option C (isolation):** Move validation and UI dependencies behind optional feature flags; allow compile without paste/validator if needed.

**Action:** Check validator changelog; if 0.20+ available, upgrade [workspace.dependencies] validator entry. If not, evaluate garde/validify alternatives or feature-gate isolation by 2026-08-30.

**Owner:** domain + api crate maintainers
**Milestone:** 2026-08-30

---

### 5. RUSTSEC-2026-0173: proc-macro-error2 safety

**Affected dependency chain:** proc-macro-error2 → validator/UI derive macros
**Impact:** Procedural macros used at compile time; affects validator derive and any UI framework macros.

**Options:**

- **Option A (preferred):** Upgrade validator to 0.20+ or replace with alternative (e.g., garde, which has better macro isolation).
- **Option B (fallback):** Move UI and validation behind feature gates to isolate proc-macro-error2 exposure.
- **Option C (deep isolation):** Use manual validation without derive macros for critical paths if replacement not available.

**Resolution (2026-07-12):** `validator_derive 0.20.1` moved the derive path to maintained `proc-macro-error3`. The locked graph no longer contains `proc-macro-error2`, and the waiver was removed.

**Owner:** domain + api + worker crate maintainers
**Milestone:** completed 2026-07-12

---

## Implementation Roadmap

### Phase 1: Quick wins (by 2026-08-15)

- [ ] Check scraper v0.23+ availability; upgrade if available.
- [ ] Check async-stripe 0.40+ availability; upgrade if available.
- [x] Update the validator derive path to `validator_derive 0.20.1`.
- [x] Re-run `cargo deny check` and remove `RUSTSEC-2026-0173`.
- [ ] Update the remaining scraper and async-stripe paths when safe replacements are available.

### Phase 2: Fallback strategies (by 2026-08-30)

- [ ] If Phase 1 patches insufficient, evaluate alternative crates (garde, validify, select.rs).
- [ ] If upgrades blocked, implement feature-gate isolation:
  - `[features] stripe = ["stripe"]` for Stripe integration.
  - `[features] validation = ["validator"]` for derive validation.
  - Update examples and CI workflows to test all feature combinations.
- [ ] Document accepted trade-offs in ADR if necessary.

### Phase 3: Verification (by 2026-09-30)

- [ ] All advisories resolved or explicitly feature-gated and documented.
- [ ] `cargo deny check` passes without ignore list (or with documented, permanent exceptions if architectural constraints exist).
- [ ] Update SECURITY.md and docs to reflect resolution status.
- [ ] Remove waiver expiry comment from deny.toml if all resolved.

---

## Acceptance Criteria

**Definition of Done for waiver purge:**

1. `cargo deny check` runs green with zero ignore entries (or documented permanent exceptions).
2. Any feature-gate isolation is tested in CI and documented in CONTRIBUTING.md.
3. No "temporary" waivers remain; any ongoing waivers are explicitly justified and reviewed per ADR.
4. R2 maturity claim in README.md and status.md updated to reflect completion.

**Failure scenario:** If by 2026-09-30 any advisory remains unresolved and unwaived, the project fails its maturity gate until resolution.

---

## References

- `deny.toml` — advisory ignore list and renewal dates.
- `Cargo.toml` — workspace dependencies and versions.
- `docs/adr/0005-dependency-advisory-waivers.md` — detailed rationale for current waivers (reference only; to be updated/retired after purge).
- SECURITY.md — audit and disclosure policy.
