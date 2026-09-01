#!/usr/bin/env python3
"""Sync workspace.dependencies from grok-build to xVora, applying xai→xvora mapping."""
import re
from pathlib import Path

GROK = Path(r"D:\Kaiyo\Project\grok-build")
XVORA = Path(r"D:\Kaiyo\Project\xVora")

# Map grok-build crate names → xVora crate names
CRATE_MAP = {
    "xai-acp-lib": "acp-lib", "xai-agent-lifecycle": "agent-lifecycle",
    "xai-chat-state": "chat-state", "xai-codebase-graph": "codebase-graph",
    "xai-crash-handler": "crash-handler", "xai-fast-worktree": "fast-worktree",
    "xai-file-utils": "file-utils", "xai-fsnotify": "xvora-fsnotify",
    "xai-gix-status": "xvora-gix-status", "xai-grok-active-sessions": "active-sessions",
    "xai-grok-agent": "xvora-agent", "xai-grok-announcements": "xvora-announcements",
    "xai-grok-auth": "xvora-auth", "xai-grok-bundle": "xvora-bundle",
    "xai-grok-config": "xvora-config", "xai-grok-config-types": "xvora-config-types",
    "xai-grok-dashboard-store": "dashboard-store", "xai-grok-diag-server": "diag-server",
    "xai-grok-env": "xvora-env", "xai-grok-extra-ca": "extra-ca",
    "xai-grok-foreign-sessions": "foreign-sessions", "xai-grok-http": "xvora-http",
    "xai-grok-markdown": "xvora-markdown", "xai-grok-markdown-core": "xvora-markdown-core",
    "xai-grok-mcp": "xvora-mcp", "xai-grok-memory": "xvora-memory",
    "xai-grok-mermaid": "xvora-mermaid", "xai-grok-models": "xvora-models",
    "xai-grok-pager": "xvora-pager", "xai-grok-pager-bin": "xvora-pager-bin",
    "xai-grok-pager-minimal": "xvora-pager-minimal",
    "xai-grok-pager-pty-harness": "xvora-pager-pty-harness",
    "xai-grok-pager-render": "xvora-pager-render", "xai-grok-paths": "xvora-paths",
    "xai-grok-plugin-marketplace": "xvora-plugin-marketplace",
    "xai-grok-sampler": "xvora-sampler", "xai-grok-sampling-types": "xvora-sampling-types",
    "xai-grok-sandbox": "xvora-sandbox", "xai-grok-secrets": "xvora-secrets",
    "xai-grok-session-events": "session-events", "xai-grok-session-search": "session-search",
    "xai-grok-shared": "xvora-shared", "xai-grok-shell": "xvora-shell",
    "xai-grok-shell-base": "xvora-shell-base",
    "xai-grok-shell-session-support": "xvora-shell-session-support",
    "xai-grok-shell-terminal": "shell-terminal", "xai-grok-status-line": "status-line",
    "xai-grok-telemetry": "xvora-telemetry", "xai-grok-test-support": "xvora-test-support",
    "xai-grok-tools": "xvora-tools", "xai-grok-tools-api": "xvora-tools-api",
    "xai-grok-version": "xvora-version", "xai-grok-workspace": "xvora-workspace",
    "xai-grok-workspace-types": "xvora-workspace-types",
    "xai-gboom": "xvora-gboom", "xai-grok-pager-diff": "pager-diff",
    "xai-grok-workspace-daemon": "workspace-daemon",
    "xai-hooks-plugins-types": "hooks-plugins-types",
    "xai-interjection-core": "interjection-core",
    "xai-message-delivery-core": "message-delivery-core",
    "xai-mixpanel": "xvora-mixpanel", "xai-prompt-queue": "prompt-queue",
    "xai-proto-build": "proto-build", "xai-ratatui-inline": "ratatui-inline",
    "xai-ratatui-textarea": "ratatui-textarea", "xai-sqlite-journal": "sqlite-journal",
    "xai-system-power": "system-power", "xai-test-utils": "test-utils",
    "xai-token-estimation": "token-estimation", "xai-tool-protocol": "tool-protocol",
    "xai-tool-runtime": "tool-runtime", "xai-tool-types": "tool-types",
    "xai-tracing": "xvora-tracing", "xai-tty-utils": "tty-utils",
    "xai-compaction-transcript": "xvora-compaction-transcript",
    "xai-dirs": "xvora-dirs", "xai-circuit-breaker": "circuit-breaker",
    "xai-computer-hub-core": "computer-hub-core",
    "xai-computer-hub-mcp-adapter": "computer-hub-mcp-adapter",
    "xai-computer-hub-sdk": "computer-hub-sdk",
    "xai-grok-compaction": "xvora-compaction",
    "xai-fuzzy-file-search": "fuzzy-file-search", "xai-workflow": "workflow",
}

def parse_deps(path):
    content = path.read_text(encoding="utf-8")
    in_ws = False
    deps = {}
    insert_idx = None
    for i, line in enumerate(content.split("\n")):
        stripped = line.strip()
        if stripped == "[workspace.dependencies]":
            in_ws = True
            continue
        if in_ws and stripped.startswith("["):
            insert_idx = i
            break
        if in_ws and "=" in stripped and not stripped.startswith("#"):
            key = stripped.split("=")[0].strip()
            deps[key] = stripped
    return deps, insert_idx

def map_dep_name(key):
    """Map a grok-build dep name to xVora dep name."""
    if key in CRATE_MAP:
        return CRATE_MAP[key]
    if key.startswith("xai-") and key[4:] in CRATE_MAP.values():
        return key[4:]  # xai-xxx → xxx (for non-xvora prefixed crates)
    return key  # non-crate dep, keep as-is

def main():
    grok_deps, grok_insert = parse_deps(GROK / "Cargo.toml")
    xvora_deps, xvora_insert = parse_deps(XVORA / "Cargo.toml")

    lines = (XVORA / "Cargo.toml").read_text(encoding="utf-8").split("\n")

    added = []
    for grok_key, grok_line in sorted(grok_deps.items()):
        x_key = map_dep_name(grok_key)

        # Skip xai- prefixed (old names we don't want)
        if grok_key.startswith("xai-") and grok_key not in CRATE_MAP:
            continue
        # Already exists?
        if x_key in xvora_deps:
            continue

        # For path deps, verify crate exists
        if "path = " in grok_line:
            m = re.search(r'path\s*=\s*"([^"]+)"', grok_line)
            if m:
                crate_dir = m.group(1).split("/")[-1]
                # Check if crate exists in xVora
                found = False
                for layer in ["codegen", "common", "build", "prod", "tests"]:
                    if (XVORA / "crates" / layer / crate_dir).exists():
                        found = True
                        break
                if not found:
                    print(f"  SKIP (no crate): {grok_key} -> {crate_dir}")
                    continue

        line_to_add = grok_line
        # Apply name mapping to the dep line itself
        for old, new in CRATE_MAP.items():
            line_to_add = line_to_add.replace(old, new)
        # Also fix xai- prefix for non-map deps
        if line_to_add.startswith("xai-") and not any(x in line_to_add for x in CRATE_MAP.values()):
            # This is a dep whose name changed from xai- to something else
            pass  # already handled above

        lines.insert(xvora_insert, line_to_add)
        xvora_insert += 1
        added.append(grok_key)

    (XVORA / "Cargo.toml").write_text("\n".join(lines), encoding="utf-8")
    print(f"Added {len(added)} workspace deps")

if __name__ == "__main__":
    main()
