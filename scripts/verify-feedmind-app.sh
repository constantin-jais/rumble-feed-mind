#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bundle="${1:-$root/target/dx/feedmind-app/release/web/public}"

[[ -f "$bundle/index.html" ]] || { echo "missing index.html" >&2; exit 1; }
find "$bundle" -type f -name '*.wasm' -print -quit | grep -q . || { echo "missing WASM bundle" >&2; exit 1; }
find "$bundle" -type f -name '*.js' -print -quit | grep -q . || { echo "missing JavaScript loader" >&2; exit 1; }
for stylesheet in tokens themes components styles; do
  find "$bundle" -type f -name "*${stylesheet}*.css" -print -quit | grep -q . || {
    echo "missing local ${stylesheet} stylesheet" >&2
    exit 1
  }
done

if grep -RInE "(src|href)=[\"'](https?:)?//" "$bundle" --include='*.html' --include='*.css' --include='*.js'; then
  echo "remote runtime asset found" >&2
  exit 1
fi
if find "$bundle" -type l -print -quit | grep -q .; then
  echo "symlinks are forbidden in the bundle" >&2
  exit 1
fi

python3 - "$bundle" <<'PY'
from pathlib import Path
import sys
root = Path(sys.argv[1]).resolve()
for path in root.rglob('*'):
    if path.is_file():
        path.resolve().relative_to(root)
print(f"feedmind-app bundle verified ({sum(1 for p in root.rglob('*') if p.is_file())} files)")
PY
