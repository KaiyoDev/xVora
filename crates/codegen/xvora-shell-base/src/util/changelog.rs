//! Changelog fetching from CDN with local disk cache.
//!
//! Both markdown (`*.external.md`) and JSON (`*.external.json`) changelogs
//! may be published per-version. Open-source xVora does **not** fetch from
//! the xAI/Grok CDN; optional remote base is GitHub raw (or env override).
//!
//! `ChangelogManager::fetch()` retrieves both formats in parallel and
//! returns a `Changelog` with optional markdown + structured entries.
//! Consumers pick the format they need:
//! - `/release-notes` uses `changelog.markdown` for rich scrollback display
//! - Welcome screen uses `changelog.entries` for bullet rendering
//!
//! When remote is unavailable, disk cache under `$XVORA_HOME` is used; the
//! pager falls back to product default bullets when empty/dummy.

use std::path::PathBuf;

/// Changelog base URL for OSS. Override with `XVORA_CHANGELOG_BASE`.
/// Empty remote failures are fine — welcome falls back to embedded / i18n.
fn changelog_base() -> String {
    std::env::var("XVORA_CHANGELOG_BASE").unwrap_or_else(|_| {
        "https://raw.githubusercontent.com/KaiyoDev/xVora/main/changelogs".to_string()
    })
}
const FETCH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);

/// Baked-in notes for the build's version (`changelogs/CURRENT.external.json`).
/// Synced by `scripts/changelog.ps1 sync` so offline / pre-push users still
/// see real bullets (never dummy CDN layout-test text).
const EMBEDDED_JSON: &str = include_str!("../../../../../changelogs/CURRENT.external.json");
const EMBEDDED_MD: &str = include_str!("../../../../../changelogs/CURRENT.external.md");

/// A single structured changelog entry from the published JSON changelog.
///
/// Shape must match the output of `render_external_json` in `changelog.sh`:
///   `{category, description, breaking_change}`
/// If you change fields here, update `changelog.sh:render_external_json` too.
///
/// All fields use `#[serde(default)]` so a single malformed entry doesn't
/// kill the entire array parse. Entries with an empty description are
/// filtered out by `bullets_from_entries`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChangelogEntry {
    /// Category label (e.g. "features", "fixes", "breaking", "performance").
    #[serde(default)]
    pub category: String,
    /// Human-readable description (may contain `**bold**` or backticks).
    #[serde(default)]
    pub description: String,
    /// Whether this entry represents a breaking change.
    #[serde(default)]
    pub breaking_change: bool,
}

/// Both formats of a version's changelog, fetched together.
pub struct Changelog {
    /// Rendered markdown (for `/release-notes` display).
    pub markdown: Option<String>,
    /// Structured entries (for welcome screen bullets).
    pub entries: Option<Vec<ChangelogEntry>>,
}

/// Manages changelog retrieval from CDN with local disk caching.
///
/// Single entry point: `fetch()` returns both markdown and JSON in one
/// `Changelog` struct. Each format is fetched independently with its own
/// cache file, so a failure in one doesn't block the other.
pub struct ChangelogManager {
    md_cache: PathBuf,
    json_cache: PathBuf,
}

impl Default for ChangelogManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangelogManager {
    pub fn new() -> Self {
        // Prefer live `$XVORA_HOME` so harness-injected homes (PTY e2e) always
        // win over a OnceLock that may have been initialised earlier with a
        // different path in the same process graph.
        Self::from_env_home()
    }

    /// Resolve cache paths from the live process environment (not the
    /// `xvora_home()` OnceLock). A seeded `$XVORA_HOME` set on the pager
    /// process is always honoured even if some earlier init path cached a
    /// different home.
    fn from_env_home() -> Self {
        let home = std::env::var_os("XVORA_HOME")
            .map(std::path::PathBuf::from)
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(crate::util::xvora_home::xvora_home);
        Self {
            md_cache: home.join("CHANGELOG.md"),
            json_cache: home.join("CHANGELOG.json"),
        }
    }

