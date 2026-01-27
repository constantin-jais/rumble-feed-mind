#!/usr/bin/env python3
"""Hook de validation des commandes Bash."""

import json
import re
import sys

BLOCKED_PATTERNS = [
    (r"\brm\s+-rf\s+/(?!\w)", "BLOCKED: Cannot delete root directory"),
    (
        r"\bgit\s+push\s+--force\s+(origin\s+)?(main|master)",
        "BLOCKED: Force push to main/master",
    ),
    (r"\bgit\s+reset\s+--hard", "BLOCKED: Hard reset requires confirmation"),
    (
        r"(password|secret|api_key|token)\s*=\s*['\"][^'\"]+['\"]",
        "BLOCKED: Potential secret",
    ),
    (r"\bcargo\s+publish\s+--no-verify", "BLOCKED: Publishing without verification"),
]

WARNING_PATTERNS = [
    (r"\bgrep\b(?!.*\|)", "WARNING: Consider using 'rg' (ripgrep)"),
    (r"\bcat\s+[^|]+$", "WARNING: Consider using the Read tool"),
]


def main():
    try:
        data = json.load(sys.stdin)
        command = data.get("tool_input", {}).get("command", "")
        if not command:
            sys.exit(0)

        for pattern, message in BLOCKED_PATTERNS:
            if re.search(pattern, command, re.IGNORECASE):
                print(message, file=sys.stderr)
                sys.exit(2)

        for pattern, message in WARNING_PATTERNS:
            if re.search(pattern, command, re.IGNORECASE):
                print(message, file=sys.stderr)

        sys.exit(0)
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
