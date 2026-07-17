# Changelog

All notable user-facing changes to **xVora** are documented here and in
[`changelogs/`](./changelogs/) (JSON + markdown per version).

**How users see updates**

1. Welcome screen Changelog bullets
2. Toast **Whats new** when the installed version differs from last launch
3. Slash `/release-notes` (full markdown when available)

**Maintainer**

```powershell
.\scripts\changelog.ps1 add -Category features -Description "..."
.\scripts\changelog.ps1 sync
```

---

## [0.2.0] - 2026-07-17

# 0.2.0

## Features

- System prompt identity is xVora by KaiyoDev (not Grok / xAI).
- Settings Language picker (en / vi / auto) with full Vietnamese UI.
- BYOK-first: no forced xAI OAuth splash; optional login only.
- Multi-provider model catalog: xAI is one provider; /model shows `provider · name` when mixed.
- Welcome logo from brand mark (braille + orange tint).
- Per-version user changelog with auto Whats-new toast on upgrade.

## Bug Fixes

- Language setting persists to [ui].language (no longer rolls back to auto).
- Eager auth never auto-starts accounts.x.ai device OAuth.
- Quit / relaunch hints say xvora --resume (not grok).
- Updates and install point at KaiyoDev/xVora GitHub, not x.ai/cli or grok-build CDN.

## Docs

- User changelog channel (changelogs/) with Whats-new toast on version change.
