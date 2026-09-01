#!/usr/bin/env python3
"""Comprehensive fix for ALL incorrectly renamed workspace dependency keys in Cargo.toml files."""
import re
from pathlib import Path

XVORA = Path(r"D:\Kaiyo\Project\xVora")

# Mapping: (wrong_key, correct_key)
# These are the actual package names from each crate's [package] name field
FIX_DEPS = {
    # crates that were originally named without xvora- prefix
    "xvora-tty-utils": "tty-utils",
    "xvora-prompt-queue": "prompt-queue",
    "xvora-ratatui-textarea": "ratatui-textarea",
    "xvora-ratatui-inline": "ratatui-inline",
    "xvora-acp-lib": "acp-lib",
    "xvora-file-utils": "file-utils",
    "xvora-chat-state": "chat-state",
    "xvora-tool-protocol": "tool-protocol",
    "xvora-tool-runtime": "tool-runtime",
    "xvora-tool-types": "tool-types",
    "xvora-token-estimation": "token-estimation",
    "xvora-hooks-plugins-types": "hooks-plugins-types",
    "xvora-hunk-tracker": "hunk-tracker",
    "xvora-sqlite-journal": "sqlite-journal",
    "xvora-system-power": "system-power",
    "xvora-tracing-macros": "tracing-macros",
    "xvora-test-utils": "test-utils",
    # Direct path deps that got rebranded
    "xvora-fast-worktree": "fast-worktree",
    "xvora-crash-handler": "crash-handler",
    "xvora-codebase-graph": "codebase-graph",
    "xvora-file-utils": "file-utils",  # also used as direct path dep
}

def fix_file(path: Path):
    content = path.read_text(encoding="utf-8")
    original = content

    # Fix both workspace deps and direct path deps
    for wrong, correct in FIX_DEPS.items():
        # Match: wrong = { ... } or wrong.workspace = true or wrong = { path = ... }
        # But NOT inside comments
        # Pattern: key at start of line (possibly indented), followed by =
        pattern = re.compile(r'(?m)^( *)' + re.escape(wrong) + r'\b(.*?)(?=\n|$)')
        def replacer(m):
            indent = m.group(1)
            rest = m.group(2)
            return indent + correct + rest
        content = pattern.sub(replacer, content)

    if content != original:
        path.write_text(content, encoding="utf-8")
        print(f"  FIXED: {path.relative_to(XVORA)}")
        return True
    return False

def main():
    print("Fixing ALL workspace dependency key renames...")
    count = 0
    for f in sorted(XVORA.rglob("Cargo.toml")):
        if fix_file(f):
            count += 1
    print(f"\nFixed {count} files!")

if __name__ == "__main__":
    main()
