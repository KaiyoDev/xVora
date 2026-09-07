#!/usr/bin/env python3
"""Rename Grok/grok references to xVora/xvora in user-guide docs.

Rules:
- 'Grok Build' → 'xVora'
- 'Grok' (capitalized, standalone word) → 'xVora'
- 'grok.com' → 'xvora.com'
- '~/.grok/' → '~/.xvora/'
- '$GROK_HOME' → '$XVORA_HOME'
- 'GROK_' prefix in env vars → 'XVORA_'
- 'grok' as command in code blocks stays 'xvora' (the binary name)
- Model names like 'grok-4.5', 'grok-4.6' stay as-is (they're real model IDs)
- 'grok-clone' in filenames → 'xvora-clone'
- 'Grokip' / 'Grokify' etc → handle case by case
"""
from pathlib import Path
import re

ROOT = Path("crates/codegen/xvora-pager/docs/user-guide")

def replace_text(text: str) -> str:
    # 1. Product name references
    text = text.replace("Grok Build", "xVora")
    text = re.sub(r'\bGrok\b', 'xVora', text)

    # 2. URLs and paths
    text = text.replace("grok.com", "xvora.com")
    text = text.replace("~/.grok/", "~/.xvora/")
    text = text.replace("$GROK_HOME", "$XVORA_HOME")
    text = text.replace("/etc/grok/", "/etc/xvora/")
    text = text.replace("ai.x.grok", "ai.x.xvora")

    # 3. Env var prefixes (but NOT model names like grok-4.5)
    text = re.sub(r'\bGROK_(\w+)', r'XVORA_\1', text)

    # 4. Command references: 'grok' as a standalone word → 'xvora'
    #    But skip model names (grok-4.x, grok-4.5, etc.)
    #    Skip URLs (x.ai/cli)
    text = re.sub(r'(?<![-.\w])(?<!\w-)(?<!x\.ai/)(?<!xvora\.com/)(?<!grok\.com/)(?<!\.)\bgrokom?\b(?![-.\w])', 'xvora', text)

    # 5. Filename references
    text = text.replace("grok-clone", "xvora-clone")
    text = text.replace("[grok clone]", "[xvora clone]")
    text = text.replace("[Grok clone]", "[xvora clone]")

    return text


def process_file(path: Path) -> bool:
    original = path.read_text(encoding="utf-8")
    modified = replace_text(original)
    if modified != original:
        path.write_text(modified, encoding="utf-8")
        return True
    return False


def rename_file(old: Path, new_name: str):
    if old.exists() and not old.name == new_name:
        new_path = old.parent / new_name
        old.rename(new_path)
        print(f"  renames {old.name} → {new_name}")
        return True
    return False


if __name__ == "__main__":
    # Rename grok-clone.md → xvora-clone.md
    old_clone = ROOT / "27-grok-clone.md"
    rename_file(old_clone, "27-xvora-clone.md")

    files = list(ROOT.rglob("*.md"))
    changed = 0
    for f in files:
        if process_file(f):
            print(f"  ✓ {f.relative_to(ROOT)}")
            changed += 1
    print(f"\nDone. {changed}/{len(files)} files updated.")
