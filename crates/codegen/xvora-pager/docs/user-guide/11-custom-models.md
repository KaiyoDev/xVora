# Providers & custom models

xVora is **multi-provider**: each model belongs to a provider (`xai`, `openai`, `ollama`, `custom`, …). **xAI is one provider** among others — not the product itself.

This guide covers selecting models, BYOK, credential rules, and third-party endpoints.

## Pure BYOK (no xAI login)

If you configure **any** of the following, xVora does not force xAI OAuth:

- A `[model.*]` entry with `api_key` / `env_key`
- A `[model.*]` entry with a non-xAI `base_url` (Ollama, OpenRouter, OpenAI, …)
- Global `XAI_API_KEY` (or legacy `XVORA_CODE_XAI_API_KEY`) — only used for **xAI provider** models
- Custom models endpoint (`[endpoints] models_base_url` / `models_list_url`)

Point `models.default` at your BYOK model and start coding.

```toml
[models]
default = "openai-gpt"

[model.openai-gpt]
model = "gpt-4o"
base_url = "https://api.openai.com/v1"
api_key = "sk-..."            # or env_key = "OPENAI_API_KEY"
api_backend = "chat_completions"
context_window = 128000
name = "GPT-4o"
provider = "openai"           # optional; inferred from base_url when omitted
```

---

## Providers

| Provider id | Typical use | Auth |
|-------------|-------------|------|
| `xai` | Built-in first-party models, `api.x.ai` / cli-chat-proxy | OAuth (`xvora login`), `XAI_API_KEY`, or per-model key |
| `openai` | OpenAI-compatible hosts | Per-model `api_key` / `env_key` |
| `anthropic` | Anthropic Messages API | `extra_headers` / key as documented below |
| `ollama` | Local `localhost` / Ollama | Usually none |
| `openrouter` | OpenRouter | Per-model key |
| `custom` | Anything else | Per-model credentials |

### How provider is chosen

1. Explicit `provider = "..."` on `[model.*]`
2. Else inferred from `base_url` / `api_base_url` (e.g. `api.openai.com` → `openai`, `api.x.ai` → `xai`)
3. Else slug heuristics (`grok-*` → `xai`)
4. Else `custom`

### UI

- **Model picker** (`Ctrl+M` from scrollback) and **`/model`**: when more than one provider is present, rows show `provider · Name` (e.g. `openai · GPT-4o`). Filter by typing the provider id.
- Catalog order is **by provider, then name**.

---

## Default models

Built-in catalog entries (e.g. `xvora`) are tagged `provider = "xai"` and use xAI provider default endpoints unless you override them. New sessions use `models.default` when set.

List models:

```bash
xvora models
```

---

## Selecting a model

### CLI

```bash
xvora -p "Hello" -m xvora
```

### Slash

```
/model xvora
/m gpt-4o
```

### Picker

`Ctrl+M` from scrollback (with the prompt focused, `Ctrl+M` toggles multiline — use `/model` instead).

### Config default

```toml
[models]
default = "openai-gpt"
```

---

## Supported API backends

Set `api_backend` on each `[model.*]`:

| Value | API | Default |
|-------|-----|---------|
| `"chat_completions"` | OpenAI Chat Completions (`/v1/chat/completions`) | Yes |
| `"responses"` | OpenAI Responses (`/v1/responses`) | |
| `"messages"` | Anthropic Messages (`/v1/messages`) | |

Provider-specific headers (e.g. Anthropic `x-api-key`) go in `extra_headers`.

---

## Configuring custom models

```toml
[model.my-model]
model = "model-id"
base_url = "https://api.example.com/v1"
name = "Display Name"
description = "Optional"
provider = "custom"                   # optional
api_key = "sk-..."
env_key = "MY_PROVIDER_KEY"           # string or array (first non-empty wins)
api_backend = "chat_completions"
temperature = 0.7
top_p = 0.95
max_completion_tokens = 8192
context_window = 128000
extra_headers = { "x-api-key" = "sk-..." }
```

### Credential resolution

1. Model `api_key`
2. Model `env_key` (first set, non-empty)
3. **Only if `provider` is `xai`:** signed-in session token (`xvora login`)
4. **Only if `provider` is `xai`:** global `XAI_API_KEY` / `XVORA_CODE_XAI_API_KEY`

Non-xAI models **never** receive an xAI OAuth JWT or global xAI key. Configure BYOK on the model (or its `env_key`).

### Context window

Set `context_window` to match the provider. New models without it default to 200,000 tokens for auto-compaction math.

### Global defaults

```toml
[models]
extra_headers = { "X-Request-Tags" = "team=example,env=prod" }
temperature = 0.7
top_p = 0.95
max_completion_tokens = 8192
```

Per-model values always win over `[models]` defaults.

---

## Overriding built-in (xAI) models

```toml
[model.xvora]
api_key = "my-api-key"
temperature = 0.5
```

Priority: your `[model.*]` > remote prefetched list > built-in defaults.

---

## Provider examples

### Anthropic (Claude)

```toml
[model.claude-opus]
model = "claude-opus-4-6"
base_url = "https://api.anthropic.com/v1"
name = "Claude Opus 4.6"
provider = "anthropic"
api_backend = "messages"
context_window = 200000
extra_headers = { "x-api-key" = "sk-ant-...", "anthropic-version" = "2023-06-01" }
```

### OpenAI

```toml
[model.gpt-4o]
model = "gpt-4o"
base_url = "https://api.openai.com/v1"
name = "GPT-4o"
provider = "openai"
env_key = "OPENAI_API_KEY"
context_window = 128000
```

### Ollama (local)

```toml
[models]
default = "ollama"

[model.ollama]
model = "llama3"
base_url = "http://127.0.0.1:11434/v1"
provider = "ollama"
context_window = 128000
```

### xAI (explicit)

```toml
[model.my-grok]
model = "grok-4"
base_url = "https://api.x.ai/v1"
provider = "xai"
env_key = "XAI_API_KEY"
context_window = 256000
```

Or use `xvora login` / `XAI_API_KEY` with built-in xAI catalog entries.

---

## Endpoints (xAI provider only)

First-party defaults (`cli-chat-proxy`, `api.x.ai`, assets) live under the **xAI provider** (see `xvora_env::xai_provider`). Override when needed:

```toml
[endpoints]
xai_api_base_url = "https://api.x.ai/v1"
# cli_chat_proxy_base_url = "https://cli-chat-proxy.example/v1"
```

BYOK models use their own `base_url` and do not require these hosts.
