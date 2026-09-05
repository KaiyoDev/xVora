//! i18n — locale-aware translations for settings, slash commands, enums, chrome, and actions.
//!
//! The current locale is stored in a process-wide `OnceLock`. It is set once at
//! startup from `[ui].language` (or `$XVORA_LANGUAGE`) and can be changed at
//! runtime via [`set_locale`] when the user flips the language setting.
//!
//! Generated modules live in `i18n/` and are produced by
//! `scripts/gen_i18n_settings_slash.py` and `scripts/gen_i18n_full_vi.py`.
//! Run those scripts after editing their source dicts to regenerate.

pub mod settings;
pub mod slash;
pub mod enums;
pub mod chrome;
pub mod actions_long_help;

use std::sync::OnceLock;

/// Supported UI locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    #[default]
    En,
    Vi,
}

impl Locale {
    fn from_str(s: &str) -> Self {
        match s {
            "vi" | "vietnamese" => Locale::Vi,
            "en" | "english" => Locale::En,
            _ => Locale::En,
        }
    }
}

/// Process-wide locale, initialised at startup and updated when the user
/// changes the `language` setting. Read-only after init.
static LOCALE: OnceLock<Locale> = OnceLock::new();

/// Returns the current UI locale. Defaults to English if not yet set.
pub fn locale() -> Locale {
    *LOCALE.get().unwrap_or(&Locale::En)
}

/// Set the UI locale. Call once at startup and again whenever the user
/// changes the `language` setting in the settings modal.
pub fn set_locale(value: &str) {
    let loc = Locale::from_str(value);
    LOCALE.set(loc).ok();
}

/// Initialize the locale from the `XVORA_LANG` / `GROK_LANG` environment
/// variable at process startup. Safe to call multiple times — only the first
/// write wins (subsequent calls are no-ops).
pub fn init_locale_from_env() {
    let val = std::env::var("XVORA_LANG")
        .or_else(|_| std::env::var("GROK_LANG"))
        .ok()
        .unwrap_or_default();
    if !val.is_empty() {
        set_locale(&val);
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
