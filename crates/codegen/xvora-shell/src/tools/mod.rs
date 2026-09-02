//! All tool execution goes through `xvora-tools` via the `ToolBridge`.
//! Types (ToolOutput, ToolInput, TodoState, etc.) come from `xvora-tools` directly.

pub mod bridge;
pub mod config;
pub mod notification_bridge;
pub mod retry;
pub(crate) mod task_completed_frame;
pub mod todo;
pub mod tool_context;

pub use self::{
    config::{BashToolConfig, FileToolset, ShellToolsetConfig},
    retry::{RetryConfig, execute_with_retry},
    tool_context::ToolContext,
};

// Re-export key types from xvora-tools for convenience
pub use self::todo::{TodoId, TodoItem, TodoPriority, TodoStatus};
pub use tools::types::output::ToolOutput;
pub use tools::types::{MCPToolInput, ToolInput};
