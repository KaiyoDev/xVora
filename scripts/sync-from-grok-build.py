#!/usr/bin/env python3
"""Sync changed crates from grok-build → xVora with xai→xvora rebrand, then commit+push."""
import os, re, shutil, subprocess, json
from pathlib import Path

GROK = Path(r"D:\Kaiyo\Project\grok-build")
XVORA = Path(r"D:\Kaiyo\Project\xVora")

# Map grok-build crate → xvora crate (codegen + common)
CRATE_MAP = {
    # codegen
    "ptyctl": ("codegen", "ptyctl"),
    "ptyctl-cli": ("codegen", "ptyctl-cli"),
    "xai-acp-lib": ("codegen", "acp-lib"),
    "xai-agent-lifecycle": ("codegen", "agent-lifecycle"),
    "xai-chat-state": ("codegen", "chat-state"),
    "xai-codebase-graph": ("codegen", "codebase-graph"),
    "xai-crash-handler": ("codegen", "crash-handler"),
    "xai-fast-worktree": ("codegen", "fast-worktree"),
    "xai-file-utils": ("codegen", "file-utils"),
    "xai-fsnotify": ("codegen", "xvora-fsnotify"),
    "xai-gix-status": ("codegen", "xvora-gix-status"),
    "xai-grok-agent": ("codegen", "xvora-agent"),
    "xai-grok-announcements": ("codegen", "xvora-announcements"),
    "xai-grok-auth": ("codegen", "xvora-auth"),
    "xai-grok-config": ("codegen", "xvora-config"),
    "xai-grok-config-types": ("codegen", "xvora-config-types"),
    "xai-grok-env": ("codegen", "xvora-env"),
    "xai-grok-hooks": ("codegen", "xvora-hooks"),
    "xai-grok-http": ("codegen", "xvora-http"),
    "xai-grok-markdown": ("codegen", "xvora-markdown"),
    "xai-grok-markdown-core": ("codegen", "xvora-markdown-core"),
    "xai-grok-mcp": ("codegen", "xvora-mcp"),
    "xai-grok-memory": ("codegen", "xvora-memory"),
    "xai-grok-mermaid": ("codegen", "xvora-mermaid"),
    "xai-grok-models": ("codegen", "xvora-models"),
    "xai-grok-pager": ("codegen", "xvora-pager"),
    "xai-grok-pager-bin": ("codegen", "xvora-pager-bin"),
    "xai-grok-pager-minimal": ("codegen", "xvora-pager-minimal"),
    "xai-grok-pager-pty-harness": ("codegen", "xvora-pager-pty-harness"),
    "xai-grok-pager-render": ("codegen", "xvora-pager-render"),
    "xai-grok-paths": ("codegen", "xvora-paths"),
    "xai-grok-plugin-marketplace": ("codegen", "xvora-plugin-marketplace"),
    "xai-grok-sampler": ("codegen", "xvora-sampler"),
    "xai-grok-sampling-types": ("codegen", "xvora-sampling-types"),
    "xai-grok-sandbox": ("codegen", "xvora-sandbox"),
    "xai-grok-secrets": ("codegen", "xvora-secrets"),
    "xai-grok-shared": ("codegen", "xvora-shared"),
    "xai-grok-shell": ("codegen", "xvora-shell"),
    "xai-grok-shell-base": ("codegen", "xvora-shell-base"),
    "xai-grok-shell-session-support": ("codegen", "xvora-shell-session-support"),
    "xai-grok-subagent-resolution": ("codegen", "xvora-subagent-resolution"),
    "xai-grok-telemetry": ("codegen", "xvora-telemetry"),
    "xai-grok-test-support": ("codegen", "xvora-test-support"),
    "xai-grok-tools": ("codegen", "xvora-tools"),
    "xai-grok-tools-api": ("codegen", "xvora-tools-api"),
    "xai-grok-update": ("codegen", "xvora-update"),
    "xai-grok-version": ("codegen", "xvora-version"),
    "xai-grok-voice": ("codegen", "xvora-voice"),
    "xai-grok-workspace": ("codegen", "xvora-workspace"),
    "xai-grok-workspace-client": ("codegen", "xvora-workspace-client"),
    "xai-grok-workspace-types": ("codegen", "xvora-workspace-types"),
    "xai-hooks-plugins-types": ("codegen", "hooks-plugins-types"),
    "xai-hunk-tracker": ("codegen", "hunk-tracker"),
    "xai-mixpanel": ("codegen", "xvora-mixpanel"),
    "xai-prompt-queue": ("codegen", "prompt-queue"),
    "xai-ratatui-inline": ("codegen", "ratatui-inline"),
    "xai-ratatui-textarea": ("codegen", "ratatui-textarea"),
    "xai-sqlite-journal": ("codegen", "sqlite-journal"),
    "xai-system-power": ("codegen", "system-power"),
    "xai-token-estimation": ("codegen", "token-estimation"),
    "xai-tracing-macros": ("codegen", "tracing-macros"),
    "xai-tty-utils": ("codegen", "tty-utils"),
    # new crates
    "xai-compaction-transcript": ("codegen", "xvora-compaction-transcript"),
    "xai-dirs": ("codegen", "xvora-dirs"),
    # common
    "xai-circuit-breaker": ("common", "circuit-breaker"),
    "xai-computer-hub-core": ("common", "computer-hub-core"),
    "xai-computer-hub-mcp-adapter": ("common", "computer-hub-mcp-adapter"),
    "xai-computer-hub-sdk": ("common", "computer-hub-sdk"),
    "xai-grok-compaction": ("common", "xvora-compaction"),
    "xai-interjection-core": ("common", "interjection-core"),
    "xai-test-utils": ("common", "test-utils"),
    "xai-tool-protocol": ("common", "tool-protocol"),
    "xai-tool-runtime": ("common", "tool-runtime"),
    "xai-tool-types": ("common", "tool-types"),
    "xai-tracing": ("common", "tracing-util"),
}

