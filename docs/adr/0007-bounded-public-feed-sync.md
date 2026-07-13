# ADR 0007 — Synchronisation locale bornée de sources publiques

- Statut : accepté pour preuve locale
- Date : 2026-07-13
- Portée : `feedmind-ingest`, `feedmind-sync`, CLI et bundle Dioxus de revue

## Contexte

La première verticale Dioxus relisait un `CuratedItemExport` déterministe embarqué. Elle prouvait le contrat et le rendu, mais pas le parcours réel `source → signal → qualification → décision → export`. Le fetch historique acceptait toute URL, suivait automatiquement les redirections et matérialisait le corps complet avant de vérifier sa taille. Cette frontière ne pouvait pas être exposée comme preuve d’alpha.

## Décision

Une commande locale additive `sync-curated` importe un fichier OPML borné, synchronise séquentiellement ses flux, applique une règle explicite et écrit au plus un nouvel export client-safe.

Le parcours applique les invariants suivants :

- 32 sources, 100 items par source et 500 items inspectés au maximum, avec valeurs courantes plus basses ;
- hôtes HTTPS exacts déclarés par `--allow-host`, y compris chaque cible de redirection ;
- validation de chaque redirection avant requête ;
- résolution DNS préalable et refus des adresses locales, privées, link-local, réservées ou de documentation ;
- timeout de 10 secondes, trois redirections et corps de 2 MiB au maximum pour la politique publique ;
- lecture du corps par chunks : la limite est appliquée pendant le téléchargement, même sans `Content-Length` ;
- état de rejeu fermé ne contenant que des hashes de sources et d’items, plafonné à 2 048 items ;
- suppression d’un ancien export lorsqu’aucun nouveau signal n’est trouvé, afin de ne pas présenter une décision périmée ;
- identifiants publics dérivés d’un hash du GUID, car un GUID de flux est souvent une URL brute.

Le bundle Dioxus peut embarquer un export live validé via `FEED_RADAR_REVIEW_EXPORT`. Le build échoue si le contrat est inconnu ou unsafe. Le navigateur reste statique : aucun fetch métier, stockage local ou fournisseur IA.

## Alternatives rejetées

### Fetch arbitraire depuis le navigateur

Rejeté : SSRF côté service, CORS, absence d’auth et persistance non spécifiée. Cela aurait transformé une preuve locale en runtime public non gouverné.

### Réutiliser directement API, PostgreSQL et worker

Différé : l’API et le worker n’exposent pas encore un contrat de revue cohérent avec `CuratedItemExport` et ajouteraient auth, Redis, RLS et opérations hébergées à la première tranche.

### Autoriser automatiquement les redirections vers un domaine voisin

Rejeté : une politique suffixe ou registrable-domain élargirait silencieusement la frontière réseau. Chaque hôte reste explicite.

## Conséquences

- La synchronisation réelle est rejouable localement et l’absence de nouveau signal est un état normal.
- Les flux qui changent de CDN ou de domaine exigent une modification explicite de l’allowlist.
- La CI reste déterministe et sans réseau métier ; une preuve live est générée manuellement et datée.
- La résolution DNS avant requête réduit le risque SSRF mais ne constitue pas un pin réseau atomique. Cette limite est acceptable uniquement pour la CLI locale avec allowlist exacte. Un service hébergé devra pinner la résolution ou déléguer le fetch à un sandbox réseau avant exposition.
- Cette décision ne passe pas Radar en `public-alpha`, n’autorise aucun DNS et ne prouve pas un import interactif.

## Vérification

```bash
cargo test -p feedmind-ingest -p feedmind-sync -p feedmind-cli -p feedmind-app
./scripts/generate-live-radar-proof.sh \
  --allow-host www.clever-cloud.com \
  --allow-host clever.cloud \
  --allow-host www.clever.cloud \
  --allow-host blog.rust-lang.org
```

Le manifeste généré reste sous `target/live-radar-proof/` et porte `publication_authorized: false`.
