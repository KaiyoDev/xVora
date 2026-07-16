//! Origin/client identification used by the telemetry engine.
//!
//! [`OriginClientInfo`] is owned by `xai-xvora-sampler` (so `SamplerConfig`
//! can use it without depending on shell). Re-exported here so the telemetry
//! engine can label events without depending on shell or sampler internals
//! beyond the type itself.

pub use xai_xvora_sampler::OriginClientInfo;

/// Construct an [`OriginClientInfo`] from `XVORA_CLIENT_NAME` /
/// `XVORA_CLIENT_VERSION` env vars. Returns `None` when `XVORA_CLIENT_NAME`
/// is unset. Free function (not an inherent method) because the type lives
/// in another crate.
pub fn origin_client_info_from_env() -> Option<OriginClientInfo> {
    std::env::var("XVORA_CLIENT_NAME")
        .ok()
        .map(|product| OriginClientInfo {
            product,
            version: std::env::var("XVORA_CLIENT_VERSION").ok(),
        })
}
