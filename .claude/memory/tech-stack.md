# Stack Technique FeedMind

## Architecture Globale

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend                              │
│                   Next.js 15 + React 19                      │
│                   Shadcn UI + Tailwind                       │
└───────────────────────────┬─────────────────────────────────┘
                            │ HTTPS/REST
┌───────────────────────────▼─────────────────────────────────┐
│                        API (Axum)                            │
│              Authentication, CRUD, Rules                     │
└───────────────────────────┬─────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│  PostgreSQL   │   │    Redis      │   │    Worker     │
│  (data store) │   │ (cache/queue) │   │  (Rust async) │
└───────────────┘   └───────────────┘   └───────┬───────┘
                                                │
                            ┌───────────────────┼───────────────┐
                            │                   │               │
                            ▼                   ▼               ▼
                    ┌───────────────┐   ┌───────────────┐   ┌───────────┐
                    │  RSS/Atom     │   │  AI Provider  │   │ Readability│
                    │  (feed-rs)    │   │  (BYOK)       │   │ (scraper)  │
                    └───────────────┘   └───────────────┘   └───────────┘
```

## Backend (Rust)

| Composant | Technologie | Usage |
|-----------|-------------|-------|
| Language | Rust 2021 | Performance, fiabilité |
| Async Runtime | Tokio | Concurrence massive (1000+ feeds) |
| HTTP Framework | Axum 0.7+ | API REST |
| Database | SQLx | Queries async, compile-time checked |
| Serialization | Serde | JSON, config |
| RSS Parsing | feed-rs | RSS 0.9/1.0/2.0, Atom, JSON Feed |
| HTML Parsing | scraper | DOM manipulation |
| Content Extract | readability | Article extraction |
| HTTP Client | reqwest | Fetch feeds |
| Regex | regex crate | Rules engine |
| Errors | thiserror + anyhow | Error handling |
| Logging | tracing | Structured logging |
| Config | config + dotenvy | Environment management |

## Frontend

| Composant | Technologie | Usage |
|-----------|-------------|-------|
| Runtime | Bun | Package management, scripts |
| Framework | Next.js 15 | SSR, routing, API routes |
| UI Framework | React 19 | Components |
| UI Components | Shadcn UI | Design system |
| Styling | Tailwind CSS 4 | Utility-first CSS |
| State (global) | Zustand | Lightweight store |
| State (server) | TanStack Query | Data fetching, cache |
| Validation | Zod | Schema validation |
| Forms | React Hook Form | Form management |
| Icons | Lucide | Icon set |

## Infrastructure

| Composant | Technologie | Usage |
|-----------|-------------|-------|
| Database | PostgreSQL 16 | Primary data store |
| Cache | Redis 7 | Session, cache, rate limiting |
| Queue | Redis Streams | Background job queue |
| Primary Host | Self-hosted | Mini PC ou VPS |
| Backup Host | Clever Cloud | Failover |
| Reverse Proxy | Caddy | HTTPS, routing |
| External Access | Cloudflare Tunnel | No static IP needed |

## IA (BYOK)

| Provider | Models supportés | Usage |
|----------|-----------------|-------|
| Anthropic | Claude 3 Haiku, Sonnet | NL rules, summaries |
| Google | Gemini 1.5 Flash, Pro | NL rules, summaries |

### Flow IA

```
Article → Batch Queue → AI Provider (user's key) → Result → Cache
                              │
                              └─→ Rate limiting per user
```

## Crates Structure

```
crates/
├── core/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── feed/          # Feed parsing, OPML
│   │   ├── article/       # Article model, content extraction
│   │   ├── rules/         # Regex + AI rules engine
│   │   ├── ai/            # AI provider abstraction
│   │   └── error.rs
│   └── Cargo.toml
│
├── api/
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/        # Axum handlers
│   │   ├── middleware/    # Auth, logging, rate limit
│   │   ├── extractors/    # Custom Axum extractors
│   │   └── state.rs       # App state
│   └── Cargo.toml
│
└── worker/
    ├── src/
    │   ├── main.rs
    │   ├── jobs/          # Feed fetch, AI processing
    │   ├── scheduler.rs   # Cron-like scheduling
    │   └── queue.rs       # Redis queue consumer
    └── Cargo.toml
```

## Apps Structure

```
apps/
└── web/
    ├── src/
    │   ├── app/           # Next.js App Router
    │   ├── components/    # React components
    │   │   ├── ui/        # Shadcn components
    │   │   ├── feed/      # Feed-specific components
    │   │   └── layout/    # Layout components
    │   ├── hooks/         # Custom React hooks
    │   ├── lib/           # Utilities, API client
    │   ├── stores/        # Zustand stores
    │   └── types/         # TypeScript types
    ├── public/
    ├── next.config.js
    ├── tailwind.config.js
    └── package.json
```

## Database Schema (simplified)

```sql
-- Users & Auth
users (id, email, created_at, tier, stripe_customer_id)
api_keys_ai (id, user_id, provider, encrypted_key)

-- Feeds & Articles
feeds (id, user_id, url, title, last_fetched, fetch_interval)
folders (id, user_id, name, parent_id)
feed_folders (feed_id, folder_id)
articles (id, feed_id, guid, title, content, url, published_at)
article_states (article_id, user_id, read, starred, tags[])

-- Rules
rules (id, user_id, feed_id NULL=global, type, pattern, action, priority)

-- Teams (v1.2)
organizations (id, name, owner_id)
org_members (org_id, user_id, role)
shared_feeds (org_id, feed_id)
```

## Performance Targets

| Metric | Target |
|--------|--------|
| API p95 latency | < 200ms |
| Feed fetch (single) | < 30s timeout |
| 1000 feeds refresh | < 5 min (parallel) |
| AI rule evaluation | < 5s per batch |
| Web bundle size | < 500KB gzipped |
| Time to first article | < 2s |
