//! Router module for managing page navigation and routing
//!
//! This module provides a flexible routing system that can be used
//! to manage different pages and views in interactive applications.

pub mod integration;
pub mod route;
pub mod router;

// Re-export commonly used types
pub use route::{Route, RouteId};
pub use router::{Router, RouterConfig, RouterError, RouterResult, RouterState};

// Re-export UI integration
pub use integration::{
    use_router, PageRenderer, RouterHandle, UIRouter, UIRouterBuildResult, UIRouterBuilder,
    UIRouterProps,
};
