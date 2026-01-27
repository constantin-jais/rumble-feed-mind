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

---

## AMD-014 : Scope V1 Révisé (Web-Only)

### Problème identifié

Le scope V1 actuel (Web + Mobile Expo + Admin + IA complète + Explicabilité) est irréaliste pour 12 semaines avec 1-2 développeurs. Risque de délai 16-20 semaines et burnout équipe.

### Solution : MVP V1 Web-Only

**Principe** : Livrer un MVP fonctionnel et stable plutôt qu'un produit incomplet sur toutes les plateformes.

#### Scope V1 Révisé (10 semaines)

```
✅ INCLUS DANS V1
─────────────────────────────────────────────────────────────────
• Web Next.js (pas Expo Web)
• Import/Export OPML
• Gestion flux et dossiers
• Règles regex uniquement
• Lecture articles (Readability)
• Recherche basique (titre, source, tags) - AMD-002
• Flux prioritaires (Hot/Warm/Cold) - AMD-006
• Login/signup simple (email + password)
• BYOK config (UI prête, IA désactivée ou mode basique)
• Dark mode
• Raccourcis clavier (j/k, m, s, etc.)

❌ DÉPLACÉ VERS V1.1
─────────────────────────────────────────────────────────────────
• Mobile Expo (iOS/Android)
• Admin dashboard
• Règles IA complètes (langage naturel)
• Explicabilité IA (raisons détaillées)
• Full-text search (contenu)
• Managed IA - déjà en V1.1 (AMD-005)
• OAuth (GitHub, Google)

❌ RESTE EN V2
─────────────────────────────────────────────────────────────────
• Collaboration équipe
• Flux partagés
• Webhooks
• API publique
```

#### Timeline Révisée

| Phase | Semaines | Contenu |
|-------|----------|---------|
| **Phase 0** | S1-S2 | Validation marché (voir AMD-015) |
| **Phase 1** | S3-S12 | MVP Web-Only |
| **Phase 2** | S13-S14 | Beta externe (20-50 users) |
| **Phase 3** | S15+ | V1.1 (Mobile, IA, Admin) |

#### Justification

1. **Next.js vs Expo Web** : Next.js est mature et performant pour apps lecture-intensive. Expo Web ajoute un risque non-nécessaire pour V1.
2. **IA en V1.1** : Le différenciateur principal peut attendre 4-6 semaines. Les règles regex suffisent pour valider le core product.
3. **Mobile en V1.1** : Les power users (cible V1) utilisent principalement desktop.

#### Impact sur FRs

```diff
FR-MOBILE-* → Déplacés vers V1.1
FR-RULE-AI-* → Déplacés vers V1.1 (sauf config BYOK UI)
FR-ADMIN-* → Déplacés vers V1.1
FR-EXPLAIN-* → Déplacés vers V1.1
```

#### Critères de succès V1 révisés

| Métrique | Target V1 | Mesure |
|----------|-----------|--------|
| **Build fonctionnel** | 100% features V1 | Checklist |
| **Dogfooding** | 5/5 users actifs quotidiennement | Logs |
| **Performance** | p95 < 200ms | Prometheus |
| **Uptime** | > 99% | Monitoring |
| **Import OPML 1000 flux** | < 2 minutes | Test manuel |

---

## AMD-015 : Phase 0 Validation Marché

### Problème identifié

Le dogfooding avec 5 utilisateurs internes ne valide pas le product-market fit. Risque de construire un produit que personne n'achète.

### Solution : Phase de validation avant développement

#### Phase 0 (2 semaines)

