#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
opml="$root/examples/demo.opml"
rule="$root/examples/demo-rule.json"
proof_dir="$root/target/live-radar-proof"
allowed_hosts=()

usage() {
  cat <<'EOF'
Usage: scripts/generate-live-radar-proof.sh [options]
  --opml PATH          OPML source set (default: examples/demo.opml)
  --rule PATH          explicit qualification rule (default: examples/demo-rule.json)
  --proof-dir PATH     generated, ignored evidence directory
  --allow-host HOST    exact HTTPS host allowed for fetch/redirect; repeatable
EOF
}

while (($#)); do
  case "$1" in
    --opml) opml="$2"; shift 2 ;;
    --rule) rule="$2"; shift 2 ;;
    --proof-dir) proof_dir="$2"; shift 2 ;;
    --allow-host) allowed_hosts+=("$2"); shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
done

if ((${#allowed_hosts[@]} == 0)); then
  echo "at least one --allow-host is required" >&2
  exit 2
fi

rm -rf "$proof_dir"
mkdir -p "$proof_dir"
export_path="$proof_dir/curated-export.json"
state_path="$proof_dir/sync-state.json"
summary_path="$proof_dir/sync-summary.json"

sync_args=(
  --opml "$opml"
  --rule "$rule"
  --output "$export_path"
  --state "$state_path"
  --max-sources 8
  --max-items-per-source 50
  --max-total-items 200
)
for host in "${allowed_hosts[@]}"; do
  sync_args+=(--allow-host "$host")
done

(
  cd "$root"
  cargo run --locked -q -p feedmind-cli -- sync-curated "${sync_args[@]}" >"$summary_path"
  cargo run --locked -q -p feedmind-cli -- validate-curated-export --file "$export_path" >/dev/null
)

if [[ "$(jq -r '.status' "$summary_path")" != "ready" ]] || [[ ! -s "$export_path" ]]; then
  echo "live synchronization produced no new explained signal" >&2
  exit 1
fi

export_abs="$(cd "$(dirname "$export_path")" && pwd)/$(basename "$export_path")"
(
  cd "$root"
  FEED_RADAR_REVIEW_EXPORT="$export_abs" \
  FEED_RADAR_REQUIRE_LIVE_EXPORT=1 \
    cargo test --locked -p feedmind-app
  FEED_RADAR_REVIEW_EXPORT="$export_abs" \
  FEED_RADAR_REQUIRE_LIVE_EXPORT=1 \
    ./scripts/build-feedmind-app.sh
  npm ci --prefix e2e
  FEED_RADAR_EXPECT_LIVE=1 npm --prefix e2e test
)

python3 - "$root" "$proof_dir" <<'PY'
from datetime import datetime, timezone
from hashlib import sha256
import json
from pathlib import Path
import sys

root = Path(sys.argv[1])
proof = Path(sys.argv[2])
bundle = root / "target/dx/feedmind-app/release/web/public"
summary = json.loads((proof / "sync-summary.json").read_text())
state = json.loads((proof / "sync-state.json").read_text())
export = json.loads((proof / "curated-export.json").read_text())

def file_hash(path: Path) -> str:
    return "sha256:" + sha256(path.read_bytes()).hexdigest()

def tree_hash(path: Path) -> str:
    digest = sha256()
    for item in sorted(p for p in path.rglob("*") if p.is_file()):
        digest.update(item.relative_to(path).as_posix().encode())
        digest.update(b"\0")
        digest.update(item.read_bytes())
        digest.update(b"\0")
    return "sha256:" + digest.hexdigest()

manifest = {
    "format": "feed-radar.live-sync-proof.v0.1",
    "generated_at": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
    "claim": "local-live-sync-proof",
    "publication_authorized": False,
    "source_url_hashes": state["source_hashes"],
    "source_count": summary["sources_imported"],
    "sources_fetched": summary["sources_fetched"],
    "items_inspected": summary["items_inspected"],
    "export_id": export["export_id"],
    "export_hash": file_hash(proof / "curated-export.json"),
    "state_hash": file_hash(proof / "sync-state.json"),
    "bundle_hash": tree_hash(bundle),
    "browser_engines": ["chromium", "firefox", "webkit"],
    "constraints": {
        "explicit_host_allowlist": True,
        "https_only": True,
        "bounded_response_body": True,
        "payload_minimized_state": True,
        "browser_network_requests": False,
        "browser_storage": False,
    },
}
(proof / "evidence-manifest.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n")
print(json.dumps(manifest, indent=2, ensure_ascii=False))
PY

echo "Live Radar proof: $proof_dir/evidence-manifest.json"
