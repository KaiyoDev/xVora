#!/usr/bin/env python3
"""Fix ALL workspace dependency key renames in all Cargo.toml files."""
import re
from pathlib import Path

XVORA = Path(r"D:\Kaiyo\Project\xVora")

# (wrong_xvora_name, correct_name) - covers all cases from rebrand over-application
FIX_DEPS = [
    # These were renamed by the rebrand regex but should keep original names
    ("xvora-tty-utils", "tty-utils"),
    ("xvora-prompt-queue", "prompt-queue"),
    ("xvora-ratatui-textarea", "ratatui-textarea"),
    ("xvora-ratatui-inline", "ratatui-inline"),
    ("xvora-acp-lib", "acp-lib"),
    ("xvora-file-utils", "file-utils"),
    ("xvora-chat-state", "chat-state"),
    ("xvora-tool-protocol", "tool-protocol"),
    ("xvora-tool-runtime", "tool-runtime"),
    ("xvora-tool-types", "tool-types"),
    ("xvora-token-estimation", "token-estimation"),
    ("xvora-hooks-plugins-types", "hooks-plugins-types"),
    ("xvora-hunk-tracker", "hunk-tracker"),
    ("xvora-sqlite-journal", "sqlite-journal"),
    ("xvora-system-power", "system-power"),
    ("xvora-tracing-macros", "tracing-macros"),
    # Direct path deps that got rebranded
    ("xvora-fast-worktree", "fast-worktree"),
    ("xvora-crash-handler", "crash-handler"),
    ("xvora-codebase-graph", "codebase-graph"),
    ("xvora-pager-diff", "pager-diff"),
    ("xvora-gboom", "gboom"),
    ("xvora-workspace-daemon", "workspace-daemon"),
    ("xvora-workflow", "workflow"),
    ("xvora-test-utils", "test-utils"),
    ("xvora-pager", "xvora-pager"),  # keep
]

def fix_file(path: Path):
    content = path.read_text(encoding="utf-8")
    original = content

    lines = content.split("\n")
    new_lines = []
    for line in lines:
        for wrong, correct in FIX_DEPS:
            # Only fix dependency lines (contain "path =" or "workspace = true"
            # or "version =", but NOT "^name = ")
            if re.match(r'^\s*name\s*=', line):
                continue
            # Also skip comment lines
            stripped = line.strip()
            if stripped.startswith("#"):
                continue
            if f'"{wrong}"' in line:
                line = line.replace(f'"{wrong}"', f'"{correct}"')
                break  # only replace once per line
        new_lines.append(line)
    content = "\n".join(new_lines)

    if content != original:
        path.write_text(content, encoding="utf-8")
        print(f"  FIXED: {path.relative_to(XVORA)}")
        return True
    return False

def main():
    print("Fixing workspace dependency key renames (all Cargo.toml files)...")
    count = 0
    for f in sorted(XVORA.rglob("Cargo.toml")):
        if fix_file(f):
            count += 1
    print(f"\nFixed {count} files!")

if __name__ == "__main__":
    main()
