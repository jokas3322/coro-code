//! Agent core logic and execution engine

pub mod base;
pub mod core;
pub mod execution;
pub mod prompt;

pub use base::{Agent, AgentResult};
pub use core::AgentCore;
pub use execution::AgentExecution;
pub use prompt::{build_system_prompt_with_context, build_user_message, TRAE_AGENT_SYSTEM_PROMPT};
