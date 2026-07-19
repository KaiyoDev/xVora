//! Build script for bundling ripgrep for the xvora-tools crate.
//!
//! - If `XVORA_TOOLS_BUNDLE_RG_PATH` is set, always bundle it
//! - Otherwise, only bundle in release builds
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

const RG_VER: &str = "15.0.0";
const BFS_VER: &str = "4.1";
const UGREP_VER: &str = "7.7.0";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bundle_rg()?;
    // bfs/ugrep back the bash-harness find/grep shadows (embedded_search_tools).
    bundle_search_tool("bfs", "BFS", BFS_VER)?;
    bundle_search_tool("ugrep", "UGREP", UGREP_VER)?;
    Ok(())
}

/// Bundle a prebuilt **static** search-tool binary (`bfs`/`ugrep`) when
/// `XVORA_TOOLS_BUNDLE_<NAME>_PATH` points at one (supplied by the release
/// pipeline). Emits
/// `cfg(bundle_<name>)` so the crate's `include_bytes!` + self-extract engages.
///
/// No auto-download (unlike ripgrep): bfs/ugrep publish no prebuilt static
/// release assets, so the release pipeline supplies the path. Unset → not
/// bundled (the runtime resolver falls back to `~/.xvora/vendor` / `$PATH`);
/// never a hard failure, so an un-wired build still succeeds.
fn bundle_search_tool(
    name: &str,
    name_uc: &str,
    ver: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let override_env = format!("XVORA_TOOLS_BUNDLE_{name_uc}_PATH");
    println!("cargo:rerun-if-env-changed={override_env}");
    // Always declare the cfg so `#[cfg(bundle_<name>)]` is lint-clean when unset.
    println!("cargo:rustc-check-cfg=cfg(bundle_{name})");

    // The consumer (`embedded_search_tools`) is `#[cfg(unix)]`, so embedding on a
    // Windows target is dead weight — skip (mirrors the ripgrep Windows skip).
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        return Ok(());
    }

    let Some(src) = env::var(&override_env).ok().filter(|s| !s.is_empty()) else {
        return Ok(());
    };

    let gen_dir = PathBuf::from(env::var("OUT_DIR")?).join(format!("bundle-{name}"));
    fs::create_dir_all(&gen_dir)?;
    let dest = gen_dir.join(format!("{name}-{ver}-override.bin"));
    let _ = fs::remove_file(&dest);
    fs::copy(&src, &dest)
        .map_err(|e| format!("copy {override_env} from {src} to {}: {e}", dest.display()))?;

    println!("cargo:rustc-cfg=bundle_{name}");
    println!("cargo:rustc-env=XVORA_TOOLS_{name_uc}_VER={ver}");
    println!("cargo:rustc-env=XVORA_TOOLS_{name_uc}_TARGET=override");
    Ok(())
}

