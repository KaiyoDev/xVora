#!/usr/bin/env python3
"""Sync the 4 missing crates from grok-build to xVora."""
import re, shutil
from pathlib import Path

GROK = Path(r"D:\Kaiyo\Project\grok-build")
XVORA = Path(r"D:\Kaiyo\Project\xVora")

# Map grok-build crate names → xVora crate names
MISSING = {
    "xai-grok-gboom": ("codegen", "xvora-gboom"),
    "xai-grok-pager-diff": ("codegen", "xvora-pager-diff"),
    "xai-grok-workspace-daemon": ("codegen", "xvora-workspace-daemon"),
    "xai-workflow": ("codegen", "xvora-workflow"),
}

def rebrand(content: str) -> str:
    content = re.sub(r'\bxai_grok_(\w+)\b', r'xvora_\1', content)
    content = re.sub(r'\bxai_(\w+)\b', r'xvora_\1', content)
    content = re.sub(r'\bxai-grok-(\w+)\b', r'xvora-\1', content)
    content = re.sub(r'\bxai-(\w+)\b', r'xvora-\1', content)
    content = re.sub(r'\bxai-grok\b', r'xvora', content)
    content = re.sub(r'\bxai\b', r'xvora', content)
    return content

def sync(src_name, dst_layer, dst_name):
    src_base = GROK / "crates" / src_name
    dst_base = XVORA / "crates" / dst_layer / dst_name
    if not src_base.exists():
        print(f"  SKIP: {src_name} not found in grok-build")
        return
    if dst_base.exists():
        shutil.rmtree(dst_base)
    shutil.copytree(src_base, dst_base)
    count = 0
    for f in dst_base.rglob("*"):
        if f.is_file():
            try:
                text = f.read_text(encoding="utf-8")
                new_text = rebrand(text)
                f.write_text(new_text, encoding="utf-8")
                count += 1
            except UnicodeDecodeError:
                pass
    print(f"  SYNC: {src_name} -> {dst_layer}/{dst_name} ({count} files)")

if __name__ == "__main__":
    for src, (layer, dst) in MISSING.items():
        sync(src, layer, dst)
