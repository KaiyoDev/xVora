//! Lightweight UI localization for xVora (English + Vietnamese).
//!
//! ## Selection
//! 1. `XVORA_LANG=vi|en` (or `vi-VN`, `en-US`, …)
//! 2. `LANG` / `LC_ALL` containing `vi`
//! 3. Default: English
//!
//! Call [`t`] for static strings. Prefer this over scattering hard-coded copy
//! so new languages stay a single-file addition.
//!
//! Phase C modules: [`actions`], [`settings`], [`slash`].

pub mod actions;
pub mod actions_long_help;
pub mod chrome;
pub mod enums;
pub mod settings;
pub mod slash;

use std::sync::RwLock;

/// Active UI language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    Vi,
}

/// Message keys used by the welcome / auth surfaces (Phase B).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Msg {
    WelcomeNewWorktree,
    WelcomeResumeSession,
    WelcomeChangelog,
    WelcomeQuit,
    WelcomeLogout,
    WelcomeSwitchAccount,
    WelcomeImportClaude,
    WelcomeSubtitle,
    WelcomeBetaLabel,
    WelcomeLoginWithPrefix,
    AuthWaitingLogin,
    AuthWaitingApproval,
    AuthGoBack,
    TrustYes,
    TrustNo,
    CliAbout,
}

/// Runtime-switchable locale (Settings → Language). `None` until first
/// [`locale`] / [`set_locale`] / [`apply_from_config`].
static LOCALE: RwLock<Option<Locale>> = RwLock::new(None);

/// Set the process locale (Settings picker, tests, startup).
pub fn set_locale(locale: Locale) {
    if let Ok(mut g) = LOCALE.write() {
        *g = Some(locale);
    }
}

/// Force locale (alias of [`set_locale`]; kept for older call sites).
pub fn init(locale: Locale) {
    set_locale(locale);
}

/// Resolved locale (detects once if never set).
pub fn locale() -> Locale {
    if let Ok(g) = LOCALE.read()
        && let Some(l) = *g
    {
        return l;
    }
    let d = detect();
    set_locale(d);
    d
}

/// Apply language from `[ui].language` after loading config.
///
/// Precedence: `XVORA_LANG` env (always wins) → config `en`/`vi`/`auto` →
/// system `LANG` → English.
pub fn apply_from_config(language: Option<&str>) {
    if let Ok(v) = std::env::var("XVORA_LANG")
        && let Some(l) = parse_lang_tag(&v)
    {
        set_locale(l);
        return;
    }
    match language.map(str::trim).filter(|s| !s.is_empty()) {
        Some("auto") | None => set_locale(detect_system()),
        Some(tag) => set_locale(parse_lang_tag(tag).unwrap_or(Locale::En)),
    }
}

/// Canonical value for the settings enum (`en` | `vi` | `auto`).
pub fn config_language_canonical(stored: Option<&str>) -> &'static str {
    match stored.map(str::trim).filter(|s| !s.is_empty()) {
        None | Some("auto") => "auto",
        Some(s) if parse_lang_tag(s) == Some(Locale::Vi) => "vi",
        Some(s) if parse_lang_tag(s) == Some(Locale::En) => "en",
        _ => "auto",
    }
}

/// Parse / detect language without writing the global (tests / system path).
pub fn detect() -> Locale {
    if let Ok(v) = std::env::var("XVORA_LANG")
        && let Some(l) = parse_lang_tag(&v)
    {
        return l;
    }
    detect_system()
}

fn detect_system() -> Locale {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG", "USER_LANGUAGE", "Language"] {
        if let Ok(v) = std::env::var(key)
            && let Some(l) = parse_lang_tag(&v)
        {
            return l;
        }
    }
    Locale::En
}