```
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 0 : VALIDATION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  SEMAINE 1 : RECHERCHE                                         │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  □ User Interviews (10 power users RSS)                        │
│    ├── 5 utilisateurs Inoreader                                │
│    ├── 3 utilisateurs Feedly                                   │
│    └── 2 utilisateurs Miniflux/self-hosted                     │
│                                                                 │
│  Questions clés :                                               │
│    • Combien de temps passez-vous à trier vos articles ?       │
│    • Quelles règles avez-vous créées ? Limites rencontrées ?   │
│    • Payeriez-vous 5€/mois pour des règles IA ?                │
│    • Qu'est-ce qui vous ferait quitter Inoreader/Feedly ?      │
│                                                                 │
│  □ Étude TAM (Total Addressable Market)                        │
│    ├── Combien de power users RSS en 2026 ?                    │
│    ├── Tendance : croissance ou déclin ?                       │
│    └── Segments : dev, analystes, journalistes, autres ?       │
│                                                                 │
│  □ Competitive Analysis                                         │
│    ├── Feedly AI features actuelles                            │
│    ├── Inoreader roadmap publique                              │
│    └── Timeline estimée si Inoreader copie l'IA                │
│                                                                 │
│  SEMAINE 2 : DÉCISION                                          │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  □ Financial Model                                              │
│    ├── CAC estimé (coût acquisition client)                    │
│    ├── CLV estimé (lifetime value)                             │
│    ├── Break-even : combien de users ?                         │
│    └── Projection Y1, Y2, Y3                                   │
│                                                                 │
│  □ Benchmark Expo Web (optionnel, peut être fait en S3)        │
│    ├── Prototype 1000 articles                                 │
│    ├── Mesurer : bundle, TTI, scroll FPS                       │
│    └── Décision : Expo Web vs Next.js                          │
│                                                                 │
│  □ Decision Point : GO / PIVOT / STOP                          │
│                                                                 │
│  GO si :                                                        │
│    ✓ 7/10 interviewés paieraient 5€/mois pour règles IA       │
│    ✓ TAM > 50k power users RSS actifs                          │
│    ✓ CAC < CLV avec marge > 50%                                │
│    ✓ Pas de signal imminent de copie par Inoreader             │
│                                                                 │
│  PIVOT si :                                                     │
│    ⚠️ Intérêt mais pricing trop bas (tester 10€/mois ?)        │
│    ⚠️ TAM B2C limité mais B2B intéressant (équipes veille)     │
│                                                                 │
│  STOP si :                                                      │
│    ✗ < 3/10 paieraient (pas de demande)                        │
│    ✗ Inoreader annonce feature similaire                       │
│    ✗ CAC > CLV (business non-viable)                           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Livrables Phase 0

| Livrable | Format | Objectif |
|----------|--------|----------|
| **Interview Notes** | Google Doc | Insights qualitatives |
| **TAM Analysis** | Spreadsheet | Taille marché estimée |
| **Competitive Brief** | 2 pages | Risques concurrence |
| **Financial Model** | Spreadsheet | CAC/CLV/break-even |
| **Go/Pivot/Stop Decision** | 1 page memo | Décision documentée |

#### Impact Timeline

```
AVANT (v2.0)
─────────────────────────────────────────────────────────────────
S1 ────────────────────────────────────────────────────────► S12
        MVP Development (12 semaines)

