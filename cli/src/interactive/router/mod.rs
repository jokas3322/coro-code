//! Router module for managing page navigation and routing
//!
//! This module provides a flexible routing system that can be used
//! to manage different pages and views in interactive applications.

pub mod core;
pub mod integration;
pub mod route;

// Re-export commonly used types
pub use core::{RouterConfig, RouterResult};
pub use route::{Route, RouteId};

// Re-export UI integration
pub use integration::{use_router_handle, UIRouter, UIRouterBuilder};
