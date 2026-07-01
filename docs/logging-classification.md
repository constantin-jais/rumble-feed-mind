# Logging classification policy

Status: required before any `trusted` or `scale-ready` claim.

## Principle

FeedMind logs must support debugging without leaking reading history, private feed content, BYOK material, auth tokens, payment identifiers, or personal data.

## Allowed by default

- component name and operation name;
- generated UUIDs and stable opaque IDs;
- counts, durations, status categories;
- sha256 hashes of content or URLs when needed for correlation;
- safe labels such as feed type, rule outcome, or export format.

## Forbidden in normal logs

- raw article content, summaries, private annotations, or full URLs for private feeds;
- email addresses and payment/customer identifiers;
- API keys, BYOK material, bearer tokens, cookies, session IDs;
- prompts, provider raw responses, or model credentials;
- downstream execution grants.

## Debug exception

A local developer may inspect fixture content in terminal output when explicitly running demo commands. That output is not a server log and must not be collected by production telemetry.

## Testable acceptance

Before promotion beyond `dojo`, add checks that prove:

1. auth/session logs never include tokens;
2. provider/BYOK structs redact secrets in `Debug` output;
3. CuratedItemExport logs include only IDs, hashes, rule labels, and constraint booleans;
4. errors include context but not raw private content.

## Current enforcement

The CLI `demo-curate` and `validate-curated-export` produce safe summaries. The security workflow includes a log privacy smoke that rejects obvious raw email, feed URL, payment identifier, and article-title logging patterns.

## Current gap

The smoke is not a full data-flow proof. Before release, API/worker logs still need an adversarial audit against real failure paths and provider/BYOK integrations.
