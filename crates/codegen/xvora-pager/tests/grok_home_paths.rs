//! `XVORA_HOME` override tests in an isolated binary so `xvora_home()`'s
//! process-wide `OnceLock` initializes from the overridden env var.

use std::path::PathBuf;

#[test]
fn xvora_home_override_path_helpers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let xvora_home = tmp.path().to_path_buf();
    unsafe {
        std::env::set_var("XVORA_HOME", &xvora_home);
    }

    assert_eq!(
        xvora_pager::util::pager_toml_path(),
        xvora_home.join("pager.toml")
    );
    assert_eq!(
        xvora_pager::util::display_xvora_home_prefix(),
        "$XVORA_HOME"
    );
    assert_eq!(
        xvora_pager::util::display_user_grok_path("config.toml"),
        "$XVORA_HOME/config.toml"
    );

    let memory_path = xvora_home.join("memory/MEMORY.md");
    assert_eq!(
        xvora_pager::util::abbreviate_path(&memory_path.display().to_string()),
        "$XVORA_HOME/memory/MEMORY.md"
    );

    assert!(xvora_pager::util::is_under_user_xvora_home(&memory_path));
    assert!(!xvora_pager::util::is_under_user_xvora_home(
        PathBuf::from("/tmp/other").as_path()
    ));
}
