//! UI framework integration for the router system
//!
//! This module provides UI framework-specific components and utilities for the router system.
//! It bridges the core router functionality with the UI component system.

use super::{Route, RouteId, Router as CoreRouter, RouterConfig, RouterResult};
use iocraft::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Type alias for page render functions
/// Each page is a function that takes hooks and returns an element
pub type PageRenderer = Box<dyn Fn(Hooks) -> AnyElement<'static> + Send + Sync>;

/// A shareable, UI-friendly handle to control the router
#[derive(Clone)]
pub struct RouterHandle(Arc<Mutex<CoreRouter>>);

impl RouterHandle {
    /// Create a new router handle
    pub fn new(router: CoreRouter) -> Self {
        Self(Arc::new(Mutex::new(router)))
    }

    /// Navigate to a route
    pub fn navigate(&self, id: impl Into<RouteId>) -> RouterResult<()> {
        let mut guard = self.0.lock().unwrap();
        guard.navigate(id)
    }

    /// Get the current route ID
    pub fn current_route_id(&self) -> RouteId {
        let guard = self.0.lock().unwrap();
        guard.current_route_id().clone()
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        let guard = self.0.lock().unwrap();
        guard.can_go_back()
    }

    /// Go back to the previous route
    pub fn go_back(&self) -> bool {
        let mut guard = self.0.lock().unwrap();
        guard.go_back()
    }
}

/// Get current route from handle (simplified version)
/// For full reactive navigation, use RouterHandle directly
pub fn use_router(handle: &RouterHandle) -> RouteId {
    handle.current_route_id()
}

/// UI router component properties
#[derive(Props)]
pub struct UIRouterProps {
    /// Router handle for navigation control
    pub handle: RouterHandle,
    /// Map of route IDs to their corresponding page renderers
    pub pages: HashMap<RouteId, PageRenderer>,
    /// Optional fallback page for unknown routes
    pub fallback_page: Option<PageRenderer>,
}

impl Default for UIRouterProps {
    fn default() -> Self {
        // Create a minimal default configuration for testing
        let config = RouterConfig::new();
        let router = CoreRouter::new(config).unwrap_or_else(|_| {
            // Fallback: create a router with a default route
            let config = RouterConfig::new()
                .add_route(Route::new("default", "Default"))
                .with_default_route("default".into());
            CoreRouter::new(config).expect("Failed to create default router")
        });

        let handle = RouterHandle::new(router);
        Self {
            handle,
            pages: HashMap::new(),
            fallback_page: None,
        }
    }
}

/// UI router component that renders different pages based on current route
#[component]
pub fn UIRouter(hooks: Hooks, props: &UIRouterProps) -> impl Into<AnyElement<'static>> {
    let current_route_id = props.handle.current_route_id();

    // Try to find the page renderer for the current route
    let page_element = if let Some(page_renderer) = props.pages.get(&current_route_id) {
        // Render the page for the current route
        page_renderer(hooks)
    } else if let Some(fallback_renderer) = &props.fallback_page {
        // Render fallback page if route not found
        fallback_renderer(hooks)
    } else {
        // Default fallback: show route not found message
        element! {
            View(
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: 100pct,
                height: 100pct,
                padding: 2,
            ) {
                Text(
                    content: "Route Not Found",
                    weight: Weight::Bold,
                    color: Color::Red
                )
                Text(
                    content: format!("Unknown route: {}", current_route_id.0)
                )
            }
        }
        .into()
    };

    element! {
        View(
            key: "router-container",
            width: 100pct,
            height: 100pct,
        ) {
            #(page_element)
        }
    }
}

/// Builder for creating UI router configurations with pages
pub struct UIRouterBuilder {
    config: RouterConfig,
    pages: HashMap<RouteId, PageRenderer>,
    fallback_page: Option<PageRenderer>,
}

impl UIRouterBuilder {
    /// Create a new UI router builder
    pub fn new() -> Self {
        Self {
            config: RouterConfig::new(),
            pages: HashMap::new(),
            fallback_page: None,
        }
    }

    /// Add a route with its corresponding page renderer
    pub fn route<F>(
        mut self,
        id: impl Into<RouteId>,
        name: impl Into<String>,
        page_renderer: F,
    ) -> Self
    where
        F: Fn(Hooks) -> AnyElement<'static> + Send + Sync + 'static,
    {
        let route = Route::new(id.into(), name.into());
        let route_id = route.id.clone();
        self.config = self.config.add_route(route);
        self.pages.insert(route_id, Box::new(page_renderer));
        self
    }

