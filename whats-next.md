# FeedMind.ai - Handoff Document

**Date:** 2026-01-27
**Contexte:** Session initiale - Nouveau projet

---

<original_task>
Aucune tâche spécifique n'a été demandée dans cette session. L'utilisateur a invoqué `/whats-next` pour documenter l'état actuel du projet FeedMind.ai.
</original_task>

<work_completed>
## Analyse du projet existant

### Structure du projet
Le projet FeedMind.ai vient d'être initialisé avec :
- Un commit initial (hash: `9e8184f`)
- Documentation BMAD complète (PRD v2.1.0)
- Configuration Claude Code (`.claude/`)

### Fichiers existants analysés
1. **`_bmad/docs/prd.md`** - PRD ultra-complet (50KB+) couvrant :
   - Vision et positionnement marché
   - Stack technique détaillée (Rust backend, Next.js frontend)
   - User journeys (5 personas)
   - Requirements fonctionnels et non-fonctionnels
   - Modèle économique (Freemium + BYOK)
   - Critères de succès (dogfooding avec 5 utilisateurs internes)

2. **`.claude/memory/CONSTITUTION.md`** - Règles non-négociables :
   - Standards de qualité Rust et Frontend
   - Coverage tests minimum (85% core, 75% API)
   - Architecture crates (core, api, worker)

3. **`.claude/memory/tech-stack.md`** - Stack technique détaillée :
   - Backend: Rust 2021, Axum 0.7+, SQLx, Tokio
   - Frontend: Next.js 15, React 19, Shadcn UI, Tailwind
   - Infra: PostgreSQL 16, Redis 7, Self-hosted primary

### État Git
- Branche: `main`
- Statut: PRD modifié (non commité)
- Dernier commit: `Initial commit: FeedMind.ai project setup`
</work_completed>

<work_remaining>
## Phase 1: Initialisation du projet (M1 - Semaine 1-4)

### 1. Structure Rust Backend
```
Priorité: HAUTE
Fichiers à créer:
- Cargo.toml (workspace)
- crates/core/Cargo.toml + src/lib.rs
- crates/api/Cargo.toml + src/main.rs
- crates/worker/Cargo.toml + src/main.rs
```

### 2. Structure Frontend Next.js
```
Priorité: HAUTE
Fichiers à créer:
- apps/web/package.json
- apps/web/next.config.js
- apps/web/tailwind.config.js
- apps/web/src/app/layout.tsx
- apps/web/src/app/page.tsx
```

### 3. Infrastructure base
```
Priorité: HAUTE
Fichiers à créer:
- docker-compose.yml (PostgreSQL, Redis)
- .env.example
- migrations/ (SQLx migrations)
```

### 4. Core: Feed Parsing (M1 critère de validation)
```
Priorité: CRITIQUE
Modules à implémenter:
- crates/core/src/feed/parser.rs (RSS 2.0, Atom 1.0, JSON Feed)
- crates/core/src/feed/opml.rs (import OPML)
- crates/core/src/feed/error.rs
Tests requis: 85% coverage
```

### 5. Core: Rules Engine
```
Priorité: HAUTE
Modules à implémenter:
- crates/core/src/rules/regex.rs (règles regex)
- crates/core/src/rules/evaluator.rs
Tests requis: 85% coverage
```

### 6. API: Endpoints de base
```
Priorité: MOYENNE
Routes à implémenter:
- POST /api/v1/feeds (add feed)
- GET /api/v1/feeds (list feeds)
- POST /api/v1/opml/import
- GET /api/v1/articles
- PATCH /api/v1/articles/:id (mark read)
```

### 7. Frontend: Écrans de base
```
Priorité: MOYENNE
Écrans à créer:
- /login, /register
- / (feed list + articles)
- /settings
```

## Phase 2: Intégration IA (M2 - Semaine 5-6)

### 8. Core: AI Provider Abstraction
```
Modules à implémenter:
- crates/core/src/ai/provider.rs (trait)
- crates/core/src/ai/anthropic.rs
- crates/core/src/ai/gemini.rs
```

### 9. AI Rules
```
Modules à implémenter:
- crates/core/src/rules/ai.rs (natural language rules)
- Preview sur 7 derniers jours
- Explicabilité (raison pour chaque match)
```

