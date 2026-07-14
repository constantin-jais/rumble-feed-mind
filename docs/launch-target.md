# Cible lancement produit — Rust-first

## Définition du lancement

Le lancement produit cible n'est pas une simple démo web. C'est une distribution self-hostable et reproductible du moteur de veille : CLI, API, worker, base PostgreSQL/Redis, puis surface produit Dioxus. La distribution web/native/desktop/mobile sera choisie après preuve du parcours Dioxus, pas présupposée.

Current readiness cockpit: [`product-readiness.md`](product-readiness.md).

## Release train

### RC0 — Core proof

- `feedmind-domain` extrait.
- Décisions et événements métier disponibles.
- CLI capable d'exécuter un parcours local minimal.
- Smoke sans base : `feedmind-cli opml-summary --file <file.opml>`.
- Smoke réseau : `feedmind-cli fetch-feed --url <feed-url>`.
- Smoke règles : `feedmind-cli evaluate-rule --article <article.json> --rule <rule.json>`.
- Gates Rust strictes vertes.

### RC1 — Server self-hostable

- API Axum branchée sur les ports domaine/storage.
- Worker fetch/rules opérationnel.
- Docker Compose propre PostgreSQL/Redis.
- Smoke test local documenté.

### RC2 — Première preuve produit Dioxus

- Les contrats UI Portal nécessaires sont stabilisés.
- Une surface Dioxus runnable couvre un parcours réel alimenté par les contrats Rust existants, sans données uniquement mockées.
- La commande de build/test et un smoke reproductible sont documentés.
- Les surfaces Next.js et Leptos restent des références historiques uniquement.

### RC3 — Parcours critiques Dioxus

- Dioxus couvre feeds, articles et règles avec décisions expliquées.
- Le contrat API est stabilisé.
- Les tests de parcours critiques sont reproductibles.

### RC4 — Distribution multi-plateforme

- Les cibles web/native/desktop/mobile sont sélectionnées à partir des preuves Dioxus et d'une décision dédiée.
- Artefacts versionnés et checksums pour les cibles retenues.
- Release notes et procédure rollback.

La séquence historique « Leptos puis Tauri 2 » a produit un spike archivé, puis a été remplacée par ADR 0002 ; elle ne constitue plus un release train actif.

## Parcours critique launch

1. Démarrer PostgreSQL/Redis.
2. Créer un utilisateur local.
3. Importer OPML.
4. Fetcher les feeds.
5. Afficher les articles.
6. Créer une règle.
7. Voir une décision expliquée avec evidence.
8. Exporter les données utilisateur.

## Release artifacts

La première matrice release couvre le binaire CLI :

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

Workflow : `.github/workflows/release.yml`.

## Gates launch

```bash
cargo fmt --all --check
cargo check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
agentic-harness goals report --config goals.toml
```

À ajouter avant RC1 :

```bash
cargo deny check
cargo audit
```

Le gate `cd apps/web && npm run lint` appartenait au legacy Next.js ; il n'est plus exécutable depuis l'archivage/retrait de cette surface et ne fait pas partie des gates actives.

## Issues GitHub de pilotage

- #1 — Epic refonte Rust-first totale et prochaine preuve produit Dioxus.
- #2 — Extraction `feedmind-domain` (terminée).
- #3 — Split `ingest/rules/sync/storage` (terminé).
- #4 — CLI client de référence (terminée après preuve du pipeline local et du JSON golden).
- #5 et #8 — trajectoire historique Leptos/Tauri fermée, remplacée par ADR 0002.
