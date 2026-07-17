# 0.2.0

## Features

- System prompt identity is xVora by KaiyoDev (not Grok / xAI).
- Settings Language picker (en / vi / auto) with full Vietnamese UI.
- BYOK-first: no forced xAI OAuth splash; optional login only.
- Welcome logo from brand mark (braille + orange tint).
- Per-version user changelog with auto Whats-new toast on upgrade.

## Bug Fixes

- Language setting persists to [ui].language (no longer rolls back to auto).
- Eager auth never auto-starts accounts.x.ai device OAuth.
- Quit / relaunch hints say xvora --resume (not grok).
- Updates and install point at KaiyoDev/xVora GitHub, not x.ai/cli or grok-build CDN.

## Docs

- User changelog channel (changelogs/) with Whats-new toast on version change.
