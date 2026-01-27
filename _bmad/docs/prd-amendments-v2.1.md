# PRD Amendments - FeedMind.ai v2.1

**Date:** 2026-01-27  
**Statut:** APPROVED  
**Appliqué au:** PRD v2.0 → v2.1

Ce document liste tous les amendements apportés au PRD suite à la review critique.

---

## Table des Amendements

| ID | Catégorie | Priorité | Description |
|----|-----------|----------|-------------|
| AMD-001 | Business | MUST-FIX | Nouvelle stratégie trial (Free tier sans CB) |
| AMD-002 | Scope | MUST-FIX | Recherche basique en V1 |
| AMD-003 | Technique | MUST-FIX | Limites techniques (feeds, timeouts) |
| AMD-004 | Sécurité | MUST-FIX | Gestion des secrets documentée |
| AMD-005 | Roadmap | SHOULD-FIX | Managed IA avancé en V1.1 |
| AMD-006 | UX | SHOULD-FIX | Concept de flux prioritaires |
| AMD-007 | Architecture | SHOULD-FIX | organization_id nullable dès V1 |
| AMD-008 | Ops | SHOULD-FIX | Estimation coûts infrastructure |
| AMD-009 | Legal | SHOULD-FIX | Section RGPD et Privacy |
| AMD-010 | Ops | SHOULD-FIX | Plan de monitoring |
| AMD-011 | UX | NICE-TO-HAVE | Rule debugger |
| AMD-012 | Technique | NICE-TO-HAVE | Benchmark Expo vs Next.js |
| AMD-013 | Stratégie | NICE-TO-HAVE | Migration Inoreader rules |

---

## AMD-001 : Nouvelle Stratégie Trial

### Problème identifié

CB obligatoire pour trial = friction majeure. Beaucoup d'utilisateurs potentiels abandonneront avant d'essayer le produit.

### Solution : Modèle Freemium + Trial

```
AVANT (v2.0)
────────────────────────────────
Free (Trial 14j) → CB requise → Pro

APRÈS (v2.1)
────────────────────────────────
Free Forever → (pas de CB) → Pro Trial 14j → (CB requise) → Pro
     │                              │
     └─ Limité mais utilisable     └─ Débloque IA
```

### Nouveaux Tiers

| Tier | Prix | CB requise | Limites | IA |
|------|------|------------|---------|-----|
| **Free** | 0€ | Non | 25 flux, 500 articles, 3 règles regex | ❌ |
| **Pro Trial** | 0€ (14j) | Oui | Illimité | ✅ BYOK |
| **Pro** | 5€/mois | Oui | Illimité | ✅ BYOK |
| **Team** | 15€/user/mois | Oui | Illimité + collab | ✅ BYOK |

### Justification

- Free tier permet de tester le produit sans engagement
- Conversion Free → Pro Trial quand l'utilisateur veut l'IA
- CB requise uniquement quand la valeur est démontrée
- Réduit le churn "j'ai oublié d'annuler"

### Impact sur les FRs

- Modifier FR-ONBOARD-04 (CB pour trial → CB pour Pro Trial)
- Ajouter FR-TIER-01 (Upgrade Free → Pro Trial)
- Ajouter FR-TIER-02 (Limite atteinte → prompt upgrade)

---

## AMD-002 : Recherche Basique en V1

### Problème identifié

Pour 1000+ flux, impossible de retrouver un article sans recherche. C'est bloquant pour les power users.

### Solution : Recherche V1 (basique)

