//! **xAI provider** network endpoints (not product identity).
//!
//! xVora is multi-provider: OpenAI / Ollama / custom models use their own
//! `base_url` + BYOK keys. These URLs are defaults for the **xAI** provider
//! only (built-in first-party models, OAuth proxy, assets, relay).
//!
//! Product code should import from this module (or the `PROD_*` aliases) and
//! treat the values as provider-scoped — never as "the only way to run xVora".

/// Production endpoints for the xAI provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XaiProviderEndpoints {
    /// cli-chat-proxy (session / OAuth / aux services).
    pub cli_chat_proxy_base_url: &'static str,
    /// Public inference API (`XAI_API_KEY` / API-key auth).
    pub api_base_url: &'static str,
    /// Profile images / static assets.
    pub asset_server_url: &'static str,
    /// Web frontend → local agent relay WebSocket.
    pub relay_ws_url: &'static str,
    /// Cloud sandbox gateway WebSocket.
    pub gateway_ws_url: &'static str,
    /// Origin for WS handshake.
    pub ws_origin: &'static str,
}

/// Compiled production defaults for the xAI provider.
pub const PRODUCTION: XaiProviderEndpoints = XaiProviderEndpoints {
    cli_chat_proxy_base_url: "https://cli-chat-proxy.grok.com/v1",
    api_base_url: "https://api.x.ai/v1",
    asset_server_url: "https://assets.grok.com",
    relay_ws_url: "wss://code.grok.com/ws/code-agent",
    gateway_ws_url: "wss://grok.com/ws/gw/",
    ws_origin: "https://grok.com",
};

pub const CLI_CHAT_PROXY_BASE_URL: &str = PRODUCTION.cli_chat_proxy_base_url;
pub const API_BASE_URL: &str = PRODUCTION.api_base_url;
pub const ASSET_SERVER_URL: &str = PRODUCTION.asset_server_url;
pub const RELAY_WS_URL: &str = PRODUCTION.relay_ws_url;
pub const GATEWAY_WS_URL: &str = PRODUCTION.gateway_ws_url;
pub const WS_ORIGIN: &str = PRODUCTION.ws_origin;

/// True when `url` targets first-party xAI / grok infrastructure for this provider.
pub fn is_first_party_url(url: &str) -> bool {
    let url = url.trim().to_ascii_lowercase();
    if url.is_empty() {
        return false;
    }
    url.contains("api.x.ai")
        || url.contains("cli-chat-proxy")
        || url.contains("grok.com")
        || url.contains("x.ai/")
        || url.contains("assets.grok.com")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_urls_are_first_party() {
        assert!(is_first_party_url(API_BASE_URL));
        assert!(is_first_party_url(CLI_CHAT_PROXY_BASE_URL));
        assert!(is_first_party_url(ASSET_SERVER_URL));
        assert!(!is_first_party_url("https://api.openai.com/v1"));
        assert!(!is_first_party_url("http://127.0.0.1:11434/v1"));
    }
}
