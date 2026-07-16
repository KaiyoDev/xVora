<div align="center">

# xVora

**Terminal AI coding agent** — full-screen TUI, headless automation, and ACP for editors.

Bring your own keys. Run local models. No forced cloud lock-in.

[Features](#features) ·
[Install](#install) ·
[Quick start](#quick-start) ·
[BYOK / custom models](#byok--custom-models) ·
[Build from source](#build-from-source) ·
[Docs](#documentation) ·
[Layout](#repository-layout) ·
[License](#license)

</div>

---

## What is xVora?

**xVora** (`xvora`) is an open terminal coding agent:

- Interactive **TUI** for day-to-day coding
- **Headless** mode for scripts and CI
- **Agent / ACP** mode for IDE integration
- Built-in tools: shell, read/edit files, search, web, subagents, skills, MCP, sandbox

Config lives under `~/.xvora/`. Environment variables use the `XVORA_*` prefix.

> This tree is maintained as **[KaiyoDev/xVora](https://github.com/KaiyoDev/xVora)**.
> It is a hard fork / rebrand of an open-sourced coding harness; first-party code is Apache-2.0.

---

## Features

| Area | Notes |
|------|--------|
| **Pure BYOK** | OpenAI / Anthropic / OpenRouter / Ollama / any OpenAI-compatible endpoint |
| **No paywall for BYOK** | Custom keys & local endpoints skip subscription gates |
| **Tools** | bash, read, edit, grep, list_dir, web, tasks/subagents, MCP, plan mode |
| **Skills & hooks** | Project and user skill packs; lifecycle hooks |
| **Worktrees** | Isolated worktrees for parallel agents |
| **Sandbox** | Optional OS-level isolation (where supported) |

---

## Install

### From source (recommended)

```sh
git clone https://github.com/KaiyoDev/xVora.git
cd xVora
cargo run -p xvora-pager-bin
```

Release binary:

```sh
cargo build -p xvora-pager-bin --release
# → target/release/xvora   (or xvora.exe on Windows)
./target/release/xvora --version
```

### Requirements

- **Rust** — pinned in [`rust-toolchain.toml`](rust-toolchain.toml) (`rustup` installs it)
- **protoc** — [`bin/protoc`](bin/protoc) (dotslash) or `protoc` on `PATH` / `$PROTOC`
- Linux / macOS are the primary hosts; Windows is best-effort

---

## Quick start

```sh
# Interactive TUI
xvora

# One-shot headless prompt
xvora -p "Explain this repository"

# Agent mode (stdio / ACP)
xvora agent stdio
```

First launch may open a browser for auth if you use cloud providers that need login.
For **pure BYOK**, configure `~/.xvora/config.toml` first (see below) — no forced cloud login.

---

## BYOK / custom models

Create `~/.xvora/config.toml`:

```toml
[models]
default = "openai-gpt"

[model.openai-gpt]
model = "gpt-4o"
base_url = "https://api.openai.com/v1"
api_key = "sk-..."                    # or: env_key = "OPENAI_API_KEY"
api_backend = "chat_completions"      # chat_completions | responses | messages
context_window = 128000
name = "GPT-4o"
```

Local Ollama (no API key):

```toml
[models]
default = "ollama"

[model.ollama]
model = "llama3"
base_url = "http://127.0.0.1:11434/v1"
context_window = 128000
```

With a custom model + key (or non-xAI `base_url`), xVora runs **without** subscription paywall.

Full guide: [`crates/codegen/xvora-pager/docs/user-guide/11-custom-models.md`](crates/codegen/xvora-pager/docs/user-guide/11-custom-models.md)

---

## Build from source

```sh
cargo check -p xvora-pager-bin          # fast validation
cargo run   -p xvora-pager-bin          # dev TUI
cargo build -p xvora-pager-bin --release
cargo test  -p xvora-config             # example per-crate tests
cargo clippy -p xvora-pager-bin
cargo fmt --all
```

Always target **specific crates** (`-p …`). Full workspace builds are large and slow.

Binary package: **`xvora-pager-bin`** → artifact name **`xvora`**.

---

## Documentation

User guide (shipped with the pager crate):

[`crates/codegen/xvora-pager/docs/user-guide/`](crates/codegen/xvora-pager/docs/user-guide/)

Topics: getting started, auth, shortcuts, slash commands, config, theming, MCP, skills, plugins, hooks, headless, sandbox, permissions, custom models.

Shell/agent deep dive:

[`crates/codegen/xvora-shell/README.md`](crates/codegen/xvora-shell/README.md)

---

## Repository layout

| Path | Role |
|------|------|
| `crates/codegen/xvora-pager-bin` | Composition root — builds the `xvora` binary |
| `crates/codegen/xvora-pager` | TUI (scrollback, prompt, modals, views) |
| `crates/codegen/xvora-shell` | Agent runtime, auth, sessions, leader/stdio/headless |
| `crates/codegen/xvora-tools` | Tool implementations |
| `crates/codegen/xvora-workspace` | Host FS, VCS, execution, checkpoints |
| `crates/codegen/xvora-config` | Config load / `~/.xvora` |
| `crates/codegen/…` | MCP, markdown, sandbox, memory, sampler, … |
| `crates/common/` | Shared protocol, tool runtime, compaction, tracing |
| `crates/build/` | Build helpers (e.g. protoc) |
| `third_party/` | Vendored Mermaid → SVG stack |

> Root `Cargo.toml` workspace members and shared dependency versions may be treated as generated in upstream workflows — prefer editing **per-crate** `Cargo.toml` files when unsure.

---

## Configuration paths

| Item | Location |
|------|----------|
| Config | `~/.xvora/config.toml` |
| Auth | `~/.xvora/auth.json` |
| Sessions / data | `~/.xvora/` |
| Env prefix | `XVORA_*` |

---

## Development notes

```sh
cargo check -p <crate>
cargo test  -p <crate>
cargo clippy -p <crate>
```

Rust edition and channel: see [`rust-toolchain.toml`](rust-toolchain.toml).

---

## Contributing

Issues and PRs are welcome on **[KaiyoDev/xVora](https://github.com/KaiyoDev/xVora)**.

Please keep changes focused; run `cargo check -p …` on touched crates before opening a PR.

---

## License

First-party code: **Apache License 2.0** — see [`LICENSE`](LICENSE).

Third-party / vendored code keeps its original licenses — see [`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES) and notices under individual crates / `third_party/`.

---

## Acknowledgments

xVora builds on ideas and open-sourced components from the broader coding-agent ecosystem. See third-party notices for attribution.
