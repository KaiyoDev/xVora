#!/usr/bin/env python3
"""Fix ALL workspace dependency issues: add missing crates + deps."""
import re, shutil, subprocess
from pathlib import Path

GROK = Path(r"D:\Kaiyo\Project\grok-build")
XVORA = Path(r"D:\Kaiyo\Project\xVora")

def rebrand(content: str) -> str:
    content = re.sub(r'\bxai_grok_(\w+)\b', r'xvora_\1', content)
    content = re.sub(r'\bxai_(\w+)\b', r'xvora_\1', content)
    content = re.sub(r'\bxai-grok-(\w+)\b', r'xvora-\1', content)
    content = re.sub(r'\bxai-(\w+)\b', r'xvora-\1', content)
    content = re.sub(r'\bxai-grok\b', r'xvora', content)
    content = re.sub(r'\bxai\b', r'xvora', content)
    return content

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

def main():
    # Step 1: Find crates referenced but not present
    print("=== Step 1: Finding missing crates ===")
    xvora_crates = set()
    for d in XVORA.rglob("Cargo.toml"):
        parent = d.parent.name
        xvora_crates.add(parent)

    # Get all xvora-* references from Cargo.toml files
    refs = set()
    for f in XVORA.rglob("Cargo.toml"):
        content = f.read_text(encoding="utf-8")
        for line in content.split("\n"):
            stripped = line.strip()
            if stripped.startswith("#") or stripped.startswith("name = "):
                continue
            m = re.match(r'^(\w+)\s*=', stripped)
            if m:
                refs.add(m.group(1))

    missing_crates = refs - xvora_crates - {"xvora", "rust", "std", "proc-macro2",
        "quote", "syn", "tokio", "serde", "anyhow", "thiserror", "futures",
        "async-trait", "tracing", "log", "bytes", "once_cell", "lazy_static",
        "cfg_if", "pin-project", "tokio-util", "futures-util", "pin-project-lite",
        "futures-core", "futures-task", "futures-sink", "memchr", "itertools",
        "either", "hashbrown", "indexmap", "smallvec", "parking_lot",
        "parking_lot_core", "lock_api", "scopeguard", "windows", "winapi",
        "ctor", "libc", "nix", "hermit-abi", "autocfg", "cc", "shlex",
        "pkg-config", "vcpkg", "cmake", "bindgen", "crossterm", "signal-hook",
        "signal-hook-registry", "mio", "socket2", "num_cpus", "getrandom",
        "rand", "rand_core", "ppv-lite86", "zerocopy", "getrandom",
        "cfg-if", "static_assertions", "subprocess", "serde_json", "serde_yaml",
        "toml", "toml_edit", "serde_spanned", "toml_datetime", "winnow",
        "unicode-id", "pest", "ucd-trie", "thread_local", "typetag",
        "typetag-impl", "inventory", " erased-serde", "bitflags", "strum",
        "strum_macros", "regex", "regex-syntax", "aho-corasick", "memchr",
        "bstr", "globset", "ignore", "walkdir", "same-file", "unicode-segmentation",
        "unicode-width", "unicode-bidi", "unicode-normalization", "unicode-bidi-mirroring",
        "lazy_static", "spin", "arrayref", "base64", "data-encoding", "encoding_rs",
        "webpki-roots", "ring", "rustls", "rustls-pemfile", "rustls-pki-types",
        "sct", "webpki", "log", "zeroize", "const-oid", "der", "pem-rfc7468",
        "spki", "smallvec", "tracing-core", "lazy_static", "cfg-if",
        "tower", "tower-layer", "tower-service", "http", "httparse",
        "http-body", "http-body-util", "itoa", "pin-project", "pin-project-internal",
        "percent-encoding", "form_urlencoded", "url", "idna", "utf-8",
        "utf8_iter", "matches", "fnv", "byteorder", "bytes", "futures-core",
        "futures-task", "futures-util", "futures-sink", "pin-project-lite",
        "slab", "libc", "cfg-if", "mio", "socket2", "parking_lot",
        "parking_lot_core", "smallvec", "lock_api", "scopeguard",
        "instant", "windows-sys", "windows-targets", "windows-link",
        "windows-core", "hermit-abi", "redox_syscall", "bitflags",
        "system-configuration", "core-foundation", "core-foundation-sys",
        "security-framework", "security-framework-sys", "libc", " cfg-if",
        "fuchsia-cprng", "wasm-bindgen", "js-sys", "wasm-bindgen-macro",
        "wasm-bindgen-backend", "wasm-bindgen-macro-support", "bumpalo",
        "proc-macro2", "unicode-ident", "quote", "syn", "once_cell",
        "rand_chacha", "rand_core", "ppv-lite86", "getrandom", "zerocopy",
        "byteorder", "cfg-if", "lazy_static", "static_assertions",
        "autocfg", "cc", "shlex", "find-msvc-tools", "pkg-config",
        "libc", "cfg-if", "jobserver", "clang-sys", "libloading",
        "libc", "cfg-if", "winapi", "lazy_static", "proc-macro2",
        "unicode-ident", "quote", "syn", "cfg-if", "libc", "windows-sys",
        "windows-targets", "windows-link", "redox_syscall", "bitflags",
        "winapi-i686-pc-windows-gnu", "winapi-x86_64-pc-windows-gnu",
        "x86", "ntapi", "winapi", "kernel32", "lazy_static", "winapi-util",
    }

    # Better approach: find path deps that don't exist
    missing = set()
    for f in sorted(XVORA.rglob("Cargo.toml")):
        content = f.read_text(encoding="utf-8")
        for line in content.split("\n"):
            stripped = line.strip()
            if "path = " in stripped and not stripped.startswith("#") and not stripped.startswith("name = "):
                m = re.match(r'^(\w+)\s*=', stripped)
                if m:
                    dep_name = m.group(1)
                    # Check if crate exists
                    for layer in ["codegen", "common", "build", "prod", "tests"]:
                        candidate = XVORA / "crates" / layer / dep_name
                        if candidate.exists() and (candidate / "Cargo.toml").exists():
                            break
                    else:
                        missing.add(dep_name)

    print(f"Missing crates: {missing}")

    # Sync missing crates from grok-build
    MISSING_CRATE_MAP = {
        "xvora-extra-ca": "xai-grok-extra-ca",
        "xvora-active-sessions": "xai-grok-active-sessions",
        "xvora-dashboard-store": "xai-grok-dashboard-store",
        "xvora-diag-server": "xai-grok-diag-server",
        "xvora-session-events": "xai-grok-session-events",
        "xvora-session-search": "xai-grok-session-search",
        "xvora-shell-terminal": "xai-grok-shell-terminal",
        "xvora-status-line": "xai-grok-status-line",
        "xvora-fuzzy-file-search": "xai-fuzzy-file-search",
        "xvora-message-delivery-core": "xai-message-delivery-core",
    }

    for xvora_name, grok_name in MISSING_CRATE_MAP.items():
        src_base = GROK / "crates/codegen" / grok_name
        if not src_base.exists():
            src_base = GROK / "crates/common" / grok_name
        if not src_base.exists():
            print(f"  SKIP {xvora_name}: not found in grok-build")
            continue
        dst_base = XVORA / "crates/codegen" / xvora_name
        if dst_base.exists():
            shutil.rmtree(dst_base)
        shutil.copytree(src_base, dst_base)
        count = 0
        for f in dst_base.rglob("*"):
            if f.is_file():
                try:
                    text = f.read_text(encoding="utf-8")
                    f.write_text(rebrand(text), encoding="utf-8")
                    count += 1
                except UnicodeDecodeError:
                    pass
        print(f"  SYNC: {grok_name} -> {xvora_name} ({count} files)")

    # Step 2: Fix dep key renames in all Cargo.toml files
    print("\n=== Step 2: Fixing dep key renames ===")
    WRONG_KEYS = {
        "xvora-extra-ca": "extra-ca",
        "xvora-active-sessions": "active-sessions",
        "xvora-dashboard-store": "dashboard-store",
        "xvora-diag-server": "diag-server",
        "xvora-session-events": "session-events",
        "xvora-session-search": "session-search",
        "xvora-shell-terminal": "shell-terminal",
        "xvora-status-line": "status-line",
        "xvora-fuzzy-file-search": "fuzzy-file-search",
        "xvora-message-delivery-core": "message-delivery-core",
    }

    for wrong, correct in WRONG_KEYS.items():
        for f in sorted(XVORA.rglob("Cargo.toml")):
            content = f.read_text(encoding="utf-8")
            if wrong in content:
                new_content = content.replace(wrong, correct)
                f.write_text(new_content, encoding="utf-8")
                print(f"  Fixed: {f.relative_to(XVORA)} ({wrong}->{correct})")

    # Step 3: Add missing deps to root Cargo.toml
    print("\n=== Step 3: Updating root Cargo.toml ===")
    grok_deps, _ = parse_deps(GROK / "Cargo.toml")
    xvora_deps, insert_idx = parse_deps(XVORA / "Cargo.toml")

    added = []
    lines = (XVORA / "Cargo.toml").read_text(encoding="utf-8").split("\n")
    for key in sorted(grok_deps.keys()):
        if key in xvora_deps:
            continue
        if key.startswith("xai-"):
            continue
        # Map grok-build dep name to xvora name
        xvora_key = key.replace("xai-", "xvora-")
        if xvora_key in xvora_deps:
            continue
        if key in xvora_deps:
            continue
        dep_line = grok_deps[key]
        # Skip path deps for crates we don't have
        if "path = " in dep_line:
            # Check if the path exists
            m = re.search(r'path\s*=\s*"([^"]+)"', dep_line)
            if m:
                crate_name = m.group(1).split("/")[-1]
                if not (XVORA / "crates" / crate_name).exists():
                    continue
        lines.insert(insert_idx, dep_line)
        insert_idx += 1
        added.append(key)

    (XVORA / "Cargo.toml").write_text("\n".join(lines), encoding="utf-8")
    print(f"  Added {len(added)} deps")

    # Step 4: Add members to root Cargo.toml
    print("\n=== Step 4: Adding members ===")
    content = (XVORA / "Cargo.toml").read_text(encoding="utf-8")
    lines = content.split("\n")
    new_members = []
    for name in ["xvora-extra-ca", "xvora-active-sessions", "xvora-dashboard-store",
                  "xvora-diag-server", "xvora-session-events", "xvora-session-search",
                  "xvora-shell-terminal", "xvora-status-line", "xvora-fuzzy-file-search",
                  "xvora-message-delivery-core"]:
        member = f'    "crates/codegen/{name}",'
        if member not in content:
            new_members.append(member)
    if new_members:
        for i, line in enumerate(lines):
            if '"crates/codegen/xvora-workspace-types"' in line:
                for m in new_members:
                    lines.insert(i+1, m)
                    i += 1
                break
        (XVORA / "Cargo.toml").write_text("\n".join(lines), encoding="utf-8")
        print(f"  Added {len(new_members)} members")

    print("\nDone! Run 'cargo check' to verify.")

if __name__ == "__main__":
    main()