    /// Fetch both markdown and JSON changelogs for the current version.
    ///
    /// Each format is fetched independently (CDN, 3 s timeout) and cached
    /// to disk. On failure, falls back to the cached copy. Either field
    /// may be `None` if offline with no cache.
    ///
    /// When `XVORA_CHANGELOG_OFFLINE` is set (PTY / integration tests), skip
    /// the CDN entirely and read only the disk cache so seeded fixtures win
    /// deterministically without network races. Paths are re-resolved from
    /// `$XVORA_HOME` so harness-injected env always applies.
    ///
    /// JSON is only cached after a successful parse to avoid poisoning the
    /// disk cache with malformed content (the markdown cache is write-through
    /// since it's consumed as raw text).
    pub fn fetch(&self) -> Changelog {
        // Always re-resolve from env so a caller holding an older manager
        // (or OnceLock lag) still reads the live harness home.
        let base = changelog_base();
        Self::from_env_home().fetch_with(changelog_offline(), &base)
    }

    /// Fetch using this manager's already-resolved cache paths, an explicit
    /// offline flag, and an explicit CDN base.
    ///
    /// Split out of [`fetch`] so unit tests can drive it against a temp home
    /// without mutating process-global env (`XVORA_HOME` /
    /// `XVORA_CHANGELOG_OFFLINE`), which races across the parallel test
    /// harness. Passing an unreachable `base` lets a test force a
    /// deterministic CDN miss instead of depending on whether the sandbox
    /// happens to block network. Production callers always go through
    /// [`fetch`], so behaviour is unchanged.
    fn fetch_with(&self, offline: bool, base: &str) -> Changelog {
        if offline {
            return sanitize_changelog(Changelog {
                markdown: read_real_markdown(&self.md_cache).or_else(embedded_markdown),
                entries: self
                    .read_real_json_cache()
                    .or_else(embedded_entries)
                    .filter(|e| !e.is_empty()),
            });
        }

        let version = xvora_version::VERSION;
        let md_url = format!("{}/{}.external.md", base, version);

        // Fetch both formats in parallel (3s timeout each → 3s total, not 6s).
        let mut markdown = None;
        let mut entries = None;
        std::thread::scope(|s| {
            let md_handle = s.spawn(|| self.fetch_and_cache(&md_url, &self.md_cache));
            let json_handle = s.spawn(|| self.fetch_json(base, version));
            markdown = md_handle.join().ok().flatten();
            entries = json_handle.join().ok().flatten();
        });

        // Fallbacks: disk cache → embedded CURRENT (repo changelogs/).
        // Dummy layout-test payloads (old x.ai CDN / poisoned ~/.xvora cache)
        // are rejected so Release Notes never show them.
        if markdown.is_none() || markdown.as_deref().is_some_and(is_dummy_changelog_markdown) {
            markdown = read_real_markdown(&self.md_cache).or_else(embedded_markdown);
        }
        if entries.is_none() || entries.as_ref().is_some_and(|e| real_entries(e).is_empty()) {
            entries = self
                .read_real_json_cache()
                .or_else(embedded_entries)
                .filter(|e| !e.is_empty());
        }

        sanitize_changelog(Changelog { markdown, entries })
    }

    /// Fetch and parse JSON changelog, caching only after successful parse.
    fn fetch_json(&self, base: &str, version: &str) -> Option<Vec<ChangelogEntry>> {
        let url = format!("{}/{}.external.json", base, version);

        // Try remote first — only cache after successful parse.
        if let Ok(raw) = fetch_blocking(&url)
            && !raw.trim().is_empty()
        {
            match serde_json::from_str::<Vec<ChangelogEntry>>(&raw) {
                Ok(entries) => {
                    let real = real_entries(&entries);
                    if real.is_empty() {
                        // Don't poison disk with CDN dummy layout-test rows.
                        return None;
                    }
                    if let Err(e) = std::fs::write(&self.json_cache, &raw) {
                        tracing::debug!(error = %e, "JSON changelog cache write failed");
                    }
                    return Some(real);
                }
                Err(e) => {
                    tracing::debug!(error = %e, "failed to parse JSON changelog from CDN");
                }
            }
        }

        self.read_real_json_cache()
    }

    fn read_real_json_cache(&self) -> Option<Vec<ChangelogEntry>> {
        let cached = read_cache(&self.json_cache)?;
        match serde_json::from_str::<Vec<ChangelogEntry>>(&cached) {
            Ok(entries) => {
                let real = real_entries(&entries);
                if real.is_empty() { None } else { Some(real) }
            }
            Err(e) => {
                tracing::debug!(error = %e, "failed to parse cached JSON changelog");
                None
            }
        }
    }

