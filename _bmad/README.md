# FeedMind - Documentation BMAD

Ce dossier contient la documentation produit créée avec la méthode [BMAD](https://github.com/Samadaeus/bmad-method).

## Structure

```
_bmad/
├── bmm/
│   ├── config.yaml              # Configuration projet BMAD
│   └── workflows/
│       └── 2-plan-workflows/
│           └── prd/
│               └── prd.md       # PRD source (workflow)
├── docs/
│   └── prd.md                   # PRD final (référence)
└── README.md                    # Ce fichier
```

## PRD Status

| Step | Nom | Status |
|------|-----|--------|
| 1 | Init | ✅ Complete |
| 2 | Discovery | ✅ Complete |
| 3 | Success Criteria | ✅ Complete |
| 4 | User Journeys | ✅ Complete |
| 5 | Domain Requirements | ✅ Complete |
| 6 | Innovation Patterns | ✅ Complete |
| 7 | Project Type | ✅ Complete |
| 8 | Scoping | ✅ Complete |
| 9 | Functional Requirements | ✅ Complete |
| 10 | Non-Functional Requirements | ✅ Complete |
| 11 | Complete | ✅ Complete |

**PRD Version:** 1.0.0  
**Date:** 2026-01-27  
**Status:** READY FOR IMPLEMENTATION

## Quick Links

- [PRD Complet](docs/prd.md)
- [Configuration BMAD](bmm/config.yaml)

## Résumé du projet

**FeedMind** - "La veille qui nourrit ton IA"

Un lecteur RSS intelligent avec :
- Règles en langage naturel (IA)
- BYOK (Bring Your Own Key)
- Multi-plateforme (Expo : Web + iOS + Android)
- Open source (AGPL-3.0)

### Stack

| Backend | Frontend | Infra |
|---------|----------|-------|
| Rust (Axum) | Expo | PostgreSQL |
| SQLx | NativeWind | Redis |
| Tokio | Expo Router | Clever Cloud |

### Roadmap

| Version | Focus |
|---------|-------|
| V1 | Produit individuel complet (5 users) |
| V1.1 | YouTube + SponsorBlock, recherche |
| V2 | Collaboration (Teams) |
| V3 | IA avancée (score pertinence) |
