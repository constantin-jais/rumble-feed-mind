# Investigation — Scraper Replacement Alternatives

**Date:** 2026-07-03  
**Advisory:** RUSTSEC-2025-0057 (fxhash via scraper)  
**Status:** Resolved by upgrade to `scraper 0.27.0` in PR #79; `cargo tree -i fxhash` returns no package; `cargo deny` / `cargo audit` are green

## Summary

`scraper` (CSS selector + HTML parsing library, built on `select.rs` + `html5ever`) carried transitive vulnerability RUSTSEC-2025-0057 via fxhash (hash table hasher). Current usage is limited to article extraction in `crates/ingest/src/extractor.rs`. Completion note (2026-07-14): the waiver was removed after upgrading `scraper` to 0.27.0 in PR #79; `cargo tree -i fxhash` no longer matches any package.

## Investigated Options

### Option 1: `select.rs` + lightweight HTML parser

**Pros:**

- `select.rs` is maintained and battle-tested for CSS selection
- Smaller dependency tree than full scraper
- Would require pairing with html5ever or html5gum

**Cons:**

- Requires two dependencies instead of one
- No reduction in complexity; fxhash would still be pulled by select.rs dependencies
- Minimal maintenance gain

### Option 2: `html5ever` (direct)

**Pros:**

- Standards-based HTML parsing
- Stable, well-tested

**Cons:**

- Heavier footprint; more transitive deps
- No CSS selector support; would need custom XPath or tree traversal
- Doesn't address fxhash advisory

### Option 3: In-house extractor via Gear Loader integration

**Pros:**

- Reusable extraction logic across Rumble products (feeds, articles, event payloads)
- Consolidation opportunity per ADR 0005
- Decouples extraction concerns from FeedMind core

**Cons:**

- Requires Gear Loader API stability; blocks on other ecosystem work
- Significant refactor; non-trivial LOC change
- Outside scope of Q2 2026 cycle

## Recommendation

**Completion note (2026-07-14):** resolved by upgrading `scraper` to 0.27.0 in PR #79; `cargo tree -i fxhash` has no match, and `cargo deny` / `cargo audit` are green.

**Historical recommendation:** Defer to Q3 2026 (post-2026-09-30 evaluation cycle). Options 1 and 2 do not meaningfully reduce advisory risk or dependency complexity. Option 3 is architecturally sound but requires coordination with Gear Loader team and is a 3-4 week effort.

### Interim Plan

1. Extend waiver RUSTSEC-2025-0057 through 2026-Q3 evaluation with explicit re-authorization requirement.
2. Document follow-up in GitHub issue (see link below).
3. Reassess landscape in August 2026:
   - Is feed-rs fixed or upgraded? (related to I7 quick-xml deps)
   - Is Gear Loader API stable enough for extraction extraction consolidation?
   - Are upstream scraper/select.rs dependencies patched?

### Post-Evaluation Decision

If no upstream patches exist by 2026-08-31, evaluate forcing safe version constraint on fxhash (if available) or executing Option 3 (Gear Loader integration) with separate Q3 2026 chantier.

## Related Artifacts

- **ADR 0005** § Removal plan for RUSTSEC-2025-0057
- **Extraction path:** `crates/ingest/src/extractor.rs` (used by article + feed parsing)
- **Follow-up GitHub issue:** [pending link after chantier merge]

---

**Archive note:** Closed by PR #79; no follow-up review scheduled.