**Scope V1** :
- Recherche par titre (ILIKE PostgreSQL)
- Recherche par source/feed name
- Recherche par tag
- Filtre par date (aujourd'hui, 7j, 30j, custom)

**Hors scope V1** (V1.1) :
- Full-text search dans le contenu (nécessite index FTS)
- Recherche sémantique IA
- Recherche dans les articles masqués

### Nouveaux FRs

#### FR-SEARCH-01 : Recherche par titre

**Actor** : User  
**Capability** : L'utilisateur peut rechercher des articles par titre.

**Acceptance Criteria** :
- [ ] Champ de recherche accessible via `/` (raccourci)
- [ ] Recherche case-insensitive
- [ ] Résultats en temps réel (debounce 300ms)
- [ ] Minimum 2 caractères pour déclencher
- [ ] Highlight des termes matchés
- [ ] Temps de réponse < 500ms (10k articles)

#### FR-SEARCH-02 : Filtres de recherche

**Actor** : User  
**Capability** : L'utilisateur peut filtrer les résultats de recherche.

**Acceptance Criteria** :
- [ ] Filtre par source/feed
- [ ] Filtre par tag
- [ ] Filtre par date (presets + custom range)
- [ ] Filtre par statut (lu/non-lu/favori)
- [ ] Combinaison de filtres (AND)

### Impact Technique

- Index PostgreSQL sur `articles.title` (gin_trgm_ops pour ILIKE rapide)
- Endpoint `GET /api/articles/search?q=...&feed=...&tag=...`
- Composant SearchBar dans l'app

---

## AMD-003 : Limites Techniques

### Problème identifié

Le PRD ne définit pas les limites de protection contre les feeds malformés ou abusifs.

### Solution : Limites explicites

#### Limites par Feed

| Limite | Valeur | Comportement si dépassé |
|--------|--------|------------------------|
| **Max items par fetch** | 500 | Tronquer aux 500 plus récents |
| **Max taille feed** | 10 MB | Timeout + erreur |
| **Timeout fetch** | 30s | Retry avec backoff |
| **Max redirects** | 5 | Erreur "Too many redirects" |
| **Max title length** | 500 chars | Tronquer |
| **Max content length** | 100 KB | Tronquer avec "..." |

#### Limites par User

| Limite | Free | Pro |
|--------|------|-----|
| **Max feeds** | 25 | 10,000 |
| **Max articles stockés** | 500 | Illimité |
| **Max règles regex** | 3 | 500 |
| **Max règles IA** | 0 | 200 |
| **Max tags** | 10 | 500 |
| **Max refresh override** | 0 | 20 |

#### Limites Système

| Limite | Valeur | Raison |
|--------|--------|--------|
| **Rate limit API** | 100 req/min/user | Protection DoS |
| **Rate limit auth** | 5 tentatives/15min/IP | Brute force |
| **Max concurrent fetches** | 50 | Ressources serveur |
| **Max AI batch size** | 5 articles | Qualité réponse LLM |
| **AI timeout** | 30s | UX acceptable |

### Nouveaux FRs

#### FR-LIMIT-01 : Gestion des dépassements

**Actor** : System  
**Capability** : Le système gère gracieusement les dépassements de limites.

**Acceptance Criteria** :
- [ ] Message clair expliquant la limite atteinte
- [ ] Pour Free : prompt d'upgrade vers Pro
- [ ] Pour Pro : suggestion de cleanup ou contact support
- [ ] Logs des dépassements (monitoring)

#### FR-LIMIT-02 : Affichage des quotas

**Actor** : User  
**Capability** : L'utilisateur peut voir sa consommation vs ses limites.

**Acceptance Criteria** :
- [ ] Page Settings > Usage
- [ ] Barres de progression (feeds, articles, règles)
- [ ] Alerte quand >80% d'une limite
- [ ] Historique de consommation (30j)

---

## AMD-004 : Gestion des Secrets

### Problème identifié

Le PRD mentionne "AES-256-GCM avec master key" sans détailler la gestion de cette master key.

### Solution : Architecture Secrets

```
┌─────────────────────────────────────────────────────────────────┐
│                    SECRETS ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  PRODUCTION (Clever Cloud)                                      │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Environment Variables (encrypted at rest by CC)               │
│  ├── MASTER_KEY_V1=base64(32 bytes random)                     │
│  ├── MASTER_KEY_V2=... (for rotation)                          │
│  ├── ACTIVE_KEY_VERSION=1                                       │
│  ├── DATABASE_URL=postgres://...                               │
│  └── REDIS_URL=redis://...                                     │
│                                                                 │
│  API Server                                                     │
│  ├── Load MASTER_KEY on startup                                │
│  ├── Derive per-user keys: HKDF(master, user_id, "api_keys")   │
│  └── Encrypt/Decrypt API keys with derived key                 │
│                                                                 │
│  Database                                                       │
│  └── api_keys table                                            │
│      ├── user_id                                               │
│      ├── provider (anthropic, google)                          │
│      ├── encrypted_key (AES-256-GCM ciphertext)               │
│      ├── nonce (unique per encryption)                         │
│      └── key_version (which master key)                        │
│                                                                 │
│  KEY ROTATION                                                   │
│  ─────────────────────────────────────────────────────────────  │
│  1. Generate new MASTER_KEY_V2                                  │
│  2. Set ACTIVE_KEY_VERSION=2                                    │
│  3. Re-encrypt all keys on next access (lazy migration)        │
│  4. After 30 days, remove MASTER_KEY_V1                        │
│                                                                 │
│  SELF-HOSTED                                                    │
│  ─────────────────────────────────────────────────────────────  │
│  Option A: .env file (simple, less secure)                     │
│  Option B: Docker secrets                                       │
│  Option C: HashiCorp Vault (enterprise)                        │
│                                                                 │
│  COMPROMISSION RESPONSE                                         │
│  ─────────────────────────────────────────────────────────────  │
│  If master key compromised:                                     │
│  1. Generate new master key immediately                        │
│  2. Force re-encryption of all user keys                       │
│  3. Notify all users to rotate their API keys                  │
│  4. Invalidate old master key                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Nouveaux NFRs

#### NFR-SEC-06 : Master Key Management

**Requirement** : La master key est gérée de manière sécurisée.

**Acceptance Criteria** :
- [ ] Master key générée avec CSPRNG (32 bytes)
- [ ] Stockée en env var, jamais en code ou DB
- [ ] Key rotation possible sans downtime
- [ ] Procédure de compromission documentée
- [ ] Backup de la master key en lieu sûr (hors serveur)

#### NFR-SEC-07 : Per-User Key Derivation

**Requirement** : Chaque utilisateur a une clé dérivée unique.

**Acceptance Criteria** :
- [ ] HKDF-SHA256 pour dérivation
- [ ] Salt = user_id (stable, unique)
- [ ] Compromission d'un user n'expose pas les autres

---

## AMD-005 : Managed IA en V1.1

### Problème identifié

BYOK exclut les non-tech. Attendre V2 pour Managed IA limite l'audience.

### Solution : Avancer Managed IA

**Nouvelle roadmap** :

```
V1.0 - MVP
└── BYOK uniquement

V1.1 - Intégrations + Managed IA  ← CHANGEMENT
├── Managed IA (+10% marge)
├── YouTube + SponsorBlock
├── Recherche full-text (contenu)
├── API Fever
└── Score pertinence

V2 - Collaboration (inchangé)
```

### Implémentation Managed IA

```
┌─────────────────────────────────────────────────────────────────┐
│                    MANAGED IA ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  USER (sans clé API)                                           │
│     │                                                          │
│     │ Active "Managed IA" dans settings                        │
│     │                                                          │
│     ▼                                                          │
│  FEEDMIND API                                                   │
│     │                                                          │
│     │ 1. Utilise la clé API FeedMind (Anthropic)              │
│     │ 2. Track tokens consommés par user                       │
│     │ 3. Facture user en fin de mois (+10% marge)             │
│     │                                                          │
│     ▼                                                          │
│  BILLING                                                        │
│     ├── Coût Anthropic : $X                                    │
│     ├── Marge FeedMind : +10%                                  │
│     └── Facture user : $X * 1.10                               │
│                                                                 │
│  LIMITES MANAGED                                                │
│     ├── Max 100k tokens/mois (soft limit)                      │
│     ├── Alerte à 80%                                           │
│     └── Hard limit à 150k (évite surprises)                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Nouveaux FRs

#### FR-AI-05 : Activation Managed IA

**Actor** : User (Pro)  
**Capability** : L'utilisateur Pro peut activer Managed IA sans fournir de clé.

**Acceptance Criteria** :
- [ ] Toggle "Utiliser FeedMind Managed IA"
- [ ] Explication du pricing (+10% sur coût réel)
- [ ] Estimation mensuelle basée sur usage actuel
- [ ] CB requise (déjà le cas pour Pro)

#### FR-AI-06 : Dashboard consommation Managed

**Actor** : User  
**Capability** : L'utilisateur peut suivre sa consommation IA managed.

**Acceptance Criteria** :
- [ ] Tokens consommés ce mois
- [ ] Coût estimé actuel
- [ ] Graphique d'évolution
- [ ] Alertes configurables (50%, 80%, 100%)
- [ ] Historique des 6 derniers mois

---

## AMD-006 : Flux Prioritaires

### Problème identifié

Avec 1000+ flux, impossible de distinguer les flux importants du bruit.

### Solution : Système de priorité

```
┌─────────────────────────────────────────────────────────────────┐
│                    FEED PRIORITY SYSTEM                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  PRIORITÉS                                                      │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  ⭐⭐⭐ HOT (Prioritaire)                                       │
│  ├── Refresh : toutes les 15 min (override auto)               │
│  ├── Notification : badge + son optionnel                      │
│  └── Affichage : toujours en haut de sidebar                   │
│                                                                 │
│  ⭐⭐ WARM (Normal)                                             │
│  ├── Refresh : Smart Polling (adaptatif)                       │
│  ├── Notification : badge uniquement                           │
│  └── Affichage : ordre alphabétique dans dossier               │
│                                                                 │
│  ⭐ COLD (Archive)                                              │
│  ├── Refresh : toutes les 24h                                  │
│  ├── Notification : aucune                                     │
│  └── Affichage : section "Archives" (collapsed)                │
│                                                                 │
│  UX                                                             │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Sidebar :                                                      │
│  ┌─────────────────────────┐                                   │
│  │ ⭐ PRIORITAIRES (12)    │ ← toujours visible                │
│  │   Hacker News           │                                   │
│  │   TechCrunch            │                                   │
│  │   ...                   │                                   │
│  ├─────────────────────────┤                                   │
│  │ 📁 Tech (45)            │                                   │
│  │ 📁 Business (23)        │                                   │
│  │ 📁 ...                  │                                   │
│  ├─────────────────────────┤                                   │
│  │ 📦 Archives (234)       │ ← collapsed par défaut            │
│  └─────────────────────────┘                                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Nouveaux FRs

#### FR-FEED-08 : Définir priorité de flux

**Actor** : User  
**Capability** : L'utilisateur peut définir la priorité d'un flux (Hot/Warm/Cold).

**Acceptance Criteria** :
- [ ] Menu contextuel ou raccourci
- [ ] 3 niveaux : Prioritaire, Normal, Archive
- [ ] Bulk action possible (sélection multiple)
- [ ] Import OPML : tous les flux en "Normal" par défaut

#### FR-FEED-09 : Vue prioritaires

**Actor** : User  
**Capability** : L'utilisateur peut voir uniquement les articles des flux prioritaires.

**Acceptance Criteria** :
- [ ] Vue "Prioritaires" dans sidebar
- [ ] Affiche uniquement les flux HOT
- [ ] Raccourci clavier `g p` (go priority)

#### FR-FEED-10 : Section Archives

**Actor** : User  
**Capability** : Les flux archivés sont regroupés dans une section dédiée.

**Acceptance Criteria** :
- [ ] Section "Archives" en bas de sidebar
- [ ] Collapsed par défaut
- [ ] Non inclus dans "Tous les articles" (sauf si explicitement ouvert)
- [ ] Refresh réduit (24h)

---

## AMD-007 : organization_id nullable

### Problème identifié

V1 est single-user mais V2 ajoute les organizations. Sans préparation, migration douloureuse.

### Solution : Préparer le schema

```sql
-- V1 Schema (préparé pour V2)

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    -- ... autres champs
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE organizations (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- En V1 : organization_id est NULL pour tous
-- En V2 : migration crée une org par user existant
CREATE TABLE feeds (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    organization_id UUID REFERENCES organizations(id), -- NULL en V1
    url TEXT NOT NULL,
    -- ... autres champs
    CONSTRAINT feeds_owner CHECK (
        user_id IS NOT NULL OR organization_id IS NOT NULL
    )
);

CREATE TABLE articles (
    id UUID PRIMARY KEY,
    feed_id UUID REFERENCES feeds(id) NOT NULL,
    user_id UUID REFERENCES users(id) NOT NULL,
    organization_id UUID REFERENCES organizations(id), -- NULL en V1
    -- ... autres champs
);

-- Index pour V2 (créé mais pas utilisé en V1)
CREATE INDEX idx_feeds_org ON feeds(organization_id) WHERE organization_id IS NOT NULL;
CREATE INDEX idx_articles_org ON articles(organization_id) WHERE organization_id IS NOT NULL;
```

### Impact

- Toutes les queries V1 filtrent par `user_id`
- organization_id reste NULL en V1
- Migration V2 :
  1. Créer une org par user existant
  2. Mettre à jour organization_id sur tous les records
  3. Ajouter les features team

---

## AMD-008 : Estimation Coûts Infrastructure

### Problème identifié

Pas de budget estimé. Risque de mauvaise surprise financière.

### Solution : Budget détaillé

#### V1 - Dogfooding (5 users)

| Service | Provider | Spec | Coût/mois |
|---------|----------|------|-----------|
| **API Server** | Clever Cloud | S (1 vCPU, 1GB RAM) | 15€ |
| **Worker** | Clever Cloud | S (1 vCPU, 1GB RAM) | 15€ |
| **PostgreSQL** | Clever Cloud | S (1GB storage) | 15€ |
| **Redis** | Clever Cloud | S (256MB) | 10€ |
| **Object Storage** | Clever Cloud | 10GB | 2€ |
| **Domain + SSL** | Cloudflare | Free tier | 0€ |
| **Monitoring** | Grafana Cloud | Free tier | 0€ |
| **Email** | Resend | 3k emails/mois free | 0€ |
| **Total V1** | | | **~57€/mois** |

#### V2 - Scale (1000 users)

| Service | Provider | Spec | Coût/mois |
|---------|----------|------|-----------|
| **API Server** | Clever Cloud | M x2 (2 vCPU, 4GB) | 120€ |
| **Worker** | Clever Cloud | M x2 (2 vCPU, 4GB) | 120€ |
| **PostgreSQL** | Clever Cloud | M (50GB, replicas) | 100€ |
| **Redis** | Clever Cloud | M (2GB) | 40€ |
| **Object Storage** | Clever Cloud | 500GB | 50€ |
| **CDN** | Cloudflare Pro | | 20€ |
| **Monitoring** | Grafana Cloud | Pro | 50€ |
| **Email** | Resend | 50k emails | 20€ |
| **Total V2** | | | **~520€/mois** |

#### Revenus vs Coûts

| Scénario | Users | MRR (5€/user) | Coûts | Marge |
|----------|-------|---------------|-------|-------|
| V1 Dogfooding | 5 | 25€ | 57€ | -32€ |
| Break-even | 15 | 75€ | 80€ | ~0€ |
| V2 Target | 500 | 2,500€ | 400€ | +2,100€ |
| V2 Max | 1000 | 5,000€ | 520€ | +4,480€ |

**Note** : Coûts IA non inclus (BYOK = 0€ pour FeedMind)

---

## AMD-009 : Section RGPD et Legal

### Problème identifié

Pas de section legal. Risque de non-conformité RGPD.

### Solution : Ajouter section Legal

#### Données collectées

| Donnée | Base légale | Rétention | Exportable |
|--------|-------------|-----------|------------|
| Email | Contrat | Jusqu'à suppression compte | Oui |
| Mot de passe (hash) | Contrat | Jusqu'à suppression | Non |
| Flux RSS | Contrat | Jusqu'à suppression | Oui (OPML) |
| Articles | Contrat | Jusqu'à suppression | Oui (JSON) |
| Clés API IA (chiffrées) | Consentement | Jusqu'à révocation | Non |
| Logs de connexion | Intérêt légitime | 90 jours | Sur demande |
| Métriques usage | Intérêt légitime | 1 an (agrégé) | Non |

#### Droits RGPD implémentés

| Droit | Implémentation | FR |
|-------|----------------|-----|
| **Accès** | Export JSON complet | FR-EXPORT-02 |
| **Rectification** | Settings > Profile | FR-PROFILE-01 |
| **Effacement** | Settings > Supprimer compte | FR-AUTH-06 |
| **Portabilité** | Export OPML + JSON | FR-EXPORT-01/02 |
| **Opposition** | Email opt-out | FR-NOTIF-02 |

#### Documents requis

| Document | Status | URL |
|----------|--------|-----|
| Privacy Policy | À créer | /legal/privacy |
| Terms of Service | À créer | /legal/terms |
| Cookie Policy | À créer | /legal/cookies |
| DPA (Data Processing Agreement) | À créer | /legal/dpa |

#### Localisation des données

| Composant | Localisation | Provider |
|-----------|--------------|----------|
| Base de données | France (Paris) | Clever Cloud |
| Object Storage | France | Clever Cloud |
| CDN | Global (cache only) | Cloudflare |
| Email | EU | Resend |

**Note** : Aucun transfert hors UE sauf pour les APIs IA (Anthropic US) - consentement explicite requis.

---

## AMD-010 : Plan de Monitoring

### Problème identifié

Pas de plan de monitoring ou on-call défini.

### Solution : Observability Stack

```
┌─────────────────────────────────────────────────────────────────┐
│                    OBSERVABILITY STACK                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  METRICS (Prometheus + Grafana Cloud)                          │
│  ─────────────────────────────────────────────────────────────  │
│  Application :                                                  │
│  ├── http_requests_total (by endpoint, status)                 │
│  ├── http_request_duration_seconds (p50, p95, p99)            │
│  ├── active_connections (WebSocket)                            │
│  ├── feed_refresh_duration_seconds                             │
│  ├── ai_evaluation_duration_seconds                            │
│  └── ai_tokens_consumed_total                                  │
│                                                                 │
│  Infrastructure :                                               │
│  ├── cpu_usage_percent                                         │
│  ├── memory_usage_bytes                                        │
│  ├── disk_usage_percent                                        │
│  └── postgres_connections_active                               │
│                                                                 │
│  Business :                                                     │
│  ├── users_active_daily                                        │
│  ├── feeds_total                                               │
│  ├── articles_fetched_total                                    │
│  └── rules_evaluated_total                                     │
│                                                                 │
│  LOGS (Vector + Grafana Loki)                                  │
│  ─────────────────────────────────────────────────────────────  │
│  Format : JSON structuré                                        │
│  {                                                              │
│    "timestamp": "2026-01-27T12:00:00Z",                        │
│    "level": "info|warn|error",                                 │
│    "service": "api|worker",                                    │
│    "trace_id": "abc123",                                       │
│    "user_id": "user_456",                                      │
│    "message": "...",                                           │
│    "context": {...}                                            │
│  }                                                              │
│                                                                 │
│  Rétention :                                                    │
│  ├── Error : 90 jours                                          │
│  ├── Warn : 30 jours                                           │
│  └── Info : 7 jours                                            │
│                                                                 │
│  ALERTING                                                       │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  CRITICAL (PagerDuty → SMS + Call)                             │
│  ├── API down > 1 min                                          │
│  ├── Error rate > 10%                                          │
│  ├── Database unreachable                                      │
│  └── Disk > 95%                                                │
│                                                                 │
│  WARNING (Slack #alerts)                                        │
│  ├── API p95 > 500ms for 5 min                                 │
│  ├── Error rate > 5%                                           │
│  ├── Disk > 80%                                                │
│  ├── Worker queue > 100 pending                                │
│  └── AI provider rate limited                                  │
│                                                                 │
│  INFO (Slack #monitoring)                                       │
│  ├── Deployment completed                                      │
│  ├── Scheduled maintenance                                     │
│  └── Weekly usage report                                       │
│                                                                 │
│  ON-CALL (V1 - équipe de 1-2)                                  │
│  ─────────────────────────────────────────────────────────────  │
│  Horaires : Best effort (pas de SLA)                           │
│  Response : < 1h pour CRITICAL en heures ouvrées              │
│  Escalation : Slack → Email → SMS (si configuré)               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Dashboards Grafana

| Dashboard | Contenu | Audience |
|-----------|---------|----------|
| **Overview** | Santé globale, requêtes, erreurs | Everyone |
| **API Performance** | Latency par endpoint, error breakdown | Dev |
| **Worker** | Queue size, job duration, failures | Dev |
| **Business** | DAU, feeds, articles, conversions | Product |
| **Infrastructure** | CPU, RAM, disk, connections | Ops |

---

## AMD-011 : Rule Debugger

### Problème identifié

À 200 règles, impossible de savoir quelle règle affecte quel article.

### Solution : Mode debug pour règles

#### FR-RULE-07 : Debug mode article

**Actor** : User  
**Capability** : L'utilisateur peut voir quelles règles affectent un article spécifique.

**Acceptance Criteria** :
- [ ] Bouton "🔍 Debug" sur chaque article
- [ ] Affiche toutes les règles évaluées
- [ ] Pour chaque règle : match (oui/non), raison, confidence
- [ ] Highlight de la règle qui a "gagné" (première qui match)
- [ ] Possibilité de tester "et si je désactive cette règle ?"

#### UI Mockup

```
┌─────────────────────────────────────────────────────────────────┐
│  🔍 DEBUG : "New crypto exchange launches"                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  RÈGLE APPLIQUÉE (celle qui a matché en premier)               │
│  ─────────────────────────────────────────────────────────────  │
│  ✅ #12 "Masquer crypto sauf BTC/ETH" (IA)                     │
│     └─ Raison : "Mentionne crypto exchange, pas BTC/ETH"       │
│     └─ Confidence : 94%                                        │
│     └─ [Désactiver cette règle]                                │
│                                                                 │
│  AUTRES RÈGLES ÉVALUÉES                                         │
│  ─────────────────────────────────────────────────────────────  │
│  ❌ #3 "Masquer clickbait" (IA) → No match (87%)               │
│  ❌ #7 "Masquer less than 5M€" (IA) → No match (montant: N/A) │
│  ⏭️ #15 "Tag 'finance'" (regex) → Skipped (article masqué)     │
│                                                                 │
│  SIMULATION                                                     │
│  ─────────────────────────────────────────────────────────────  │
│  Si je désactive #12 :                                         │
│  → Article serait VISIBLE                                      │
│  → #15 appliquerait le tag "finance"                           │
│                                                                 │
│  [Restaurer cet article] [Fermer]                              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## AMD-012 : Benchmark Expo vs Next.js

### Problème identifié

Expo Web peut être moins performant que Next.js pur pour une app de lecture intensive.

### Solution : Benchmark précoce

#### Plan de test (M2)

| Test | Métrique | Target | Fail threshold |
|------|----------|--------|----------------|
| **Bundle size** | JS initial | < 300KB | > 500KB |
| **TTI** | Time to Interactive | < 2s | > 4s |
| **FCP** | First Contentful Paint | < 1s | > 2s |
| **Scroll perf** | 1000 articles | 60fps | < 30fps |
| **Memory** | After 1h usage | < 200MB | > 500MB |
| **Lighthouse** | Performance score | > 80 | < 60 |

#### Décision tree

```
Si Expo Web passe les benchmarks :
  → Continuer avec Expo unifié (Web + Mobile)

Si Expo Web échoue :
  → Option A : Optimiser Expo (lazy loading, virtualization)
  → Option B : Next.js pour Web, Expo pour Mobile (2 apps)
  → Option C : PWA avec Next.js (pas d'app native)
```

#### Timeline

- **M2** : Benchmark avec 1000 articles mockés
- **M3** : Décision go/no-go sur Expo Web
- **M4** : Si pivot nécessaire, implémenter avant beta

---

## AMD-013 : Migration Inoreader Rules

### Problème identifié

Les power users Inoreader ont des dizaines de règles. Pas de chemin de migration.

### Solution : Script de migration (V1.1)

#### Format Inoreader Rules (reverse-engineered)

```json
// Export OPML Inoreader avec rules (non-standard)
{
  "rules": [
    {
      "name": "Hide crypto spam",
      "conditions": [
        {"field": "title", "operator": "contains", "value": "crypto"},
        {"field": "title", "operator": "not_contains", "value": "bitcoin"}
      ],
      "actions": ["mark_read", "move_to_folder:Spam"]
    }
  ]
}
```

#### Mapping vers FeedMind

| Inoreader | FeedMind |
|-----------|----------|
| `contains` | Regex `.*value.*` |
| `not_contains` | Regex négatif ou règle IA |
| `mark_read` | Action `hide` |
| `move_to_folder` | Action `tag` |
| `star` | Action `favorite` |

#### FR-MIGRATE-01 : Import règles Inoreader

**Actor** : User  
**Capability** : L'utilisateur peut importer ses règles depuis Inoreader.

**Acceptance Criteria** :
- [ ] Upload fichier export Inoreader
- [ ] Preview des règles converties
- [ ] Avertissement si conversion imparfaite
- [ ] Suggestion de conversion en règle IA pour les cas complexes
- [ ] Import sélectif (checkbox par règle)

---

## Résumé des Changements

### Scope V1 mis à jour

```diff
V1.0 (MVP)
+ Recherche basique (titre, source, tags)        ← AJOUTÉ
+ Concept flux prioritaires (Hot/Warm/Cold)      ← AJOUTÉ
+ Free tier sans CB (25 flux, 3 règles)          ← MODIFIÉ
+ Limites techniques explicites                  ← AJOUTÉ
+ organization_id nullable (préparation V2)      ← AJOUTÉ
- CB obligatoire pour trial                      ← SUPPRIMÉ
```

### Roadmap mise à jour

```
V1.0 - MVP
├── Core features (inchangé)
├── + Recherche basique
├── + Flux prioritaires
├── + Free tier sans CB
└── + Limites techniques

V1.1 - Intégrations + Managed IA
├── Managed IA (+10%)                 ← AVANCÉ depuis V2
├── YouTube + SponsorBlock
├── Recherche full-text
├── API Fever
├── Score pertinence
├── Migration règles Inoreader        ← AJOUTÉ
└── Rule debugger                     ← AJOUTÉ

V2 - Collaboration (inchangé mais préparé)
```

### Nouveaux documents à créer

| Document | Priorité | Status |
|----------|----------|--------|
| ADR-001: Gestion des secrets | MUST | À créer |
| Privacy Policy | MUST | À créer |
| Terms of Service | MUST | À créer |
| Runbook: Incident Response | SHOULD | À créer |
| Benchmark Report: Expo Web | SHOULD | M2 |

---

**Status du PRD après amendements : v2.1 - READY FOR IMPLEMENTATION**
