//! i18n — locale-aware translations for settings labels and descriptions.
//!
//! This module is a thin stub that delegates to static English strings until
//! a full internationalisation layer is wired in. It exists so the settings
//! registry can call `crate::i18n::settings::category(...)` etc. without
//! requiring a separate i18n crate dependency at this layer.

pub mod settings {
    /// Locale-aware category label. Returns the English fallback.
    pub fn category(en: &'static str) -> &'static str {
        en
    }

    /// Locale-aware setting label. Returns the English fallback.
    pub fn setting_label(_key: &str, en: &'static str) -> &'static str {
        en
    }

    /// Locale-aware setting description. Returns the English fallback.
    pub fn setting_description(_key: &str, en: &'static str) -> &'static str {
        en
    }
}

/// Canonicalise a raw UI language string to a registry choice.
/// `None` / empty / unknown → `"auto"`.
pub fn config_language_canonical(value: Option<&'static str>) -> &'static str {
    let raw = value.unwrap_or_default().trim();
    if raw.eq_ignore_ascii_case("vi") || raw.eq_ignore_ascii_case("vietnamese") {
        "vi"
    } else if raw.eq_ignore_ascii_case("en") || raw.eq_ignore_ascii_case("english") {
        "en"
    } else {
        "auto"
    }
}