## Milestones PRD
| Milestone | Échéance | Critères |
|-----------|----------|----------|
| M1: Alpha fonctionnelle | S+4 | Import OPML, lecture articles, règles regex |
| M2: IA intégrée | S+6 | Règles langage naturel, explicabilité |
| M3: Mobile ready | S+8 | App iOS/Android, sync |
| M4: Dogfooding complet | S+10 | 5 users migrent d'Inoreader |
| M5: V1 stable | S+12 | 30 jours sans bug bloquant |
</work_remaining>

<attempted_approaches>
Aucune approche n'a été tentée - c'est le début du projet. Cette session était une analyse de l'état existant.
</attempted_approaches>

<critical_context>
## Décisions architecturales clés

### Backend Rust
- **Workspace Cargo** avec 3 crates: `core`, `api`, `worker`
- **Axum 0.7+** pour le framework HTTP
- **SQLx** avec vérification compile-time des queries SQL
- **Tokio** pour l'async runtime

### Frontend
- **Next.js 15** avec App Router
- **Bun** comme runtime (pas npm/pnpm)
- **Shadcn UI** pour les composants (à installer via CLI)
- **Tailwind CSS 4**

### Conventions obligatoires
- **Pas de `unwrap()`** sans justification
- **Pas de `any`** sans commentaire
- **Couleurs via CSS variables** (jamais de hex hardcodé)
- **Response API**: toujours `{ data, meta }`
- **Pagination**: cursor-based
- **Tests**: minimum 70% global, 85% core

### BYOK (Bring Your Own Key)
- Les utilisateurs fournissent leur propre clé API IA
- Providers V1: Anthropic (Claude), Google (Gemini)
- Clés chiffrées si stockées (AES-256-GCM)
- Jamais de clés en logs

### Philosophie V1: Dogfooding
- **PAS d'utilisateurs externes** en V1
- Objectif: 5 personnes internes migrent d'Inoreader
- Métriques: 100% DAU, -50% temps de tri

### License
- **AGPL-3.0** : code source doit être partagé si modifié et exposé publiquement

## Contraintes techniques (PRD AMD-003)
- Max 1000 flux par user (free: 25)
- Max 10 règles IA simultanées
- Timeout fetch: 30s
- Refresh minimum: 15min
- Cache images: 24h (free), 30j (pro)

## Fichiers de référence
- PRD complet: `_bmad/docs/prd.md`
- Amendments: `_bmad/docs/prd-amendments-v2.1.md`
- Constitution: `.claude/memory/CONSTITUTION.md`
- Stack: `.claude/memory/tech-stack.md`
- Standards Rust: `.claude/standards/code-style-rust.md`
- Standards Frontend: `.claude/standards/code-style-frontend.md`
</critical_context>

<current_state>
## État actuel: Phase 0 - Documentation complète, code à démarrer

### Livrables
| Livrable | Statut |
|----------|--------|
| PRD | ✅ Complet (v2.1.0) |
| Constitution technique | ✅ Complet |
| Stack technique | ✅ Définie |
| Structure Rust backend | ❌ Non commencé |
| Structure Next.js frontend | ❌ Non commencé |
| Docker Compose | ❌ Non commencé |
| Database migrations | ❌ Non commencé |
| Premier endpoint API | ❌ Non commencé |

### Git
- Une modification non commitée: `_bmad/docs/prd.md`
- Action recommandée: commiter les dernières modifications du PRD avant de commencer le code

### Prochaine action recommandée
1. Commiter le PRD modifié
2. Créer le `Cargo.toml` workspace
3. Créer les 3 crates vides (core, api, worker)
4. Créer `docker-compose.yml` avec PostgreSQL + Redis
5. Commencer par `crates/core/src/feed/parser.rs` (critère M1)

### Questions ouvertes
- Authentication: Clerk vs custom JWT ? (PRD mentionne les deux)
- Hosting primaire: quel mini PC ou VPS ?
- Domaine: feedmind.ai déjà réservé ?
</current_state>

---

**Généré par:** Claude Code
**Pour continuer:** Copier ce document dans une nouvelle session et demander de commencer par le "Prochaine action recommandée"
