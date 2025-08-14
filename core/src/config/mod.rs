//! Minimal configuration module for coro-code core
//!
//! Only exports pure data types. All loading logic is in CLI layer.

pub mod types;

pub use types::{ModelParams, Protocol, ResolvedLlmConfig};