pub fn parse_lang_tag(raw: &str) -> Option<Locale> {
    let s = raw.trim().to_ascii_lowercase();
    if s.is_empty() || s == "c" || s.starts_with("c.") || s == "auto" {
        return None;
    }
    // `vi`, `vi_VN`, `vi-VN.UTF-8`, …
    let primary = s
        .split(['_', '-', '.', '@'])
        .next()
        .unwrap_or("");
    match primary {
        "vi" => Some(Locale::Vi),
        "en" => Some(Locale::En),
        _ => None,
    }
}

/// Translate a UI message for the process locale.
pub fn t(msg: Msg) -> &'static str {
    match locale() {
        Locale::Vi => vi(msg),
        Locale::En => en(msg),
    }
}

fn en(msg: Msg) -> &'static str {
    match msg {
        Msg::WelcomeNewWorktree => "New worktree",
        Msg::WelcomeResumeSession => "Resume session",
        Msg::WelcomeChangelog => "Changelog",
        Msg::WelcomeQuit => "Quit",
        Msg::WelcomeLogout => "Logout",
        Msg::WelcomeSwitchAccount => "Switch account",
        Msg::WelcomeImportClaude => "Import Claude settings",
        Msg::WelcomeSubtitle => "Thanks for trying xVora — feedback with /feedback!",
        Msg::WelcomeBetaLabel => "xVora Beta  ",
        Msg::WelcomeLoginWithPrefix => "Login with",
        Msg::AuthWaitingLogin => "Waiting for login to complete...",
        Msg::AuthWaitingApproval => "Waiting for approval...",
        Msg::AuthGoBack => "  go back",
        Msg::TrustYes => "Yes, proceed",
        Msg::TrustNo => "No, quit",
        Msg::CliAbout => "xVora — Terminal AI (BYOK-first)",
    }
}

fn vi(msg: Msg) -> &'static str {
    match msg {
        Msg::WelcomeNewWorktree => "Worktree mới",
        Msg::WelcomeResumeSession => "Tiếp tục phiên",
        Msg::WelcomeChangelog => "Nhật ký thay đổi",
        Msg::WelcomeQuit => "Thoát",
        Msg::WelcomeLogout => "Đăng xuất",
        Msg::WelcomeSwitchAccount => "Đổi tài khoản",
        Msg::WelcomeImportClaude => "Nhập cài đặt Claude",
        Msg::WelcomeSubtitle => "Cảm ơn bạn đã dùng xVora — góp ý qua /feedback!",
        Msg::WelcomeBetaLabel => "xVora Beta  ",
        Msg::WelcomeLoginWithPrefix => "Đăng nhập với",
        Msg::AuthWaitingLogin => "Đang chờ đăng nhập hoàn tất...",
        Msg::AuthWaitingApproval => "Đang chờ phê duyệt...",
        Msg::AuthGoBack => "  quay lại",
        Msg::TrustYes => "Có, tiếp tục",
        Msg::TrustNo => "Không, thoát",
        Msg::CliAbout => "xVora — AI terminal (ưu tiên BYOK)",
    }
}

/// `Login with {label}` / `Đăng nhập với {label}`.
pub fn login_with(label: &str) -> String {
    format!("{} {}", t(Msg::WelcomeLoginWithPrefix), label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lang_tags() {
        assert_eq!(parse_lang_tag("vi"), Some(Locale::Vi));
        assert_eq!(parse_lang_tag("vi_VN.UTF-8"), Some(Locale::Vi));
        assert_eq!(parse_lang_tag("en-US"), Some(Locale::En));
        assert_eq!(parse_lang_tag("C"), None);
        assert_eq!(parse_lang_tag("fr_FR"), None);
    }

    #[test]
    fn vietnamese_welcome_keys_are_non_empty() {
        for msg in [
            Msg::WelcomeNewWorktree,
            Msg::WelcomeResumeSession,
            Msg::WelcomeChangelog,
            Msg::WelcomeQuit,
            Msg::WelcomeSubtitle,
        ] {
            assert!(!vi(msg).is_empty());
            assert_ne!(vi(msg), en(msg));
        }
    }
}
