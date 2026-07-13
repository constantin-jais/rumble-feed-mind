#!/usr/bin/env python3
"""Fail-closed provenance check for the vendored Libre IA web design assets."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "surfaces" / "ui" / "assets" / "libre-ia"
EXPECTED_REVISION = "a0aa89ee673f359fc3d197cd0afea7e0a802b0b9"
EXPECTED_SUBJECTS = {
    "tokens.css": (
        "design-system/tokens/tokens.css",
        "2f6a9b84145a8f899b85c03b59f3465af5cc0b685d24017fa2db7f1ce40c5e7a",
    ),
    "themes.css": (
        "design-system/tokens/themes.css",
        "7aee6919cfa2520afba22c76e0de006198e47942854f15e88d10cd196cf00421",
    ),
    "components.css": (
        "design-system/components/components.css",
        "281b3eca969d7e1d0a7b59ac25e68ef4ad24bd5c7df98378becc4b7122d8ed62",
    ),
    "contrast-report.json": (
        "dist/reports/contrast-report.json",
        "65a99cb1f0ed1a8b694ad1946bf8beea0b20617143f6a5aaf4e16736be3939e9",
    ),
}
EXPECTED_METADATA = {
    "design-system.lock.json": "e444664ac65ab2e2a886e0f9422feed7bdc674d56d8fbedf9f9053f4456c02cc",
    "manifest.json": "f21f763bd3be922afe4115789495b4a8dd29db6cc00dd1b37a99d8dd6f35a61e",
    "provenance.json": "b6808c07e51bb09386369b42a67e9f4379782962b5cbbf60dc82a4a301392855",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load(name: str) -> dict:
    path = ASSETS / name
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise SystemExit(f"invalid design-system metadata {path}: {error}") from error
    if not isinstance(value, dict):
        raise SystemExit(f"design-system metadata must be an object: {path}")
    return value


def main() -> int:
    for name, expected_digest in EXPECTED_METADATA.items():
        if sha256(ASSETS / name) != expected_digest:
            raise SystemExit(f"pinned design-system metadata drift: {name}")

    lock = load("design-system.lock.json")
    manifest = load("manifest.json")
    provenance = load("provenance.json")
    contrast = load("contrast-report.json")

    if lock.get("format") != "libre-ia.design-system-lock.v1":
        raise SystemExit("unsupported design-system lock format")
    builder = lock.get("builder", {})
    if builder.get("repository") != "https://github.com/libre-ai/client-kit":
        raise SystemExit("unexpected design-system builder repository")
    if builder.get("revision") != EXPECTED_REVISION:
        raise SystemExit("design-system builder revision drift")
    if lock.get("manifest", {}).get("sha256") != sha256(ASSETS / "manifest.json"):
        raise SystemExit("design-system manifest hash mismatch")
    if lock.get("provenance", {}).get("sha256") != sha256(ASSETS / "provenance.json"):
        raise SystemExit("design-system provenance hash mismatch")
    if manifest.get("format") != "libre-ia.design-system.v2" or manifest.get("version") != "2.0.0":
        raise SystemExit("unsupported design-system manifest")

    provenance_builder = provenance.get("predicate", {}).get("builder", {})
    if provenance_builder.get("repository") != "https://github.com/libre-ai/client-kit":
        raise SystemExit("unexpected provenance builder repository")
    if provenance_builder.get("revision") != EXPECTED_REVISION:
        raise SystemExit("provenance builder revision drift")
    subjects = {
        item.get("name"): item.get("digest", {}).get("sha256")
        for item in provenance.get("subject", [])
        if isinstance(item, dict)
    }
    for local_name, (subject_name, expected_digest) in EXPECTED_SUBJECTS.items():
        local_digest = sha256(ASSETS / local_name)
        digest = subjects.get(subject_name)
        if local_digest != expected_digest or digest != expected_digest:
            raise SystemExit(f"design-system subject mismatch: {local_name}")

    checks = contrast.get("checks")
    if not isinstance(checks, list) or not checks:
        raise SystemExit("contrast report contains no checks")
    if any(not isinstance(check, dict) or check.get("passes_wcag_aa") is not True for check in checks):
        raise SystemExit("design-system contrast report contains a failed check")

    for css_name in ("tokens.css", "themes.css", "components.css"):
        css = (ASSETS / css_name).read_text(encoding="utf-8").lower()
        if "http://" in css or "https://" in css or "@import" in css:
            raise SystemExit(f"remote or imported asset forbidden in {css_name}")

    print(
        f"design-system provenance verified: {len(EXPECTED_SUBJECTS)} assets, "
        f"{len(checks)} contrast checks, revision {EXPECTED_REVISION[:8]}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
