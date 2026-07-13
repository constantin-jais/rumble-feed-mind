#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cli="${DIOXUS_CLI:-}"

if [[ -z "$cli" && -x "${HOME}/.cargo/bin/dx" ]]; then
  cli="${HOME}/.cargo/bin/dx"
elif [[ -z "$cli" ]]; then
  cli="$(command -v dx || true)"
fi

if [[ -z "$cli" ]] || ! "$cli" --version 2>/dev/null | grep -q '^dioxus 0\.7\.9 '; then
  echo "dioxus-cli 0.7.9 is required (cargo install dioxus-cli --version 0.7.9 --locked)" >&2
  exit 1
fi

python3 "$root/scripts/verify-design-system.py"

bundle="$root/target/dx/feedmind-app/release/web/public"
rm -rf "$bundle"
(
  cd "$root/surfaces/ui"
  "$cli" build --release --web --locked
)
"$root/scripts/verify-feedmind-app.sh" "$bundle"
echo "Feed Radar local product bundle: $bundle"
