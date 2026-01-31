---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
inputDocuments: []
workflowType: 'prd'
lastStep: 11
documentCounts:
  briefs: 0
  research: 0
  brainstorming: 1
  projectDocs: 0
bmadVersion: '2.1-ultra-complete-amended'
amendmentsApplied: [AMD-001, AMD-002, AMD-003, AMD-004, AMD-005, AMD-006, AMD-007, AMD-008, AMD-009, AMD-010, AMD-011, AMD-012, AMD-013]
---

# Product Requirements Document - FeedMind.ai

**Author:** Constantin Jais  
**Date:** 2026-01-27  
**Version:** 2.1.0 (BMAD Ultra-Complete + Amendments)  
**Tagline:** "La veille qui nourrit ton IA"  
**License:** AGPL-3.0  
**Amendments:** Voir `prd-amendments-v2.1.md` pour le détail des modifications

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Project Classification](#project-classification)
3. [Success Criteria](#success-criteria)
4. [User Journeys](#user-journeys)
5. [Domain-Specific Requirements](#domain-specific-requirements)
6. [Innovation & Novel Patterns](#innovation--novel-patterns)
7. [Scoping (MVP V1)](#scoping-mvp-v1)
8. [Functional Requirements](#functional-requirements)
9. [Non-Functional Requirements](#non-functional-requirements)
10. [Technical Limits & Constraints](#technical-limits--constraints) *(AMD-003)*
11. [Security & Secrets Management](#security--secrets-management) *(AMD-004)*
12. [Infrastructure & Costs](#infrastructure--costs) *(AMD-008)*
13. [Legal & RGPD](#legal--rgpd) *(AMD-009)*
14. [Monitoring & Operations](#monitoring--operations) *(AMD-010)*
15. [Appendices](#appendices)

---

## Executive Summary

### Vision Statement

FeedMind.ai est un **pipeline de veille intelligent** qui transforme le flux RSS chaotique en connaissance structurée, prête à alimenter des systèmes IA (RAG, bases de connaissance, assistants personnels).

**Notre conviction** : L'information n'a de valeur que si elle est filtrée, comprise et actionnable. Les lecteurs RSS traditionnels sont des agrégateurs passifs - FeedMind est un **curateur actif** qui comprend le contenu et s'adapte à vos besoins.

### Le Problème que Nous Résolvons

#### Problème Principal : La Noyade Informationnelle

Les professionnels de la veille, développeurs, chercheurs et équipes tech font face à une contradiction douloureuse : ils ont besoin de suivre beaucoup de sources pour rester informés, mais le volume les submerge.

**Statistiques typiques d'un power user** :
- 500-2000 flux RSS suivis
- 300-500 nouveaux articles/jour
- 2-3h de tri manuel quotidien
- 80% des articles ne sont pas pertinents pour eux

**Ce que font les utilisateurs aujourd'hui** :
1. **Inoreader/Feedly** : Règles regex limitées, pas de compréhension sémantique
2. **Scripts maison** : Maintenance lourde, pas de mobile, fragile
3. **Abandon** : Beaucoup abandonnent et se contentent de Twitter/LinkedIn

#### Problèmes Secondaires

| Problème | Impact | Solutions actuelles insuffisantes |
|----------|--------|-----------------------------------|
| **Règles trop techniques** | Les regex excluent les non-développeurs | Aucune alternative sémantique |
| **Vendor lock-in** | Données piégées dans les silos | Export OPML partiel, pas de données |
| **Prix élevé** | Inoreader Pro à 9.99$/mois | Pas de tier intermédiaire |
| **Mobile dégradé** | Apps mobiles des concurrents médiocres | Pas d'investissement |
| **Pas d'explicabilité** | Les règles agissent sans expliquer | Interface opaque |
| **Silos IA** | L'info reste piégée, pas d'export vers RAG | Aucune solution |

### Ce qui Rend FeedMind.ai Unique

#### Différenciateurs Primaires (Defensible)

| Différenciateur | Pourquoi c'est unique | Barrière à l'entrée |
|-----------------|----------------------|---------------------|
| **Règles en langage naturel** | Premier lecteur RSS avec filtrage sémantique IA | Expertise prompt engineering + feedback loop |
| **Explicabilité native** | Chaque décision IA est justifiée en langage humain | Architecture conçue pour ça dès le départ |
| **BYOK (Bring Your Own Key)** | L'utilisateur contrôle ses coûts IA | Modèle économique radicalement différent |
| **Pipeline vers l'IA** | Export structuré pour RAG/bases de connaissance | API conçue pour ce use case |

#### Différenciateurs Secondaires (Table stakes améliorés)

| Différenciateur | Ce que font les autres | Ce que fait FeedMind |
|-----------------|------------------------|---------------------|
| **Multi-plateforme** | Web + apps natives séparées | Expo unifié : une codebase, sync parfait |
| **No vendor lock-in** | Export OPML partiel | Export complet (flux + articles + règles + tags) |
| **Open Source** | Aucun concurrent open source | AGPL-3.0, self-host possible |
| **Smart Polling** | Refresh fixe toutes les heures | Adaptatif selon activité du flux |
| **Privacy-first** | Tracking analytics, images non proxiées | Zéro tracking, images proxiées |

### Positionnement Marché

```
                    Prix
                     ↑
        Expensive    │    Feedly Pro (12$/mois)
                     │         ●
                     │    Inoreader Pro (9.99$/mois)
                     │         ●
        Mid-range    │                      FeedMind Pro (5€/mois)
                     │                           ◆
        Affordable   │    Feedly Basic
                     │         ●
                     │
        Free         │    Inoreader Free
                     │         ●
                     └─────────────────────────────────────────→ Intelligence
                        Basic        Regex        Semantic AI
                       Folders       Rules          Rules
```

**FeedMind se positionne** : Prix accessible + Intelligence IA = Segment inexploité

### Audience Cible

#### Segment Primaire : Power Users Techniques (V1-V2)

**Profil** :
- Développeurs, architectes, tech leads
- Suivent 200-2000 flux RSS
- À l'aise avec les concepts techniques
- Utilisent déjà des outils comme Inoreader
- Ont potentiellement une clé API OpenAI/Anthropic

**Douleur principale** : "Je passe 2h/jour à trier des articles. Les regex ne suffisent plus."

**Willingness to pay** : 5-15€/mois pour un outil qui leur fait gagner du temps

#### Segment Secondaire : Équipes Veille (V2+)

**Profil** :
- Équipes de veille concurrentielle, market intelligence
- Analystes, consultants
- Besoin de partager des flux et des insights
- Rapports périodiques à produire

**Douleur principale** : "On fait de la veille en silo, on duplique les efforts."

**Willingness to pay** : 15-50€/mois par équipe

---

## Project Classification

### Classification Technique

| Dimension | Classification | Justification |
|-----------|----------------|---------------|
| **Type de projet** | SaaS Platform + Self-hosted | Cloud managed + option Docker |
| **Domaine métier** | Content Aggregation / Knowledge Management | RSS + IA + Curation |
| **Complexité** | **Élevée** | Multi-plateforme, IA, real-time sync |
| **Contexte** | Greenfield | Nouveau projet, aucune dette technique |
| **Time-to-market** | 3-4 mois (MVP) | Équipe de 1-2 devs |

### Stack Technique Détaillée

#### Backend (Rust)

| Composant | Technologie | Justification |
|-----------|-------------|---------------|
| **Langage** | Rust 2021 Edition | Performance, fiabilité, async natif |
| **Framework HTTP** | Axum 0.7+ | Ergonomique, tower ecosystem |
| **Async Runtime** | Tokio | Standard de facto Rust async |
| **ORM/Database** | SQLx | Type-safe, compile-time checked |
| **RSS Parsing** | feed-rs | Robuste, gère tous les formats |
| **HTML Parsing** | scraper + readability | Extraction contenu clean |
| **Background Jobs** | Custom queue sur Redis | Simple, observable |
| **Serialization** | serde + serde_json | Standard Rust |
| **Validation** | validator | Validation déclarative |
| **Error Handling** | thiserror + anyhow | Errors structurées |

#### Frontend (Expo Unifié)

| Composant | Technologie | Justification |
|-----------|-------------|---------------|
| **Framework** | Expo SDK 52+ | Une codebase, 3 plateformes |
| **Routing** | Expo Router | File-based, deep linking natif |
| **Styling** | NativeWind 4+ | Tailwind pour React Native |
| **State Global** | Zustand | Simple, performant, TypeScript |
| **Data Fetching** | TanStack Query v5 | Cache intelligent, optimistic updates |
| **Forms** | React Hook Form + Zod | Validation type-safe |
| **Icons** | Lucide React Native | Consistant, léger |
| **Animations** | Reanimated 3 | 60fps, gesture-driven |

#### Infrastructure

| Composant | Technologie | Justification |
|-----------|-------------|---------------|
| **Database** | PostgreSQL 16 | JSONB, full-text search, fiable |
| **Cache/Queue** | Redis 7 | Pub/sub, streams, cache |
| **Storage** | S3-compatible (MinIO/Clever) | Images, exports |
| **Hosting Cloud** | Clever Cloud | France, RGPD, PostgreSQL géré |
| **Hosting Self** | Docker Compose | Simple, reproductible |
| **CI/CD** | GitHub Actions | Standard, gratuit pour OSS |
| **Monitoring** | Prometheus + Grafana | Métriques, alertes |
| **Logs** | Vector + Loki | Logs structurés, recherche |

#### IA Providers (BYOK)

| Provider | Modèles supportés | Use case |
|----------|-------------------|----------|
| **Anthropic** | Claude 3.5 Sonnet, Claude 3 Haiku | Règles IA, résumés |
| **Google** | Gemini 1.5 Pro, Gemini 1.5 Flash | Alternative, moins cher |
| **OpenAI** (V2) | GPT-4o, GPT-4o-mini | Compatibilité étendue |
| **Local** (V3+) | Ollama (Llama, Mistral) | Self-host complet |

### Architecture Monorepo

```
feedmind/
├── .github/
│   └── workflows/          # CI/CD pipelines
├── crates/
│   ├── core/              # 🦀 Business logic
│   │   ├── src/
│   │   │   ├── feeds/     # Feed parsing, fetching
│   │   │   ├── rules/     # Regex + AI rules engine
│   │   │   ├── articles/  # Article management
│   │   │   ├── ai/        # AI provider abstraction
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── api/               # 🌐 HTTP API (Axum)
│   │   ├── src/
│   │   │   ├── routes/    # API endpoints
│   │   │   ├── middleware/# Auth, rate limit
│   │   │   ├── extractors/# Request parsing
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   └── worker/            # ⚙️ Background jobs
│       ├── src/
│       │   ├── jobs/      # Fetch, AI processing
│       │   ├── scheduler/ # Smart polling
│       │   └── main.rs
│       └── Cargo.toml
├── apps/
│   └── app/               # 📱 Expo unified app
│       ├── app/           # Expo Router (file-based)
│       │   ├── (tabs)/    # Tab navigation
│       │   ├── (auth)/    # Auth screens
│       │   ├── feed/      # Feed screens
│       │   ├── article/   # Article screens
│       │   └── settings/  # Settings screens
│       ├── components/    # React Native components
│       │   ├── ui/        # Base UI (NativeWind)
│       │   └── features/  # Feature components
│       ├── hooks/         # Custom hooks
│       ├── lib/           # API client, utils
│       ├── stores/        # Zustand stores
│       └── package.json
├── packages/
│   └── shared/            # 📦 Shared types/utils
│       ├── src/
│       │   ├── types/     # TypeScript types
│       │   └── validation/# Zod schemas
│       └── package.json
├── docs/                  # 📚 Documentation
├── docker/                # 🐳 Docker configs
├── Cargo.toml             # Workspace root
├── package.json           # Bun workspace root
└── README.md
```

### Modèle Économique Détaillé *(AMD-001 Applied)*

#### Stratégie Freemium + Trial

```
FREE FOREVER ──────► PRO TRIAL (14j) ──────► PRO (5€/mois)
    │                      │                      │
    │ Pas de CB           │ CB requise          │ Facturation
    │ 25 flux, 3 règles   │ Illimité + IA       │ Continue
    │ Pas d'IA            │                      │
    └──────────────────────┴──────────────────────┘
```

#### Tiers et Pricing

| Tier | Prix | CB requise | Cible |
|------|------|------------|-------|
| **Free** | 0€ forever | Non | Découverte, petits besoins |
| **Pro Trial** | 0€ (14j) | Oui | Test des features Pro + IA |
| **Pro** | 5€/mois ou 50€/an | Oui | Power users |
| **Team** | 15€/user/mois | Oui | Équipes (V2) |

#### Limites par Tier

| Limite | Free | Pro Trial | Pro | Team |
|--------|------|-----------|-----|------|
| **Durée** | Illimitée | 14 jours | Illimitée | Illimitée |
| **Flux RSS** | 25 | Illimité | Illimité | Illimité |
| **Articles stockés** | 500 | Illimité | Illimité | Illimité |
| **Règles regex** | 3 | Illimité | Illimité | Illimité |
| **Règles IA** | ❌ | ✅ (BYOK) | ✅ (BYOK/Managed) | ✅ (BYOK/Managed) |
| **Résumé IA** | ❌ | ✅ (BYOK) | ✅ (BYOK/Managed) | ✅ (BYOK/Managed) |
| **Recherche** | Titre seul | Full-text | Full-text | Full-text |
| **Flux prioritaires** | 3 | Illimité | Illimité | Illimité |
| **Cache images** | 24h | 30 jours | 30 jours | 30 jours |
| **Override refresh** | 0 | 20 flux | 20 flux | 50 flux |
| **Export OPML** | ✅ | ✅ | ✅ | ✅ |
| **Export complet** | ❌ | ✅ | ✅ | ✅ |
| **API access** | ❌ | ❌ | Lecture | Lecture + Écriture |
| **Users par org** | 1 | 1 | 1 | 10 (extensible) |
| **Flux partagés** | ❌ | ❌ | ❌ | ✅ |
| **Support** | Community | Email | Email 48h | Prioritaire |

#### Managed IA *(AMD-005 - V1.1)*

**Option pour utilisateurs sans clé API** :
- FeedMind utilise sa propre clé Anthropic
- Facturation au token + 10% marge
- Limite soft : 100k tokens/mois
- Dashboard de suivi de consommation

#### Économie BYOK

**Pourquoi BYOK ?**
- L'utilisateur paie directement son provider IA (Anthropic, Google)
- FeedMind ne prend pas de marge sur les tokens
- Pas de coûts imprévisibles pour FeedMind
- L'utilisateur garde le contrôle total de ses dépenses

**Coût IA estimé par utilisateur (Claude 3.5 Sonnet)** :
- ~100 règles IA évaluées/jour × 30 jours = 3000 évaluations/mois
- ~500 tokens/évaluation en moyenne
- 1.5M tokens/mois ≈ 4.50$/mois (input) + 2.25$/mois (output)
- **Total : ~7$/mois en coût IA direct**

**Option Managed (V2+)** :
- FeedMind proxie les appels IA avec +10% de marge
- Pour utilisateurs qui ne veulent pas gérer leurs clés
- Facturation au token consommé

---

## Success Criteria

### Philosophie V1 : Dogfooding Strict

**Principe fondamental** : FeedMind V1 n'a pas d'utilisateurs externes. Le produit est un succès quand l'équipe fondatrice (5 personnes maximum) l'utilise quotidiennement à la place de leurs outils actuels.

**Pourquoi cette approche ?**
1. Les meilleurs produits naissent de la douleur personnelle de leurs créateurs
2. Pas de biais "utilisateur qui ne se plaint pas" - on vit les problèmes
3. Itération ultra-rapide sans gestion de support externe
4. Focus sur la qualité plutôt que la croissance prématurée

### Métriques de Succès V1

#### Métriques d'Adoption (L'équipe utilise-t-elle vraiment FeedMind ?)

| Métrique | Cible | Mesure | Fréquence |
|----------|-------|--------|-----------|
| **Daily Active Users** | 5/5 (100%) | Logs connexion | Quotidien |
| **Flux importés** | 1000+ par user | COUNT(feeds) | Hebdo |
| **Règles actives** | 50+ par user | COUNT(rules WHERE active) | Hebdo |
| **Sessions/jour** | ≥2 par user | Analytics internes | Quotidien |
| **Temps dans l'app** | ≥30min/jour | Analytics internes | Quotidien |

#### Métriques de Remplacement (A-t-on vraiment remplacé Inoreader ?)

| Métrique | Cible | Mesure | Fréquence |
|----------|-------|--------|-----------|
| **Retour à Inoreader** | 0 users | Self-report | Hebdo |
| **Temps de tri quotidien** | -50% vs avant | Self-report | Mensuel |
| **Articles lus/jour** | Stable ou + | COUNT(read_at) | Hebdo |
| **Règles IA utilisées** | ≥10 par user | COUNT(ai_rules) | Hebdo |

#### Métriques Techniques (Le système tient-il la charge ?)

| Métrique | Cible | Mesure | Fréquence |
|----------|-------|--------|-----------|
| **Uptime** | ≥99% | Monitoring | Continu |
| **API p95 latency** | <200ms | Prometheus | Continu |
| **Refresh 1000 flux** | <5min | Job duration | Hebdo |
| **Bugs bloquants** | 0 | Issue tracker | Continu |
| **Sync lag** | <2s | Test manuel | Hebdo |

#### Métriques IA (L'IA est-elle utile ?)

| Métrique | Cible | Mesure | Fréquence |
|----------|-------|--------|-----------|
| **Précision règles IA** | ≥80% | Feedback "correct/incorrect" | Hebdo |
| **Faux positifs** | <20% | Articles restaurés/masqués | Hebdo |
| **Utilisation résumés** | ≥1/jour par user | COUNT(summaries) | Hebdo |
| **Temps évaluation batch** | <10s (10 articles) | Job duration | Continu |

### Le "Moment Magique"

**Définition** : Le moment où l'utilisateur réalise que FeedMind est différent.

**Scénario** :
1. L'utilisateur crée sa première règle en langage naturel : "Masquer les articles clickbait"
2. En moins de 30 secondes, il voit :
   - La liste des articles qui SERAIENT masqués (preview)
   - Pour chaque article, l'explication de POURQUOI il serait masqué
3. Il active la règle
4. Un article arrive, est masqué automatiquement
5. Il peut voir dans "Masqués" :
   - L'article avec sa raison "Titre sensationnaliste détecté"
   - Les boutons [Restaurer] et [C'était correct ✓]

**Indicateur du moment magique** : L'utilisateur sourit et pense "Ah ouais, c'est ça que je voulais".

### Critères d'Échec (Red Flags)

| Signal | Seuil Critique | Action |
|--------|----------------|--------|
| **Un user retourne à Inoreader** | 1 user sur 5 | Stop, post-mortem, pivot |
| **IA trop imprécise** | >30% faux positifs | Améliorer prompts, ajouter few-shot |
| **Performance dégradée** | Refresh >15min | Optimiser worker, scaling |
| **Mobile inutilisable** | 1 user n'utilise pas mobile | Focus UX mobile |
| **Import OPML échoue** | 1 import 1000+ flux fail | Débugger parser |
| **Sync cassé** | Délai >30s entre devices | Investiguer WebSocket |
| **Coût IA explosif** | >20$/mois/user | Optimiser batching, caching |

### Timeline et Milestones

| Milestone | Échéance | Critères de validation |
|-----------|----------|------------------------|
| **M1 : Alpha fonctionnelle** | S+4 | Import OPML, lecture articles, règles regex |
| **M2 : IA intégrée** | S+6 | Règles langage naturel, explicabilité |
| **M3 : Mobile ready** | S+8 | App iOS/Android fonctionnelle, sync |
| **M4 : Dogfooding complet** | S+10 | 5 users migrent d'Inoreader |
| **M5 : V1 stable** | S+12 | 30 jours sans bug bloquant |

---

## User Journeys

### Personas

#### Persona 1 : Thomas Lefèvre - Le CTO Curieux

**Démographie** :
- 38 ans, Paris
- CTO d'une startup SaaS B2B (25 employés)
- 15 ans d'expérience tech

**Contexte professionnel** :
- Suit 800+ flux RSS (tech, business, concurrence)
- Utilise Inoreader Pro depuis 5 ans
- A déjà créé 150+ règles regex
- Passe 1h30/jour sur sa veille

**Frustrations actuelles** :
- "Mes règles regex sont devenues un monstre Frankenstein ingérable"
- "Je rate des articles importants car mes filtres sont trop agressifs"
- "Impossible de dire 'montre-moi les levées de fonds > 5M€'"
- "L'app mobile d'Inoreader est lente et buggée"

**Ce qu'il espère** :
- Réduire son temps de veille à 45min/jour
- Des règles qu'il peut écrire en français
- Comprendre pourquoi un article a été filtré
- Une vraie app mobile performante

**Citation** : "Je veux un outil qui comprenne ce que je cherche, pas juste ce que j'écris."

**Clé API** : A déjà une clé Anthropic pour des projets internes.

---

#### Persona 2 : Marie Chen - L'Analyste Méthodique

**Démographie** :
- 29 ans, Lyon
- Analyste en veille concurrentielle dans un cabinet de conseil
- 4 ans d'expérience

**Contexte professionnel** :
- Suit 200 flux pour 3 clients différents
- Utilise Feedly + Google Alerts + scripts Python
- Produit des rapports hebdomadaires
- Frustrée par l'éclatement de ses outils

**Frustrations actuelles** :
- "J'ai 4 outils différents, rien n'est centralisé"
- "Mes scripts Python cassent régulièrement"
- "Je passe plus de temps à trier qu'à analyser"
- "Les non-tech de mon équipe ne peuvent pas utiliser mes outils"

**Ce qu'elle espère** :
- Un seul outil pour toute sa veille
- Pouvoir créer des règles sans coder
- Partager des flux avec son équipe (V2)
- Générer des résumés automatiques pour ses rapports

**Citation** : "J'ai besoin d'un outil que je peux aussi montrer à ma manager."

**Clé API** : N'en a pas, mais le cabinet pourrait en obtenir une.

---

#### Persona 3 : Alex Durand - Le Développeur Indie

**Démographie** :
- 34 ans, Bordeaux (remote)
- Développeur freelance full-stack
- Contribue à l'open source

**Contexte professionnel** :
- Suit 400 flux (dev, design, indie hacking)
- Utilise Miniflux (self-hosted) + Reeder (iOS)
- Aime le contrôle sur ses données
- Budget limité

**Frustrations actuelles** :
- "Miniflux est bien mais basique, pas de règles intelligentes"
- "Je voudrais filtrer les articles promotionnels automatiquement"
- "Les SaaS veulent mes données, je préfère self-host"
- "Pas envie de payer 10$/mois pour Inoreader"

**Ce qu'il espère** :
- Une solution open source qu'il peut self-host
- Des règles IA sans être piégé dans un SaaS
- Un prix raisonnable pour la version cloud
- Pouvoir contribuer au projet

**Citation** : "Si c'est open source et que je peux l'héberger, je suis preneur."

**Clé API** : A une clé OpenAI pour ses projets perso.

---

#### Persona 4 : Sophie Martinez - L'Administratrice Système

**Démographie** :
- 42 ans, Toulouse
- Ops/SRE dans une ESN
- Gère l'infrastructure de 5 clients

**Contexte professionnel** :
- Suit 150 flux (sécurité, CVE, release notes)
- A besoin d'alertes sur les vulnérabilités
- Utilise un mélange de RSS + mailing lists
- Très sensible à la sécurité et la vie privée

**Frustrations actuelles** :
- "Je rate parfois des CVE critiques noyées dans le bruit"
- "Mes outils actuels trackent tout ce que je lis"
- "Pas de filtrage intelligent sur les niveaux de sévérité"
- "L'export de mes données est un cauchemar"

**Ce qu'elle espère** :
- Filtrer automatiquement par sévérité CVE
- Zéro tracking, privacy-first
- Export complet de ses données à tout moment
- Intégration possible avec ses outils de ticketing (V2)

**Citation** : "Je veux un outil qui respecte ma vie privée autant que moi."

**Clé API** : Pourrait en obtenir une via son entreprise.

---

### Journey 1 : Thomas Découvre FeedMind - La Migration d'un Power User

**Contexte** : Thomas, CTO curieux, découvre FeedMind sur Hacker News. Le titre "RSS reader with AI-powered natural language rules" l'intrigue. Il a 5 minutes avant une réunion.

---

**Acte 1 : La Découverte (5 minutes)**

Thomas clique sur le lien. La landing page affiche le tagline "La veille qui nourrit ton IA". Ses yeux scannent rapidement :
- "Règles en langage naturel" → Intéressant
- "BYOK - Utilisez votre propre clé API" → Ah, pas de surprise sur les coûts
- "Open source AGPL-3.0" → Respect

Il clique sur [Essai gratuit 14 jours]. On lui demande une carte bancaire. Il hésite une seconde, puis se dit "Au moins ils sont honnêtes, pas de fake free tier". Il entre sa CB.

L'onboarding démarre : "Importez vos flux". Thomas exporte son OPML d'Inoreader (847 flux, 42 dossiers). Upload. Barre de progression... 45 secondes plus tard : "847 flux importés dans 42 dossiers. Première synchronisation en cours."

Il doit aller en réunion. L'app lui dit : "Je finis de récupérer vos articles. Revenez dans 10 minutes."

**Émotion** : Curiosité prudente. "On verra si c'est vraiment mieux."

---

**Acte 2 : La Première Session (30 minutes)**

Après sa réunion, Thomas rouvre FeedMind. 12,847 articles non-lus. Normal, il a 847 flux. L'interface est clean, similaire à Inoreader mais plus moderne.

Il navigue avec j/k (raccourcis qu'il connaît). Ça répond vite. Il ouvre un article, le contenu s'affiche en mode "Readability" - propre, sans pubs. Il swipe pour voir l'original. Bien.

Il décide de tester LA feature : les règles IA. Settings > Règles > Nouvelle règle IA.

**Écran** : "Décrivez ce que vous voulez filtrer en français"

Il tape : "Masquer les articles qui parlent de crypto sauf Bitcoin et Ethereum"

L'interface lui montre :
```
Preview sur vos 7 derniers jours :
• 23 articles seraient masqués
• 156 articles seraient conservés

Exemples d'articles masqués :
1. "Dogecoin pumps 30% after Elon tweet" 
   → Crypto mentionnée : Dogecoin (non Bitcoin/Ethereum)
   
2. "New Solana NFT marketplace launches"
   → Crypto mentionnée : Solana (non Bitcoin/Ethereum)
   
3. "SHIB holders expect rally"
   → Crypto mentionnée : SHIB (non Bitcoin/Ethereum)
```

Thomas vérifie quelques exemples. Tous corrects. Il valide.

Puis il pense à une règle qu'il n'a jamais pu faire en regex :
"Garder uniquement les levées de fonds supérieures à 5 millions d'euros"

Preview :
```
Preview sur vos 7 derniers jours :
• 89 articles seraient masqués
• 7 articles seraient conservés

Exemples d'articles conservés :
1. "Mistral AI raises €600M Series B"
   → Levée détectée : 600M€ > 5M€ ✓
   
2. "French startup secures $12M seed"
   → Levée détectée : 12M$ (~11M€) > 5M€ ✓

Exemples d'articles masqués :
1. "Local bakery gets €50K grant"
   → Montant détecté : 50K€ < 5M€
```

**Le déclic**. Thomas sourit. "Putain, c'est ça que je voulais depuis des années."

Il passe 20 minutes à créer 5 autres règles IA. À chaque fois, il vérifie le preview, ajuste si besoin ("Non attends, inclus aussi les Series A même si le montant n'est pas mentionné").

**Émotion** : Excitation. Le moment magique s'est produit.

---

**Acte 3 : L'Adoption (Semaine 1)**

Thomas utilise FeedMind quotidiennement. Quelques observations :

**Jour 2** : Il migre ses 150 règles regex manuellement. Fastidieux mais nécessaire. Il en profite pour en transformer 30 en règles IA.

**Jour 3** : Il teste l'app mobile pendant son trajet. Elle est fluide. Le swipe pour marquer lu fonctionne bien. Il lit 15 articles dans le métro.

**Jour 4** : Il découvre la vue "Masqués". Un article sur une acquisition de 8M€ a été masqué par erreur (la règle "levées > 5M" ne l'avait pas capté car le mot "acquisition" et non "levée"). Il restaure l'article et ajuste sa règle : "Garder les levées de fonds OU acquisitions supérieures à 5 millions".

**Jour 5** : Il configure sa clé Anthropic dans FeedMind. L'app lui montre une estimation : "~200 évaluations/jour, coût estimé : ~2$/mois". Acceptable.

**Fin de semaine** : Il n'a pas ouvert Inoreader depuis 4 jours.

**Émotion** : Satisfaction. "C'est mon nouvel outil."

---

**Acte 4 : La Fidélisation (Mois 1)**

**Semaine 2** : Thomas recommande FeedMind à deux collègues. "Testez au moins, le filtrage IA c'est game changer."

**Semaine 3** : Il a maintenant 47 règles IA actives. Son temps de veille est passé de 1h30 à 50 minutes/jour. Il lit moins d'articles mais des articles plus pertinents.

**Semaine 4** : Il découvre un bug mineur (un raccourci qui ne marche pas sur Safari). Il ouvre une issue GitHub. Réponse en 4h, fix déployé en 24h. "Ah ouais, c'est réactif."

**Fin du mois** : Thomas passe en Pro (5€/mois) sans hésiter. Il annule Inoreader.

**Émotion** : Loyauté. "C'est rare qu'un outil tienne ses promesses."

---

**Requirements révélés par ce journey** :

| Requirement | Capability |
|-------------|------------|
| FR-ONBOARD-01 | Import OPML avec preview et confirmation |
| FR-ONBOARD-02 | Détection automatique des dossiers depuis OPML |
| FR-RULE-AI-01 | Création de règle en langage naturel |
| FR-RULE-AI-02 | Preview de l'effet sur les 7 derniers jours |
| FR-RULE-AI-03 | Explication détaillée pour chaque article filtré |
| FR-RULE-AI-04 | Modification de règle avec re-preview |
| FR-HIDDEN-01 | Vue des articles masqués avec raisons |
| FR-HIDDEN-02 | Restauration d'un article masqué |
| FR-UX-01 | Raccourcis clavier (j/k, m, s, etc.) |
| FR-UX-02 | App mobile avec gestures (swipe) |
| FR-SYNC-01 | Sync temps réel multi-device |
| FR-AI-01 | Configuration clé API avec estimation coût |

---

### Journey 2 : Marie Centralise sa Veille - L'Analyste en Quête d'Efficacité

**Contexte** : Marie, analyste en veille, en a marre de jongler entre Feedly, Google Alerts et ses scripts Python. Elle cherche une solution unifiée.

---

**Acte 1 : Le Ras-le-bol (La douleur)**

Lundi matin. Marie doit produire un rapport de veille pour un client. Elle ouvre :
- Feedly pour les flux RSS
- Gmail pour les Google Alerts
- Un notebook Jupyter pour ses scripts de scraping
- Un Google Doc pour prendre des notes

30 minutes à copier-coller entre les outils. Elle pense : "Il doit y avoir une meilleure façon de faire."

Elle google "RSS reader AI filtering" et trouve FeedMind.

---

**Acte 2 : L'Évaluation (Test du trial)**

Marie crée un compte. Elle n'a pas d'OPML (ses flux sont éparpillés), donc elle ajoute manuellement ses 50 flux principaux.

Elle découvre qu'elle peut créer des dossiers : "Client A - Concurrence", "Client B - Marché", "Client C - Tendances".

Elle teste une règle IA : "Garder uniquement les annonces de nouveaux produits ou partenariats"

Preview : 12 articles sur 200 seraient conservés. Elle vérifie - c'est pertinent.

**Problème** : Elle n'a pas de clé API. L'app lui propose deux options :
1. "Obtenez une clé API Anthropic" (lien vers la doc)
2. "Utiliser FeedMind Managed (+10%)" (Coming soon V2)

Elle choisit l'option 1, crée un compte Anthropic, génère une clé. 10 minutes.

Elle configure la clé. L'app teste : "Clé valide ✓. Modèle disponible : Claude 3.5 Sonnet."

---

**Acte 3 : La Découverte de la Valeur**

Marie crée plusieurs règles pour chaque client :

**Client A (concurrence) :**
- "Alerter sur les acquisitions ou fusions dans le secteur bancaire"
- "Masquer les actualités générales, garder uniquement les annonces corporate"

**Client B (marché) :**
- "Garder les études de marché et rapports d'analystes"
- "Masquer les articles d'opinion sans données"

Elle découvre les résumés IA. Pour un article long de 3000 mots, elle clique [Résumer]. En 5 secondes :

```
📝 Résumé IA :
- Étude McKinsey sur l'IA générative dans le retail
- Impact estimé : +15% de productivité
- Adoption actuelle : 23% des retailers US
- Barrières : coût, compétences, données
- Recommandation : commencer par le service client
```

"C'est exactement ce que je mets dans mes rapports !" Elle peut copier-coller les points clés.

---

**Acte 4 : La Productivité Retrouvée**

**Semaine 1** : Marie a centralisé 80% de sa veille dans FeedMind. Ses scripts Python sont abandonnés.

**Semaine 2** : Son temps de production de rapport passe de 3h à 1h30. Elle utilise massivement les résumés IA.

**Fin du trial** : Elle demande à son manager si l'équipe peut payer l'abonnement. 5€/mois pour gagner 6h/semaine ? Validé immédiatement.

**Question de Marie** : "Est-ce qu'on pourra partager des flux en équipe ?" Elle voit dans la roadmap que c'est prévu V2. Elle s'inscrit pour être notifiée.

**Émotion** : Soulagement. "Enfin un outil qui me fait gagner du temps."

---

**Requirements révélés par ce journey** :

| Requirement | Capability |
|-------------|------------|
| FR-FEED-01 | Ajout manuel de flux par URL |
| FR-FEED-02 | Organisation en dossiers avec noms personnalisés |
| FR-AI-02 | Validation de clé API avec test réel |
| FR-AI-03 | Résumé IA d'article avec format structuré |
| FR-AI-04 | Copie du résumé en un clic |
| FR-RULE-AI-05 | Règles multi-conditions (acquisitions OU fusions) |
| NFR-UX-01 | Onboarding sans OPML possible |
| FR-NOTIF-01 (V2) | Notification de nouvelles features |
| FR-TEAM-01 (V2) | Flux partagés en équipe |

---

### Journey 3 : Alex Self-Host FeedMind - Le Dev Indie Autonome

**Contexte** : Alex, développeur freelance, veut essayer FeedMind mais préfère self-host. Il découvre que le projet est open source.

---

**Acte 1 : La Découverte Open Source**

Alex voit un post sur Mastodon : "Nouveau lecteur RSS open source avec filtrage IA - AGPL-3.0". Il clique.

Le README GitHub l'accueille :
```
# FeedMind.ai
La veille qui nourrit ton IA | Open Source RSS Reader with AI-powered filtering

## Quick Start (Docker)
docker-compose up -d

## Features
✅ Natural language filtering rules
✅ BYOK - Use your own AI API key  
✅ Self-hostable
✅ Multi-platform (Web, iOS, Android)
```

Il clone le repo, lit le `docker-compose.yml`. Simple : PostgreSQL, Redis, API Rust, Worker Rust, App Node.

---

**Acte 2 : L'Installation**

Alex tape :
```bash
git clone https://github.com/feedmind/feedmind.git
cd feedmind
cp .env.example .env
# Edit .env with his PostgreSQL and Redis config
docker-compose up -d
```

5 minutes plus tard, il accède à `http://localhost:3000`. L'app tourne.

Il crée un compte, importe son OPML de Miniflux (412 flux). Tout fonctionne.

Pour l'IA, il entre sa clé OpenAI (il n'a pas de clé Anthropic). L'app dit : "OpenAI sera supporté en V2. Actuellement : Anthropic, Google Gemini."

Déception mineure. Il crée un compte Anthropic et obtient une clé.

---

**Acte 3 : La Contribution**

Après une semaine d'utilisation, Alex remarque un bug : les feeds Atom 0.3 ne parsent pas correctement les dates.

Il ouvre une issue, puis regarde le code. C'est du Rust propre, bien structuré. Il trouve le bug dans `crates/core/src/feeds/parser.rs` ligne 234.

Il fork, fix, ouvre une PR. Review en 24h, mergé en 48h. Son nom apparaît dans le CHANGELOG.

**Émotion** : Appartenance. "C'est aussi mon projet maintenant."

---

**Acte 4 : L'Évangélisation**

Alex écrit un article de blog : "Why I switched from Miniflux to FeedMind". Il explique :
- Le filtrage IA est un game-changer
- Le self-host fonctionne parfaitement
- La codebase Rust est propre
- La communauté est réactive

L'article fait 2000 vues. 15 stars GitHub en une semaine.

**Émotion** : Fierté. "J'ai trouvé mon outil et j'y contribue."

---

**Requirements révélés par ce journey** :

| Requirement | Capability |
|-------------|------------|
| NFR-DEPLOY-01 | Docker Compose simple et documenté |
| NFR-DEPLOY-02 | Variables d'environnement bien documentées |
| FR-AUTH-SELF-01 | Création de compte sans vérification externe |
| NFR-CODE-01 | Codebase bien structurée et documentée |
| NFR-COMMUNITY-01 | Process de contribution clair (CONTRIBUTING.md) |
| NFR-COMMUNITY-02 | Review de PR en <48h |
| FR-COMPAT-01 | Support de tous les formats RSS/Atom |

---

### Journey 4 : Sophie Sécurise sa Veille CVE - L'Ops Privacy-First

**Contexte** : Sophie, SRE, a besoin d'une veille sécurité efficace et respectueuse de sa vie privée.

---

**Acte 1 : L'Évaluation Sécurité**

Sophie découvre FeedMind. Avant même de créer un compte, elle vérifie :

**Checklist sécurité de Sophie** :
- [x] Open source ? → Oui, AGPL-3.0. Elle peut auditer le code.
- [x] Self-host possible ? → Oui, Docker.
- [x] Tracking analytics ? → Elle cherche dans le code. Aucun Google Analytics, aucun Mixpanel. ✓
- [x] Privacy policy ? → "Zéro tracking. Vos données vous appartiennent." ✓
- [x] BYOK ? → Oui, les clés IA ne transitent jamais par FeedMind. ✓
- [x] Export données ? → OPML + JSON complet. ✓

Elle décide de tester la version cloud d'abord, avec l'idée de migrer en self-host si ça lui convient.

---

**Acte 2 : La Configuration Veille Sécu**

Sophie importe ses flux sécurité :
- NVD (National Vulnerability Database)
- CERT-FR
- Plusieurs blogs sécu (Krebs, Schneier, etc.)
- Release notes de ses vendors (Kubernetes, Docker, etc.)

Elle crée des règles IA ciblées :

**Règle 1** : "Alerter sur les CVE avec score CVSS ≥ 7.0"
```
Preview : 15 articles/semaine
Exemple : "CVE-2024-1234: Critical RCE in OpenSSL (CVSS 9.8)"
→ Score CVSS détecté : 9.8 ≥ 7.0 ✓
```

**Règle 2** : "Masquer les articles généraux sur la cybersécurité, garder uniquement les vulnérabilités techniques"
```
Preview : 80% des articles masqués
Conservés : CVE, patches, advisories
Masqués : Articles d'opinion, tendances marché
```

**Règle 3** : "Alerter sur les releases de Kubernetes, Docker, PostgreSQL"
```
Preview : 5 articles/semaine
→ Notifications sur les versions importantes
```

---

**Acte 3 : La Confiance Privacy**

Sophie utilise FeedMind pendant un mois. Elle vérifie régulièrement :

**Test 1 : Trafic réseau**
Elle route FeedMind via Wireshark. Aucun appel vers des trackers. Seuls appels : API FeedMind, Anthropic (pour l'IA).

**Test 2 : Images**
Elle remarque que les images sont proxiées via `img.feedmind.ai`. Elle vérifie les headers - pas de referrer vers les sources. ✓

**Test 3 : Export**
Elle teste l'export complet. JSON avec tous ses articles, tags, règles. Elle peut tout récupérer.

**Émotion** : Confiance. "Ils font ce qu'ils disent."

---

**Acte 4 : L'Intégration Workflow**

Sophie intègre FeedMind dans son workflow :

**Routine quotidienne** :
1. Matin : Ouvre FeedMind, vue "Non-lus" filtrée par règle "CVE critiques"
2. 5-10 articles à vérifier en priorité
3. Pour chaque CVE pertinent : tag "à traiter" + note sur les systèmes impactés
4. Export des articles tagués vers son système de ticketing (via export JSON pour l'instant, API en V2)

**Gain de temps** : De 45min de tri à 15min ciblé.

**Wish-list Sophie** (pour V2) :
- Webhook quand une règle match (pour alertes Slack)
- Intégration API avec Jira/ServiceNow
- Filtrage par vendor spécifique dans les CVE

**Émotion** : Efficacité. "Je ne rate plus les CVE critiques."

---

**Requirements révélés par ce journey** :

| Requirement | Capability |
|-------------|------------|
| NFR-PRIV-01 | Zéro tracking analytics tiers |
| NFR-PRIV-02 | Images proxiées sans referrer |
| NFR-PRIV-03 | Clés API jamais loggées ni stockées en clair |
| FR-EXPORT-01 | Export OPML complet |
| FR-EXPORT-02 | Export JSON (articles, tags, règles) |
| FR-TAG-01 | Système de tags pour organisation |
| FR-RULE-AI-06 | Règles basées sur des critères numériques (CVSS) |
| FR-WEBHOOK-01 (V2) | Webhooks sur match de règle |
| FR-API-01 (V2) | API pour intégration tierce |

---

### Journey 5 : Admin Système - Surveillance et Maintenance

**Contexte** : FeedMind est en production avec 5 utilisateurs. L'admin système doit surveiller et maintenir.

---

**Acte 1 : Monitoring Quotidien**

L'admin ouvre le dashboard `/admin`. Vue d'ensemble :

```
┌─────────────────────────────────────────────────┐
│ FeedMind Admin Dashboard                        │
├─────────────────────────────────────────────────┤
│ Status: ✅ Healthy                              │
│                                                 │
│ Users: 5 active                                 │
│ Feeds: 4,234 total                              │
│ Articles: 156,789 stored                        │
│                                                 │
│ Storage:                                        │
│ ├── PostgreSQL: 2.3 GB / 10 GB (23%)           │
│ ├── Redis: 156 MB / 1 GB (15%)                 │
│ └── S3 Images: 4.1 GB / 50 GB (8%)             │
│                                                 │
│ Last 24h:                                       │
│ ├── Feeds refreshed: 4,234                     │
│ ├── Articles fetched: 12,456                   │
│ ├── AI evaluations: 3,421                      │
│ └── Errors: 23 (0.5%)                          │
└─────────────────────────────────────────────────┘
```

---

**Acte 2 : Gestion des Erreurs**

L'admin clique sur "23 erreurs". Liste des feeds en erreur :

```
Feed Errors (last 24h)
─────────────────────────────────────────────────
1. blog.example.com/rss
   Error: Connection timeout (3 attempts)
   Last success: 2 hours ago
   Action: [Retry now] [Disable feed]

2. news.startup.io/feed.xml  
   Error: 404 Not Found
   Last success: 7 days ago
   Action: [Retry now] [Disable feed] [Notify user]

3. medium.com/@author/rss
   Error: 429 Too Many Requests
   Last success: 1 hour ago
   Action: [Retry now] [Reduce frequency]
```

L'admin voit que `news.startup.io` retourne 404 depuis 7 jours. Il clique "Notify user" pour prévenir le propriétaire du feed.

Pour Medium, il réduit la fréquence de polling pour ce domaine.

---

**Acte 3 : Alertes Proactives**

L'admin a configuré des alertes :

**Alerte reçue par email** :
```
⚠️ FeedMind Alert: Storage threshold reached

PostgreSQL storage at 82% (8.2 GB / 10 GB)

Recommendation: 
- Archive old articles (>90 days, read, not favorited)
- Consider upgrading storage
- Run VACUUM FULL on large tables

[View dashboard] [Run cleanup job]
```

L'admin clique "Run cleanup job". Les articles de plus de 90 jours, lus et non-favoris, sont archivés (métadonnées conservées, contenu supprimé).

Après cleanup : 5.1 GB / 10 GB (51%)

---

**Acte 4 : Métriques Performance**

L'admin vérifie les métriques Prometheus/Grafana :

```
API Performance (last 24h)
─────────────────────────────────────────────────
Endpoint             p50      p95      p99
GET /articles       45ms     120ms    180ms    ✅
GET /feeds          32ms      78ms    130ms    ✅
POST /rules/ai      890ms   2.1s     3.4s     ⚠️
GET /hidden         67ms     145ms    220ms    ✅

Worker Performance
─────────────────────────────────────────────────
Job                  Avg      Max      Queue
feed_refresh        1.2s     8.3s     0 pending  ✅
ai_evaluate         2.3s     12.1s    5 pending  ✅
image_proxy         0.3s     1.2s     0 pending  ✅
```

Le `POST /rules/ai` est lent (3.4s au p99) à cause des appels IA. Normal et attendu.

Le queue "ai_evaluate" a 5 jobs pending - acceptable. Si ça dépasse 100, il faudra scaler le worker.

---

**Requirements révélés par ce journey** :

| Requirement | Capability |
|-------------|------------|
| FR-ADMIN-01 | Dashboard avec métriques globales |
| FR-ADMIN-02 | Vue des feeds en erreur avec actions |
| FR-ADMIN-03 | Notification des utilisateurs sur erreurs persistantes |
| FR-ADMIN-04 | Configuration des alertes (email/Slack) |
| FR-ADMIN-05 | Job de cleanup/archivage |
| FR-ADMIN-06 | Métriques Prometheus exportées |
| NFR-OBS-01 | Logs structurés pour debugging |
| NFR-OBS-02 | Métriques de performance par endpoint |
| NFR-OBS-03 | Queue monitoring |

---

### Résumé des Journeys - Requirements Mapping

| Journey | Persona | Theme | Key Requirements |
|---------|---------|-------|------------------|
| **J1** | Thomas (CTO) | Migration power user | Import OPML, règles IA, preview, explicabilité |
| **J2** | Marie (Analyste) | Centralisation veille | Dossiers, résumés IA, config API key |
| **J3** | Alex (Dev Indie) | Self-host | Docker, contribution OSS, formats RSS |
| **J4** | Sophie (SRE) | Sécurité/Privacy | Privacy, export, CVE filtering |
| **J5** | Admin | Maintenance | Dashboard, alertes, métriques |

---

## Domain-Specific Requirements

### RSS/Atom : L'Héritage du Web Ouvert

#### Contexte Historique

RSS (Really Simple Syndication) et Atom sont des formats de syndication web créés au début des années 2000. Malgré leur âge, ils restent la méthode la plus fiable et universelle pour suivre du contenu web.

**Pourquoi RSS persiste** :
- Standard ouvert, aucun vendor lock-in
- Fonctionne même quand les APIs sont fermées (Twitter, Reddit)
- Respectueux de la vie privée (pas de tracking)
- Découplage total entre producteur et consommateur de contenu

#### Formats Supportés

| Format | Versions | Particularités | Prévalence |
|--------|----------|----------------|------------|
| **RSS 2.0** | 2.0 | Le plus répandu, bien standardisé | 70% |
| **RSS 1.0** | 1.0 | Basé sur RDF, plus complexe | 5% |
| **RSS 0.9x** | 0.91, 0.92 | Legacy, rarement utilisé | 2% |
| **Atom** | 0.3, 1.0 | Plus strict, meilleur typage dates | 20% |
| **JSON Feed** | 1.0, 1.1 | Moderne, facile à parser | 3% |

**Décision** : Supporter tous les formats. La bibliothèque `feed-rs` en Rust gère cette complexité.

#### Challenges Techniques RSS

##### 1. Dates Malformées

**Problème** : Les dates RSS sont souvent invalides ou dans des formats non-standard.

```
<!-- RSS 2.0 spec: RFC 822 -->
<pubDate>Mon, 06 Jan 2025 12:00:00 GMT</pubDate>

<!-- Ce qu'on trouve en réalité -->
<pubDate>2025-01-06</pubDate>
<pubDate>Jan 6, 2025</pubDate>
<pubDate>6/1/2025 12:00</pubDate>
<pubDate>yesterday</pubDate>
```

**Solution** : Parser flexible avec fallbacks :
1. Essayer RFC 822
2. Essayer ISO 8601
3. Essayer formats courants (regex patterns)
4. Fallback : date de fetch

##### 2. Encodage Caractères

**Problème** : Mélange d'encodages (UTF-8, ISO-8859-1, Windows-1252).

**Solution** :
1. Détecter l'encodage via BOM ou déclaration XML
2. Si absent, essayer UTF-8
3. Fallback : détection heuristique avec `chardetng`

##### 3. Contenu HTML dans CDATA

**Problème** : Le contenu est souvent du HTML brut dans des CDATA, parfois mal échappé.

```xml
<description><![CDATA[
<p>Article avec <a href="lien">link</a> et <script>alert('xss')</script></p>
]]></description>
```

**Solution** :
1. Parser le HTML avec `scraper`
2. Sanitizer (whitelist de tags autorisés)
3. Extraire le texte pour l'indexation

##### 4. Feeds Invalides

**Problème** : Beaucoup de feeds ne respectent pas les specs.

```xml
<!-- Manque le namespace -->
<rss>
  <channel>
    <!-- Champs obligatoires manquants -->
    <item>
      <title>Un article</title>
      <!-- Pas de link, pas de guid -->
    </item>
  </channel>
</rss>
```

**Solution** : Parsing permissif qui accepte le maximum :
- Générer des GUIDs à partir de title+date si absent
- Accepter les feeds sans namespace
- Log des warnings pour debugging

##### 5. Déduplication

**Problème** : Même article peut apparaître plusieurs fois (GUID change, légères modifications).

**Solution** :
1. Dédup par GUID exact
2. Si GUID absent : hash(title + link)
3. Dédup fuzzy sur titre (distance de Levenshtein <3)

#### OPML : Le Format d'Export

**OPML** (Outline Processor Markup Language) est le standard pour exporter/importer des listes de flux RSS.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<opml version="2.0">
  <head>
    <title>My RSS Feeds</title>
    <dateCreated>Mon, 06 Jan 2025 12:00:00 GMT</dateCreated>
  </head>
  <body>
    <outline text="Tech" title="Tech">
      <outline type="rss" text="Hacker News" 
               xmlUrl="https://news.ycombinator.com/rss" 
               htmlUrl="https://news.ycombinator.com"/>
      <outline type="rss" text="TechCrunch" 
               xmlUrl="https://techcrunch.com/feed/"/>
    </outline>
    <outline text="News">
      <!-- nested folders -->
    </outline>
  </body>
</opml>
```

**Challenges OPML** :
- Encodage (UTF-8 vs ISO-8859-1)
- Profondeur de nesting variable
- Attributs inconsistants (`xmlUrl` vs `xmlurl` vs `url`)

**Décision** : Parser permissif, normaliser à l'import.

### IA Générative : Intégration Responsable

#### Modèle BYOK

**Principe** : FeedMind ne possède pas les clés API IA. L'utilisateur fournit sa propre clé.

**Avantages** :
| Pour l'utilisateur | Pour FeedMind |
|--------------------|---------------|
| Contrôle des coûts | Pas de risque financier |
| Choix du provider | Pas de marge à gérer |
| Confidentialité des données | Simplicité opérationnelle |
| Pas de limites FeedMind | Focus sur le produit |

**Inconvénients** :
| Pour l'utilisateur | Pour FeedMind |
|--------------------|---------------|
| Doit créer un compte API | Pas de revenue sur l'IA |
| Doit monitorer sa conso | UX onboarding plus complexe |

#### Providers Supportés (V1)

| Provider | Modèle | Use case | Coût estimé* |
|----------|--------|----------|--------------|
| **Anthropic** | Claude 3.5 Sonnet | Règles IA, résumés | ~$0.003/article |
| **Anthropic** | Claude 3 Haiku | Résumés rapides | ~$0.0002/article |
| **Google** | Gemini 1.5 Pro | Alternative, long context | ~$0.002/article |
| **Google** | Gemini 1.5 Flash | Alternative économique | ~$0.0001/article |

*Coût estimé pour ~500 tokens/article

#### Architecture des Prompts

**Règle IA - Évaluation d'article** :

```
System: Tu es un assistant de filtrage RSS. Ton rôle est d'évaluer si un article 
correspond à une règle définie par l'utilisateur. Réponds UNIQUEMENT en JSON.

User: 
Règle: "Masquer les articles clickbait"

Article:
Titre: "You won't BELIEVE what happened next!"
Contenu: [premiers 500 caractères]
Source: buzzfeed.com

Évalue si cet article doit être masqué selon la règle.
Réponds en JSON:
{
  "match": true/false,
  "confidence": 0.0-1.0,
  "reason": "Explication courte en français"
}
```

**Réponse attendue** :
```json
{
  "match": true,
  "confidence": 0.95,
  "reason": "Titre sensationnaliste avec majuscules excessives et formulation clickbait typique"
}
```

#### Optimisations Coûts IA

| Technique | Économie | Implémentation |
|-----------|----------|----------------|
| **Batching** | -40% | Évaluer 10 articles par requête |
| **Caching** | -30% | Cache LRU sur hash(titre+règle) |
| **Modèle adaptatif** | -50% | Haiku pour règles simples, Sonnet pour complexes |
| **Early exit** | -20% | Si regex match, skip IA |
| **Truncation** | -25% | Max 500 tokens de contenu |

**Coût estimé après optimisations** : ~$3-5/mois pour un power user (1000 flux, 100 règles).

#### Sécurité des Clés API

| Mesure | Implémentation |
|--------|----------------|
| **Stockage** | Chiffré AES-256-GCM, clé dérivée de master key |
| **Transmission** | HTTPS uniquement, jamais en logs |
| **Affichage** | Masqué en UI (`sk-...xxxx`) |
| **Validation** | Test API à la configuration |
| **Révocation** | Utilisateur peut supprimer à tout moment |

### Extraction de Contenu (Readability)

#### Le Problème du Web Moderne

Les pages web sont polluées par :
- Pubs et trackers (50% du poids de page)
- Menus de navigation
- Sidebars et widgets
- Paywalls et modals
- Contenu dynamique (JS)

Les flux RSS ne contiennent souvent qu'un extrait.

#### Solution : Readability

**Readability** est un algorithme (initialement de Mozilla) qui extrait le contenu principal d'une page HTML.

**Pipeline d'extraction** :
```
Article RSS (extrait) 
    → Fetch page complète
    → Parse HTML (scraper)
    → Readability extraction
    → Sanitize HTML
    → Store clean content
```

**Implémentation Rust** : `readability-rs` ou port custom de Mozilla Readability.

**Fallbacks** :
1. Si JS required : Playwright headless (coûteux, batch)
2. Si paywall : Garder l'extrait RSS
3. Si timeout : Garder l'extrait RSS

### Privacy et Images

#### Problème : Le Tracking par Images

Quand un navigateur charge une image :
```
GET https://tracker.example.com/pixel.gif?user=123&article=456
Referer: https://feedmind.ai/article/789
User-Agent: Mozilla/5.0...
```

L'éditeur du flux sait :
- Qui lit (IP, fingerprint)
- Quand
- Quel article
- Via quel outil

#### Solution : Proxy Images

FeedMind proxie toutes les images :

```
Original: https://blog.example.com/images/photo.jpg?tracking=abc
Proxied:  https://img.feedmind.ai/v1/hash123456
```

**Pipeline proxy** :
1. URL originale → hash SHA256
2. Fetch image côté serveur (pas de Referer)
3. Strip métadonnées EXIF
4. Strip query params tracking
5. Cache (Redis/S3 selon tier)
6. Serve via CDN

**Protection** :
- [x] Pas de Referer vers la source
- [x] IP utilisateur jamais exposée
- [x] Métadonnées EXIF supprimées
- [x] Tracking pixels (1x1) bloqués
- [x] Query params tracking supprimés

---

## Innovation & Novel Patterns

### Pattern 1 : Règles IA en Langage Naturel

#### Le Problème Résolu

**Avant FeedMind** : Pour filtrer "les articles sur la crypto sauf Bitcoin", l'utilisateur devait écrire :
```regex
^(?!.*\b(bitcoin|btc)\b).*\b(crypto|cryptocurrency|ethereum|eth|solana|dogecoin|...)\b.*$
```

Problèmes :
- Syntaxe complexe
- Maintenance pénible (ajouter chaque nouvelle crypto)
- Pas de compréhension sémantique ("Elon parle de son chien" ≠ article crypto)

**Avec FeedMind** :
```
"Masquer les articles sur la crypto sauf Bitcoin"
```

#### Comment ça Marche

```
┌─────────────────────────────────────────────────────────────────┐
│                      RÈGLE IA PIPELINE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. USER INPUT                                                  │
│     "Masquer les articles crypto sauf Bitcoin"                  │
│                           │                                     │
│                           ▼                                     │
│  2. RULE STORAGE                                                │
│     ┌─────────────────────────────────────────┐                │
│     │ id: rule_123                            │                │
│     │ type: ai                                │                │
│     │ prompt: "Masquer les articles..."       │                │
│     │ action: hide                            │                │
│     │ scope: global                           │                │
│     │ active: true                            │                │
│     └─────────────────────────────────────────┘                │
│                           │                                     │
│                           ▼                                     │
│  3. NEW ARTICLE ARRIVES                                         │
│     ┌─────────────────────────────────────────┐                │
│     │ title: "Dogecoin pumps 30%"             │                │
│     │ content: "Elon Musk tweeted..."         │                │
│     │ source: coindesk.com                    │                │
│     └─────────────────────────────────────────┘                │
│                           │                                     │
│                           ▼                                     │
│  4. CHECK CACHE                                                 │
│     hash(title + rule_id) → cache miss                         │
│                           │                                     │
│                           ▼                                     │
│  5. AI EVALUATION (batched)                                     │
│     ┌─────────────────────────────────────────┐                │
│     │ Request to Claude API:                  │                │
│     │ - Rule prompt                           │                │
│     │ - Article (title + truncated content)   │                │
│     │ - Output format: JSON                   │                │
│     └─────────────────────────────────────────┘                │
│                           │                                     │
│                           ▼                                     │
│  6. AI RESPONSE                                                 │
│     ┌─────────────────────────────────────────┐                │
│     │ {                                       │                │
│     │   "match": true,                        │                │
│     │   "confidence": 0.92,                   │                │
│     │   "reason": "Article sur Dogecoin,      │                │
│     │              ne mentionne pas Bitcoin"  │                │
│     │ }                                       │                │
│     └─────────────────────────────────────────┘                │
│                           │                                     │
│                           ▼                                     │
│  7. APPLY ACTION + STORE                                        │
│     - Article marked as hidden                                  │
│     - Reason stored for explicability                          │
│     - Result cached for similar articles                       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Validation de l'Innovation

| Question | Réponse |
|----------|---------|
| **Est-ce nouveau ?** | Oui. Aucun lecteur RSS n'utilise l'IA pour le filtrage sémantique. |
| **Est-ce utile ?** | Oui. Résout un vrai problème (regex trop complexes). |
| **Est-ce faisable ?** | Oui. APIs LLM disponibles, coût acceptable avec BYOK. |
| **Barrière à l'entrée ?** | Moyenne. Nécessite expertise prompts + infra. |

#### Risques et Mitigations

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| **Hallucinations IA** | Moyen | Moyen | Confidence threshold (>0.7), option "demander confirmation" |
| **Coût IA explosif** | Moyen | Haut | BYOK, batching, caching, limites par user |
| **Latence évaluations** | Moyen | Moyen | Async processing, pas bloquant pour lecture |
| **API provider down** | Faible | Moyen | Fallback règles regex, multi-provider |
| **Prompts malicieux** | Faible | Faible | Sanitization input, rate limiting |

---

### Pattern 2 : Explicabilité Systématique

#### Le Problème Résolu

Les systèmes IA traditionnels sont des "boîtes noires". L'utilisateur voit le résultat mais pas le raisonnement.

**Frustration type** : "Pourquoi cet article a été masqué ? Je ne comprends pas."

#### Solution : Chaque Décision est Justifiée

```
┌─────────────────────────────────────────────────────────────────┐
│  ARTICLE MASQUÉ                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  📰 "New Solana NFT marketplace launches with $50M backing"     │
│      ├── Source: techcrunch.com                                │
│      └── Date: 27 Jan 2026                                     │
│                                                                 │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  🤖 POURQUOI MASQUÉ ?                                           │
│                                                                 │
│  Règle : "Masquer les articles crypto sauf Bitcoin/Ethereum"    │
│                                                                 │
│  Analyse IA :                                                   │
│  • Crypto détectée : Solana (blockchain layer 1)                │
│  • NFT mentionné (souvent associé crypto)                       │
│  • Bitcoin/Ethereum : Non mentionnés                            │
│  • Confidence : 94%                                             │
│                                                                 │
│  Conclusion : Article crypto hors exceptions → masqué           │
│                                                                 │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  [🔄 Restaurer cet article]  [✓ Masquage correct]               │
│                                                                 │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  💡 Cette décision était-elle incorrecte ?                      │
│     Votre feedback améliore la précision des futures évaluat.  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Implémentation

| Composant | Détail |
|-----------|--------|
| **Prompt design** | Demande explicitement la raison dans le JSON de sortie |
| **Stockage** | Raison stockée avec l'article dans `hidden_articles` |
| **UI** | Affichage de la raison dans la vue "Masqués" |
| **Feedback loop** | Boutons "Correct" / "Incorrect" pour améliorer (V3) |

#### Valeur Utilisateur

| Bénéfice | Description |
|----------|-------------|
| **Confiance** | L'utilisateur comprend les décisions |
| **Debugging** | Peut identifier les règles mal formulées |
| **Apprentissage** | Découvre ce que l'IA "voit" dans les articles |
| **Contrôle** | Peut corriger les erreurs facilement |

---

### Pattern 3 : BYOK (Bring Your Own Key)

#### Le Problème Résolu

**Modèle classique SaaS IA** :
- Le SaaS paie les tokens → facture l'utilisateur avec marge
- Utilisateur paie un forfait → peur de dépasser, limites frustrantes
- Coûts imprévisibles pour le SaaS → pricing complexe

**Problèmes** :
- Lock-in sur le provider choisi par le SaaS
- Pas de contrôle sur les coûts réels
- Questions de confidentialité (le SaaS voit tout)

#### Solution BYOK

```
┌─────────────────────────────────────────────────────────────────┐
│                     ARCHITECTURE BYOK                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  UTILISATEUR                                                    │
│     │                                                          │
│     │ 1. Configure sa clé API                                  │
│     │    (Anthropic, Google, etc.)                             │
│     │                                                          │
│     ▼                                                          │
│  FEEDMIND                                                       │
│     │                                                          │
│     │ 2. Stocke la clé chiffrée                                │
│     │    (AES-256, jamais en clair)                            │
│     │                                                          │
│     │ 3. Quand IA nécessaire :                                 │
│     │    - Déchiffre la clé                                    │
│     │    - Appelle le provider directement                     │
│     │    - Ne log jamais les prompts/réponses                  │
│     │                                                          │
│     ▼                                                          │
│  PROVIDER IA (Anthropic, Google)                               │
│     │                                                          │
│     │ 4. Facture directement l'utilisateur                     │
│     │    (via son compte provider)                             │
│     │                                                          │
│     ▼                                                          │
│  RÉSULTAT                                                       │
│     - FeedMind : 0€ de coût IA                                 │
│     - Utilisateur : paie uniquement ce qu'il consomme          │
│     - Confidentialité : données user → provider directement    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Avantages Compétitifs

| Pour FeedMind | Pour l'Utilisateur |
|---------------|-------------------|
| Pas de risque financier IA | Contrôle total des coûts |
| Pricing simple et prévisible | Choix du provider |
| Pas de marge à gérer | Pas de limites artificielles |
| Focus sur le produit | Confidentialité renforcée |

#### Option Managed (V2+)

Pour les utilisateurs qui ne veulent pas gérer leurs clés :

```
FeedMind Managed IA (+10%)
─────────────────────────────
• Pas besoin de créer un compte Anthropic/Google
• FeedMind paie les tokens et refacture +10%
• Facturation au token consommé
• Dashboard de suivi de consommation
```

---

### Pattern 4 : Smart Polling Adaptatif

#### Le Problème Résolu

**Polling fixe** (toutes les heures) :
- Blog inactif depuis 6 mois → 8760 requêtes/an pour rien
- Site d'actualité temps réel → 60 minutes de retard possible

**Gaspillage** + **Mauvaise UX**

#### Solution : Adapter au Comportement du Flux

```
┌─────────────────────────────────────────────────────────────────┐
│                   SMART POLLING ALGORITHM                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  INPUT: Historique de publication du flux                       │
│                                                                 │
│  CALCUL DU SCORE D'ACTIVITÉ :                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ activity_score = (posts_7d × 2) + posts_30d             │   │
│  │                                                          │   │
│  │ Exemples :                                               │   │
│  │ • Blog mort (0 posts/mois) : score = 0                   │   │
│  │ • Blog occasionnel (2 posts/mois) : score = 2           │   │
│  │ • News site (5 posts/jour) : score = 70 + 150 = 220     │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  MAPPING SCORE → INTERVALLE :                                   │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ Score      │ Intervalle  │ Exemple                     │    │
│  ├────────────┼─────────────┼─────────────────────────────┤    │
│  │ 0          │ 24h         │ Blog abandonné              │    │
│  │ 1-5        │ 6h          │ Blog perso mensuel          │    │
│  │ 6-20       │ 1h          │ Blog tech hebdo             │    │
│  │ 21-50      │ 30min       │ Site d'actu                 │    │
│  │ 51+        │ 15min       │ News temps réel             │    │
│  └────────────┴─────────────┴─────────────────────────────┘    │
│                                                                 │
│  DÉTECTION DE CHANGEMENT :                                      │
│  Si un flux "dormant" publie soudainement :                    │
│  → Reclasser temporairement en "actif" (7 jours)               │
│  → Retour progressif à la normale si pas de suite              │
│                                                                 │
│  OVERRIDE UTILISATEUR :                                         │
│  L'utilisateur peut forcer un intervalle spécifique            │
│  (limité par tier : Free=3, Pro=20)                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Économie de Ressources

| Scénario | Polling fixe 1h | Smart Polling | Économie |
|----------|-----------------|---------------|----------|
| 100 flux dormants | 2400 req/jour | 100 req/jour | **96%** |
| 100 flux actifs | 2400 req/jour | 4800 req/jour | -100% (mais justifié) |
| Mix réaliste (80/20) | 2400 req/jour | 1080 req/jour | **55%** |

---

### Pattern 5 : No Vendor Lock-in

#### Philosophie

> "Vos données vous appartiennent. Vous devez pouvoir partir à tout moment."

#### Implémentation Concrète

| Fonctionnalité | Détail |
|----------------|--------|
| **Export OPML** | Tous les flux + structure dossiers, en 1 clic |
| **Export JSON** | Articles, tags, règles, tout l'historique |
| **Import OPML** | Depuis n'importe quel lecteur RSS |
| **Open Source** | Code disponible, self-host possible |
| **API (V2)** | Extraction programmatique de toutes les données |
| **Standards ouverts** | RSS/Atom/JSON Feed, pas de format propriétaire |

#### Format d'Export JSON

```json
{
  "export_version": "1.0",
  "exported_at": "2026-01-27T12:00:00Z",
  "user": {
    "email": "user@example.com"
  },
  "feeds": [
    {
      "url": "https://example.com/rss",
      "title": "Example Blog",
      "folder": "Tech/Blogs",
      "added_at": "2025-06-15T10:00:00Z",
      "settings": {
        "refresh_override": null,
        "rules": ["rule_123"]
      }
    }
  ],
  "articles": [
    {
      "id": "article_456",
      "feed_url": "https://example.com/rss",
      "title": "Article Title",
      "url": "https://example.com/post/123",
      "published_at": "2026-01-20T09:00:00Z",
      "read": true,
      "read_at": "2026-01-20T10:30:00Z",
      "favorited": true,
      "tags": ["important", "ai"],
      "content_hash": "sha256:abc123"
    }
  ],
  "rules": [
    {
      "id": "rule_123",
      "type": "ai",
      "prompt": "Masquer les articles clickbait",
      "action": "hide",
      "scope": "global",
      "created_at": "2025-12-01T14:00:00Z"
    }
  ],
  "tags": ["important", "ai", "to-read"]
}
```

---

## Scoping (MVP V1)

### Philosophie de Scope

**Principe** : Faire moins, mais excellemment.

V1 est un produit pour **5 power users** (l'équipe fondatrice). Pas de features marketing, pas de growth hacks, pas de social. Juste un outil de veille excellent.

### In Scope V1 (MUST HAVE)

#### Authentification & Compte

| Feature | Description | Priorité |
|---------|-------------|----------|
| Email/Password | Création de compte classique | P0 |
| OAuth Google | Login rapide | P0 |
| OAuth GitHub | Login pour devs | P1 |
| CB pour trial | Empreinte CB, pas de charge | P0 |
| Reset password | Email de récupération | P0 |
| Delete account | Suppression complète RGPD | P1 |

#### Gestion des Flux

| Feature | Description | Priorité |
|---------|-------------|----------|
| Import OPML | Jusqu'à 2000+ flux | P0 |
| Ajout manuel | Par URL | P0 |
| Dossiers | Organisation hiérarchique | P0 |
| Export OPML | Tous les flux | P0 |
| Vue erreurs | Flux en erreur avec détails | P1 |
| Force refresh | Actualisation manuelle | P1 |
| Override intervalle | Forcer un intervalle | P2 |

#### Gestion des Articles

| Feature | Description | Priorité |
|---------|-------------|----------|
| Liste articles | Tous, non-lus, favoris, par flux/dossier | P0 |
| Marquer lu/non-lu | Individuel | P0 |
| Favoris | Sauvegarder un article | P0 |
| Tags | Organisation par tags | P1 |
| Marquer tout lu | En batch | P0 |
| Marquer anciens lu | Avant une date | P1 |
| Contenu complet | Extraction Readability | P0 |
| Ouvrir original | Dans navigateur externe | P0 |
| Vues multiples | Liste, expanded, colonnes, cartes, magazine | P1 |

#### Règles de Filtrage

| Feature | Description | Priorité |
|---------|-------------|----------|
| Règles regex | Pattern matching classique | P0 |
| Règles IA | Langage naturel | P0 |
| Scope global/flux | Application ciblée | P0 |
| Actions | Hide, tag, favorite | P0 |
| Preview | Effet sur 7 derniers jours | P0 |
| Explicabilité | Raison affichée pour chaque match | P0 |
| CRUD règles | Créer, modifier, supprimer | P0 |
| Activer/désactiver | Sans supprimer | P1 |

#### Vue Masqués

| Feature | Description | Priorité |
|---------|-------------|----------|
| Liste masqués | Tous les articles cachés | P0 |
| Filtrer par règle | Voir l'effet d'une règle | P1 |
| Voir raison | Explication du masquage | P0 |
| Restaurer | Annuler le masquage | P0 |
| Confirmer correct | Feedback positif | P1 |
| Vider | Supprimer tous les masqués | P2 |

#### IA (BYOK)

| Feature | Description | Priorité |
|---------|-------------|----------|
| Config Anthropic | Clé API Claude | P0 |
| Config Google | Clé API Gemini | P0 |
| Test validité | Vérification à la config | P0 |
| Résumé article | Résumé IA d'un article | P1 |
| Estimation coût | Afficher coût estimé | P2 |

#### Pipeline Images

| Feature | Description | Priorité |
|---------|-------------|----------|
| Proxy images | Servir via FeedMind | P0 |
| Cache | Selon tier (48h/30j) | P0 |
| Strip EXIF | Supprimer métadonnées | P0 |
| Strip tracking | Supprimer query params | P0 |
| Block pixels | Bloquer images 1x1 | P1 |

#### Synchronisation

| Feature | Description | Priorité |
|---------|-------------|----------|
| Sync read state | Temps réel entre devices | P0 |
| Sync favoris | Temps réel | P0 |
| Sync tags | Temps réel | P1 |
| Sync règles | Temps réel | P1 |

#### UX

| Feature | Description | Priorité |
|---------|-------------|----------|
| Web app | Desktop browser | P0 |
| iOS app | iPhone/iPad | P0 |
| Android app | Phones/tablets | P0 |
| Raccourcis clavier | Parité Inoreader | P0 |
| Gestures mobile | Swipe, long press, pull | P0 |
| Thème sombre | Light/dark/system | P0 |
| Vue par défaut | Configurable | P1 |

#### Admin (Basic)

| Feature | Description | Priorité |
|---------|-------------|----------|
| Métriques stockage | DB, cache, images | P1 |
| Vue erreurs | Feeds en erreur globaux | P1 |
| Alertes disk | Warning si >80% | P2 |

### Out of Scope V1 (PLANNED LATER)

#### V1.1 (Intégrations)

| Feature | Raison du report |
|---------|------------------|
| Recherche full-text | Complexe (index Postgres FTS ou Meilisearch) |
| YouTube + SponsorBlock | Intégration externe, pas critique pour veille RSS |
| API Fever | Pour clients tiers, pas urgent |
| Score pertinence IA | Nécessite feedback loop, V3 |
| Traduction IA | Nice-to-have |
| Migration Inoreader | Script d'import avancé |

#### V2 (Collaboration)

| Feature | Raison du report |
|---------|------------------|
| Flux partagés | Multi-user complexity |
| Règles d'équipe | Permissions à gérer |
| Organisations | Multi-tenant avancé |
| Webhooks | API event-driven |
| Export API | REST API complète |
| Newsletters | Ingestion email |
| Managed IA (+10%) | Business model à valider |

#### V3 (IA Avancée)

| Feature | Raison du report |
|---------|------------------|
| Feedback loop IA | Amélioration continue des prompts |
| Score pertinence personnalisé | Learning sur lectures |
| Player YouTube embed | Embed SponsorBlock natif |
| Offline mode | Cache local, sync différé |

### Limites par Tier (V1)

| Limite | Free (Trial 14j) | Pro (5€/mois) |
|--------|------------------|---------------|
| Durée | 14 jours | Illimitée |
| Flux RSS | 50 | Illimité |
| Articles stockés | 1000 | Illimité |
| Règles regex | 5 | Illimité |
| Règles IA | ❌ | ✅ (BYOK) |
| Résumé IA | ❌ | ✅ (BYOK) |
| Cache images | 48h | 30 jours |
| Override refresh | 3 flux | 20 flux |
| Export OPML | ✅ | ✅ |
| Export JSON | ❌ | ✅ |
| Support | Community | Email 48h |

---

## Functional Requirements

### Conventions

- **Actor** : Qui effectue l'action (User, System, Admin)
- **Capability** : Ce que l'acteur peut faire
- **Acceptance Criteria** : Conditions de validation

### FR-ONBOARD : Onboarding & Import

#### FR-ONBOARD-01 : Création de compte email/password

**Actor** : User  
**Capability** : L'utilisateur peut créer un compte avec email et mot de passe.

**Acceptance Criteria** :
- [ ] Email valide requis (format + unicité)
- [ ] Password min 8 caractères, 1 majuscule, 1 chiffre
- [ ] Email de confirmation envoyé
- [ ] Compte actif après clic lien confirmation
- [ ] Temps de création < 30s

---

#### FR-ONBOARD-02 : Connexion OAuth Google

**Actor** : User  
**Capability** : L'utilisateur peut se connecter via son compte Google.

**Acceptance Criteria** :
- [ ] Bouton "Continuer avec Google" visible
- [ ] Redirect vers consent Google
- [ ] Création automatique de compte si nouveau
- [ ] Récupération email et nom depuis Google
- [ ] Temps de connexion < 5s après consent

---

#### FR-ONBOARD-03 : Connexion OAuth GitHub

**Actor** : User  
**Capability** : L'utilisateur peut se connecter via son compte GitHub.

**Acceptance Criteria** :
- [ ] Bouton "Continuer avec GitHub" visible
- [ ] Redirect vers consent GitHub
- [ ] Création automatique de compte si nouveau
- [ ] Récupération email depuis GitHub
- [ ] Temps de connexion < 5s après consent

---

#### FR-ONBOARD-04 : Enregistrement CB pour trial

**Actor** : User  
**Capability** : L'utilisateur doit enregistrer une carte bancaire pour accéder au trial gratuit.

**Acceptance Criteria** :
- [ ] Formulaire Stripe Elements intégré
- [ ] Validation CB en temps réel
- [ ] Empreinte CB uniquement (0€ prélevé)
- [ ] Message clair "Aucun prélèvement pendant 14 jours"
- [ ] Accès immédiat après validation CB

---

#### FR-ONBOARD-05 : Import OPML

**Actor** : User  
**Capability** : L'utilisateur peut importer un fichier OPML contenant jusqu'à 2000+ flux.

**Acceptance Criteria** :
- [ ] Upload fichier .opml ou .xml
- [ ] Parsing des formats OPML 1.0 et 2.0
- [ ] Preview avant import (nombre de flux, dossiers)
- [ ] Gestion des encodages (UTF-8, ISO-8859-1)
- [ ] Création automatique des dossiers
- [ ] Progress bar pendant l'import
- [ ] Import 1000 flux en < 2 minutes
- [ ] Import 2000 flux en < 5 minutes
- [ ] Rapport de fin (succès, erreurs, doublons ignorés)
- [ ] Dédup automatique des flux déjà présents

---

#### FR-ONBOARD-06 : Ajout manuel de flux

**Actor** : User  
**Capability** : L'utilisateur peut ajouter un flux manuellement par URL.

**Acceptance Criteria** :
- [ ] Champ URL avec validation format
- [ ] Auto-découverte du feed (HTML page → RSS link)
- [ ] Preview du flux (titre, description, derniers articles)
- [ ] Choix du dossier de destination
- [ ] Détection des doublons
- [ ] Ajout en < 5 secondes

---

### FR-FEED : Gestion des Flux

#### FR-FEED-01 : Organisation en dossiers

**Actor** : User  
**Capability** : L'utilisateur peut organiser ses flux en dossiers hiérarchiques.

**Acceptance Criteria** :
- [ ] Créer un dossier (nom unique)
- [ ] Créer des sous-dossiers (max 3 niveaux)
- [ ] Renommer un dossier
- [ ] Supprimer un dossier (avec confirmation si non-vide)
- [ ] Drag & drop pour réorganiser
- [ ] Flux non-classés dans "Sans dossier"

---

#### FR-FEED-02 : Déplacement de flux

**Actor** : User  
**Capability** : L'utilisateur peut déplacer un flux entre dossiers.

**Acceptance Criteria** :
- [ ] Drag & drop depuis la sidebar
- [ ] Menu contextuel "Déplacer vers..."
- [ ] Sélection multiple possible
- [ ] Confirmation pour déplacement en batch (>10 flux)

---

#### FR-FEED-03 : Suppression de flux

**Actor** : User  
**Capability** : L'utilisateur peut supprimer un flux de sa liste.

**Acceptance Criteria** :
- [ ] Confirmation requise
- [ ] Articles associés marqués "orphelins" (conservés 30j puis supprimés)
- [ ] Possibilité de ré-ajouter le même flux plus tard

---

#### FR-FEED-04 : Export OPML

**Actor** : User  
**Capability** : L'utilisateur peut exporter tous ses flux en OPML.

**Acceptance Criteria** :
- [ ] Export de tous les flux actifs
- [ ] Structure des dossiers préservée
- [ ] Format OPML 2.0 valide
- [ ] Téléchargement fichier .opml
- [ ] Temps d'export < 10 secondes (1000 flux)

---

#### FR-FEED-05 : Vue des erreurs de flux

**Actor** : User  
**Capability** : L'utilisateur peut voir les flux en erreur et leurs détails.

**Acceptance Criteria** :
- [ ] Badge "Erreur" sur flux problématiques
- [ ] Vue dédiée "Flux en erreur"
- [ ] Détail de l'erreur (code HTTP, message, date)
- [ ] Nombre de tentatives échouées
- [ ] Date de dernière réussite
- [ ] Actions : Réessayer, Désactiver, Supprimer

---

#### FR-FEED-06 : Force refresh

**Actor** : User  
**Capability** : L'utilisateur peut forcer l'actualisation immédiate d'un flux.

**Acceptance Criteria** :
- [ ] Bouton "Actualiser" sur chaque flux
- [ ] Feedback visuel pendant le refresh
- [ ] Nouveaux articles apparaissent immédiatement
- [ ] Rate limit : 1 force refresh / flux / 5 minutes

---

#### FR-FEED-07 : Override intervalle de refresh

**Actor** : User (Pro)  
**Capability** : L'utilisateur Pro peut forcer un intervalle de refresh spécifique.

**Acceptance Criteria** :
- [ ] Options : 15min, 30min, 1h, 6h, 24h, "Auto" (défaut)
- [ ] Limite Free : 3 overrides
- [ ] Limite Pro : 20 overrides
- [ ] Avertissement si override plus fréquent que l'activité du flux

---

### FR-ARTICLE : Gestion des Articles

#### FR-ART-01 : Liste des articles

**Actor** : User  
**Capability** : L'utilisateur peut voir la liste des articles avec différents filtres.

**Acceptance Criteria** :
- [ ] Vue "Tous les articles"
- [ ] Vue "Non-lus" (défaut)
- [ ] Vue "Favoris"
- [ ] Vue par flux spécifique
- [ ] Vue par dossier (agrégé)
- [ ] Tri par date (récent first, défaut)
- [ ] Pagination infinie (lazy load)
- [ ] Temps de chargement initial < 500ms

---

#### FR-ART-02 : Marquer lu/non-lu

**Actor** : User  
**Capability** : L'utilisateur peut marquer un article comme lu ou non-lu.

**Acceptance Criteria** :
- [ ] Clic/tap sur article → marque lu automatiquement
- [ ] Raccourci `m` pour toggle lu/non-lu
- [ ] Swipe gauche (mobile) → marque lu
- [ ] Feedback visuel immédiat (style différent)
- [ ] Sync temps réel sur autres devices

---

#### FR-ART-03 : Favoris

**Actor** : User  
**Capability** : L'utilisateur peut ajouter/retirer un article de ses favoris.

**Acceptance Criteria** :
- [ ] Raccourci `s` ou `f` pour toggle favori
- [ ] Swipe droite (mobile) → toggle favori
- [ ] Icône étoile visible sur article favori
- [ ] Vue "Favoris" accessible depuis sidebar
- [ ] Sync temps réel

---

#### FR-ART-04 : Tags

**Actor** : User  
**Capability** : L'utilisateur peut ajouter des tags à un article.

**Acceptance Criteria** :
- [ ] Raccourci `t` pour ouvrir le sélecteur de tags
- [ ] Créer un nouveau tag à la volée
- [ ] Tags existants en autocomplétion
- [ ] Multiple tags par article
- [ ] Filtrer par tag dans la vue articles
- [ ] Couleur personnalisable par tag (P2)

---

#### FR-ART-05 : Marquer tout lu

**Actor** : User  
**Capability** : L'utilisateur peut marquer tous les articles visibles comme lus.

**Acceptance Criteria** :
- [ ] Raccourci `Shift+A`
- [ ] Confirmation si >50 articles
- [ ] Applicable sur vue actuelle (flux, dossier, tous)
- [ ] Feedback "X articles marqués lus"

---

#### FR-ART-06 : Marquer anciens comme lus

**Actor** : User  
**Capability** : L'utilisateur peut marquer tous les articles plus anciens qu'une date comme lus.

**Acceptance Criteria** :
- [ ] Raccourci `l` (mark older as read)
- [ ] Marque comme lus tous les articles avant l'article sélectionné
- [ ] Feedback "X articles marqués lus"

---

#### FR-ART-07 : Lecture contenu complet (Readability)

**Actor** : User  
**Capability** : L'utilisateur peut lire le contenu complet extrait de la page originale.

**Acceptance Criteria** :
- [ ] Bouton "Voir article complet"
- [ ] Extraction via Readability
- [ ] Affichage clean (texte + images)
- [ ] Fallback sur contenu RSS si extraction échoue
- [ ] Indicateur si contenu extrait ou RSS

---

#### FR-ART-08 : Ouvrir article original

**Actor** : User  
**Capability** : L'utilisateur peut ouvrir l'article original dans un navigateur externe.

**Acceptance Criteria** :
- [ ] Raccourci `v` ou `Enter`
- [ ] Ouvre dans nouvel onglet (web)
- [ ] Ouvre dans navigateur système (mobile)
- [ ] Marque automatiquement comme lu

---

#### FR-ART-09 : Navigation clavier

**Actor** : User  
**Capability** : L'utilisateur peut naviguer entre articles avec le clavier.

**Acceptance Criteria** :
- [ ] `j` / `↓` : article suivant
- [ ] `k` / `↑` : article précédent
- [ ] `n` : article non-lu suivant (sans ouvrir)
- [ ] `p` : article non-lu précédent (sans ouvrir)
- [ ] `Shift+J` : flux suivant
- [ ] `Shift+K` : flux précédent
- [ ] `o` ou `Enter` : ouvrir/fermer article
- [ ] `?` : aide raccourcis

---

#### FR-ART-10 : Vues multiples

**Actor** : User  
**Capability** : L'utilisateur peut changer la vue d'affichage des articles.

**Acceptance Criteria** :
- [ ] Vue Liste (compact, titre + source + date)
- [ ] Vue Expanded (titre + extrait + image)
- [ ] Vue Colonnes (3 colonnes type TweetDeck)
- [ ] Vue Cartes (grid de cards)
- [ ] Vue Magazine (featured + liste)
- [ ] Raccourcis `1` à `5` pour changer de vue
- [ ] Mémorisation de la préférence

---

### FR-RULE : Règles de Filtrage

#### FR-RULE-01 : Créer règle regex

**Actor** : User  
**Capability** : L'utilisateur peut créer une règle de filtrage basée sur regex.

**Acceptance Criteria** :
- [ ] Éditeur avec champ regex
- [ ] Validation syntaxe regex en temps réel
- [ ] Cible : titre, contenu, source, auteur
- [ ] Options : case sensitive, whole word
- [ ] Action : masquer, tagger, favoriser
- [ ] Scope : global ou flux spécifique
- [ ] Test sur un exemple d'article

---

#### FR-RULE-02 : Créer règle IA (langage naturel)

**Actor** : User (Pro + BYOK)  
**Capability** : L'utilisateur peut créer une règle en langage naturel.

**Acceptance Criteria** :
- [ ] Champ texte libre en français
- [ ] Exemples suggérés ("Masquer les articles clickbait")
- [ ] Validation que clé IA est configurée
- [ ] Action : masquer, tagger, favoriser
- [ ] Scope : global ou flux spécifique
- [ ] Indicateur de coût estimé (tokens)

---

#### FR-RULE-03 : Preview de règle

**Actor** : User  
**Capability** : L'utilisateur peut prévisualiser l'effet d'une règle avant de l'activer.

**Acceptance Criteria** :
- [ ] Preview sur les 7 derniers jours d'articles
- [ ] Liste des articles qui SERAIENT affectés
- [ ] Pour règles IA : raison pour chaque article
- [ ] Compteur "X articles seraient masqués"
- [ ] Possibilité d'ajuster la règle et re-prévisualiser
- [ ] Temps de preview < 30s (règle IA, 100 articles)

---

#### FR-RULE-04 : Explicabilité des règles IA

**Actor** : System  
**Capability** : Le système fournit une explication pour chaque article filtré par règle IA.

**Acceptance Criteria** :
- [ ] Chaque match IA stocké avec une raison
- [ ] Raison en français, lisible par un humain
- [ ] Affichée dans la vue "Masqués"
- [ ] Affichée dans le preview de règle
- [ ] Confidence score (0-100%) visible

---

#### FR-RULE-05 : Gestion des règles (CRUD)

**Actor** : User  
**Capability** : L'utilisateur peut créer, modifier, supprimer ses règles.

**Acceptance Criteria** :
- [ ] Liste de toutes les règles
- [ ] Éditer une règle existante (re-preview)
- [ ] Supprimer une règle (confirmation)
- [ ] Dupliquer une règle
- [ ] Réordonner les règles (priorité)

---

#### FR-RULE-06 : Activer/désactiver règle

**Actor** : User  
**Capability** : L'utilisateur peut activer ou désactiver une règle sans la supprimer.

**Acceptance Criteria** :
- [ ] Toggle on/off visible
- [ ] Règle désactivée = articles ne sont plus filtrés
- [ ] Articles déjà masqués restent masqués
- [ ] Indicateur visuel (grisé si désactivé)

---

### FR-HIDDEN : Vue des Articles Masqués

#### FR-HIDE-01 : Liste des masqués

**Actor** : User  
**Capability** : L'utilisateur peut voir tous les articles masqués par les règles.

**Acceptance Criteria** :
- [ ] Vue dédiée "Masqués" dans sidebar
- [ ] Liste tous les articles cachés
- [ ] Triés par date de masquage (récent first)
- [ ] Indicateur du nombre total

---

#### FR-HIDE-02 : Filtrer par règle

**Actor** : User  
**Capability** : L'utilisateur peut filtrer les masqués par règle appliquée.

**Acceptance Criteria** :
- [ ] Dropdown de sélection de règle
- [ ] Voir uniquement les articles masqués par cette règle
- [ ] Compteur par règle

---

#### FR-HIDE-03 : Voir raison du masquage

**Actor** : User  
**Capability** : L'utilisateur peut voir pourquoi chaque article a été masqué.

**Acceptance Criteria** :
- [ ] Nom de la règle qui a matché
- [ ] Pour règles IA : explication détaillée
- [ ] Confidence score (IA)
- [ ] Date de masquage

---

#### FR-HIDE-04 : Restaurer un article

**Actor** : User  
**Capability** : L'utilisateur peut restaurer un article masqué.

**Acceptance Criteria** :
- [ ] Bouton "Restaurer"
- [ ] Article redevient visible dans les vues normales
- [ ] Article marqué "exception" pour cette règle
- [ ] La règle ne re-masquera pas cet article

---

#### FR-HIDE-05 : Confirmer masquage correct

**Actor** : User  
**Capability** : L'utilisateur peut confirmer qu'un masquage était correct.

**Acceptance Criteria** :
- [ ] Bouton "Masquage correct ✓"
- [ ] Feedback stocké pour analytics
- [ ] Article reste masqué
- [ ] Utilisé pour améliorer les prompts (V3)

---

### FR-AI : Fonctionnalités IA (BYOK)

#### FR-AI-01 : Configuration clé Anthropic

**Actor** : User  
**Capability** : L'utilisateur peut configurer sa clé API Anthropic.

**Acceptance Criteria** :
- [ ] Champ pour clé API (format `sk-ant-...`)
- [ ] Test de validité en temps réel
- [ ] Affichage masqué (`sk-...xxxx`)
- [ ] Stockage chiffré (AES-256)
- [ ] Modèle détecté automatiquement
- [ ] Possibilité de supprimer/remplacer

---

#### FR-AI-02 : Configuration clé Google Gemini

**Actor** : User  
**Capability** : L'utilisateur peut configurer sa clé API Google AI.

**Acceptance Criteria** :
- [ ] Champ pour clé API
- [ ] Test de validité
- [ ] Stockage chiffré
- [ ] Choix du modèle (Pro/Flash)

---

#### FR-AI-03 : Résumé IA d'article

**Actor** : User (Pro + BYOK)  
**Capability** : L'utilisateur peut demander un résumé IA d'un article.

**Acceptance Criteria** :
- [ ] Bouton "Résumer" sur chaque article
- [ ] Résumé structuré (points clés)
- [ ] Affichage en < 10 secondes
- [ ] Résumé stocké (pas de re-génération)
- [ ] Copie en un clic
- [ ] Coût estimé affiché

---

#### FR-AI-04 : Estimation coût IA

**Actor** : User  
**Capability** : L'utilisateur peut voir une estimation de ses coûts IA.

**Acceptance Criteria** :
- [ ] Dashboard avec tokens consommés (30 derniers jours)
- [ ] Estimation coût en $ basée sur pricing provider
- [ ] Breakdown par type (règles, résumés)
- [ ] Alerte configurable si seuil dépassé

---

### FR-IMG : Pipeline Images

#### FR-IMG-01 : Proxy images

**Actor** : System  
**Capability** : Le système proxy toutes les images via son propre domaine.

**Acceptance Criteria** :
- [ ] URLs réécrites : `blog.com/img.jpg` → `img.feedmind.ai/hash`
- [ ] Pas de Referer vers la source
- [ ] Headers privacy (no-cache, etc.)
- [ ] Support JPEG, PNG, GIF, WebP
- [ ] Fallback image si source indisponible

---

#### FR-IMG-02 : Cache images

**Actor** : System  
**Capability** : Le système cache les images selon le tier de l'utilisateur.

**Acceptance Criteria** :
- [ ] Free : cache 48h (Redis)
- [ ] Pro : cache 30 jours (S3)
- [ ] Invalidation si image source change
- [ ] Métriques de cache hit/miss

---

#### FR-IMG-03 : Strip EXIF et tracking

**Actor** : System  
**Capability** : Le système supprime les métadonnées EXIF et paramètres de tracking.

**Acceptance Criteria** :
- [ ] EXIF supprimé (localisation, device, etc.)
- [ ] Query params tracking supprimés (`?utm_*`, `?ref=*`)
- [ ] Images 1x1 (tracking pixels) bloquées
- [ ] Logs des images bloquées (admin)

---

### FR-SYNC : Synchronisation

#### FR-SYNC-01 : Sync état de lecture temps réel

**Actor** : System  
**Capability** : L'état de lecture est synchronisé entre tous les devices en temps réel.

**Acceptance Criteria** :
- [ ] WebSocket connection maintenue
- [ ] Propagation < 2 secondes
- [ ] Gestion déconnexion/reconnexion
- [ ] Conflit résolu : "plus récent gagne"
- [ ] Offline : queue locale, sync au retour

---

#### FR-SYNC-02 : Sync favoris et tags

**Actor** : System  
**Capability** : Les favoris et tags sont synchronisés entre devices.

**Acceptance Criteria** :
- [ ] Même mécanisme que read state
- [ ] Propagation < 2 secondes
- [ ] Nouveaux tags créés disponibles partout

---

### FR-UX : Interface Utilisateur

#### FR-UX-01 : Application multi-plateforme

**Actor** : User  
**Capability** : L'utilisateur peut accéder à FeedMind depuis Web, iOS et Android.

**Acceptance Criteria** :
- [ ] Web : Chrome, Firefox, Safari, Edge (2 dernières versions)
- [ ] iOS : 15+ (iPhone et iPad)
- [ ] Android : 10+
- [ ] Même codebase (Expo)
- [ ] Expérience cohérente

---

#### FR-UX-02 : Raccourcis clavier (parité Inoreader)

**Actor** : User  
**Capability** : L'utilisateur peut utiliser tous les raccourcis clavier d'Inoreader.

**Acceptance Criteria** :
- [ ] Navigation : j/k, n/p, Shift+j/k
- [ ] Actions : m, s/f, t, v
- [ ] Vues : 1-5
- [ ] Global : a (ajouter), r (refresh), / (search), ? (aide)
- [ ] Batch : Shift+a (tout lu), l (anciens lu)
- [ ] Aide accessible via `?`

---

#### FR-UX-03 : Gestures mobile

**Actor** : User  
**Capability** : L'utilisateur peut utiliser des gestures sur mobile.

**Acceptance Criteria** :
- [ ] Swipe gauche : marquer lu
- [ ] Swipe droite : favoris
- [ ] Long press : menu contextuel
- [ ] Pull down : rafraîchir
- [ ] Animations fluides (60fps)

---

#### FR-UX-04 : Thème sombre

**Actor** : User  
**Capability** : L'utilisateur peut choisir entre thème clair, sombre ou système.

**Acceptance Criteria** :
- [ ] Toggle dans settings
- [ ] Options : Light, Dark, System
- [ ] Transition fluide
- [ ] Persistance de la préférence
- [ ] Respect du préférence système

---

### FR-ADMIN : Administration

#### FR-ADMIN-01 : Dashboard métriques

**Actor** : Admin  
**Capability** : L'administrateur peut voir les métriques globales du système.

**Acceptance Criteria** :
- [ ] Nombre d'utilisateurs actifs
- [ ] Nombre de flux total
- [ ] Articles stockés (count + size)
- [ ] Utilisation stockage (DB, Redis, S3)
- [ ] Graphiques 7/30 jours

---

#### FR-ADMIN-02 : Vue erreurs système

**Actor** : Admin  
**Capability** : L'administrateur peut voir les erreurs système.

**Acceptance Criteria** :
- [ ] Flux en erreur (tous users)
- [ ] Jobs échoués (worker)
- [ ] Erreurs API (rate limits providers)
- [ ] Actions : retry, disable, notify user

---

#### FR-ADMIN-03 : Alertes

**Actor** : Admin  
**Capability** : L'administrateur reçoit des alertes en cas de problème.

**Acceptance Criteria** :
- [ ] Alerte email si disk >80%
- [ ] Alerte si job queue >100 pending
- [ ] Alerte si uptime <99%
- [ ] Configuration des seuils

---

## Non-Functional Requirements

### NFR-PERF : Performance

#### NFR-PERF-01 : API Response Time

**Requirement** : Les endpoints API doivent répondre rapidement.

| Metric | Target |
|--------|--------|
| p50 | < 50ms |
| p95 | < 200ms |
| p99 | < 500ms |

**Acceptance Criteria** :
- [ ] Mesuré via Prometheus
- [ ] Dashboards Grafana
- [ ] Alertes si p95 > 300ms pendant 5min

---

#### NFR-PERF-02 : Refresh Performance

**Requirement** : Le refresh des flux doit être rapide même pour les gros volumes.

| Scenario | Target |
|----------|--------|
| 100 flux | < 30s |
| 500 flux | < 2min |
| 1000 flux | < 5min |
| 2000 flux | < 10min |

**Acceptance Criteria** :
- [ ] Parallel fetching (concurrency configurable)
- [ ] Smart retry (backoff exponentiel)
- [ ] Progress visible pour l'utilisateur

---

#### NFR-PERF-03 : Import OPML Performance

**Requirement** : L'import OPML doit gérer les gros fichiers.

| Scenario | Target |
|----------|--------|
| 100 flux | < 10s |
| 500 flux | < 30s |
| 1000 flux | < 2min |
| 2000 flux | < 5min |

---

#### NFR-PERF-04 : AI Evaluation Performance

**Requirement** : Les évaluations IA doivent être raisonnablement rapides.

| Scenario | Target |
|----------|--------|
| 1 article (single) | < 3s |
| 10 articles (batch) | < 10s |
| 50 articles (batch) | < 30s |

---

#### NFR-PERF-05 : App Performance

**Requirement** : L'application doit être performante sur tous les devices.

| Metric | Target |
|--------|--------|
| Time to First Article | < 3s (cold start) |
| Time to Interactive | < 2s |
| Frame rate | 60fps (animations) |
| App bundle size | < 15MB (iOS/Android) |

---

### NFR-SCALE : Scalabilité

#### NFR-SCALE-01 : Users V1

**Requirement** : V1 doit supporter l'équipe fondatrice.

| Metric | Target V1 |
|--------|-----------|
| Users concurrents | 5 |
| Flux total | 10,000 |
| Articles stockés | 500,000 |
| Règles total | 1,000 |

---

#### NFR-SCALE-02 : Users V2

**Requirement** : V2 doit supporter une croissance initiale.

| Metric | Target V2 |
|--------|-----------|
| Users total | 1,000 |
| Users concurrents | 100 |
| Flux total | 1,000,000 |
| Articles stockés | 50,000,000 |

---

### NFR-SEC : Sécurité

#### NFR-SEC-01 : Transport Security

**Requirement** : Toutes les communications sont chiffrées.

**Acceptance Criteria** :
- [ ] HTTPS obligatoire (redirect HTTP → HTTPS)
- [ ] TLS 1.2+ uniquement
- [ ] HSTS header activé
- [ ] Certificate pinning (mobile, optionnel)

---

#### NFR-SEC-02 : Data at Rest

**Requirement** : Les données sensibles sont chiffrées au repos.

**Acceptance Criteria** :
- [ ] Clés API IA : AES-256-GCM
- [ ] Mots de passe : Argon2id
- [ ] Tokens de session : signés HMAC-SHA256
- [ ] Backup DB : chiffré

---

#### NFR-SEC-03 : Authentication & Authorization

**Requirement** : L'authentification est robuste.

**Acceptance Criteria** :
- [ ] JWT avec expiration courte (15min access, 7d refresh)
- [ ] Rate limiting sur login (5 tentatives / 15min / IP)
- [ ] Session invalidation côté serveur possible
- [ ] RBAC pour admin vs user

---

#### NFR-SEC-04 : Data Isolation

**Requirement** : Les données utilisateurs sont isolées.

**Acceptance Criteria** :
- [ ] Queries toujours scopées par user_id
- [ ] Impossible d'accéder aux données d'un autre user
- [ ] Tests d'intrusion pour vérifier l'isolation

---

#### NFR-SEC-05 : Security Headers

**Requirement** : Les headers de sécurité sont configurés.

**Acceptance Criteria** :
- [ ] Content-Security-Policy
- [ ] X-Content-Type-Options: nosniff
- [ ] X-Frame-Options: DENY
- [ ] Referrer-Policy: strict-origin-when-cross-origin
- [ ] Permissions-Policy

---

### NFR-REL : Fiabilité

#### NFR-REL-01 : Uptime

**Requirement** : Le service doit être hautement disponible.

| Metric | Target |
|--------|--------|
| Uptime mensuel | ≥ 99.5% |
| Downtime max/mois | < 3.6h |
| MTTR | < 1h |

---

#### NFR-REL-02 : Data Durability

**Requirement** : Les données ne doivent pas être perdues.

| Metric | Target |
|--------|--------|
| Durabilité | 99.99% |
| RPO | < 24h |
| RTO | < 4h |

**Acceptance Criteria** :
- [ ] Backup PostgreSQL quotidien
- [ ] Backup testé mensuellement (restore)
- [ ] Point-in-time recovery possible

---

#### NFR-REL-03 : Graceful Degradation

**Requirement** : Le système reste utilisable en cas de panne partielle.

**Acceptance Criteria** :
- [ ] Si IA provider down → règles regex fonctionnent
- [ ] Si Redis down → pas de cache, mais lecture fonctionne
- [ ] Si Worker down → pas de refresh auto, mais lecture fonctionne

---

### NFR-COMPAT : Compatibilité

#### NFR-COMPAT-01 : Navigateurs Web

**Requirement** : Support des navigateurs modernes.

| Browser | Versions |
|---------|----------|
| Chrome | 2 dernières |
| Firefox | 2 dernières |
| Safari | 2 dernières |
| Edge | 2 dernières |

---

#### NFR-COMPAT-02 : Mobile OS

**Requirement** : Support des OS mobiles récents.

| OS | Versions |
|----|----------|
| iOS | 15+ |
| Android | 10+ |

---

#### NFR-COMPAT-03 : Formats RSS/Atom

**Requirement** : Support de tous les formats de syndication.

| Format | Versions |
|--------|----------|
| RSS | 0.91, 0.92, 1.0, 2.0 |
| Atom | 0.3, 1.0 |
| JSON Feed | 1.0, 1.1 |
| OPML | 1.0, 2.0 |

---

### NFR-MAINT : Maintenabilité

#### NFR-MAINT-01 : Code Quality

**Requirement** : Le code doit être de haute qualité.

**Acceptance Criteria** :
- [ ] Test coverage > 70%
- [ ] Linting (Clippy pour Rust, ESLint pour TS)
- [ ] Formatting (rustfmt, Prettier)
- [ ] CI/CD avec checks obligatoires

---

#### NFR-MAINT-02 : Documentation

**Requirement** : Le projet doit être documenté.

**Acceptance Criteria** :
- [ ] README avec quick start
- [ ] API documentation (OpenAPI)
- [ ] Architecture Decision Records (ADRs)
- [ ] Contributing guide

---

#### NFR-MAINT-03 : Observability

**Requirement** : Le système doit être observable.

**Acceptance Criteria** :
- [ ] Logs structurés (JSON)
- [ ] Metrics Prometheus
- [ ] Tracing distribué (optionnel V2)
- [ ] Dashboards Grafana
- [ ] Alerting

---

### NFR-PRIV : Privacy

#### NFR-PRIV-01 : Zero Tracking

**Requirement** : Aucun tracking analytics tiers.

**Acceptance Criteria** :
- [ ] Pas de Google Analytics
- [ ] Pas de Mixpanel/Amplitude
- [ ] Pas de pixels tiers
- [ ] Analytics internes uniquement (self-hosted)

---

#### NFR-PRIV-02 : Image Privacy

**Requirement** : Les images ne leakent pas d'informations.

**Acceptance Criteria** :
- [ ] Toutes les images proxiées
- [ ] Pas de Referer vers sources
- [ ] EXIF supprimé
- [ ] IP utilisateur jamais exposée aux sources

---

#### NFR-PRIV-03 : API Key Privacy

**Requirement** : Les clés API sont protégées.

**Acceptance Criteria** :
- [ ] Jamais loggées
- [ ] Jamais affichées en clair en UI
- [ ] Chiffrées at rest
- [ ] Transmises uniquement en HTTPS

---

#### NFR-PRIV-04 : Data Portability

**Requirement** : L'utilisateur peut récupérer toutes ses données.

**Acceptance Criteria** :
- [ ] Export OPML (flux)
- [ ] Export JSON (tout)
- [ ] Temps d'export < 5min
- [ ] Format documenté

---

#### NFR-PRIV-05 : Right to be Forgotten

**Requirement** : L'utilisateur peut supprimer son compte et toutes ses données.

**Acceptance Criteria** :
- [ ] Bouton "Supprimer mon compte"
- [ ] Confirmation requise
- [ ] Suppression complète en < 24h
- [ ] Email de confirmation de suppression

---

## Technical Limits & Constraints *(AMD-003)*

### Limites par Feed

| Limite | Valeur | Comportement si dépassé |
|--------|--------|------------------------|
| **Max items par fetch** | 500 | Tronquer aux 500 plus récents |
| **Max taille feed** | 10 MB | Timeout + erreur |
| **Timeout fetch** | 30s | Retry avec backoff exponentiel |
| **Max redirects** | 5 | Erreur "Too many redirects" |
| **Max title length** | 500 chars | Tronquer |
| **Max content length** | 100 KB | Tronquer avec indicateur |

### Limites par User

| Limite | Free | Pro |
|--------|------|-----|
| **Max feeds** | 25 | 10,000 |
| **Max articles stockés** | 500 | Illimité (soft: 1M) |
| **Max règles regex** | 3 | 500 |
| **Max règles IA** | 0 | 200 |
| **Max tags** | 10 | 500 |
| **Max flux prioritaires** | 3 | Illimité |

### Limites Système

| Limite | Valeur | Raison |
|--------|--------|--------|
| **Rate limit API** | 100 req/min/user | Protection DoS |
| **Rate limit auth** | 5 tentatives/15min/IP | Brute force |
| **Max concurrent fetches** | 50 global | Ressources serveur |
| **Max AI batch size** | 5 articles | Qualité réponse LLM |
| **AI timeout** | 30s | UX acceptable |
| **WebSocket connections** | 10/user | Ressources serveur |

### Comportement Dégradé

| Situation | Comportement |
|-----------|--------------|
| Redis down | Pas de cache, lecture DB directe |
| AI provider down | Règles regex fonctionnent, IA désactivée temporairement |
| Worker down | Pas de refresh auto, lecture fonctionne |
| DB read-only | Mode lecture seule, pas de write |

---

## Security & Secrets Management *(AMD-004)*

### Architecture des Secrets

```
┌─────────────────────────────────────────────────────────────────┐
│                    SECRETS ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Environment Variables (Clever Cloud encrypted)                │
│  ├── MASTER_KEY_V1=base64(32 bytes random)                     │
│  ├── MASTER_KEY_V2=... (for rotation)                          │
│  ├── ACTIVE_KEY_VERSION=1                                       │
│  ├── DATABASE_URL=postgres://...                               │
│  └── REDIS_URL=redis://...                                     │
│                                                                 │
│  Key Derivation (per-user)                                      │
│  └── HKDF-SHA256(master_key, user_id, "api_keys")              │
│                                                                 │
│  Storage (PostgreSQL)                                           │
│  └── api_keys: encrypted_key (AES-256-GCM), nonce, key_version │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Key Rotation Procedure

1. Générer nouveau `MASTER_KEY_V{N+1}`
2. Ajouter en env var
3. Set `ACTIVE_KEY_VERSION=N+1`
4. Lazy migration : re-chiffrer au prochain accès
5. Après 30 jours : supprimer `MASTER_KEY_V{N}`

### Compromission Response

| Étape | Action | Timeline |
|-------|--------|----------|
| 1 | Générer nouvelle master key | Immédiat |
| 2 | Force re-encryption all keys | < 1h |
| 3 | Notifier tous les users | < 2h |
| 4 | Invalider ancienne master key | < 4h |
| 5 | Post-mortem | < 48h |

### Self-Hosted Security

| Mode | Stockage secrets | Sécurité |
|------|-----------------|----------|
| **Simple** | `.env` file | Basique |
| **Docker** | Docker secrets | Moyenne |
| **Enterprise** | HashiCorp Vault | Élevée |

---

## Infrastructure & Costs *(AMD-008)*

### V1 - Dogfooding (5 users)

| Service | Provider | Spec | Coût/mois |
|---------|----------|------|-----------|
| **API Server** | Clever Cloud | S (1 vCPU, 1GB) | 15€ |
| **Worker** | Clever Cloud | S (1 vCPU, 1GB) | 15€ |
| **PostgreSQL** | Clever Cloud | S (1GB) | 15€ |
| **Redis** | Clever Cloud | S (256MB) | 10€ |
| **Object Storage** | Clever Cloud | 10GB | 2€ |
| **Domain + SSL** | Cloudflare | Free | 0€ |
| **Monitoring** | Grafana Cloud | Free | 0€ |
| **Email** | Resend | 3k/mois free | 0€ |
| **Total V1** | | | **~57€/mois** |

### V2 - Scale (1000 users)

| Service | Provider | Spec | Coût/mois |
|---------|----------|------|-----------|
| **API Server** | Clever Cloud | M x2 | 120€ |
| **Worker** | Clever Cloud | M x2 | 120€ |
| **PostgreSQL** | Clever Cloud | M (50GB) | 100€ |
| **Redis** | Clever Cloud | M (2GB) | 40€ |
| **Object Storage** | Clever Cloud | 500GB | 50€ |
| **CDN** | Cloudflare Pro | | 20€ |
| **Monitoring** | Grafana Cloud | Pro | 50€ |
| **Email** | Resend | 50k | 20€ |
| **Total V2** | | | **~520€/mois** |

### Break-Even Analysis

| Scénario | Users Pro | MRR | Coûts | Marge |
|----------|-----------|-----|-------|-------|
| V1 | 5 | 25€ | 57€ | -32€ |
| Break-even | 15 | 75€ | 80€ | ~0€ |
| Rentable | 100 | 500€ | 150€ | +350€ |
| V2 Target | 500 | 2,500€ | 400€ | +2,100€ |

---

## Legal & RGPD *(AMD-009)*

### Données Collectées

| Donnée | Base légale | Rétention | Exportable |
|--------|-------------|-----------|------------|
| Email | Contrat | Jusqu'à suppression | Oui |
| Password (hash) | Contrat | Jusqu'à suppression | Non |
| Flux RSS | Contrat | Jusqu'à suppression | Oui (OPML) |
| Articles | Contrat | Jusqu'à suppression | Oui (JSON) |
| Clés API IA | Consentement | Jusqu'à révocation | Non |
| Logs connexion | Intérêt légitime | 90 jours | Sur demande |
| Métriques usage | Intérêt légitime | 1 an (agrégé) | Non |

### Droits RGPD

| Droit | Implémentation |
|-------|----------------|
| **Accès** | Export JSON complet (Settings) |
| **Rectification** | Settings > Profile |
| **Effacement** | Settings > Supprimer compte |
| **Portabilité** | Export OPML + JSON |
| **Opposition** | Email opt-out dans settings |

### Documents Légaux Requis

| Document | URL | Status |
|----------|-----|--------|
| Privacy Policy | /legal/privacy | À créer |
| Terms of Service | /legal/terms | À créer |
| Cookie Policy | /legal/cookies | À créer |
| DPA | /legal/dpa | À créer |

### Localisation des Données

| Composant | Localisation | Provider |
|-----------|--------------|----------|
| Database | France (Paris) | Clever Cloud |
| Object Storage | France | Clever Cloud |
| CDN | Global (cache) | Cloudflare |
| Email | EU | Resend |
| AI APIs | US | Anthropic/Google* |

*Consentement explicite requis pour transfert hors UE

---

## Monitoring & Operations *(AMD-010)*

### Metrics Stack

```
Application Metrics (Prometheus)
├── http_requests_total (by endpoint, status)
├── http_request_duration_seconds (p50, p95, p99)
├── active_websocket_connections
├── feed_refresh_duration_seconds
├── ai_evaluation_duration_seconds
└── ai_tokens_consumed_total

Infrastructure Metrics
├── cpu_usage_percent
├── memory_usage_bytes
├── disk_usage_percent
└── postgres_connections_active

Business Metrics
├── users_active_daily
├── feeds_total
├── articles_fetched_total
└── rules_evaluated_total
```

### Alerting Rules

| Severity | Condition | Channel | Response |
|----------|-----------|---------|----------|
| **CRITICAL** | API down > 1min | PagerDuty | Immediate |
| **CRITICAL** | Error rate > 10% | PagerDuty | Immediate |
| **CRITICAL** | Disk > 95% | PagerDuty | Immediate |
| **WARNING** | API p95 > 500ms 5min | Slack | < 1h |
| **WARNING** | Error rate > 5% | Slack | < 1h |
| **WARNING** | Disk > 80% | Slack | < 4h |
| **WARNING** | Worker queue > 100 | Slack | < 1h |
| **INFO** | Deployment done | Slack | N/A |

### Logs

| Level | Retention | Use case |
|-------|-----------|----------|
| ERROR | 90 jours | Debugging, alerting |
| WARN | 30 jours | Monitoring |
| INFO | 7 jours | Audit |
| DEBUG | 24h (dev only) | Development |

### On-Call (V1)

- **Horaires** : Best effort (pas de SLA formel)
- **Response** : < 1h pour CRITICAL en heures ouvrées
- **Escalation** : Slack → Email → SMS
- **Runbook** : À documenter

### Dashboards

| Dashboard | Audience | Refresh |
|-----------|----------|---------|
| Overview | Everyone | 30s |
| API Performance | Dev | 10s |
| Worker | Dev | 30s |
| Business | Product | 5min |
| Infrastructure | Ops | 30s |

---

## Appendices

### A. Glossaire

| Terme | Définition |
|-------|------------|
| **BYOK** | Bring Your Own Key - l'utilisateur fournit sa propre clé API IA |
| **Smart Polling** | Algorithme adaptatif de refresh basé sur l'activité du flux |
| **Explicabilité** | Capacité à expliquer pourquoi une décision IA a été prise |
| **Readability** | Algorithme d'extraction du contenu principal d'une page web |
| **OPML** | Outline Processor Markup Language - format d'export de flux RSS |
| **Dogfooding** | Utiliser son propre produit en interne |
| **Power user** | Utilisateur avancé avec des besoins élevés (1000+ flux, règles complexes) |
| **Vendor lock-in** | Situation où l'utilisateur est piégé dans un écosystème propriétaire |

### B. Références

| Ressource | URL | Description |
|-----------|-----|-------------|
| Inoreader | https://inoreader.com | Concurrent principal |
| Feedly | https://feedly.com | Concurrent |
| SponsorBlock API | https://sponsor.ajay.app/ | API pour skip sponsors YouTube |
| feed-rs | https://github.com/feed-rs/feed-rs | Bibliothèque Rust RSS |
| Expo | https://expo.dev | Framework React Native |
| Axum | https://github.com/tokio-rs/axum | Framework HTTP Rust |
| NativeWind | https://nativewind.dev | Tailwind pour React Native |
| Anthropic | https://anthropic.com | Provider IA (Claude) |
| Google AI | https://ai.google.dev | Provider IA (Gemini) |

### C. Changelog PRD

| Version | Date | Auteur | Changements |
|---------|------|--------|-------------|
| 1.0.0 | 2026-01-27 | Constantin | Version initiale |
| 2.0.0 | 2026-01-27 | Constantin | Version BMAD Ultra-Complete |

### D. Raccourcis Clavier (Référence Complète)

```
┌─────────────────────────────────────────────────────────────────┐
│                    FEEDMIND KEYBOARD SHORTCUTS                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  NAVIGATION                                                     │
│  ─────────────────────────────────────────────────────────────  │
│  j / ↓         Article suivant (ouvre si fermé)                │
│  k / ↑         Article précédent                               │
│  n             Article non-lu suivant (sans ouvrir)            │
│  p             Article non-lu précédent (sans ouvrir)          │
│  Shift + j     Flux suivant                                    │
│  Shift + k     Flux précédent                                  │
│  o / Enter     Ouvrir/fermer l'article sélectionné             │
│  g puis h      Aller à Home                                    │
│  g puis s      Aller à Favoris (Starred)                       │
│  g puis a      Aller à Tous les articles                       │
│                                                                 │
│  ACTIONS                                                        │
│  ─────────────────────────────────────────────────────────────  │
│  m             Marquer lu/non-lu                               │
│  s / f         Ajouter/retirer des favoris                     │
│  t             Ajouter un tag                                  │
│  v             Ouvrir dans un nouvel onglet                    │
│  Shift + a     Marquer tout comme lu (vue actuelle)            │
│  l             Marquer les plus anciens comme lus              │
│                                                                 │
│  VUES                                                           │
│  ─────────────────────────────────────────────────────────────  │
│  1             Vue Liste                                       │
│  2             Vue Expanded                                    │
│  3             Vue Colonnes                                    │
│  4             Vue Cartes                                      │
│  5             Vue Magazine                                    │
│                                                                 │
│  GLOBAL                                                         │
│  ─────────────────────────────────────────────────────────────  │
│  a             Ajouter un nouveau flux                         │
│  r             Rafraîchir le flux actuel                       │
│  /             Ouvrir la recherche                             │
│  ?             Afficher l'aide des raccourcis                  │
│  Escape        Fermer modal/panneau                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### E. Gestures Mobile (Référence Complète)

```
┌─────────────────────────────────────────────────────────────────┐
│                     FEEDMIND MOBILE GESTURES                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  LISTE D'ARTICLES                                               │
│  ────────────────────────────────────���────────────────────────  │
│  Tap              Ouvrir l'article                              │
│  Swipe ←          Marquer comme lu                              │
│  Swipe →          Ajouter aux favoris                           │
│  Long press       Menu contextuel (tag, partager, etc.)        │
│                                                                 │
│  VUE ARTICLE                                                    │
│  ─────────────────────────────────────────────────────────────  │
│  Pull down        Ouvrir dans navigateur externe               │
│  Swipe ←/→        Article précédent/suivant                    │
│  Double tap       Zoom (images)                                │
│                                                                 │
│  GLOBAL                                                         │
│  ─────────────────────────────────────────────────────────────  │
│  Pull down        Rafraîchir (sur liste principale)            │
│  Edge swipe       Navigation (retour)                          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

**Document Status : ✅ COMPLETE (BMAD Ultra-Complete v2.0)**

Ce PRD est prêt pour review et approbation avant le début de l'implémentation.

**Checklist de validation BMAD** :
- [x] Executive Summary avec vision, problème, différenciateurs
- [x] Project Classification avec stack technique complète
- [x] Success Criteria avec métriques mesurables
- [x] User Journeys narratifs avec 5 personas détaillés
- [x] Domain-Specific Requirements (RSS, IA, Privacy)
- [x] Innovation Patterns avec validation et risques
- [x] Scoping clair (In/Out of scope)
- [x] Functional Requirements avec acceptance criteria
- [x] Non-Functional Requirements mesurables
- [x] Appendices de référence

---

*Généré avec la méthode BMAD (Build Measure Analyze Deploy)*  
*Version : 2.0.0 Ultra-Complete*