APRÈS (v2.1)
─────────────────────────────────────────────────────────────────
S1 ─► S2     S3 ──────────────────────────────────────────► S12
Phase 0      MVP Development (10 semaines)
Validation
```

---

## AMD-016 : Stratégie Go-To-Market

### Problème identifié

Aucune stratégie d'acquisition au-delà de "post sur Hacker News". Risque de rester à 5 utilisateurs indéfiniment.

### Solution : Plan GTM structuré

#### Stratégie d'Acquisition

```
┌─────────────────────────────────────────────────────────────────┐
│                    GO-TO-MARKET STRATEGY                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  PHASE 1 : BETA PRIVÉE (S13-S14)                               │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Objectif : 20-50 utilisateurs externes                        │
│                                                                 │
│  Sources :                                                      │
│  ├── Twitter/X (RSS enthusiasts)                               │
│  ├── Mastodon (tech community)                                 │
│  ├── r/rss subreddit                                           │
│  ├── Indie Hackers                                             │
│  └── Network personnel                                         │
│                                                                 │
│  Messaging :                                                    │
│  "Looking for power RSS users to test an open-source          │
│   reader with AI-powered natural language filtering.           │
│   Free beta access + influence on roadmap."                    │
│                                                                 │
│  Success criteria :                                             │
│  ├── Churn < 5%/semaine                                        │
│  ├── NPS > 30                                                  │
│  └── Feature requests documentés                               │
│                                                                 │
│  PHASE 2 : LAUNCH PUBLIC (S15+)                                │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Day 1 : Hacker News + Product Hunt (simultané)                │
│  ├── HN : "Show HN: FeedMind - RSS reader with AI rules"       │
│  ├── PH : Launch avec screenshots, démo vidéo                  │
│  └── Twitter thread explicatif                                 │
│                                                                 │
│  Week 1+ : Content Marketing                                   │
│  ├── Blog post : "Why I built an AI-powered RSS reader"       │
│  ├── Blog post : "10 advanced RSS filtering rules"            │
│  ├── Blog post : "BYOK: Why we don't charge for AI"           │
│  └── YouTube : Demo walkthrough (5 min)                        │
│                                                                 │
│  Ongoing : Community Building                                   │
│  ├── Discord server (support + feedback)                       │
│  ├── GitHub Discussions (features, bugs)                       │
│  ├── Changelog public (nouvelles features)                     │
│  └── Newsletter mensuelle                                      │
│                                                                 │
│  CANAUX PRIORITAIRES                                            │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Tier 1 (Focus principal) :                                    │
│  ├── Hacker News                                               │
│  ├── Reddit (r/rss, r/selfhosted, r/productivity)             │
│  └── Twitter/X tech community                                  │
│                                                                 │
│  Tier 2 (Secondaire) :                                         │
│  ├── Product Hunt                                              │
│  ├── Indie Hackers                                             │
│  ├── Dev.to / Hashnode                                         │
│  └── Mastodon / Fediverse                                      │
│                                                                 │
│  Tier 3 (Si budget) :                                          │
│  ├── Sponsoring newsletters tech                               │
│  ├── YouTube sponsoring (tech reviewers)                       │
│  └── Google Ads (keywords RSS)                                 │
│                                                                 │
│  BUDGET MARKETING                                               │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Phase 1-2 : 0€ (organic only)                                 │
│  Phase 3+ : 100-500€/mois si CAC < CLV                         │
│                                                                 │
│  Channels payants envisageables :                               │
│  ├── Newsletter sponsoring : 50-200€/insertion                 │
│  ├── Google Ads : 0.50-2€/clic                                 │
│  └── Twitter Ads : 1-3€/engagement                             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Métriques GTM

| Métrique | Target S15 | Target S20 | Target S30 |
|----------|------------|------------|------------|
| **Signups** | 100 | 500 | 2000 |
| **DAU** | 30 | 150 | 500 |
| **Pro conversions** | 5 | 50 | 200 |
| **MRR** | 25€ | 250€ | 1000€ |
| **Churn mensuel** | < 10% | < 8% | < 5% |

#### Content Calendar (Mois 1)

| Semaine | Content | Channel |
|---------|---------|---------|
| S15 | Launch post + demo video | HN, PH, Twitter |
| S16 | "Why I built FeedMind" | Blog, HN |
| S17 | "10 Advanced RSS Rules" | Blog, Reddit |
| S18 | Tutorial video | YouTube, Twitter |

---

## AMD-017 : Free Tier Révisé (100 flux)

### Problème identifié

Le Free tier actuel (25 flux) empêche les power users (800+ flux) de tester FeedMind. Ils ne peuvent même pas importer une fraction significative de leurs flux pour évaluer le produit.

### Solution : Free tier plus généreux

#### Comparaison

| Limite | AVANT (v2.0) | APRÈS (v2.1) | Justification |
|--------|--------------|--------------|---------------|
| **Flux** | 25 | 100 | Permet test réel par power users |
| **Articles stockés** | 500 | 2000 | 20 articles × 100 flux = raisonnable |
| **Règles regex** | 3 | 10 | Permet tester la feature sérieusement |
| **Tags** | 10 | 20 | Organisation minimale |
| **Cache images** | 24h | 24h | Inchangé |

#### Impact Coûts

```
AVANT (25 flux Free)
─────────────────────────────────────────────────────────────────
• Storage : 25 flux × 20 articles × 10KB = 5MB/user
• Si 1000 Free users : 5GB stockage
• Coût : ~0.05€/mois

APRÈS (100 flux Free)
─────────────────────────────────────────────────────────────────
• Storage : 100 flux × 20 articles × 10KB = 20MB/user
• Si 1000 Free users : 20GB stockage
• Coût : ~0.20€/mois

Delta : +0.15€/mois pour 1000 users = négligeable
```

#### Expérience utilisateur