def rebrand(content: str) -> str:
    """Rebrand xai/xai-grok → xvora in source code."""
    # Rust ident: xai_grok_*, xai_*
    content = re.sub(r'\bxai_grok_(\w+)\b', r'xvora_\1', content)
    content = re.sub(r'\bxai_(\w+)\b', r'xvora_\1', content)
    # Crate names in Cargo.toml: xai-grok-*, xai-*
    content = re.sub(r'\bxai-grok-(\w+)\b', r'xvora-\1', content)
    content = re.sub(r'\bxai-(\w+)\b', r'xvora-\1', content)
    # Binary name references: xai-grok → xvora
    content = re.sub(r'\bxai-grok\b', r'xvora', content)
    # Loose xai → xvora (package names, module prefixes)
    content = re.sub(r'\bxai\b', r'xvora', content)
    return content

def get_changed_crates(ref_from: str, ref_to: str) -> set[str]:
    result = subprocess.run(
        ["git", "diff", "--name-only", f"{ref_from}..{ref_to}"],
        cwd=str(GROK), capture_output=True, text=True, check=True
    )
    crates = set()
    for line in result.stdout.strip().split("\n"):
        if not line.startswith("crates/"):
            continue
        parts = line.replace("\\", "/").split("/")
        # crates/codegen/<crate>/... or crates/common/<crate>/...
        if len(parts) >= 3:
            crates.add(parts[2])
    return crates

def copy_and_rebrand(src_path: Path, dst_path: Path):
    """Copy file and rebrand content."""
    dst_path.parent.mkdir(parents=True, exist_ok=True)
    try:
        content = src_path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        shutil.copy2(src_path, dst_path)
        return True
    new_content = rebrand(content)
    dst_path.write_text(new_content, encoding="utf-8")
    return True

def sync_crate(grok_crate: str, layer: str, xvora_crate: str):
    """Copy entire crate from grok-build to xVora with rebranding."""
    src_base = GROK / "crates" / layer / grok_crate
    if not src_base.exists():
        print(f"  SKIP (not found): {grok_crate}")
        return

    dst_base = XVORA / "crates" / layer / xvora_crate
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
                pass  # binary file, leave as-is

    print(f"  + {grok_crate} -> {layer}/{xvora_crate} ({count} files)")

def update_root_cargo_toml(changed_crates: set[str]):
    """Update root Cargo.toml members list and workspace.dependencies."""
    # Read current root Cargo.toml
    root_toml = XVORA / "Cargo.toml"
    content = root_toml.read_text(encoding="utf-8")

    # Find all member paths currently listed
    import tomllib  # Python 3.11+
    # Use regex-based approach since we need to edit TOML
    lines = content.split("\n")

    # We'll do targeted replacements
    # Add new crate members that don't exist yet
    new_members = []
    new_deps = []

    for grok_crate in sorted(changed_crates):
        if grok_crate not in CRATE_MAP:
            continue
        layer, xvora_crate = CRATE_MAP[grok_crate]
        member_path = f'    "crates/{layer}/{xvora_crate}",'
        dep_line = f'{xvora_crate} = {{ path = "crates/{layer}/{xvora_crate}" }}'

        if member_path not in content:
            new_members.append(member_path)
        # For deps, check if xvora_crate dep already exists
        dep_check = f'{xvora_crate} = {{ path ='
        if dep_check not in content:
            new_deps.append(dep_line)

    if new_members:
        print(f"  Adding {len(new_members)} new members to Cargo.toml")
    if new_deps:
        print(f"  Adding {len(new_deps)} new deps to Cargo.toml")

