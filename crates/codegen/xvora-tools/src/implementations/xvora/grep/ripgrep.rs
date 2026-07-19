use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[cfg(bundle_rg)]
const RG_BYTES: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/bundle-rg/rg-",
    env!("XVORA_TOOLS_RG_VER"),
    "-",
    env!("XVORA_TOOLS_RG_TARGET"),
    ".bin"
));

#[cfg(bundle_rg)]
fn resolve_bundled_rg() -> std::io::Result<PathBuf> {
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    // On Windows, keep a `.exe` suffix so CreateProcess reliably treats the
    // extracted PE as an executable when the path is resolved.
    #[cfg(windows)]
    let name = concat!(
        "rg-",
        env!("XVORA_TOOLS_RG_VER"),
        "-",
        env!("XVORA_TOOLS_RG_TARGET"),
        ".exe"
    );
    #[cfg(not(windows))]
    let name = concat!(
        "rg-",
        env!("XVORA_TOOLS_RG_VER"),
        "-",
        env!("XVORA_TOOLS_RG_TARGET")
    );
    let p = crate::util::xvora_home().join("vendor").join(name);
    if !p.exists() {
        fs::create_dir_all(p.parent().unwrap())?;
        fs::write(&p, RG_BYTES)?;
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&p)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&p, perms)?;
        }
    }
    Ok(p)
}

/// Get the path to the ripgrep executable.
///
/// In release builds with bundling enabled, this extracts the bundled ripgrep
/// binary to ~/.xvora/vendor/ and returns that path.
/// Otherwise, probes common install locations then falls back to `rg` on PATH.
pub fn rg_path() -> PathBuf {
    static RG_EXEC: OnceLock<PathBuf> = OnceLock::new();
    RG_EXEC
        .get_or_init(|| {
            #[cfg(bundle_rg)]
            {
                resolve_bundled_rg().unwrap_or_else(|_| resolve_system_rg())
            }
            #[cfg(not(bundle_rg))]
            {
                resolve_system_rg()
            }
        })
        .clone()
}

/// Locate a system / packaging-supplied ripgrep without assuming PATH works.
fn resolve_system_rg() -> PathBuf {
    // RG_BIN_PATH: explicit override (tests / packaging can set this).
    if let Ok(p) = std::env::var("RG_BIN_PATH") {
        let pb = PathBuf::from(&p);
        if pb.is_file() {
            return pb;
        }
    }
    // Some hermetic test runners set RUNFILES_DIR and ship rg as a
    // data dependency rather than on PATH. Scan for a directory
    // entry containing "ripgrep_hermetic" and prefer arch-scoped
    // paths when present.
    if let Ok(rf) = std::env::var("RUNFILES_DIR") {
        let base = PathBuf::from(rf);
        if let Ok(entries) = std::fs::read_dir(&base) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                if name.to_string_lossy().contains("ripgrep_hermetic") {
                    for sub in ["amd64/rg", "arm64/rg", "rg", "rg.exe"] {
                        let candidate = entry.path().join(sub);
                        if candidate.is_file() {
                            return candidate;
                        }
                    }
                }
            }
        }
    }

    // Sidecar next to the running binary (portable installs).
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        for name in ["rg.exe", "rg"] {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return candidate;
            }
        }
        // Common nested layouts: bin/, tools/
        for sub in ["bin", "tools", "vendor"] {
            for name in ["rg.exe", "rg"] {
                let candidate = dir.join(sub).join(name);
                if candidate.is_file() {
                    return candidate;
                }
            }
        }
    }

    // ~/.cargo/bin and ~/.xvora/vendor
    if let Some(home) = dirs::home_dir() {
        for rel in [
            Path::new(".cargo").join("bin").join("rg.exe"),
            Path::new(".cargo").join("bin").join("rg"),
            Path::new(".xvora").join("vendor").join("rg.exe"),
            Path::new(".xvora").join("vendor").join("rg"),
        ] {
            let candidate = home.join(rel);
            if candidate.is_file() {
                return candidate;
            }
        }
    }

    // PATH lookup via `where` (Windows) / `which` (Unix) — more reliable than
    // relying on CreateProcess PATH search for bare `rg` in some shells.
    if let Some(found) = which_on_path("rg") {
        return found;
    }
    if let Some(found) = which_on_path("rg.exe") {
        return found;
    }

    #[cfg(windows)]
    {
        PathBuf::from("rg.exe")
    }
    #[cfg(not(windows))]
    {
        PathBuf::from("rg")
    }
}

fn which_on_path(name: &str) -> Option<PathBuf> {
    #[cfg(windows)]
    let output = std::process::Command::new("where.exe")
        .arg(name)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    #[cfg(not(windows))]
    let output = std::process::Command::new("which")
        .arg(name)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first = stdout.lines().next()?.trim();
    if first.is_empty() {
        return None;
    }
    let p = PathBuf::from(first);
    p.is_file().then_some(p)
}
