#!/usr/bin/env python3
"""Fix workspace dependency key renames in all Cargo.toml files."""
import re
from pathlib import Path

XVORA = Path(r"D:\Kaiyo\Project\xVora")

# (wrong_xvora_name, correct_name) mappings
FIX_DEPS = [
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
    ("xvora-compaction-transcript", "xvora-compaction-transcript"),  # keep as-is
    ("xvora-dirs", "xvora-dirs"),  # keep as-is
]

def fix_cargo_toml(path: Path):
    content = path.read_text(encoding="utf-8")
    original = content

    lines = content.split("\n")
    new_lines = []
    for line in lines:
        for wrong, correct in FIX_DEPS:
            # Only fix dependency lines (contain "path =" or "version =" but NOT "^name = ")
            if re.match(r'^\s*name\s*=', line):
                continue
            if f'"{wrong}"' in line:
                line = line.replace(f'"{wrong}"', f'"{correct}"')
                break  # only replace once per line
        new_lines.append(line)
    content = "\n".join(new_lines)

    if content != original:
        path.write_text(content, encoding="utf-8")
        print(f"  FIXED: {path.relative_to(XVORA)}")
    else:
        print(f"  OK:    {path.relative_to(XVORA)}")

def main():
    print("Fixing workspace dependency key renames...")
    for f in sorted(XVORA.rglob("Cargo.toml")):
        fix_cargo_toml(f)
    print("\nDone!")

if __name__ == "__main__":
    main()