    /// Shared fetch-and-cache: try remote (3 s timeout), cache on success,
    /// fall back to disk cache on failure. Rejects dummy markdown.
    fn fetch_and_cache(&self, url: &str, cache_path: &std::path::Path) -> Option<String> {
        if let Ok(content) = fetch_blocking(url)
            && !content.trim().is_empty()
            && !is_dummy_changelog_markdown(&content)
        {
            if let Err(e) = std::fs::write(cache_path, &content) {
                tracing::debug!(error = %e, path = %cache_path.display(), "cache write failed");
            }
            return Some(content);
        }
        read_real_markdown(cache_path)
    }
}

/// When set, `ChangelogManager::fetch` skips the CDN and only reads disk cache.
/// Used by PTY harness tests that seed `CHANGELOG.{md,json}` under a temp home.
fn changelog_offline() -> bool {
    std::env::var_os("XVORA_CHANGELOG_OFFLINE").is_some_and(|v| !v.is_empty() && v != "0")
}

fn read_cache(path: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .filter(|c| !c.trim().is_empty())
}

fn read_real_markdown(path: &std::path::Path) -> Option<String> {
    let s = read_cache(path)?;
    if is_dummy_changelog_markdown(&s) {
        None
    } else {
        Some(s)
    }
}

fn real_entries(entries: &[ChangelogEntry]) -> Vec<ChangelogEntry> {
    entries
        .iter()
        .filter(|e| !e.description.is_empty())
        .filter(|e| !is_dummy_changelog_description(&e.description))
        .cloned()
        .collect()
}

/// Drop dummy payloads and ensure Release Notes markdown exists when we have
/// structured entries (synthesizes markdown from JSON if needed).
fn sanitize_changelog(mut c: Changelog) -> Changelog {
    if let Some(ref md) = c.markdown
        && is_dummy_changelog_markdown(md)
    {
        c.markdown = None;
    }
    if let Some(ref entries) = c.entries {
        let real = real_entries(entries);
        c.entries = if real.is_empty() { None } else { Some(real) };
    }
    if c.entries.is_none() {
        c.entries = embedded_entries();
    }
    if c.markdown.is_none() {
        if let Some(ref entries) = c.entries {
            c.markdown = Some(markdown_from_entries(xvora_version::VERSION, entries));
        } else {
            c.markdown = embedded_markdown();
        }
    }
    c
}

/// Build Release Notes markdown from structured entries (welcome / modal).
pub fn markdown_from_entries(version: &str, entries: &[ChangelogEntry]) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {version}\n\n"));
    let order = [
        ("breaking", "Breaking"),
        ("features", "Features"),
        ("fixes", "Bug Fixes"),
        ("performance", "Performance"),
        ("docs", "Docs"),
        ("chore", "Chore"),
    ];
    for (key, title) in order {
        let rows: Vec<&ChangelogEntry> = entries
            .iter()
            .filter(|e| {
                if key == "breaking" {
                    e.breaking_change
                } else {
                    !e.breaking_change && e.category.eq_ignore_ascii_case(key)
                }
            })
            .collect();
        if rows.is_empty() {
            continue;
        }
        out.push_str(&format!("## {title}\n\n"));
        for e in rows {
            out.push_str(&format!("- {}\n", e.description));
        }
        out.push('\n');
    }
    out
}

fn embedded_entries() -> Option<Vec<ChangelogEntry>> {
    match serde_json::from_str::<Vec<ChangelogEntry>>(EMBEDDED_JSON) {
        Ok(entries) => {
            let real = real_entries(&entries);
            if real.is_empty() { None } else { Some(real) }
        }
        Err(e) => {
            tracing::debug!(error = %e, "embedded changelog JSON parse failed");
            None
        }
    }
}

fn embedded_markdown() -> Option<String> {
    let s = EMBEDDED_MD.trim();
    if s.is_empty() || is_dummy_changelog_markdown(s) {
        None
    } else {
        Some(EMBEDDED_MD.to_string())
    }
}

// ---------------------------------------------------------------------------
// What's-new: notify once per installed version
// ---------------------------------------------------------------------------

