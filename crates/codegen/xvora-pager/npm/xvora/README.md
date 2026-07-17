# Xvora

Bring xVora into your terminal. Fast, flicker-free CLI built for plans, subagents, and parallel work.

**[GitHub](https://github.com/KaiyoDev/xVora)** | **[Releases](https://github.com/KaiyoDev/xVora/releases)**

## Install

```bash
npm i -g @kaiyodev/xvora
```

Or download a binary from [GitHub Releases / CI artifacts](https://github.com/KaiyoDev/xVora/releases).

## Get Started

```bash
# Launch the interactive TUI
xvora

# Run a single task
xvora -p "Explain this codebase"
```

xVora is **BYOK-first** — no forced xAI login. Configure models in `~/.xvora/config.toml` or set provider API keys (e.g. `XAI_API_KEY`, `OPENAI_API_KEY`).

## Update

```bash
xvora update
```

Updates resolve from **GitHub Releases** (`KaiyoDev/xVora`), not x.ai/cli.
