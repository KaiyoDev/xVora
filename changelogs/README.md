# User update channel (`changelogs/`)

This folder is the **single source of truth** for what end-users see when
xVora starts or upgrades.

| File | Role |
|------|------|
| `{version}.external.json` | Welcome bullets + What's-new (array of entries) |
| `{version}.external.md` | Full notes (`/release-notes`) |
| `CURRENT.external.*` | Mirror of the version in `crates/codegen/xvora-version` (embedded in binary) |
| `manifest.json` | Index of published versions (for tooling / future multi-version UI) |

## Entry shape (JSON)

```json
{
  "category": "features | fixes | breaking | performance | docs",
  "description": "User-facing one-liner (English; VI UI labels are separate i18n).",
  "breaking_change": false
}
```

## Remote URL (after push to `main`)

```
https://raw.githubusercontent.com/KaiyoDev/xVora/main/changelogs/{version}.external.json
```

The binary also **embeds** `CURRENT.external.json`, so users still get notes
offline / before GitHub propagates.

## Maintainer workflow

```powershell
.\scripts\changelog.ps1 add -Category fixes -Description "Fixed Language persist."
.\scripts\changelog.ps1 sync
```

Agents and humans: **every user-visible change** → one `add` (or batch) before commit.
Do **not** leave only code comments — users never see those.
