//! Session synthesis and in-process e2e harness for the load-perf and fork bench tests.
//!
//! This module lives in `xvora-shell` (feature `test-support`) rather than `xvora-test-support`.
//! Synthesis drives the real `JsonlStorageAdapter`, so the reverse dependency would be circular.

pub mod e2e;
pub mod synth;
