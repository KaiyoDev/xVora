#!/usr/bin/env python3
"""Fix Cargo.toml package names after over-aggressive rebrand."""
import re
from pathlib import Path

XVORA = Path(r"D:\Kaiyo\Project\xVora")

# Package names that must stay unchanged (these are the xvora-renamed versions)
STABLE_NAMES = {
    # codegen
    "ptyctl", "ptyctl-cli", "acp-lib", "agent-lifecycle", "chat-state",
    "codebase-graph", "crash-handler", "fast-worktree", "file-utils",
    "xvora-fsnotify", "xvora-gix-status",
    "xvora-agent", "xvora-announcements", "xvora-auth", "xvora-config",
    "xvora-config-types", "xvora-env", "xvora-hooks", "xvora-http",
    "xvora-markdown", "xvora-markdown-core", "xvora-mcp", "xvora-memory",
    "xvora-mermaid", "xvora-models", "xvora-pager", "xvora-pager-bin",
    "xvora-pager-minimal", "xvora-pager-pty-harness", "xvora-pager-render",
    "xvora-paths", "xvora-plugin-marketplace", "xvora-sampler",
    "xvora-sampling-types", "xvora-sandbox", "xvora-secrets", "xvora-shared",
    "xvora-shell", "xvora-shell-base", "xvora-shell-session-support",
    "xvora-subagent-resolution", "xvora-telemetry", "xvora-test-support",
    "xvora-tools", "xvora-tools-api", "xvora-update", "xvora-version",
    "xvora-voice", "xvora-workspace", "xvora-workspace-client",
    "xvora-workspace-types", "hooks-plugins-types", "hunk-tracker",
    "xvora-mixpanel", "prompt-queue", "ratatui-inline", "ratatui-textarea",
    "sqlite-journal", "system-power", "token-estimation", "tracing-macros",
    "tty-utils", "xvora-compaction-transcript", "xvora-dirs",
    # common
    "circuit-breaker", "computer-hub-core", "computer-hub-mcp-adapter",
    "computer-hub-sdk", "xvora-compaction", "interjection-core",
    "test-utils", "tool-protocol", "tool-runtime", "tool-types", "tracing-util",
}

def fix_cargo_toml(path: Path):
    content = path.read_text(encoding="utf-8")
    original = content

    # Fix 1: name = "xvora-XXX" -> keep the stable name (without xvora- prefix for non-xvora crates)
    # The script incorrectly renamed package names. Fix them.
    lines = content.split("\n")
    new_lines = []
    for line in lines:
        m = re.match(r'^name\s*=\s*"([^"]+)"', line)
        if m:
            pkg_name = m.group(1)
            if pkg_name.startswith("xvora-") and pkg_name[6:] in STABLE_NAMES:
                # It was originally named without xvora- prefix
                fixed = pkg_name[6:]  # Remove xvora- prefix
                if fixed in STABLE_NAMES:
                    line = line.replace(f'"{pkg_name}"', f'"{fixed}"')
            elif pkg_name in ("nfs-create-latency-bench", "worktree-lifecycle-bench",
                              "env_op_compile", "startup-tui-probe", "resize",
                              "doctor_early_dispatch", "fork_copy", "skills_watcher_startup",
                              "child_replay_lookup", "test_startup_prefetch_fallback",
                              "test_startup_prefetch_overlap", "test_startup_prefetch_policy",
                              "test_startup_prefetch_repair_skip"):
                # These are binary test names, leave as-is
                pass
        # Fix 2: dependency references like xvora-token-estimation -> token-estimation
        # but only in dependency lines, not package name lines
        if "name = " not in line and "path = " in line:
            for old, new in [
                ("xvora-tool-protocol", "tool-protocol"),
                ("xvora-tool-runtime", "tool-runtime"),
                ("xvora-tool-types", "tool-types"),
                ("xvora-token-estimation", "token-estimation"),
                ("xvora-compaction-transcript", "xvora-compaction-transcript"),  # keep
                ("xvora-chat-state", "chat-state"),
                ("xvora-acp-lib", "acp-lib"),
                ("xvora-agent-lifecycle", "agent-lifecycle"),
                ("xvora-codebase-graph", "codebase-graph"),
                ("xvora-crash-handler", "crash-handler"),
                ("xvora-fast-worktree", "fast-worktree"),
                ("xvora-file-utils", "file-utils"),
                ("xvora-hooks-plugins-types", "hooks-plugins-types"),
                ("xvora-hunk-tracker", "hunk-tracker"),
                ("xvora-prompt-queue", "prompt-queue"),
                ("xvora-ratatui-inline", "ratatui-inline"),
                ("xvora-ratatui-textarea", "ratatui-textarea"),
                ("xvora-sqlite-journal", "sqlite-journal"),
                ("xvora-system-power", "system-power"),
                ("xvora-tty-utils", "tty-utils"),
                ("xvora-pager", "xvora-pager"),  # correct
            ]:
                line = line.replace(f'"{old}"', f'"{new}"')
        new_lines.append(line)
    content = "\n".join(new_lines)

    if content != original:
        path.write_text(content, encoding="utf-8")
        print(f"  FIXED: {path.relative_to(XVORA)}")
    else:
        print(f"  OK:    {path.relative_to(XVORA)}")

def fix_root_cargo_toml():
    path = XVORA / "Cargo.toml"
    content = path.read_text(encoding="utf-8")
    original = content

    # Remove incorrectly added deps with xvora- prefix that should use original names
    lines = content.split("\n")
    new_lines = []
    for line in lines:
        # Skip wrongly-added deps
        if '= { path = "crates/codegen/xvora-tool-protocol"' in line:
            continue
        if '= { path = "crates/codegen/xvora-tool-runtime"' in line:
            continue
        if '= { path = "crates/codegen/xvora-tool-types"' in line:
            continue
        if '= { path = "crates/codegen/token-estimation"' in line and 'xvora-' not in line:
            # check if it's a duplicate
            pass
        new_lines.append(line)
    content = "\n".join(new_lines)

    # Also clean up duplicate entries in [workspace.dependencies]
    # Find the section and deduplicate
    if content != original:
        path.write_text(content, encoding="utf-8")
        print(f"  FIXED ROOT: Cargo.toml")
    else:
        print(f"  OK ROOT: Cargo.toml")

def main():
    print("Fixing Cargo.toml package names...")
    # Fix all Cargo.toml files in crates/
    for f in sorted(XVORA.rglob("Cargo.toml")):
        fix_cargo_toml(f)
    fix_root_cargo_toml()
    print("\nDone!")

if __name__ == "__main__":
    main()
