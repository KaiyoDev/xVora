#!/usr/bin/env python3
"""One-pass fix: rename all incorrectly prefixed dep keys in all Cargo.toml files."""
import re, subprocess
from pathlib import Path

XVORA = Path(r"D:\Kaiyo\Project\xVora")

# These are the WRONG names (from rebrand over-application) -> CORRECT names
FIX_MAP = {
    # Direct path deps that got wrong prefix
    "xvora-crash-handler": "crash-handler",
    "xvora-fast-worktree": "fast-worktree",
    "xvora-codebase-graph": "codebase-graph",
    "xvora-file-utils": "file-utils",
    "xvora-hooks-plugins-types": "hooks-plugins-types",
    "xvora-hunk-tracker": "hunk-tracker",
    "xvora-sqlite-journal": "sqlite-journal",
    "xvora-system-power": "system-power",
    "xvora-tracing-macros": "tracing-macros",
    "xvora-test-utils": "test-utils",
    "xvora-pager-diff": "pager-diff",
    "xvora-gboom": "gboom",
    "xvora-workspace-daemon": "workspace-daemon",
    "xvora-workflow": "workflow",
    # Workspace deps with wrong prefix
    "xvora-tty-utils": "tty-utils",
    "xvora-prompt-queue": "prompt-queue",
    "xvora-ratatui-textarea": "ratatui-textarea",
    "xvora-ratatui-inline": "ratatui-inline",
    "xvora-acp-lib": "acp-lib",
    "xvora-chat-state": "chat-state",
    "xvora-tool-protocol": "tool-protocol",
    "xvora-tool-runtime": "tool-runtime",
    "xvora-tool-types": "tool-types",
    "xvora-token-estimation": "token-estimation",
    "xvora-extra-ca": "extra-ca",
    "xvora-active-sessions": "active-sessions",
    "xvora-dashboard-store": "dashboard-store",
    "xvora-diag-server": "diag-server",
    "xvora-session-events": "session-events",
    "xvora-session-search": "session-search",
    "xvora-shell-terminal": "shell-terminal",
    "xvora-status-line": "status-line",
    "xvora-fuzzy-file-search": "fuzzy-file-search",
    "xvora-message-delivery-core": "message-delivery-core",
    "xvora-computer-hub-core": "computer-hub-core",
    "xvora-computer-hub-sdk": "computer-hub-sdk",
    "xvora-interjection-core": "interjection-core",
    "xvora-proto-build": "proto-build",
}

def main():
    changed = 0
    for f in sorted(XVORA.rglob("Cargo.toml")):
        content = f.read_text(encoding="utf-8")
        original = content
        for wrong, correct in FIX_MAP.items():
            content = content.replace(f'"{wrong}"', f'"{correct}"')
        if content != original:
            f.write_text(content, encoding="utf-8")
            print(f"  FIXED: {f.relative_to(XVORA)}")
            changed += 1
    print(f"\nFixed {changed} files.")

if __name__ == "__main__":
    main()
