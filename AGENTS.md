# AGENTS.md — xVora Project Notes

## Project Identity
- **Product name:** xVora (not Grok, not xAI)
- **Repository:** KaiyoDev/xVora
- **Binary name:** `xvora` (CLI command)
- **Config dir:** `~/.xvora/` (env: `$XVORA_HOME`)
- **Env vars:** `XVORA_*` prefix (e.g. `XVORA_LANG`, `XVORA_VERSION`)
- **Model names** like `grok-4.5`, `grok-4.6` are real upstream model IDs — keep as-is

---

## Build & Test Policy

> **All builds and tests run on GitHub Actions (`windows-latest`). Local builds are not required.**

---

## i18n (Internationalization)

### Current state
- **Stub → full implementation** completed in this session
- `crates/codegen/xvora-pager/src/i18n.rs` — locale engine with `Locale::En` / `Locale::Vi`
- Generated files in `crates/codegen/xvora-pager/src/i18n/`:
  - `settings.rs` — labels, descriptions, categories
  - `slash.rs` — slash command descriptions
  - `enums.rs` — enum choice displays (theme, permission mode, etc.)
  - `chrome.rs` — UI chrome (footer, toasts, tips)
  - `actions_long_help.rs` — action cheatsheet hints
- Scripts: `scripts/gen_i18n_settings_slash.py`, `scripts/gen_i18n_full_vi.py`
- Run scripts after editing their source dicts to regenerate

### Language switching
- Setting `ui.language` in `/settings` → `Action::SetLocale` → `i18n::set_locale()`
- Change takes effect **immediately** (no restart needed)
- Env override: `XVORA_LANG=vi` or `GROK_LANG=vi` at startup

---

## Auto-Compact

### Current state
- **NOT exposed in UI settings** — only configurable via `config.toml`
- Config keys:
  - `session.auto_compact_threshold_percent` (default 85)
  - `model.<id>.auto_compact_threshold_percent` (per-model override)
- To add to UI: add `SettingMeta` entry in `crates/codegen/xvora-pager/src/settings/defs.rs`

### How it works
- Triggers at `threshold_percent` of context window usage
- Prefire starts at `threshold - 10%` (background pass-1)
- Two-pass: pass-1 summarizes prefix in background, pass-2 applies cached summary
- Suppression states: `SUPPRESS_TURN`, `SUPPRESS_STICKY`, `SUPPRESS_UNTIL_SUCCESS`, `SUPPRESS_AUTH`

---

## Code Conventions

### Rust
- Workspace root: `D:\Kaiyo\Project\xVora`
- Main binary crate: `crates/codegen/xvora-pager-bin`
- TUI crate: `crates/codegen/xvora-pager`
- Shell/backend crate: `crates/codegen/xvora-shell`
- Config crate: `crates/codegen/xvora-config`
- Rust toolchain: `1.94.1` (see `rust-toolchain.toml`)

### Docs
- User guide: `crates/codegen/xvora-pager/docs/user-guide/`
- All docs use `xvora` (lowercase) for CLI commands, `xVora` (capital V) for product name
- Model names like `grok-4.5` stay unchanged (real upstream IDs)
- After editing `scripts/gen_i18n_*.py`, run them to regenerate `src/i18n/*.rs`

### Changelog
- Maintain entries in `changelogs/CURRENT.external.json` (machine-readable)
- Sync to `CHANGELOG.md` with: `.\scripts\changelog.ps1 sync`
- Add entry: `.\scripts\changelog.ps1 add -Category features -Description "..."`

---

## Key Architecture

```
xvora-pager-bin      ← CLI entry point (main.rs)
  └─ xvora-pager     ← TUI application (app/, views/, settings/, i18n/)
       └─ xvora-shell ← Agent runtime, session management, compaction
            └─ xvora-config ← Config loading, path resolution (~/.xvora/)
            └─ xvora-dirs   ← Home directory resolution ($XVORA_HOME / ~/.xvora/)
```
