#!/usr/bin/env python3
"""Bulk rename grok -> xvora in .rs files, preserving model names (grok-4.x, grok-3)."""

import os
import re
import sys

SKIP_PATTERNS = [
    r'\bgrok-4\.',   # model names
    r'\bgrok-3',     # model names
    r'\bgrok-shell',  # protocol name
    r'\bgrokShell',   # JSON key
    r'\bgrok\.com\b', # auth provider
    r'\bgrok_build\b',# module name
    r'\bgrokday\b',   # theme name
    r'\bgrok-build\b',# reference name
    r'\bgrok_home\b', # already renamed to xvora_home
    r'\bGROK_HOME\b', # already handled
    r'\.grok-snapshots', # btrfs snapshot dir (internal)
    r'refs/grok/',    # git ref (internal protocol)
    r'grok-nfs-worktree', # test fixture name
]

# Patterns to replace
REPLACEMENTS = [
    # Function/identifier renames
    (r'\bgrok_home_in\b', 'xvora_home_in'),
    (r'\bGrokHomeSource\b', 'XvoraHomeSource'),
    (r'\bgrok_home\b', 'xvora_home'),
    (r'\bGROK_HOME\b', 'XVORA_HOME'),
    # Path literals
    (r'~/.grok/', '~/.xvora/'),
    (r'\.grok/', '.xvora/'),
    (r'\.grok\b', '.xvora'),
    # Comments and strings
    (r'\bgrok build\b', 'xvora build'),
    (r'\bgrok\b', 'xvora'),  # catch-all for remaining
]


def should_skip(line):
    for pattern in SKIP_PATTERNS:
        if re.search(pattern, line, re.IGNORECASE):
            return True
    return False


def process_file(path):
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()

    original = content
    for pattern, replacement in REPLACEMENTS:
        content = re.sub(pattern, replacement, content, flags=re.IGNORECASE)

    if content != original:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False


def main():
    crates_dir = os.path.join(os.path.dirname(__file__), '..', 'crates')
    count = 0
    for root, dirs, files in os.walk(crates_dir):
        # Skip target dirs
        dirs[:] = [d for d in dirs if d not in ('target', '.git', '__pycache__')]
        for fname in files:
            if not fname.endswith('.rs'):
                continue
            path = os.path.join(root, fname)
            if process_file(path):
                count += 1
                print(f"Updated: {os.path.relpath(path, crates_dir)}")

    print(f"\nTotal files updated: {count}")


if __name__ == '__main__':
    main()
