# Web Search Tool Cho xVora — Multi-Provider Pattern

**Nguồn cảm hứng:** [ultimate-web-search-skill](https://github.com/code-yeongyu/ultimate-web-search-skill/blob/main/SKILL.md)

---

## 1. Vấn Đề Hiện Tại

xVora hiện có `web_search` tool nhưng **chỉ gọi xAI Responses API** — phụ thuộc vào xAI key, không có fallback.

```
Hiện tại: web_search → xAI Responses API (api.x.ai)
         ↓ fail / 401
         ✗ Không có fallback
```

SKILL.md từ GitHub giải quyết bằng cách:
- Hỗ trợ **12 providers** (DuckDuckGo free, Tavily, Brave, Perplexity, Serper...)
- Fallback chain tự động
- Zero-config (DuckDuckGo hoạt động không cần key)
- Kết quả JSON lưu file để pipe qua jq/rg

---

## 2. Thiết Kế Multi-Provider Search Tool Cho xVora

### Provider Matrix

| Provider | Auth | Cost | Best For |
|---|---|---|---|
| `duckduckgo` | none | free | Quick factual, always works |
| `mwmbl` | none | free | Free fallback for real links |
| `tavily` | key | free tier | LLM-friendly clean snippets |
| `brave` | key | paid | Privacy, independent index |
| `serper` | key | cheap (~$3/1K) | Google organic results |
| `google-cse` | key+cse_id | free 100/day | Official Google results |
| `exa` | key | paid | Semantic search |
| `z-ai` | key | paid | Chinese queries |
| `perplexity` | key | paid (small tier) | Recency-filtered |
| `xai` | key | pay-per-call | Grok web_search (existing) |
| `openai` | key | paid | GPT-synthesized search |
| `anthropic` | key | paid | Claude-synthesized search |

### Fallback Chain Logic

```
Search called
    ↓
Try provider[0] (configured default or "xai")
    ↓ success → return result
    ↓ 4xx/5xx/rate_limit → try next
    ↓
Try provider[1] (fallback[0])
    ↓ ...
    ↓
All failed → return combined error with partial results
```

---

## 3. Implementation Plan

### 3.1 Directory Structure

```
crates/codegen/xvora-tools/src/implementations/
├── web_search/                # existing — xAI Responses API only
│   ├── mod.rs
│   ├── tool.rs
│   ├── client.rs
│   └── types.rs
└── xvora_web_search/          # NEW — multi-provider
    ├── mod.rs
    ├── tool.rs
    ├── client.rs
    ├── providers/
    │   ├── mod.rs
    │   ├── duckduckgo.rs
    │   ├── tavily.rs
    │   ├── brave.rs
    │   ├── serper.rs
    │   └── base.rs
    └── types.rs
```

### 3.2 Types

```rust
// crates/codegen/xvora-tools/src/implementations/xvora_web_search/types.rs
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

/// Provider identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchProvider {
    DuckDuckGo,
    Mwmbl,
    Tavily,
    Brave,
    Serper,
    GoogleCse,
    Exa,
    ZaI,
    Perplexity,
    Xai,
    OpenAi,
    Anthropic,
}

impl Default for SearchProvider {
    fn default() -> Self { Self::DuckDuckGo }
}

impl std::fmt::Display for SearchProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self).to_lowercase()
    }
}

/// Configuration for all providers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct XvoraWebSearchConfig {
    /// Primary provider (tried first)
    #[serde(default)]
    pub primary: SearchProvider,
    /// Fallback providers in order
    #[serde(default)]
    pub fallback: Vec<SearchProvider>,
    /// Provider-specific API keys
    #[serde(default)]
    pub providers: IndexMap<String, ProviderConfig>,
    /// Domain filters (applied to all providers that support it)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded_domains: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    /// For Google CSE
    pub cse_id: Option<String>,
    /// Override base URL for self-hosted / proxy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

/// Search input (same schema as existing web_search for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct XvoraWebSearchInput {
    pub query: String,
    #[schemars(description = "Optional list of domains to restrict search to.")]
    pub allowed_domains: Option<Vec<String>>,
    #[schemars(description = "Max results per provider (default 10).")]
    pub max_results: Option<u8>,
}

/// Output format compatible with existing xVora web_search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XvoraWebSearchOutput {
    pub query: String,
    pub content: String,
    pub citations: Vec<String>,
    pub allowed_domains: Option<Vec<String>>,
    /// Which provider(s) were tried
    pub providers_tried: Vec<String>,
    /// Success provider name
    pub provider: String,
}
```

### 3.3 Provider Trait

```rust
// crates/codegen/xvora-tools/src/implementations/xvora_web_search/providers/base.rs
use async_trait::async_trait;
use super::super::types::{XvoraWebSearchInput, XvoraWebSearchOutput};

#[async_trait]
pub trait WebSearchProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    async fn search(&self, input: &XvoraWebSearchInput)
        -> Result<XvoraWebSearchOutput, SearchError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("API key required for {0}")]
    MissingKey(String),
    #[error("HTTP {0}: {1}")]
    Http(u16, String),
    #[error("Rate limited")]
    RateLimited,
    #[error("No results found")]
    NoResults,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SearchError {
    /// Returns true if this error is retryable (rate limit, timeout, 5xx)
    pub fn is_retryable(&self) -> bool {
        matches!(self, SearchError::Http(429, _) | SearchError::Http(500.., _) | SearchError::Other(_))
    }
}
```

### 3.4 DuckDuckGo Provider (Zero Config)

```rust
// crates/codegen/xvora-tools/src/implementations/xvora_web_search/providers/duckduckgo.rs

use super::{SearchError, WebSearchProvider};
use crate::implementations::xvora_web_search::types::*;

pub struct DuckDuckGoProvider;

#[derive(Debug, Deserialize)]
struct DdgResult {
    text: String,
    url: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct DdgResponse {
    results: Vec<DdgResult>,
}

#[async_trait]
impl WebSearchProvider for DuckDuckGoProvider {
    fn name(&self) -> &str { "duckduckgo" }

    fn is_available(&self) -> bool { true }  // always available, no key needed

    async fn search(&self, input: &XvoraWebSearchInput) -> Result<XvoraWebSearchOutput, SearchError> {
        let query = urlencoding::encode(&input.query);
        let url = format!("https://html.duckduckgo.com/html/?q={query}");

        let resp = reqwest::Client::new()
            .get(&url)
            .header("User-Agent", "xVora-Agent/1.0")
            .send()
            .await
            .map_err(SearchError::from)?;

        let html = resp.text().await.map_err(SearchError::from)?;
        let results = parse_ddg_html(&html, input.max_results.unwrap_or(10) as usize);

        let citations: Vec<String> = results.iter().map(|r| r.url.clone()).collect();
        let content = format_results_text(&results);

        Ok(XvoraWebSearchOutput {
            query: input.query.clone(),
            content,
            citations,
            allowed_domains: input.allowed_domains.clone(),
            providers_tried: vec!["duckduckgo".to_string()],
            provider: "duckduckgo".to_string(),
        })
    }
}

fn parse_ddg_html(html: &str, limit: usize) -> Vec<DdgResult> {
    // Parse <a class="result__a">, <a class="result__snippet">, etc.
    // Using regex for simplicity (can switch to html5ever for robustness)
    let mut results = Vec::new();

    let title_re = regex::Regex::new(r#"<a[^>]*class="result__a"[^>]*href="([^"]+)"[^>]*>(.*?)</a>"#).unwrap();
    let snippet_re = regex::Regex::new(r#"<a[^>]*class="result__snippet"[^>]*>(.*?)</a>"#).unwrap();

    for cap in title_re.captures_iter(html) {
        let url = &cap[1];
        let title = clean_html(&cap[2]);
        if results.len() >= limit { break; }

        // Find matching snippet
        let snippet = snippet_re
            .captures_after(html, &cap)
            .and_then(|c| Some(clean_html(&c[1])))
            .unwrap_or_default();

        results.push(DdgResult {
            text: snippet,
            url: url.to_string(),
            title: title.to_string(),
        });
    }
    results
}

fn clean_html(s: &str) -> String {
    s.replace("<b>", "")
     .replace("</b>", "")
     .replace("&nbsp;", " ")
     .replace("&amp;", "&")
     .replace("&gt;", ">")
     .replace("&lt;", "<")
     .replace('\"', "\"")
}

fn format_results_text(results: &[DdgResult]) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }
    let mut text = String::from("Search Results:\n\n");
    for (i, r) in results.iter().enumerate() {
        text.push_str(&format!(
            "[{}] {} — {}\n    {}\n\n",
            i + 1,
            r.title,
            r.url,
            r.text
        ));
    }
    text
}
```

### 3.5 Tavily Provider (Free Tier)

```rust
// crates/codegen/xvora-tools/src/implementations/xvora_web_search/providers/tavily.rs

use super::{SearchError, WebSearchProvider};
use crate::implementations::xvora_web_search::types::*;
use indexmap::IndexMap;

pub struct TavilyProvider {
    api_key: String,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct TavilyResult {
    title: String,
    url: String,
    content: String,
    score: f64,
}

#[derive(Debug, Deserialize)]
struct TavilyResponse {
    results: Vec<TavilyResult>,
    answer: Option<String>,
}

impl TavilyProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl WebSearchProvider for TavilyProvider {
    fn name(&self) -> &str { "tavily" }

    fn is_available(&self) -> bool { !self.api_key.is_empty() }

    async fn search(&self, input: &XvoraWebSearchInput) -> Result<XvoraWebSearchOutput, SearchError> {
        if self.api_key.is_empty() {
            return Err(SearchError::MissingKey("tavily".to_string()));
        }

        let mut headers = IndexMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let body = serde_json::json!({
            "query": input.query,
            "max_results": input.max_results.unwrap_or(10),
            "search_depth": "basic",
            "include_answer": true,
            "topic": "general"
        });

        let resp = self.http
            .post("https://api.tavily.com/search")
            .headers(headers.clone())
            .json(&body)
            .send()
            .await
            .map_err(SearchError::from)?;

        if resp.status() == 401 {
            return Err(SearchError::Http(401, "Invalid Tavily API key".to_string()));
        }
        if resp.status() == 429 {
            return Err(SearchError::RateLimited);
        }
        if !resp.status().is_success() {
            let body = resp.text().await.map_err(SearchError::from)?;
            return Err(SearchError::Http(resp.status().as_u16(), body));
        }

        let tavily_resp: TavilyResponse = resp.json().await.map_err(SearchError::from)?;

        let citations: Vec<String> = tavily_resp.results.iter().map(|r| r.url.clone()).collect();
        let content = match tavily_resp.answer {
            Some(a) => format!("Answer: {a}\n\nResults:\n{}", format_tavily_results(&tavily_resp.results)),
            None => format_tavily_results(&tavily_resp.results),
        };

        Ok(XvoraWebSearchOutput {
            query: input.query.clone(),
            content,
            citations,
            allowed_domains: input.allowed_domains.clone(),
            providers_tried: vec!["tavily".to_string()],
            provider: "tavily".to_string(),
        })
    }
}

fn format_tavily_results(results: &[TavilyResult]) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }
    let mut text = String::from("Search Results:\n\n");
    for (i, r) in results.iter().enumerate() {
        text.push_str(&format!(
            "[{}] {} (score: {:.2})\n    {}\n    {}\n\n",
            i + 1, r.title, r.score, r.url, r.content
        ));
    }
    text
}
```

### 3.6 Brave Provider

```rust
// crates/codegen/xvora-tools/src/implementations/xvora_web_search/providers/brave.rs

use super::{SearchError, WebSearchProvider};
use crate::implementations::xvora_web_search::types::*;

pub struct BraveProvider {
    api_key: String,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct BraveWebResult {
    description: String,
    url: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct BraveResponse {
    web: Option<BraveWebResults>,
}

#[derive(Debug, Deserialize)]
struct BraveWebResults {
    results: Vec<BraveWebResult>,
}

impl BraveProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key, http: reqwest::Client::new() }
    }
}

#[async_trait]
impl WebSearchProvider for BraveProvider {
    fn name(&self) -> &str { "brave" }
    fn is_available(&self) -> bool { !self.api_key.is_empty() }

    async fn search(&self, input: &XvoraWebSearchInput) -> Result<XvoraWebSearchOutput, SearchError> {
        if self.api_key.is_empty() {
            return Err(SearchError::MissingKey("brave".to_string()));
        }

        let url = format!(
            "https://api.search.brave.com/res/v1/web/search?q={}",
            urlencoding::encode(&input.query)
        );

        let resp = self.http.get(&url)
            .header("Accept", "application/json")
            .header("X-Subscription-Token", &self.api_key)
            .send()
            .await
            .map_err(SearchError::from)?;

        if resp.status() == 401 { return Err(SearchError::Http(401, "Invalid Brave API key".into())); }
        if resp.status() == 429 { return Err(SearchError::RateLimited); }
        if !resp.status().is_success() {
            let body = resp.text().await.map_err(SearchError::from)?;
            return Err(SearchError::Http(resp.status().as_u16(), body));
        }

        let brave_resp: BraveResponse = resp.json().await.map_err(SearchError::from)?;
        let results = brave_resp.web
            .map(|w| w.results)
            .unwrap_or_default();

        let citations: Vec<String> = results.iter().map(|r| r.url.clone()).collect();
        let content = format_brave_results(&results);

        Ok(XvoraWebSearchOutput {
            query: input.query.clone(),
            content,
            citations,
            allowed_domains: input.allowed_domains.clone(),
            providers_tried: vec!["brave".to_string()],
            provider: "brave".to_string(),
        })
    }
}

fn format_brave_results(results: &[BraveWebResult]) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }
    let mut text = String::from("Search Results:\n\n");
    for (i, r) in results.iter().enumerate() {
        text.push_str(&format!("[{}] {} — {}\n    {}\n\n", i + 1, r.title, r.url, r.description));
    }
    text
}
```

### 3.7 Fallback Search Client

```rust
// crates/codegen/xvora-tools/src/implementations/xvora_web_search/client.rs

use super::types::{XvoraWebSearchConfig, XvoraWebSearchInput, XvoraWebSearchOutput};
use super::providers::{WebSearchProvider, SearchError};
use super::providers::duckduckgo::DuckDuckGoProvider;
use super::providers::tavily::TavilyProvider;
use super::providers::brave::BraveProvider;
// ... more providers

pub struct XvoraWebSearchClient {
    providers: Vec<Box<dyn WebSearchProvider>>,
    config: XvoraWebSearchConfig,
}

impl XvoraWebSearchClient {
    pub fn from_config(config: &XvoraWebSearchConfig) -> Self {
        let mut providers: Vec<Box<dyn WebSearchProvider>> = Vec::new();

        // Add primary provider
        providers.push(Box::new(match &config.primary {
            _ if config.primary == crate::implementations::xvora_web_search::types::SearchProvider::DuckDuckGo
                || config.primary == crate::implementations::xvora_web_search::types::SearchProvider::Mwmbl
            => DuckDuckGoProvider,
            crate::implementations::xvora_web_search::types::SearchProvider::Tavily => {
                let key = config.providers.get("tavily")
                    .and_then(|p| p.api_key.clone())
                    .or_else(|| std::env::var("TAVILY_API_KEY").ok())
                    .unwrap_or_default();
                TavilyProvider::new(key)
            }
            crate::implementations::xvora_web_search::types::SearchProvider::Brave => {
                let key = config.providers.get("brave")
                    .and_then(|p| p.api_key.clone())
                    .or_else(|| std::env::var("BRAVE_API_KEY").ok())
                    .unwrap_or_default();
                BraveProvider::new(key)
            }
            // Add more provider mappings...
        }));

        // Add fallback providers (available ones only)
        for provider in &config.fallback {
            if let Some(p) = Self::create_provider(provider, config) {
                providers.push(p);
            }
        }

        // Always append DuckDuckGo as last resort
        providers.push(Box::new(DuckDuckGoProvider));

        Self { providers, config: config.clone() }
    }

    fn create_provider(
        provider: &super::types::SearchProvider,
        config: &XvoraWebSearchConfig,
    ) -> Option<Box<dyn WebSearchProvider>> {
        match provider {
            super::types::SearchProvider::DuckDuckGo | super::types::SearchProvider::Mwmbl => {
                Some(Box::new(DuckDuckGoProvider))
            }
            super::types::SearchProvider::Tavily => {
                let key = config.providers.get("tavily")
                    .and_then(|p| p.api_key.clone())
                    .or_else(|| std::env::var("TAVILY_API_KEY").ok())?;
                Some(Box::new(TavilyProvider::new(key)))
            }
            super::types::SearchProvider::Brave => {
                let key = config.providers.get("brave")
                    .and_then(|p| p.api_key.clone())
                    .or_else(|| std::env::var("BRAVE_API_KEY").ok())?;
                Some(Box::new(BraveProvider::new(key)))
            }
            // Add more cases...
            _ => None,
        }
    }

    /// Search with automatic fallback across providers.
    /// Returns Ok with first successful result, or Err with all errors if all fail.
    pub async fn search(&self, input: &XvoraWebSearchInput) -> Result<XvoraWebSearchOutput, Vec<SearchError>> {
        let mut errors = Vec::new();
        let mut providers_tried = Vec::new();

        for provider in &self.providers {
            if !provider.is_available() {
                continue;
            }
            providers_tried.push(provider.name().to_string());

            match provider.search(input).await {
                Ok(result) => {
                    let mut result = result;
                    result.providers_tried = providers_tried;
                    return Ok(result);
                }
                Err(e) => {
                    if !e.is_retryable() {
                        // Non-retryable error (401, missing key) — skip this provider
                        tracing::debug!(provider = provider.name(), error = %e, "provider error, trying next");
                    } else {
                        tracing::warn!(provider = provider.name(), error = %e, "provider error, will retry");
                    }
                    errors.push(e);
                }
            }
        }

        Err(errors)
    }
}
```

### 3.8 Tool Implementation

```rust
// crates/codegen/xvora-tools/src/implementations/xvora_web_search/tool.rs

use super::client::XvoraWebSearchClient;
use super::types::*;
use crate::types::output::ToolOutput;
use crate::types::tool::{ToolKind, ToolNamespace};
use crate::types::tool_metadata::ToolMetadata;

pub const XVORA_WEB_SEARCH_NAME: &str = "xvora_web_search";

#[derive(Debug, Default)]
pub struct XvoraWebSearchTool {
    client: Option<XvoraWebSearchClient>,
}

impl XvoraWebSearchTool {
    pub fn new(client: Option<XvoraWebSearchClient>) -> Self {
        Self { client }
    }

    pub fn with_client(client: XvoraWebSearchClient) -> Self {
        Self { client: Some(client) }
    }
}

impl ToolMetadata for XvoraWebSearchTool {
    fn kind(&self) -> ToolKind {
        ToolKind::WebSearch
    }

    fn tool_namespace(&self) -> ToolNamespace {
        ToolNamespace::GrokBuild
    }

    fn description_template(&self) -> &str {
        "Search the web using multiple providers with automatic fallback. \
         Uses DuckDuckGo by default (no key needed), then falls back to \
         configured providers (Tavily, Brave, Serper, etc.). \
         Returns synthesized results with citations."
    }
}

impl xvora_tool_runtime::Tool for XvoraWebSearchTool {
    type Args = XvoraWebSearchInput;
    type Output = ToolOutput;

    fn id(&self) -> xvora_tool_protocol::ToolId {
        xvora_tool_protocol::ToolId::new(XVORA_WEB_SEARCH_NAME).expect("valid tool id")
    }

    fn description(&self, _ctx: &::xvora_tool_runtime::ListToolsContext) -> xvora_tool_types::ToolDescription {
        xvora_tool_types::ToolDescription::new(
            XVORA_WEB_SEARCH_NAME,
            ToolMetadata::sanitized_description_template(self),
        )
    }

    fn capabilities(&self) -> xvora_tool_protocol::ToolCapabilities {
        xvora_tool_protocol::ToolCapabilities {
            is_read_only: true,
            tool_scope: Some(xvora_tool_protocol::ToolScope::Read),
            ..Default::default()
        }
    }

    #[tracing::instrument(name = "tool.xvora_web_search", skip_all)]
    async fn run(
        &self,
        ctx: xvora_tool_runtime::ToolCallContext,
        input: XvoraWebSearchInput,
    ) -> Result<ToolOutput, xvora_tool_runtime::ToolError> {
        let client = self.client.as_ref()
            .ok_or_else(|| xvora_tool_runtime::ToolError::execution(
                self.id(),
                "Web search client not configured".to_string(),
            ))?;

        let result = client.search(&input).await.map_err(|errors| {
            let last_err = errors.last().map(|e| e.to_string()).unwrap_or("unknown".to_string());
            xvora_tool_runtime::ToolError::execution(
                self.id(),
                format!("All providers failed: {}. Last error: {}",
                    errors.len(), last_err),
            )
        })?;

        Ok(ToolOutput::Text(result.content.into()))
    }
}
```

### 3.9 Module Registration

```rust
// crates/codegen/xvora-tools/src/implementations/mod.rs

pub mod xvora_web_search;  // multi-provider search tool

// In the grok_build module re-export:
pub use xvora_web_search::tool::XvoraWebSearchTool;
pub use xvora_web_search::types::{XvoraWebSearchConfig, XvoraWebSearchInput, SearchProvider};
pub use xvora_web_search::client::XvoraWebSearchClient;
```

### 3.10 Config Schema Update

```rust
// crates/codegen/xvora-agent/src/config.rs — thêm field mới

pub struct AgentDefinition {
    // ... existing fields ...

    /// Multi-provider web search configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xvora_web_search: Option<XvoraWebSearchConfig>,
}
```

---

## 4. Cấu Hình Thực Tế

### Trường hợp 1: Không có API key nào (DuckDuckGo free)

```toml
# agent.yaml
[xvora_web_search]
primary = "duckduckgo"
fallback = ["tavily", "brave", "serper"]
```

→ Hoạt động ngay, không cần key. DuckDuckGo là default.

### Trường hợp 2: Có Tavily free tier

```toml
[xvora_web_search]
primary = "tavily"
fallback = ["duckduckgo", "brave"]

[xvora_web_search.providers.tavily]
api_key = "tvly-xxxxxxxx"
```

### Trường hợp 3: Fallback chain tùy chỉnh

```toml
[xvora_web_search]
primary = "brave"
fallback = [
    "tavily",      # free tier fallback
    "serper",      # cheap Google
    "duckduckgo",  # always works
]

[xvora_web_search.providers.brave]
api_key = "${BRAVE_API_KEY}"

[xvora_web_search.providers.tavily]
api_key = "${TAVILY_API_KEY}"
```

---

## 5. Prompt Instruction Cho LLM

Thêm vào system prompt template:

```markdown
## Web Search Tool (`xvora_web_search`)

You have access to a multi-provider web search tool. Use it when:

| Situation | Use? |
|-----------|------|
| Current/recent info needed | ✅ |
| Vendor docs, API changes this month | ✅ |
| Security advisories, CVEs | ✅ |
| "What's the latest..." questions | ✅ |
| Real-world code examples | ✅ |
| Well-known syntax/concepts | ❌ Use training knowledge |
| Local codebase search | ❌ Use grep/read_file |

**Providers:** DuckDuckGo (always), Tavily, Brave, Serper (if configured).
**Domains:** You can add `allowed_domains` to restrict results.
```

---

## 6. So Sánh Với xAI Web Search Hiện Tại

| Aspect | xAI web_search (hiện tại) | xvora_web_search (mới) |
|---|---|---|
| **Provider** | xAI Responses API only | Multi-provider (12+) |
| **API key** | xAI key required | DuckDuckGo free, others optional |
| **Fallback** | ❌ Không có | ✅ Tự động |
| **Cost** | Pay-per-call (xAI) | Free tier available |
| **Result format** | LLM-synthesized + citations | Same + raw results from providers |
| **Domain filter** | ✅ allowed/excluded | ✅ across all providers |
| **API key provider** | ✅ SharedApiKeyProvider | ✅ Same pattern |

---

## 7. Dependency additions

```toml
# crates/codegen/xvora-tools/Cargo.toml
async-trait = "0.1"
regex = "1"
urlencoding = "2"
```

---

## References

- [ultimate-web-search-skill SKILL.md](https://github.com/code-yeongyu/ultimate-web-search-skill/blob/main/SKILL.md)
- [xAI Web Search docs](https://docs.x.ai/features/web-search)
- [Brave Search API](https://api.search.brave.com/app/documentation/web-search/get-started)
- [Tavily API](https://tavily.com/)
- [Serper.dev](https://serper.dev/)
