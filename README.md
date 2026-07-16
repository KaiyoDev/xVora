<div align="center">

<!-- Full-width banner (logo already includes wordmark) -->
<img src="assets/logo.png" alt="xVora Code" width="100%">

<br>

### xVora Code — Terminal AI, BYOK-first

TUI · Headless · ACP · Bring your own models

<br>

[![CI](https://github.com/KaiyoDev/xVora/actions/workflows/ci.yml/badge.svg)](https://github.com/KaiyoDev/xVora/actions/workflows/ci.yml)
&nbsp;
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
&nbsp;
[![Release](https://img.shields.io/github/v/release/KaiyoDev/xVora?include_prereleases&label=release)](https://github.com/KaiyoDev/xVora/releases)

<br>

[Install](#install)
&nbsp;·&nbsp;
[Quick start](#quick-start)
&nbsp;·&nbsp;
[BYOK](#byok--custom-models)
&nbsp;·&nbsp;
[CI binary](#ci--download-binary)
&nbsp;·&nbsp;
[Docs](#documentation)
&nbsp;·&nbsp;
[License](#license)

</div>

---

## Overview

**xVora** (`xvora`) is an open-source **terminal coding agent** — [KaiyoDev/xVora](https://github.com/KaiyoDev/xVora).

| Mode | What it does |
|------|----------------|
| **TUI** | Full-screen interactive coding session |
| **Headless** | Scripts & CI — `xvora -p "…"` |
| **Agent / ACP** | Editor integration — `xvora agent stdio` |

**Bring your own key.** OpenAI · Anthropic · OpenRouter · Ollama · any OpenAI-compatible API.  
Pure BYOK setups skip subscription paywalls.

```
~/.xvora/          # config, auth, sessions
XVORA_*            # environment prefix
```

---

## Install

### From source

```sh
git clone https://github.com/KaiyoDev/xVora.git
cd xVora
cargo run -p xvora-pager-bin
```

Release build:

```sh
cargo build -p xvora-pager-bin --release
# → target/release/xvora   (xvora.exe on Windows)
```

### Requirements

- **Rust** — see [`rust-toolchain.toml`](rust-toolchain.toml) (installed by `rustup`)
- **protoc** — [`bin/protoc`](bin/protoc) or system `protoc` / `$PROTOC`
- Primary hosts: Linux & macOS · Windows: best-effort

---

## Quick start

```sh
xvora                              # interactive TUI
xvora -p "Explain this repo"       # headless
xvora agent stdio                  # ACP / agent mode
```

Prefer BYOK first? Write `~/.xvora/config.toml` (below) **before** first launch — no login required for custom endpoints.

---

## BYOK / custom models

```toml
# ~/.xvora/config.toml

[models]
default = "openai-gpt"

[model.openai-gpt]
model = "gpt-4o"
base_url = "https://api.openai.com/v1"
api_key = "sk-..."                 # or: env_key = "OPENAI_API_KEY"
api_backend = "chat_completions"   # chat_completions | responses | messages
context_window = 128000
name = "GPT-4o"
```

Local (e.g. Ollama):

```toml
[models]
default = "ollama"

[model.ollama]
model = "llama3"
base_url = "http://127.0.0.1:11434/v1"
context_window = 128000
```

If you set a custom model with credentials or a non-first-party `base_url`, xVora **skips subscription paywalls**.

Guide: [`crates/codegen/xvora-pager/docs/user-guide/11-custom-models.md`](crates/codegen/xvora-pager/docs/user-guide/11-custom-models.md)

---

## Features

- **Tools** — shell, read/edit, search, list dir, web, tasks & subagents, MCP
- **Skills & hooks** — reusable packs and project lifecycle scripts
- **Worktrees** — isolated workspaces for parallel agents
- **Sandbox** — optional OS-level isolation (where supported)
- **Theming** — TUI themes, welcome logo (braille), animations

---

## CI / download binary (Windows)

Workflow: [`.github/workflows/ci.yml`](.github/workflows/ci.yml)

On push to `main` (and PRs):

1. `cargo check` + `cargo build --release -p xvora-pager-bin` on **`windows-latest`**
2. Upload artifact **`xvora-win-x64`** (`xvora.exe`)
3. **Only the newest run stays active** (`cancel-in-progress`)
4. Older completed runs of this workflow are cleaned up

**Download:** [Actions → CI → latest green run → Artifacts → `xvora-win-x64`](https://github.com/KaiyoDev/xVora/actions)

```powershell
.\xvora.exe --version
```

---

## Build notes

```sh
cargo check  -p xvora-pager-bin
cargo run    -p xvora-pager-bin
cargo build  -p xvora-pager-bin --release
cargo test   -p xvora-config
cargo clippy -p xvora-pager-bin
cargo fmt --all
```

Always pass **`-p <crate>`** — a full workspace build is large and slow.

| Crate | Role |
|-------|------|
| `xvora-pager-bin` | Binary entry (`xvora`) |
| `xvora-pager` | TUI |
| `xvora-shell` | Agent runtime |
| `xvora-tools` | Tools |
| `xvora-workspace` | FS / VCS / execution |
| `xvora-config` | Config & `~/.xvora` |

More layout detail under `crates/codegen/` and `crates/common/`.

---

## Documentation

| Doc | Path |
|-----|------|
| User guide | [`crates/codegen/xvora-pager/docs/user-guide/`](crates/codegen/xvora-pager/docs/user-guide/) |
| Shell / agent | [`crates/codegen/xvora-shell/README.md`](crates/codegen/xvora-shell/README.md) |

---

## Paths

| Item | Location |
|------|----------|
| Config | `~/.xvora/config.toml` |
| Auth | `~/.xvora/auth.json` |
| Data / sessions | `~/.xvora/` |
| Env prefix | `XVORA_*` |

---

## Contributing

Issues and PRs: **[github.com/KaiyoDev/xVora](https://github.com/KaiyoDev/xVora)**

Keep changes focused; run `cargo check -p …` on touched crates before opening a PR.

---

## License

**Apache License 2.0** — see [`LICENSE`](LICENSE).

Copyright SpaceXAI (upstream) and KaiyoDev (xVora modifications).

Third-party notices: [`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES).

---

## Acknowledgments

xVora is derived from **[Grok Build](https://github.com/xai-org/grok-build)** — thank you to the authors and contributors of that open-source project.
