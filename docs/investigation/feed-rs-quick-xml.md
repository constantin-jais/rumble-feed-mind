# Investigation — Feed-rs & Quick-xml Advisory Resolution

**Date:** 2026-07-03  
**Advisories:** RUSTSEC-2026-0194, RUSTSEC-2026-0195 (quick-xml via feed-rs 2.3)  
**Hard Deadline:** 2026-08-31 (29-day decision window per I7 plan)  
**Waiver Expiry:** 2026-09-30 (per ADR 0005)  
**Status:** Deferring decision until 2026-08-31; awaiting upstream feed-rs patch

## Current State

- **feed-rs:** 2.3.1 (latest in Cargo.lock as of 2026-07-03)
- **quick-xml:** 0.37.5 (transitive via feed-rs)
- **Upstream Status:** No feed-rs 2.4 or patch release published as of 2026-07-03
- **Advisory Dates:** Both advisories target quick-xml versions likely ≤0.36.x

## Options Evaluated

### Option A: Await upstream patch (feed-rs ≥2.4)

**Prerequisite:** feed-rs upstream publishes fix with upgraded quick-xml dependency  
**Timeline:** Unknown; no indication of planned release  
**Status:** Blocked on feed-rs maintainer action

### Option B: Force safe quick-xml constraint

**Approach:** Directly constrain quick-xml to a patch version known to fix RUSTSEC-2026-0194 & 2026-0195  
**Feasibility:** Requires identifying safe version from RustSec advisory metadata  
**Timeline:** Executable immediately if safe version is determined  
**Risk:** May break compatibility if feed-rs constraints differ

### Option C: Escalate upstream (last resort)

**Approach:** Open GitHub issue on feed-rs requesting timeline or quick-xml upgrade  
**Prerequisite:** Options A & B are not feasible  
**Timeline:** Deferred to post-deadline if feed-rs remains unresponsive  
**Risk:** May result in waiver extension past 2026-09-30 without re-authorization

## Decision

**Defer to 2026-08-31 decision point.** As of 2026-07-03:

- Feed-rs upstream shows no immediate fix planned
- Quick-xml safe version requires advisory inspection
- Window remains (29 days) to escalate or force constraint

### Execution Triggers

**By 2026-08-31:**

1. If feed-rs 2.4+ released → execute Option A
2. If quick-xml safe version identified → execute Option B
3. Otherwise → escalate feed-rs upstream (Option C) + document deferral to post-2026-09-30 cycle

### Hard Constraint

**Do NOT extend waivers past 2026-09-30 without explicit re-authorization via ADR amendment or separate RFD.** If Option C chosen, escalation becomes a forge-level decision.

## Follow-up

- **Review Date:** 2026-08-31
- **GitHub Issue:** [pending after chantier merge]
- **Related:** ADR 0005 § quick-xml removal plan, ADR 0032 ecosystem ratification

---

**Next Review:** 2026-08-31 (29-day decision window)
