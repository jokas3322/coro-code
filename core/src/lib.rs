//! # coro-code Core
//!
//! Core library for coro-code - a high-performance AI coding agent.
//!
//! This library provides the fundamental building blocks for creating AI agents
//! that can interact with codebases, execute tools, and perform complex software
//! engineering tasks.

// Core modules
pub mod agent;
pub mod config;
pub mod error;
pub mod llm;
pub mod output;
pub mod tools;
pub mod trajectory;

// Re-export commonly used types
pub use agent::{Agent, AgentBuilder, AgentConfig, OutputMode};
pub use config::{ModelParams, Protocol, ResolvedLlmConfig};
pub use trajectory::TrajectoryRecorder;

/// Current version of the coro-core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize tracing for the library
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

/// Initialize tracing with a specific debug mode
pub fn init_tracing_with_debug(debug: bool) {
    let filter = if debug { "debug" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(filter))
        .init();
}
