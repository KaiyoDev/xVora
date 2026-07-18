#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    dead_code
)]
//! Backend environment presets for the Xvora CLI crate family: endpoint URL
//! defaults, environment selection, and env-var test support.
//!
//! ## Multi-provider note
//!
//! Product defaults for **first-party xAI** live under [`xai_provider`].
//! Other providers (OpenAI, Ollama, custom) use per-model `base_url` / keys —
//! they do not inherit these URLs as "the product host".

/// xAI provider production endpoints (not product identity).
pub mod xai_provider;

/// The endpoint set for one backend environment (legacy shape).
///
/// Values are currently the **xAI provider** production set. Prefer
/// [`xai_provider::PRODUCTION`] when writing new code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XvoraEndpoints {
    pub cli_chat_proxy_base_url: &'static str,
    pub asset_server_url: &'static str,
    pub relay_ws_url: &'static str,
    pub gateway_ws_url: &'static str,
    pub ws_origin: &'static str,
}

const PRODUCTION_ENDPOINTS: XvoraEndpoints = XvoraEndpoints {
    cli_chat_proxy_base_url: xai_provider::CLI_CHAT_PROXY_BASE_URL,
    asset_server_url: xai_provider::ASSET_SERVER_URL,
    relay_ws_url: xai_provider::RELAY_WS_URL,
    gateway_ws_url: xai_provider::GATEWAY_WS_URL,
    ws_origin: xai_provider::WS_ORIGIN,
};