/// Filename under `$XVORA_HOME` recording the last version for which the user
/// already saw the What's-new toast / welcome bullets.
pub const LAST_SEEN_VERSION_FILE: &str = "last_seen_version";

/// Read the last version the user was shown What's-new for.
pub fn read_last_seen_version(home: &std::path::Path) -> Option<String> {
    let path = home.join(LAST_SEEN_VERSION_FILE);
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Persist the version after What's-new has been surfaced.
pub fn write_last_seen_version(home: &std::path::Path, version: &str) {
    let path = home.join(LAST_SEEN_VERSION_FILE);
    if let Err(e) = std::fs::write(&path, version.trim()) {
        tracing::debug!(
            error = %e,
            path = %path.display(),
            "failed to write last_seen_version"
        );
    }
}

/// Whether this launch should surface What's-new (version changed or first run).
pub fn should_notify_whats_new(home: &std::path::Path, installed: &str) -> bool {
    match read_last_seen_version(home) {
        None => true,
        Some(prev) => prev.trim() != installed.trim(),
    }
}

/// Build a short toast for a new version (used by the pager on startup).
pub fn whats_new_toast(version: &str, bullets: &[String]) -> String {
    if bullets.is_empty() {
        format!("\u{2728} What's new in xVora {version}")
    } else {
        format!(
            "\u{2728} What's new in xVora {version}: {}",
            bullets.first().map(String::as_str).unwrap_or("")
        )
    }
}

/// Strip `**bold**` markers and backticks from a description string.
fn strip_markdown_inline(s: &str) -> String {
    s.replace("**", "").replace('`', "")
}

/// Convert changelog entries to plain-text bullet strings.
///
/// Strips `**bold**` and backtick formatting from each description,
/// skips entries with empty descriptions (from tolerant deserialization),
/// drops xAI CDN **dummy / layout-test** placeholders (served for unreleased
/// versions like `0.2.0-dev`), and returns at most `max` entries.
pub fn bullets_from_entries(entries: &[ChangelogEntry], max: usize) -> Vec<String> {
    entries
        .iter()
        .filter(|e| !e.description.is_empty())
        .filter(|e| !is_dummy_changelog_description(&e.description))
        .take(max)
        .map(|e| strip_markdown_inline(&e.description))
        .collect()
}

/// CDN placeholders for unreleased / test builds (e.g. `0.2.0-dev`).
fn is_dummy_changelog_description(description: &str) -> bool {
    let d = description.to_ascii_lowercase();
    d.contains("dummy changelog")
        || d.contains("dummy feature")
        || d.contains("dummy bug")
        || d.contains("for testing purposes")
        || d.contains("for layout testing")
        || d.contains("verify the welcome screen")
}

/// True if markdown is the old x.ai layout-test / dummy release notes body.
fn is_dummy_changelog_markdown(md: &str) -> bool {
    let d = md.to_ascii_lowercase();
    d.contains("dummy changelog")
        || d.contains("dummy feature")
        || d.contains("dummy bug")
        || d.contains("for testing purposes")
        || d.contains("for layout testing")
        || d.contains("verify the welcome screen")
}

/// Blocking HTTP fetch. Callers (`std::thread::scope` threads) are already
/// off the tokio runtime, so no extra thread spawn is needed.
fn fetch_blocking(url: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(FETCH_TIMEOUT)
        .build()?;
    let resp = client.get(url).send()?;
    if !resp.status().is_success() {
        anyhow::bail!("HTTP {}", resp.status());
    }
    Ok(resp.text()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a manager pointing at `home` directly, bypassing the global
    /// `$XVORA_HOME` env so tests never race the parallel harness.
    fn manager_for(home: &std::path::Path) -> ChangelogManager {
        ChangelogManager {
            md_cache: home.join("CHANGELOG.md"),
            json_cache: home.join("CHANGELOG.json"),
        }
    }

    #[test]
    fn offline_mode_reads_seeded_disk_cache_only() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("xvora-home");
        std::fs::create_dir_all(&home).unwrap();
        std::fs::write(home.join("CHANGELOG.md"), "# seeded offline md\n").unwrap();
        std::fs::write(
            home.join("CHANGELOG.json"),
            r#"[{"category":"features","description":"seeded entry","breaking_change":false}]"#,
        )
        .unwrap();

        // Offline path: read only the seeded disk cache, no network.
        let changelog = manager_for(&home).fetch_with(true, &changelog_base());
        assert_eq!(
            changelog.markdown.as_deref(),
            Some("# seeded offline md\n"),
            "offline mode must return seeded markdown"
        );
        let entries = changelog.entries.expect("seeded json entries");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].description, "seeded entry");
    }

    #[test]
    fn cdn_miss_falls_back_to_env_home_disk_cache() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("xvora-home-fallback");
        std::fs::create_dir_all(&home).unwrap();
        std::fs::write(home.join("CHANGELOG.md"), "# fallback md\n").unwrap();

        // Non-offline path with an unreachable CDN base: the remote fetch
        // fails deterministically (no dependency on the sandbox blocking
        // network), so the on-disk cache must win.
        let changelog = manager_for(&home).fetch_with(false, "http://127.0.0.1:1");
        assert_eq!(
            changelog.markdown.as_deref(),
            Some("# fallback md\n"),
            "CDN miss must fall back to the seeded CHANGELOG.md"
        );
    }

    #[test]
    fn embedded_current_json_parses_and_is_non_empty() {
        let entries = embedded_entries().expect("CURRENT.external.json must embed");
        assert!(!entries.is_empty());
        assert!(entries.iter().all(|e| !e.description.is_empty()));
    }

    #[test]
    fn whats_new_notifies_once_per_version() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path();
        assert!(should_notify_whats_new(home, "0.2.0-dev"));
        write_last_seen_version(home, "0.2.0-dev");
        assert!(!should_notify_whats_new(home, "0.2.0-dev"));
        assert!(should_notify_whats_new(home, "0.2.1"));
    }

    #[test]
    fn bullets_drop_cdn_dummy_layout_test_entries() {
        let entries = vec![
            ChangelogEntry {
                category: "features".into(),
                description: "This is a dummy changelog entry for testing purposes.".into(),
                breaking_change: false,
            },
            ChangelogEntry {
                category: "features".into(),
                description:
                    "Another dummy feature to verify the welcome screen renders correctly.".into(),
                breaking_change: false,
            },
            ChangelogEntry {
                category: "fixes".into(),
                description: "Dummy bug fix entry for layout testing.".into(),
                breaking_change: false,
            },
            ChangelogEntry {
                category: "features".into(),
                description: "Real feature: Vietnamese UI.".into(),
                breaking_change: false,
            },
        ];
        let bullets = bullets_from_entries(&entries, 5);
        assert_eq!(bullets, vec!["Real feature: Vietnamese UI.".to_string()]);
    }

    #[test]
    fn bullets_strips_markdown_and_respects_max() {
        let entries = vec![
            ChangelogEntry {
                category: "features".into(),
                description: "Added **dark mode** support".into(),
                breaking_change: false,
            },
            ChangelogEntry {
                category: "fixes".into(),
                description: "Fixed `crash` on startup".into(),
                breaking_change: false,
            },
            ChangelogEntry {
                category: "performance".into(),
                description: "Faster **rendering** of `code` blocks".into(),
                breaking_change: false,
            },
        ];

        let bullets = bullets_from_entries(&entries, 2);
        assert_eq!(bullets.len(), 2);
        assert_eq!(bullets[0], "Added dark mode support");
        assert_eq!(bullets[1], "Fixed crash on startup");
    }

    #[test]
    fn bullets_skips_empty_descriptions() {
        let entries = vec![
            ChangelogEntry {
                category: "features".into(),
                description: "Good entry".into(),
                breaking_change: false,
            },
            ChangelogEntry {
                category: String::new(),
                description: String::new(), // bad entry from tolerant deser
                breaking_change: false,
            },
            ChangelogEntry {
                category: "fixes".into(),
                description: "Another good one".into(),
                breaking_change: false,
            },
        ];
        let bullets = bullets_from_entries(&entries, 10);
        assert_eq!(bullets, vec!["Good entry", "Another good one"]);
    }

    #[test]
    fn tolerant_deserialization_partial_entry() {
        // Missing description field → defaults to empty string, not a parse error
        let json = r#"[{"category":"features"},{"description":"ok"}]"#;
        let entries: Vec<ChangelogEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].description, "");
        assert_eq!(entries[1].category, "");
        assert_eq!(entries[1].description, "ok");
    }
}
