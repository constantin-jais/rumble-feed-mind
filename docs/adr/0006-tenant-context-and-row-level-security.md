# ADR 0006 — Tenant context and PostgreSQL row-level security

## Status

Accepted design; runtime rollout is blocked until the isolation tests below are green.

## Context

`db-security-manifest.json` classifies 18 tables as user-scoped. The SQL corpus is fully parsed, but the `protected_branch` inspection profile correctly blocks because those tables do not enable and force PostgreSQL row-level security (RLS).

The application currently uses one `DATABASE_URL` for migrations and runtime queries. API handlers, background jobs and CLI commands execute directly on shared pools. Adding `FORCE ROW LEVEL SECURITY` now would either break those paths (no tenant context is set) or tempt a permissive policy that only creates the appearance of isolation.

The worker also has legitimate cross-tenant jobs (scheduled refresh, retention, dunning and webhook processing), while login/registration must locate a user before an authenticated tenant identifier exists. These are explicit trust boundaries, not reasons to let the general application role bypass RLS.

## Decision

RLS will be delivered as a security migration, not as a manifest waiver.

### Roles

Provision distinct non-login ownership and runtime roles outside product migrations:

- `feed_radar_owner`: owns schema objects and runs migrations only;
- `feed_radar_app`: API runtime role, never owns tables and never has `BYPASSRLS`;
- `feed_radar_worker`: internal jobs/webhooks role, never owns tables and receives only table-specific grants/policies required by worker operations;
- `feed_radar_auth`: login/registration boundary, restricted to reviewed security-definer functions rather than direct table access;
- `feed_radar_readonly`: optional operational read-only role, subject to explicit tenant context.

No production role may combine owner/migrator and application duties. Development defaults must exercise the same separation.

### Tenant context

Authenticated API database work runs in a transaction and sets a transaction-local UUID before any tenant query:

```sql
SELECT set_config('app.user_id', $1, true);
```

Policies compare the direct or derived owner to:

```sql
NULLIF(current_setting('app.user_id', true), '')::uuid
```

`SET LOCAL`/transaction-local configuration is mandatory: session-level tenant state on a pooled connection is forbidden because it can leak between requests.

### Derived ownership

- `rule_evaluations` derives the user through `article_id -> articles.user_id`;
- `article_tags` derives the user through `article_id -> articles.user_id`;
- `feed_categories` derives the user through `feed_id -> feeds.user_id`.

Policies for derived tables must use an `EXISTS` join matching these foreign-key paths for both `USING` and `WITH CHECK`.

### Trusted non-user paths

- Authentication uses narrowly scoped, reviewed security-definer functions with a fixed `search_path`; raw `users`/`sessions` access is not granted to `feed_radar_auth`.
- Worker cross-tenant access is explicit per table and operation. The worker role does not receive ownership, superuser or `BYPASSRLS`.
- CLI administration and migrations use the owner DSN only for explicit operator commands; user-facing import/export uses the application role plus tenant context.
- Health checks use `SELECT 1` and need no product-table privilege.

### Rollout sequence

1. Provision separated roles in local/CI PostgreSQL.
2. Introduce transaction-scoped tenant helpers and migrate every authenticated API/CLI call site.
3. Introduce restricted auth functions and worker policies; test their negative boundaries.
4. Add RLS policies, then `ENABLE` and `FORCE ROW LEVEL SECURITY` on all 18 tenant tables.
5. Run cross-tenant SQLx tests using non-owner roles.
6. Require `wrench-db-inspect --profile protected_branch` only after runtime and inspection tests pass together.

The migration must be atomic. There is no intermediate production state with forced RLS and unprepared runtime code.

## Required tests

- API role with tenant A cannot read, insert, update or delete tenant B rows, even when query predicates are missing or malicious.
- Tenant context is cleared at transaction end and cannot leak through the pool.
- Missing/malformed tenant context fails closed.
- Derived tables reject ownership paths crossing tenants.
- Auth role cannot query product tables directly; reviewed functions fix `search_path`.
- Worker role can perform only the enumerated cross-tenant operations and cannot alter policies/schema.
- Migration/owner credentials are absent from API and worker configuration.
- `wrench-db-inspect` reports zero parser errors, zero unknown tables and complete enabled/forced RLS coverage.

## Rejected alternatives

- **Keep one owner/application role:** table owners can bypass ordinary RLS and compromise impact is unbounded.
- **Session-level `SET app.user_id`:** pooled connections can reuse another tenant's context.
- **Permissive `current_user = 'feedmind'` policy:** passes superficial checks while preserving the current bypass.
- **Manifest waivers:** hide a critical isolation gap without reducing risk.
- **Enable RLS before adapting runtime:** causes an avoidable production outage.

## Consequences

- Positive: isolation is enforced below every adapter, including queries that forget `user_id` predicates.
- Positive: migration, auth, API and worker trust are explicit and auditable.
- Cost: API database access must become transaction-scoped and worker/auth paths require dedicated tests.
- Rollout gate: Feed Radar and any global Bolt DB gate remain blocked until this ADR is implemented and proven end to end.