```
BEFORE : Thomas (800 flux) arrive
─────────────────────────────────────────────────────────────────
1. Crée compte Free
2. Tente import OPML (800 flux)
3. "Erreur : limite 25 flux dépassée"
4. Abandonne, retourne sur Inoreader

AFTER : Thomas (800 flux) arrive
─────────────────────────────────────────────────────────────────
1. Crée compte Free
2. Import OPML → "100 flux importés sur 800"
3. Teste pendant 3 jours avec ses flux principaux
4. Convaincu → upgrade Pro Trial pour import complet
```

#### Alternative considérée

**Option B : Import illimité pendant 7 jours**
- Pro : Power users peuvent tout tester
- Con : 7 jours trop court pour évaluer vraiment
- Con : Complexité UI (timer, downgrade, etc.)

**Décision** : Free 100 flux (option retenue) - plus simple et suffisant.

#### FRs impactés

```diff
- FR-TIER-FREE : 25 flux, 500 articles, 3 règles
+ FR-TIER-FREE : 100 flux, 2000 articles, 10 règles
```

---

## AMD-018 : Métriques Succès Détaillées

### Problème identifié

Les métriques actuelles ("5/5 DAU") ne mesurent pas la satisfaction ni la rétention. Impossible de savoir si le produit résout vraiment le problème.

### Solution : Framework de métriques complet

#### Métriques V1 Révisées

```
┌─────────────────────────────────────────────────────────────────┐
│                    SUCCESS METRICS V1                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ENGAGEMENT (Le produit est-il utilisé ?)                      │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Métrique         │ Target   │ Red Flag │ Mesure               │
│  ─────────────────┼──────────┼──────────┼────────────────────  │
│  DAU/MAU ratio    │ > 40%    │ < 20%    │ Analytics            │
│  Sessions/jour    │ ≥ 2      │ < 1      │ Analytics            │
│  Articles lus/j   │ ≥ 20     │ < 5      │ DB query             │
│  Durée session    │ > 15min  │ < 5min   │ Analytics            │
│                                                                 │
│  SATISFACTION (Le produit résout-il le problème ?)             │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Métrique         │ Target   │ Red Flag │ Mesure               │
│  ─────────────────┼──────────┼──────────┼────────────────────  │
│  NPS              │ > 50     │ < 20     │ Survey in-app        │
│  Recommandation   │ > 80%    │ < 50%    │ Survey               │
│  Support tickets  │ < 5/mois │ > 20     │ Zendesk/Discord      │
│  Feature requests │ > 10     │ < 3      │ GitHub issues        │
│                                                                 │
│  RÉTENTION (Les users reviennent-ils ?)                        │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Métrique         │ Target   │ Red Flag │ Mesure               │
│  ─────────────────┼──────────┼──────────┼────────────────────  │
│  D1 retention     │ > 80%    │ < 50%    │ Cohort analysis      │
│  D7 retention     │ > 60%    │ < 30%    │ Cohort analysis      │
│  D30 retention    │ > 40%    │ < 20%    │ Cohort analysis      │
│  Churn mensuel    │ < 5%     │ > 15%    │ Subscription data    │
│                                                                 │
│  ADOPTION FEATURES (Les features clés sont-elles utilisées ?)  │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Métrique         │ Target   │ Red Flag │ Mesure               │
│  ─────────────────┼──────────┼──────────┼────────────────────  │
│  Règles créées    │ ≥ 5/user │ < 2      │ DB query             │
│  Flux prioritaires│ ≥ 10/usr │ < 3      │ DB query             │
│  Recherche usage  │ > 50%    │ < 10%    │ Analytics            │
│  Raccourcis kbd   │ > 30%    │ < 5%     │ Analytics            │
│                                                                 │
│  PERFORMANCE (Le système est-il fiable ?)                      │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Métrique         │ Target   │ Red Flag │ Mesure               │
│  ─────────────────┼──────────┼──────────┼────────────────────  │
│  API p95 latency  │ < 200ms  │ > 500ms  │ Prometheus           │
│  Uptime           │ > 99.5%  │ < 99%    │ Monitoring           │
│  Error rate       │ < 1%     │ > 5%     │ Logs                 │
│  Feed refresh     │ < 5min   │ > 15min  │ Job metrics          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Métriques IA (V1.1+)

| Métrique | Target | Red Flag | Mesure |
|----------|--------|----------|--------|
| **Précision règles IA** | > 85% | < 70% | Feedback "correct/incorrect" |
| **Faux positifs** | < 15% | > 30% | Articles restaurés / masqués |
| **Temps évaluation** | < 5s/batch | > 15s | Job metrics |
| **Adoption IA** | > 60% Pro | < 30% | DB query |

#### Dashboard Metrics

```
┌─────────────────────────────────────────────────────────────────┐
│  📊 FeedMind Metrics Dashboard                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  TODAY                          THIS WEEK                       │
│  ───────────────────────────    ───────────────────────────    │
│  DAU: 5/5 ✅                    NPS: 62 ✅                      │
│  Sessions: 12                   Retention D7: 100% ✅           │
│  Articles read: 156             New rules: 23                   │
│  Avg session: 24min ✅          Support tickets: 2 ✅           │
│                                                                 │
│  PERFORMANCE                    HEALTH                          │
│  ───────────────────────────    ───────────────────────────    │
│  API p95: 145ms ✅              Uptime: 99.8% ✅                │
│  Feed refresh: 3.2min ✅        Error rate: 0.3% ✅             │
│  DB connections: 12/50          Redis memory: 45%               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Survey NPS (in-app, après 7 jours)

