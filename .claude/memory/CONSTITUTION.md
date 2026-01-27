# FeedMind Constitution v1.0

> Règles non-négociables pour le développement.

---

## Article 1: Stack Technique

| Backend | Frontend | Infrastructure |
|---------|----------|----------------|
| Rust Edition 2021 | Next.js 15 + React 19 | PostgreSQL |
| Axum + Tokio | Bun runtime | Redis (cache + queues) |
| SQLx (async) | Shadcn UI + Tailwind | Clever Cloud (backup) |
| feed-rs + scraper | Zustand + TanStack Query | Self-hosted (primary) |

---

## Article 2: Architecture

```
crates/
├── core/       → Logique métier (feeds, rules, AI)
├── api/        → Serveur Axum (REST API)
└── worker/     → Background jobs (fetch, AI processing)

apps/
└── web/        → Next.js frontend

_bmad/          → Documentation BMAD (PRD, architecture)
```

---

## Article 3: Qualité Rust

| Interdit | Alternative |
|----------|-------------|
| `unwrap()` sans justification | `?` ou `expect("reason")` |
| `unsafe` sans commentaire | Commentaire obligatoire |
| `panic!()` en lib | `Result<T, E>` |
| `clone()` inutile | Références ou Arc |
| SQL inline | SQLx queries typées |

---

## Article 4: Qualité Frontend

| Interdit | Alternative |
|----------|-------------|
| `any` sans justification | `unknown` ou type explicite |
| Couleurs hardcodées | CSS variables |
| useEffect pour fetch | TanStack Query |
| État global pour tout | Zustand pour global, local sinon |

---

## Article 5: Tests

| Scope | Coverage Min |
|-------|--------------|
| Core (rules, parsing) | 85% |
| API routes | 75% |
| Worker | 80% |
| Global | 70% |

---

## Article 6: API Design

- Response envelope `{ data, meta }`
- Error codes structurés
- Request ID partout
- Pagination cursor-based
- Auth via Bearer token (Clerk ou custom JWT)

---

## Article 7: IA

| Règle | Description |
|-------|-------------|
| BYOK only (V1) | User fournit sa clé API |
| Providers | Anthropic (Claude), Google (Gemini) |
| Pas de stockage clés | Clés en session/encrypted, jamais en clair |
| Fallback gracieux | Si IA échoue, feature dégradée sans crash |
| Batch processing | Grouper les appels IA pour réduire coûts |

---

## Article 8: Sécurité

| Règle | Description |
|-------|-------------|
| Secrets | Jamais en code, toujours env vars |
| User data | Isolation stricte par tenant |
| API keys IA | Chiffrées at-rest si stockées |
| CORS | Whitelist strict en prod |
| Rate limiting | Par user et par endpoint |

---

## Article 9: Git

- Conventional Commits
- CI passe avant merge
- Pas de force push sur main
- PRs obligatoires pour features

---

## Article 10: License

**AGPL-3.0** : Le code source doit être partagé si le service est modifié et exposé publiquement.
