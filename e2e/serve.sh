#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bundle="$root/target/dx/feedmind-app/release/web/public"
serve_root="$root/target/feedmind-app-e2e"
[[ -f "$bundle/index.html" ]] || "$root/scripts/build-feedmind-app.sh"
rm -rf "$serve_root"
mkdir -p "$serve_root/app"
cp -R "$bundle"/. "$serve_root/app"/
exec python3 -m http.server 8934 --bind 127.0.0.1 --directory "$serve_root"