```
┌─────────────────────────────────────────────────────────────────┐
│  How likely are you to recommend FeedMind to a colleague?       │
│                                                                 │
│  0   1   2   3   4   5   6   7   8   9   10                    │
│  ○   ○   ○   ○   ○   ○   ○   ○   ○   ○   ○                     │
│  ├───────────────┼───────────────┼───────────────┤              │
│  Not at all      Neutral         Extremely                      │
│                                                                 │
│  [Skip] [Submit]                                                │
└─────────────────────────────────────────────────────────────────┘
```

---

## AMD-019 : KMS Obligatoire pour Secrets

### Problème identifié

AMD-004 décrit le chiffrement des secrets mais laisse la master key en env var Clever Cloud. C'est un single point of failure : si la master key fuite, tous les secrets sont compromis.

### Solution : KMS obligatoire

#### Architecture Secrets Révisée

```
┌─────────────────────────────────────────────────────────────────┐
│                    SECRETS ARCHITECTURE v2                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  AVANT (AMD-004) - INSUFFISANT                                 │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Environment Variables (Clever Cloud)                          │
│  └── MASTER_KEY_V1=base64(...)  ← Risque : leak = game over    │
│                                                                 │
│  APRÈS (AMD-019) - RECOMMANDÉ                                  │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Option A : HashiCorp Vault (Self-hosted)                      │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Vault Server (separate VM)                             │   │
│  │  ├── Master key stored in Vault                         │   │
│  │  ├── Auto-unseal with cloud KMS                         │   │
│  │  ├── Audit logging enabled                              │   │
│  │  └── Access policies per service                        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                        │                                        │
│                        ▼                                        │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  FeedMind API                                           │   │
│  │  ├── Request master key from Vault on startup           │   │
│  │  ├── Cache in memory (never disk)                       │   │
│  │  ├── Refresh every 1h                                   │   │
│  │  └── Graceful degradation if Vault unavailable          │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Option B : Clever Cloud Vault (Managed)                       │
│  ─────────────────────────────────────────────────────────────  │
│  Si Clever Cloud propose un service Vault/KMS :                │
│  • Utiliser directement                                        │
│  • Moins d'infra à gérer                                       │
│  • Vérifier compliance RGPD                                    │
│                                                                 │
│  Option C : Minimum Viable Security (V1 only)                  │
│  ─────────────────────────────────────────────────────────────  │
│  Si KMS impossible pour V1 :                                   │
│  1. Master key en env var (comme AMD-004)                      │
│  2. MAIS : audit trail obligatoire                             │
│  3. ET : incident playbook testé                               │
│  4. ET : migration vers KMS planifiée pour V1.1                │
│                                                                 │
│  EXIGENCES NON-NÉGOCIABLES                                     │
│  ─────────────────────────────────────────────────────────────  │
│                                                                 │
│  Quelle que soit l'option :                                    │
│  ✓ Master key jamais en code source                            │
│  ✓ Master key jamais loggée                                    │
│  ✓ Rotation possible sans downtime                             │
│  ✓ Audit trail de tous les accès                               │
│  ✓ Incident playbook documenté et testé                        │
│  ✓ Backup offline (papier) de la master key                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Incident Playbook : Master Key Compromised

```
┌─────────────────────────────────────────────────────────────────┐
│  🚨 INCIDENT : MASTER KEY COMPROMISED                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  DÉTECTION                                                      │
│  ─────────────────────────────────────────────────────────────  │
│  • Alerte : accès non-autorisé au KMS/Vault                    │
│  • Alerte : env var exposée dans logs/repo                     │
│  • Rapport : utilisateur signale activité suspecte             │
│                                                                 │
│  RÉPONSE IMMÉDIATE (< 15 minutes)                              │
│  ─────────────────────────────────────────────────────────────  │
│  1. □ Confirmer la compromission (pas de faux positif)         │
│  2. □ Générer nouvelle master key (MASTER_KEY_V2)              │
│  3. □ Déployer nouvelle key en production                      │
│  4. □ Révoquer ancienne key du KMS/env vars                    │
│                                                                 │
│  MITIGATION (< 1 heure)                                        │
│  ─────────────────────────────────────────────────────────────  │
│  5. □ Re-chiffrer toutes les clés API utilisateurs             │
│       (batch job avec nouvelle master key)                     │
│  6. □ Invalider toutes les sessions actives                    │
│  7. □ Forcer re-authentification de tous les users             │
│                                                                 │
│  COMMUNICATION (< 2 heures)                                    │
│  ─────────────────────────────────────────────────────────────  │
│  8. □ Email à tous les utilisateurs :                          │
│       "Security incident - please rotate your API keys"        │
│  9. □ Blog post / status page update                           │
│  10.□ Si RGPD applicable : notifier CNIL (72h)                 │
│                                                                 │
│  POST-INCIDENT (< 1 semaine)                                   │
│  ─────────────────────────────────────────────────────────────  │
│  11.□ Root cause analysis (comment c'est arrivé)               │
│  12.□ Mesures préventives (comment éviter)                     │
│  13.□ Mise à jour du playbook si nécessaire                    │
│  14.□ Communication finale aux utilisateurs                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

#### Impact sur Constitution

```diff
Article 8: Sécurité
+ | Master key storage | KMS obligatoire (Vault/Cloud KMS) |
+ | Si KMS impossible V1 | Env var + audit trail + playbook |
+ | Audit trail | Tous les decrypt loggés (tamper-evident) |
+ | Incident playbook | Documenté, testé trimestriellement |
```

#### Timeline KMS

| Phase | Action | Deadline |
|-------|--------|----------|
| **V1** | Env var + audit trail + playbook testé | M1 |
| **V1.1** | Migration vers HashiCorp Vault | M3 |
| **V2** | Vault HA (haute disponibilité) | M6 |

---

## Résumé des Amendements v2.1 (mise à jour)

### Nouveaux Amendements

| ID | Catégorie | Priorité | Description |
|----|-----------|----------|-------------|
| AMD-014 | Scope | MUST-FIX | Scope V1 révisé (Web-only, 10 semaines) |
| AMD-015 | Process | MUST-FIX | Phase 0 validation marché (2 semaines) |
| AMD-016 | Business | SHOULD-FIX | Stratégie Go-To-Market |
| AMD-017 | Business | MUST-FIX | Free tier révisé (100 flux) |
| AMD-018 | Metrics | SHOULD-FIX | Métriques succès détaillées |
| AMD-019 | Sécurité | MUST-FIX | KMS obligatoire pour secrets |

### Roadmap Finale

```
PHASE 0 (S1-S2) : Validation Marché
├── User interviews (10)
├── TAM analysis
├── Financial model
└── Go/Pivot/Stop decision

PHASE 1 (S3-S12) : MVP Web-Only
├── Core features
├── Règles regex
├── Free tier 100 flux
└── Dogfooding 5 users

PHASE 2 (S13-S14) : Beta Externe
├── 20-50 beta testers
├── Mesurer NPS, retention
└── Itérer selon feedback

PHASE 3 (S15+) : V1.1 + Launch
├── Mobile (Expo iOS/Android)
├── Règles IA + explicabilité
├── KMS (HashiCorp Vault)
├── Launch public (HN, PH)
└── Content marketing
```

---

**Status du PRD après amendements : v2.1.1 - READY FOR PHASE 0 VALIDATION**
