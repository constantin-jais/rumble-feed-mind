# API Design - FeedMind

## Base URL

```
Production: https://api.feedmind.app/v1
Development: http://localhost:3001/v1
```

## Authentication

```
Authorization: Bearer <jwt_token>
```

Tokens issued via Clerk or custom auth endpoint.

## Endpoints

### Auth

```
POST /auth/register          # Create account
POST /auth/login             # Get JWT
POST /auth/refresh           # Refresh token
DELETE /auth/logout          # Invalidate token
```

### User

```
GET    /user/me              # Current user profile
PATCH  /user/me              # Update profile
GET    /user/me/usage        # Usage stats (feeds, rules, etc.)
DELETE /user/me              # Delete account
```

### AI Keys (BYOK)

```
GET    /user/ai-keys         # List configured providers
POST   /user/ai-keys         # Add AI provider key
DELETE /user/ai-keys/:id     # Remove AI key
POST   /user/ai-keys/:id/test # Test key validity
```

### Feeds

```
GET    /feeds                # List user's feeds
POST   /feeds                # Subscribe to feed
GET    /feeds/:id            # Feed details
PATCH  /feeds/:id            # Update feed settings
DELETE /feeds/:id            # Unsubscribe
POST   /feeds/:id/refresh    # Force refresh
POST   /feeds/import         # Import OPML (multipart)
GET    /feeds/export         # Export OPML
POST   /feeds/discover       # Auto-discover feed from URL
```

### Folders

```
GET    /folders              # List folders
POST   /folders              # Create folder
PATCH  /folders/:id          # Update folder
DELETE /folders/:id          # Delete folder
POST   /folders/:id/feeds    # Add feed to folder
DELETE /folders/:id/feeds/:feedId # Remove feed from folder
```

### Articles

```
GET    /articles             # List articles (paginated, filterable)
GET    /articles/:id         # Article details
GET    /articles/:id/content # Full extracted content (Readability)
PATCH  /articles/:id         # Update state (read, starred, tags)
POST   /articles/mark-read   # Batch mark as read
```

#### Articles Query Params

```
?feed_id=123              # Filter by feed
?folder_id=456            # Filter by folder
?status=unread|read|all   # Read status
?starred=true             # Starred only
?tag=tech                 # By tag
?search=keyword           # Full-text search (v1.1)
?since=2026-01-01         # Published after
?cursor=abc123            # Pagination cursor
?limit=50                 # Items per page (max 100)
```

### Rules

```
GET    /rules                # List all rules
POST   /rules                # Create rule
GET    /rules/:id            # Rule details
PATCH  /rules/:id            # Update rule
DELETE /rules/:id            # Delete rule
POST   /rules/:id/test       # Test rule against sample articles
POST   /rules/preview        # Preview rule results (dry-run)
```

#### Rule Types

```json
{
  "type": "regex",
  "scope": "global" | "feed",
  "feed_id": null | "123",
  "field": "title" | "content" | "author" | "url",
  "pattern": "crypto|bitcoin",
  "flags": "i",
  "action": "hide" | "tag" | "star",
  "action_value": "crypto"
}

{
  "type": "ai",
  "scope": "global" | "feed",
  "feed_id": null | "123",
  "prompt": "Hide articles about cryptocurrency",
  "action": "hide" | "tag" | "star",
  "action_value": null
}
```

### AI Features

```
POST   /ai/summarize         # Summarize article(s)
POST   /ai/translate         # Translate article content
POST   /ai/categorize        # Auto-suggest tags
```

### Fever API (v1.1)

```
POST   /fever                # Fever API compatible endpoint
```

### Admin / Billing

```
GET    /billing/subscription # Current subscription
POST   /billing/checkout     # Create Stripe checkout session
POST   /billing/portal       # Stripe customer portal
POST   /billing/webhook      # Stripe webhook handler
```

---

## Response Envelope

### Success

```json
{
  "data": { ... },
  "meta": {
    "request_id": "req_abc123xyz",
    "timestamp": "2026-01-27T10:30:00Z",
    "pagination": {
      "cursor": "next_abc123",
      "has_more": true,
      "total": 1523
    }
  }
}
```

### Error

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Feed URL is required",
    "details": {
      "field": "url",
      "reason": "missing"
    }
  },
  "meta": {
    "request_id": "req_abc123xyz",
    "timestamp": "2026-01-27T10:30:00Z"
  }
}
```

---

## Error Codes

| Code | HTTP | Description |
|------|------|-------------|
| VALIDATION_ERROR | 400 | Input validation failed |
| UNAUTHORIZED | 401 | Missing or invalid token |
| FORBIDDEN | 403 | Insufficient permissions |
| NOT_FOUND | 404 | Resource not found |
| CONFLICT | 409 | Resource already exists |
| TIER_LIMIT | 402 | Plan limit reached |
| RATE_LIMITED | 429 | Too many requests |
| AI_ERROR | 502 | AI provider error |
| AI_KEY_INVALID | 422 | User's AI key is invalid |
| FEED_UNREACHABLE | 422 | Cannot fetch feed URL |
| INTERNAL_ERROR | 500 | Server error |

---

## Rate Limiting

| Tier | Limit |
|------|-------|
| Free | 60 req/min |
| Pro | 300 req/min |
| Team | 600 req/min |

Headers:
```
X-RateLimit-Limit: 300
X-RateLimit-Remaining: 299
X-RateLimit-Reset: 1706352600
```

---

## Pagination

Cursor-based pagination for all list endpoints:

```json
{
  "data": [...],
  "meta": {
    "pagination": {
      "cursor": "eyJpZCI6MTIzfQ",
      "has_more": true
    }
  }
}
```

Next page: `?cursor=eyJpZCI6MTIzfQ`

---

## Webhooks (future)

```
POST /webhooks/configure     # Configure webhook URL
GET  /webhooks/events        # List available events
```

Events:
- `article.new` - New article matched rules
- `feed.error` - Feed fetch failed
- `rule.matched` - Rule matched article
