# feedmind-web-rs

Surface web Rust-first de `rumble-feed-mind`.

## Statut

Squelette Leptos SSR compilable. Cette app remplace progressivement `apps/web` comme cible durable.

## Parcours à migrer depuis Next legacy

1. Import OPML
2. Liste des flux
3. Liste des articles
4. Détail article
5. Édition des règles
6. Affichage des décisions avec evidence
7. Export snapshot

## Gate

```bash
cargo check -p feedmind-web-rs
```