/// Download + embed ripgrep. Unchanged behavior; split out of `main` so the new
/// search-tool bundling runs regardless of ripgrep's early returns.
fn bundle_rg() -> Result<(), Box<dyn std::error::Error>> {
    // Only bundle in release builds to avoid slowing down cargo check.
    println!("cargo:rerun-if-env-changed=XVORA_TOOLS_BUNDLE_RG_PATH");
    // Declare our custom cfg to the compiler so cfg(bundle_rg) is recognized by lints
    println!("cargo:rustc-check-cfg=cfg(bundle_rg)");

    let gen_dir = PathBuf::from(env::var("OUT_DIR")?).join("bundle-rg");
    fs::create_dir_all(&gen_dir)?;

    // Decide whether to bundle: path override OR release build
    let path_override = env::var("XVORA_TOOLS_BUNDLE_RG_PATH").ok();
    let is_release = env::var("PROFILE").as_deref() == Ok("release");
    if path_override.is_none() && !is_release {
        return Ok(());
    }

    // Expose cfg so the crate can include the bundled bytes.
    println!("cargo:rustc-cfg=bundle_rg");
    println!("cargo:rustc-env=XVORA_TOOLS_RG_VER={}", RG_VER);

    // If a local rg binary is provided, copy it directly (skips target check).
    if let Some(path) = path_override {
        let dest = gen_dir.join(format!("rg-{}-override.bin", RG_VER));
        println!("cargo:rustc-env=XVORA_TOOLS_RG_TARGET=override");
        let _ = fs::remove_file(&dest);
        fs::copy(PathBuf::from(path.clone()), &dest).map_err(|e| {
            format!(
                "Failed copying XVORA_TOOLS_BUNDLE_RG_PATH: {e} from path {path} to dest {}",
                dest.display()
            )
        })?;
        return Ok(());
    }

    // Determine supported ripgrep asset triple for auto-download.
    // Unix: .tar.gz with binary `rg`. Windows: .zip with `rg.exe`.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let (asset_triple, is_zip) = match (target_os.as_str(), target_arch.as_str(), target_env.as_str())
    {
        ("macos", "aarch64", _) => ("aarch64-apple-darwin", false),
        ("macos", "x86_64", _) => ("x86_64-apple-darwin", false),
        ("linux", "x86_64", _) => ("x86_64-unknown-linux-musl", false),
        ("linux", "aarch64", _) => ("aarch64-unknown-linux-gnu", false),
        ("windows", "x86_64", "gnu") => ("x86_64-pc-windows-gnu", true),
        ("windows", "x86_64", _) => ("x86_64-pc-windows-msvc", true),
        ("windows", "aarch64", _) => ("aarch64-pc-windows-msvc", true),
        _ => {
            return Err(format!(
                "Unsupported target for ripgrep bundling: {os}-{arch}-{env}. Set XVORA_TOOLS_BUNDLE_RG_PATH to a local rg binary for offline or unsupported builds.",
                os = target_os,
                arch = target_arch,
                env = target_env
            )
            .into());
        }
    };

    println!("cargo:rustc-env=XVORA_TOOLS_RG_TARGET={}", asset_triple);
    let dest = gen_dir.join(format!("rg-{}-{}.bin", RG_VER, asset_triple));
    let _ = fs::remove_file(&dest);

    let ext = if is_zip { "zip" } else { "tar.gz" };
    let url = format!(
        "https://github.com/BurntSushi/ripgrep/releases/download/{v}/ripgrep-{v}-{t}.{ext}",
        v = RG_VER,
        t = asset_triple,
        ext = ext
    );

    let bytes: Vec<u8> = {
        let resp = reqwest::blocking::get(&url).map_err(|e| {
            format!(
                "Failed to download ripgrep: {}\nSet XVORA_TOOLS_BUNDLE_RG_PATH to a local rg for offline builds.",
                e
            )
        })?;
        if !resp.status().is_success() {
            return Err(format!(
                "HTTP {} downloading ripgrep. Set XVORA_TOOLS_BUNDLE_RG_PATH for offline builds.",
                resp.status()
            )
            .into());
        }
        resp.bytes()?.to_vec()
    };

    let found = if is_zip {
        extract_rg_from_zip(&bytes, &dest)?
    } else {
        extract_rg_from_tar_gz(&bytes, &dest)?
    };

    if !found {
        return Err(format!(
            "Could not find 'rg'/'rg.exe' in ripgrep archive {}. Set XVORA_TOOLS_BUNDLE_RG_PATH for offline builds.",
            url
        )
        .into());
    }

    Ok(())
}

fn extract_rg_from_tar_gz(
    bytes: &[u8],
    dest: &std::path::Path,
) -> Result<bool, Box<dyn std::error::Error>> {
    let gz = flate2::read::GzDecoder::new(bytes);
    let mut ar = tar::Archive::new(gz);
    for entry in ar.entries()? {
        let mut e = entry?;
        let p = e.path()?;
        if p.file_name().is_some_and(|n| n == "rg") {
            let mut data = Vec::new();
            io::copy(&mut e, &mut data)?;
            fs::write(dest, &data)?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn extract_rg_from_zip(
    bytes: &[u8],
    dest: &std::path::Path,
) -> Result<bool, Box<dyn std::error::Error>> {
    let cursor = io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().replace('\\', "/");
        let base = name.rsplit('/').next().unwrap_or(name.as_str());
        if base.eq_ignore_ascii_case("rg.exe") || base == "rg" {
            let mut data = Vec::new();
            io::copy(&mut file, &mut data)?;
            fs::write(dest, &data)?;
            return Ok(true);
        }
    }
    Ok(false)
}
