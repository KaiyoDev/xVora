//! `xvora_home` override tests in an isolated binary so `xvora_home()`'s process-wide `OnceLock` initializes from the overridden env var.

use std::path::PathBuf;

#[test]
#[serial_test::serial(xvora_home)]
fn grok_home_override_path_helpers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let xvora_home = tmp.path().to_path_buf();
    unsafe {
        std::env::set_var("xvora_home", &xvora_home);
    }

    assert_eq!(
        xvora_pager::util::pager_toml_path(),
        xvora_home.join("pager.toml")
    );
    assert_eq!(xvora_pager::util::display_grok_home_prefix(), "$xvora_home");
    assert_eq!(
        xvora_pager::util::display_user_grok_path("config.toml"),
        "$xvora_home/config.toml"
    );

    let memory_path = xvora_home.join("memory/MEMORY.md");
    assert_eq!(
        xvora_pager::util::abbreviate_path(&memory_path.display().to_string()),
        "$xvora_home/memory/MEMORY.md"
    );

    // The copy toast abbreviates paths the same way, so a custom $xvora_home outside $HOME still shows the short form
    assert_eq!(
        xvora_pager::clipboard::display_copy_path(&xvora_home.join("last-copy.txt")),
        "$xvora_home/last-copy.txt"
    );

    assert!(xvora_pager::util::is_under_user_grok_home(&memory_path));
    assert!(!xvora_pager::util::is_under_user_grok_home(
        PathBuf::from("/tmp/other").as_path()
    ));
}

/// Isolated because `xvora_home()`'s `OnceLock` is already initialized by the time the shared lib-test binary reaches a case like this.
#[test]
#[serial_test::serial(xvora_home)]
fn disk_usage_run_creates_no_grok_home() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let ghost = tmp.path().join("ghost-home");
    unsafe {
        std::env::set_var("xvora_home", &ghost);
    }

    for json in [false, true] {
        xvora_pager::disk_usage_cmd::run(xvora_pager::disk_usage_cmd::DiskUsageArgs { json })
            .expect("a missing home is not an error");
        assert!(
            !ghost.exists(),
            "xvora du must not create the home it reports on (json={json})"
        );
    }
}
