#!/usr/bin/env python3
"""Fix remaining grok command references → xvora in docs."""
from pathlib import Path
import re

ROOT = Path("crates/codegen/xvora-pager/docs/user-guide")

def fix_line(text: str) -> str:
    # Replace standalone "grok" NOT followed by -digit (preserves grok-4.6 etc.)
    text = re.sub(r'(?<![.\w-])\bgrok\b(?!-\d)', 'xvora', text)
    return text

def process_file(path: Path) -> bool:
    original = path.read_text(encoding="utf-8")
    modified = fix_line(original)
    if modified != original:
        path.write_text(modified, encoding="utf-8")
        return True
    return False


if __name__ == "__main__":
    files = list(ROOT.rglob("*.md"))
    changed = 0
    for f in files:
        if process_file(f):
            print(f"  ✓ {f.relative_to(ROOT)}")
            changed += 1
    print(f"\nDone. {changed}/{len(files)} files updated.")
