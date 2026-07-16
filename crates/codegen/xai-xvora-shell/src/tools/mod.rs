//! Tool infrastructure for xai-xvora-shell.
//!
//! All tool execution goes through `xai-xvora-tools` via the `ToolBridge`.
//! Types (ToolOutput, ToolInput, TodoState, etc.) come from `xai-xvora-tools` directly.

pub mod bridge;
pub mod config;
pub mod notification_bridge;
pub mod retry;
pub mod todo;
pub mod tool_context;

pub use self::{
    config::{BashToolConfig, FileToolset, ShellToolsetConfig},
    retry::{RetryConfig, execute_with_retry},
    tool_context::ToolContext,
};

// Re-export key types from xai-xvora-tools for convenience
pub use self::todo::{TodoId, TodoItem, TodoPriority, TodoStatus};
pub use xai_xvora_tools::types::output::ToolOutput;
pub use xai_xvora_tools::types::{MCPToolInput, ToolInput};