    /// Set a fallback page for unknown routes
    pub fn fallback<F>(mut self, fallback_renderer: F) -> Self
    where
        F: Fn(Hooks) -> AnyElement<'static> + Send + Sync + 'static,
    {
        self.fallback_page = Some(Box::new(fallback_renderer));
        self
    }

    /// Set the default route
    pub fn default(mut self, id: impl Into<RouteId>) -> Self {
        self.config = self.config.with_default_route(id.into());
        self
    }

    /// Build the UI router props with a handle
    pub fn build(self) -> RouterResult<UIRouterBuildResult> {
        let router = CoreRouter::new(self.config)?;
        let handle = RouterHandle::new(router);

        Ok(UIRouterBuildResult {
            props: UIRouterProps {
                handle: handle.clone(),
                pages: self.pages,
                fallback_page: self.fallback_page,
            },
            handle,
        })
    }
}

/// Result of building a UI router with handle
pub struct UIRouterBuildResult {
    /// UI router props for the component
    pub props: UIRouterProps,
    /// Router handle for navigation control
    pub handle: RouterHandle,
}

impl Default for UIRouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_router_builder() {
        let build_result = UIRouterBuilder::new()
            .route("home", "Home", |_hooks| {
                element! { Text(content: "Home Page") }.into()
            })
            .route("about", "About", |_hooks| {
                element! { Text(content: "About Page") }.into()
            })
            .default("home")
            .build()
            .expect("Failed to build router");

        assert_eq!(build_result.handle.current_route_id().0, "home");
        assert_eq!(build_result.props.pages.len(), 2);
    }

    #[test]
    fn test_default_router_props() {
        let props = UIRouterProps::default();
        assert_eq!(props.handle.current_route_id().0, "default");
        assert!(props.pages.is_empty());
        assert!(props.fallback_page.is_none());
    }

    #[test]
    fn test_router_handle() {
        let config = RouterConfig::new()
            .add_route(Route::new("home", "Home"))
            .add_route(Route::new("about", "About"))
            .with_default_route("home".into());

        let router = CoreRouter::new(config).expect("Failed to create router");
        let handle = RouterHandle::new(router);

        // Test initial state
        assert_eq!(handle.current_route_id().0, "home");
        assert!(!handle.can_go_back());

        // Test navigation
        handle.navigate("about").expect("Failed to navigate");
        assert_eq!(handle.current_route_id().0, "about");
        assert!(handle.can_go_back());

        // Test go back
        assert!(handle.go_back());
        assert_eq!(handle.current_route_id().0, "home");
        assert!(!handle.can_go_back());
    }

    #[test]
    fn test_sugar_builder_methods() {
        let build_result = UIRouterBuilder::new()
            .route("home", "Home", |_hooks| {
                element! { Text(content: "Home") }.into()
            })
            .route("settings", "Settings", |_hooks| {
                element! { Text(content: "Settings") }.into()
            })
            .default("home")
            .build()
            .expect("Failed to build router");

        assert_eq!(build_result.handle.current_route_id().0, "home");
        assert_eq!(build_result.props.pages.len(), 2);
    }

    #[test]
    fn test_router_error_types() {
        use crate::interactive::router::RouterError;

        // Test RouterError variants
        let error = RouterError::NoRoutes;
        assert_eq!(error.to_string(), "No routes configured");

        let error = RouterError::RouteNotFound("test".to_string());
        assert_eq!(error.to_string(), "Route 'test' not found");

        let error = RouterError::InitialRouteMissing("missing".to_string());
        assert_eq!(
            error.to_string(),
            "Initial route 'missing' not found in configuration"
        );
    }

    #[test]
    fn test_route_id_traits() {
        let route_id = RouteId::from("test");

        // Test Display
        assert_eq!(format!("{}", route_id), "test");

        // Test AsRef<str>
        let s: &str = route_id.as_ref();
        assert_eq!(s, "test");

        // Test Borrow<str>
        use std::borrow::Borrow;
        let s: &str = route_id.borrow();
        assert_eq!(s, "test");
    }

    #[test]
    fn test_backward_compatibility() {
        // Test that new API works correctly
        let config = RouterConfig::new()
            .add_route(Route::new("home", "Home"))
            .with_default_route("home".into());

        let mut router = CoreRouter::new(config).expect("Failed to create router");

        router.navigate("home").expect("Failed to navigate");

        assert_eq!(router.current_route_id().0, "home");
    }
}
