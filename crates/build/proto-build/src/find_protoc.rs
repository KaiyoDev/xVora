use anyhow::{Context, bail};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn check_protoc_good(protoc: &Path) -> anyhow::Result<()> {
    let output = Command::new(protoc)
        .arg("--version")
        .output()
        .context("Failed to execute protoc")?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "protoc --version failed, likely dotslash is missing; \
             try `cargo install dotslash`; stdout: {stdout:?}, stderr: {stderr:?}"
        );
    }
    Ok(())
}

fn is_github_actions() -> bool {
    env::var_os("GITHUB_ACTIONS").is_some()
}

/// Find `protoc` command.
///
/// Search order:
/// 1. `$PROTOC` environment variable (set by Bazel `build_script_env` or user override)
/// 2. `bin/protoc` walking up parent directories (dotslash wrapper for local dev)
/// 3. `protoc` on `$PATH` (system install or other tooling)
///
/// When `bin/protoc` exists but fails to execute (e.g. the dotslash wrapper running
/// in Bazel remote execution where `dotslash` is not installed), the error is not fatal —
/// we fall through to the PATH-based lookup instead.
///
/// Returns `Ok(None)` if not found and not in a strict environment (GitHub Actions).
pub fn find_protoc() -> anyhow::Result<Option<PathBuf>> {
    // 1. Check the PROTOC env var first. This is the standard override used by prost-build
    //    and is set by Bazel cargo_build_script build_script_env to point at a hermetic
    //    protoc binary instead of the dotslash wrapper.
    if let Ok(protoc_env) = env::var("PROTOC") {
        let protoc = PathBuf::from(&protoc_env);
        if protoc.try_exists()? {
            check_protoc_good(&protoc)?;
            return Ok(Some(protoc));
        }
    }

    // 2. Walk up directories looking for a usable protoc.
    //
    // Unix: prefer `bin/protoc` (DotSlash launcher for local dev).
    // Windows: skip DotSlash `bin/protoc` (shebang/JSON, not a PE binary —
    // CreateProcess fails with os error 193). Instead look for a hermetic
    // install under `tools/protoc-*/bin/protoc.exe` (see CONTRIBUTING /
    // download of protoc-*-win64.zip).
    {
        let cwd = env::current_dir()?;
        let mut dir = cwd.clone();
        let mut dir_rel = PathBuf::new();
        loop {
            if cfg!(windows) {
                // tools/protoc-<ver>/bin/protoc.exe
                let tools = dir.join("tools");
                if tools.is_dir()
                    && let Ok(entries) = fs::read_dir(&tools)
                {
                    let mut candidates: Vec<PathBuf> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_name().to_string_lossy().starts_with("protoc-"))
                        .map(|e| e.path().join("bin").join("protoc.exe"))
                        .filter(|p| p.is_file())
                        .collect();
                    // Prefer higher version directory names last sort.
                    candidates.sort();
                    if let Some(protoc) = candidates.pop() {
                        match check_protoc_good(&protoc) {
                            Ok(()) => return Ok(Some(protoc)),
                            Err(e) => {
                                eprintln!(
                                    "tools protoc at `{}` failed: {e:#}; trying PATH",
                                    protoc.display()
                                );
                            }
                        }
                    }
                }
            } else {
                // Return relative path to make build more deterministic.
                let protoc = dir_rel.join("bin/protoc");
                if protoc.try_exists()? {
                    match check_protoc_good(&protoc) {
                        Ok(()) => return Ok(Some(protoc)),
                        Err(e) => {
                            // bin/protoc exists but can't execute — likely the
                            // dotslash wrapper without dotslash installed.
                            // Fall through to PATH-based lookup below.
                            eprintln!(
                                "bin/protoc found at `{}` but failed to execute: {e:#}; \
                                 trying protoc from PATH as fallback",
                                protoc.display()
                            );
                            break;
                        }
                    }
                }
            }
            if !dir.pop() {
                break;
            }
            dir_rel.push("..");
        }
    }

    // 3. Try protoc from PATH (system install or other tooling).
    if check_protoc_good(Path::new("protoc")).is_ok() {
        return Ok(Some(PathBuf::from("protoc")));
    }

    // 4. Not found anywhere.
    if is_github_actions() {
        return Err(anyhow::anyhow!(
            "`protoc` not found (checked $PROTOC env, tools/protoc-*/bin/protoc.exe, bin/protoc, and PATH)"
        ));
    }
    eprintln!(
        "`protoc` not found; on Windows download protoc win64 zip into tools/protoc-<ver>/ \
         (e.g. tools/protoc-29.3/bin/protoc.exe) or set $PROTOC"
    );
    Ok(None)
}
