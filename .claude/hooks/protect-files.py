#!/usr/bin/env python3
"""Hook de protection des fichiers sensibles."""

import json
import sys

PROTECTED_PATTERNS = [
    ".env",
    ".env.local",
    ".env.production",
    "*.pem",
    "*.key",
    "*.p12",
    "credentials.json",
    "secrets.json",
    ".git/",
    "node_modules/",
    "target/",
    "Cargo.lock",
    "bun.lockb",
]
ALLOWED_FILES = [".env.example", ".env.template", "Cargo.toml", "package.json"]


def is_protected(file_path: str) -> tuple[bool, str]:
    if not file_path:
        return False, ""
    for allowed in ALLOWED_FILES:
        if file_path.endswith(allowed):
            return False, ""
    for pattern in PROTECTED_PATTERNS:
        if pattern.startswith("*"):
            if file_path.endswith(pattern[1:]):
                return True, f"Protected: {pattern}"
        elif pattern.endswith("/"):
            if pattern[:-1] in file_path:
                return True, f"Protected directory: {pattern}"
        elif pattern in file_path:
            return True, f"Protected: {pattern}"
    return False, ""


def main():
    try:
        data = json.load(sys.stdin)
        file_path = data.get("tool_input", {}).get("file_path", "")
        protected, reason = is_protected(file_path)
        if protected:
            output = {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": reason,
                }
            }
            print(json.dumps(output))
        sys.exit(0)
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