def update_workspace_deps(changed_crates: set[str]):
    """Update [workspace.dependencies] in root Cargo.toml with new crates."""
    root_toml = XVORA / "Cargo.toml"
    content = root_toml.read_text(encoding="utf-8")

    changes = []
    for grok_crate in sorted(changed_crates):
        if grok_crate not in CRATE_MAP:
            continue
        layer, xvora_crate = CRATE_MAP[grok_crate]
        dep_entry = f'{xvora_crate} = {{ path = "crates/{layer}/{xvora_crate}" }}'
        if dep_entry not in content:
            changes.append(dep_entry)

    return changes

def main():
    print("=" * 60)
    print("Syncing grok-build -> xVora (xai -> xvora rebrand)")
    print("=" * 60)

    # Get changed crates since open-source publish
    changed_crates = get_changed_crates("c68e39f6", "HEAD")
    print(f"\nChanged crates in grok-build: {len(changed_crates)}")

    # Sync each crate
    synced = []
    for grok_crate in sorted(changed_crates):
        if grok_crate not in CRATE_MAP:
            print(f"  ⏭ skip (no map): {grok_crate}")
            continue
        layer, xvora_crate = CRATE_MAP[grok_crate]
        sync_crate(grok_crate, layer, xvora_crate)
        synced.append((grok_crate, xvora_crate))

    # Update root Cargo.toml
    print("\nUpdating root Cargo.toml...")
    new_deps = update_workspace_deps(changed_crates)
    if new_deps:
        root_toml = XVORA / "Cargo.toml"
        content = root_toml.read_text(encoding="utf-8")
        # Insert new deps before the closing of [workspace.dependencies]
        # Find last dependency line
        lines = content.split("\n")
        insert_idx = None
        for i, line in enumerate(lines):
            if line.strip() == "[workspace.dependencies]":
                # Find end of section
                for j in range(i+1, len(lines)):
                    if lines[j].startswith("[") and not lines[j].startswith("[["):
                        insert_idx = j
                        break
                if insert_idx is None:
                    insert_idx = len(lines)
                break

        if insert_idx:
            for dep in new_deps:
                lines.insert(insert_idx, dep)
                insert_idx += 1
            root_toml.write_text("\n".join(lines), encoding="utf-8")
            print(f"  Added {len(new_deps)} new workspace dependencies")

    # Update Cargo.toml members list
    print("\nUpdating members list...")
    root_toml = XVORA / "Cargo.toml"
    content = root_toml.read_text(encoding="utf-8")
    lines = content.split("\n")
    new_members = []
    for grok_crate, xvora_crate in synced:
        layer, _ = CRATE_MAP[grok_crate]
        member = f'    "crates/{layer}/{xvora_crate}",'
        if member not in content:
            new_members.append(member)

    if new_members:
        # Insert after existing members block
        for i, line in enumerate(lines):
            if '"crates/common/tracing-util"' in line or '"crates/codegen/tty-utils"' in line:
                # Insert before the closing ]
                for j in range(i+1, len(lines)):
                    if lines[j].strip() == "]":
                        for m in new_members:
                            lines.insert(j, m)
                            j += 1
                        break
                break
        root_toml.write_text("\n".join(lines), encoding="utf-8")
        print(f"  Added {len(new_members)} new members")

    # Summary
    print(f"\n{'=' * 60}")
    print(f"SYNC COMPLETE: {len(synced)} crates synced")
    print(f"{'=' * 60}\n")

    # Output JSON summary for caller
    summary = {
        "synced_crates": [{"grok": g, "xvora": x} for g, x in synced],
        "new_deps_added": len(new_deps),
        "new_members_added": len(new_members),
    }
    with open(XVORA / ".sync_summary.json", "w") as f:
        json.dump(summary, f, indent=2)
    print(json.dumps(summary, indent=2))

if __name__ == "__main__":
    main()