/// Alias: xAI provider cli-chat-proxy (legacy name kept for callers).
pub const PROD_CLI_CHAT_PROXY_BASE_URL: &str = xai_provider::CLI_CHAT_PROXY_BASE_URL;
/// Alias: xAI provider asset server.
pub const PROD_ASSET_SERVER_URL: &str = xai_provider::ASSET_SERVER_URL;
/// Alias: xAI provider relay WebSocket.
pub const PROD_RELAY_WS_URL: &str = xai_provider::RELAY_WS_URL;
/// Alias: xAI provider cloud gateway WebSocket.
pub const PROD_GATEWAY_WS_URL: &str = xai_provider::GATEWAY_WS_URL;
/// Alias: xAI provider WS origin.
pub const PROD_WS_ORIGIN: &str = xai_provider::WS_ORIGIN;
/// Alias: xAI provider public API base (`https://api.x.ai/v1`).
pub const PROD_XAI_API_BASE_URL: &str = xai_provider::API_BASE_URL;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XvoraEnvironment {
    #[default]
    Production,
}
impl XvoraEnvironment {
    pub fn from_flags(_dev: bool, _staging: bool) -> Self {
        XvoraEnvironment::Production
    }
    /// Indicator string for display; `None` for Production.
    pub fn indicator(&self) -> Option<&'static str> {
        match self {
            XvoraEnvironment::Production => None,
        }
    }
    pub fn is_production(&self) -> bool {
        matches!(self, XvoraEnvironment::Production)
    }
    fn env_prefix(&self) -> &'static str {
        match self {
            XvoraEnvironment::Production => "XVORA_PRODUCTION",
        }
    }
    /// Compiled endpoint set for this environment (xAI provider production).
    pub fn endpoints(&self) -> XvoraEndpoints {
        match self {
            XvoraEnvironment::Production => PRODUCTION_ENDPOINTS,
        }
    }
    /// Env-var override when set, else the compiled endpoint.
    fn resolve(&self, var_suffix: &str, compiled: &'static str) -> String {
        std::env::var(format!("{}{var_suffix}", self.env_prefix()))
            .unwrap_or_else(|_| compiled.to_string())
    }
    pub fn cli_chat_proxy_base_url(&self) -> String {
        self.resolve(
            "_CLI_CHAT_PROXY_BASE_URL",
            self.endpoints().cli_chat_proxy_base_url,
        )
    }
    pub fn ws_origin(&self) -> String {
        self.resolve("_WS_ORIGIN", self.endpoints().ws_origin)
    }
    pub fn asset_server_url(&self) -> String {
        self.resolve("_ASSET_SERVER_URL", self.endpoints().asset_server_url)
    }
    /// The relay WebSocket URL (web UI driving a local agent). Not the
    /// cloud-sandbox gateway ([`Self::gateway_ws_url`]); different protocols.
    pub fn relay_ws_url(&self) -> String {
        self.resolve("_WS_URL", self.endpoints().relay_ws_url)
    }
    /// The gateway WebSocket URL for `/cloud new` sandboxes. The shell's
    /// `XVORA_GATEWAY_URL` opt-in takes precedence.
    pub fn gateway_ws_url(&self) -> String {
        self.resolve("_GATEWAY_WS_URL", self.endpoints().gateway_ws_url)
    }
}
impl std::fmt::Display for XvoraEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XvoraEnvironment::Production => write!(f, "production"),
        }
    }
}
/// Serializes env-var mutation across tests; `std::env` is process-global.
///
/// Always available (not `cfg(test)` only) so dependent crates' test
/// targets can use it — `cfg(test)` on a dependency is off when that
/// dependency is built as a library for a parent package's tests.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner())
}
/// RAII env-var override for tests: constructors snapshot the prior value
/// under [`ENV_LOCK`], `Drop` restores it, panics included.
pub struct EnvVarGuard {
    key: &'static str,
    prev: Option<String>,
    _lock: std::sync::MutexGuard<'static, ()>,
}
impl EnvVarGuard {
    pub fn set(key: &'static str, value: &str) -> Self {
        let lock = env_lock();
        let prev = std::env::var(key).ok();
        unsafe { std::env::set_var(key, value) };
        Self {
            key,
            prev,
            _lock: lock,
        }
    }
    pub fn remove(key: &'static str) -> Self {
        let lock = env_lock();
        let prev = std::env::var(key).ok();
        unsafe { std::env::remove_var(key) };
        Self {
            key,
            prev,
            _lock: lock,
        }
    }
    /// Update the value while still holding the env lock.
    pub fn set_value(&self, value: &str) {
        unsafe { std::env::set_var(self.key, value) };
    }
}
impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match self.prev.take() {
            Some(prev) => unsafe { std::env::set_var(self.key, prev) },
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The env-var prefixes are an operator interface; do not rename.
    #[test]
    fn test_env_prefix() {
        assert_eq!(
            XvoraEnvironment::Production.env_prefix(),
            "XVORA_PRODUCTION"
        );
    }

    #[test]
    fn env_var_guard_set_value_updates_then_restores_on_drop() {
        const KEY: &str = "XVORA_ENV_VAR_GUARD_SET_VALUE_PROBE";
        let before = std::env::var(KEY).ok();
        {
            let guard = EnvVarGuard::set(KEY, "initial");
            assert_eq!(std::env::var(KEY).ok().as_deref(), Some("initial"));
            guard.set_value("updated");
            assert_eq!(
                std::env::var(KEY).ok().as_deref(),
                Some("updated"),
                "set_value must update the env var while the guard is live"
            );
        }
        assert_eq!(
            std::env::var(KEY).ok(),
            before,
            "Drop must restore the pre-guard snapshot (was {before:?})"
        );
    }

    /// Guards against conflating the relay and gateway endpoints (a relay
    /// loop mistakenly connecting to `wss://grok.com/ws/gw/`).
    #[test]
    fn relay_and_gateway_urls_are_distinct() {
        assert_ne!(
            XvoraEnvironment::Production.relay_ws_url(),
            XvoraEnvironment::Production.gateway_ws_url(),
        );
    }

    #[test]
    fn test_from_flags() {
        assert_eq!(
            XvoraEnvironment::from_flags(false, false),
            XvoraEnvironment::Production
        );
    }

    #[test]
    fn prod_aliases_match_xai_provider() {
        assert_eq!(
            PROD_CLI_CHAT_PROXY_BASE_URL,
            xai_provider::CLI_CHAT_PROXY_BASE_URL
        );
        assert_eq!(PROD_XAI_API_BASE_URL, xai_provider::API_BASE_URL);
        assert_eq!(PROD_ASSET_SERVER_URL, xai_provider::ASSET_SERVER_URL);
        assert_eq!(
            XvoraEnvironment::Production
                .endpoints()
                .cli_chat_proxy_base_url,
            xai_provider::PRODUCTION.cli_chat_proxy_base_url
        );
    }
}
