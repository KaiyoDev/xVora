//! Shared utilities used by both `xvora-shell` and its downstream clients
//! (e.g. `xvora-pager-render`). This crate sits upstream of `xvora-shell`
//! so it must never depend on it.

pub mod clipboard;
pub mod placeholder_images;
pub mod session;
pub mod stderr;
pub mod ui_config;
