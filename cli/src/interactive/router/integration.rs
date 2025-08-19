//! UI framework integration for the router system
//!
//! This module provides UI framework-specific components and utilities for the router system.
//! It bridges the core router functionality with the UI component system using reactive state management.

use super::{Route, RouteId, RouterConfig, RouterResult};
use iocraft::prelude::*;
use std::collections::HashMap;

/// Type alias for page render functions
/// Each page is a function that takes hooks and returns an element
pub type PageRenderer = Box<dyn Fn(Hooks) -> AnyElement<'static> + Send + Sync>;

/// Reactive router configuration that holds route definitions
#[derive(Clone)]
pub struct ReactiveRouterConfig {
    /// Router configuration for validation and metadata
    pub config: RouterConfig,
    /// Maximum history size
    pub max_history: usize,
}

impl ReactiveRouterConfig {
    /// Create a new reactive router config from configuration
    pub fn new(config: RouterConfig) -> RouterResult<Self> {
        // Validate that we have at least one route
        if config.routes().is_empty() {
            return Err(super::core::RouterError::NoRoutes);
        }

        Ok(Self {
            config,
            max_history: 50,
        })
    }

    /// Get the initial route for this configuration
    pub fn initial_route(&self) -> RouteId {
        if let Some(default_route) = self.config.default_route() {
            default_route.clone()
        } else if let Some((route_id, _)) = self.config.routes().iter().next() {
            route_id.clone()
        } else {
            RouteId::from("default") // Fallback
        }
    }
}

/// Reactive router handle that uses iocraft's state management
/// This provides a simpler, more direct approach to routing without async complexity
#[derive(Clone)]
pub struct ReactiveRouterHandle {
    /// Current route state - changes automatically trigger UI updates
    current_route: State<RouteId>,
    /// Navigation history for back functionality
    history: State<Vec<RouteId>>,
    /// Router configuration for validation and metadata
    config: ReactiveRouterConfig,
}

impl ReactiveRouterHandle {
    /// Create a new reactive router handle with hooks
    /// This must be called from within a component context
    pub fn new_with_hooks(hooks: &mut Hooks, config: ReactiveRouterConfig) -> Self {
        let initial_route = config.initial_route();

        // Initialize reactive states
        let current_route = hooks.use_state(|| initial_route);
        let history = hooks.use_state(Vec::new);

        Self {
            current_route,
            history,
            config,
        }
    }

    /// Navigate to a route - this is now synchronous and reactive
    pub fn navigate(&mut self, id: impl Into<RouteId>) -> RouterResult<()> {
        let route_id = id.into();

        // Validate route exists
        if !self.config.config.routes().contains_key(&route_id) {
            return Err(super::core::RouterError::RouteNotFound(route_id.0));
        }

        // Update history if route is different
        let current = self.current_route.read().clone();
        if current != route_id {
            let mut hist = self.history.read().clone();
            hist.insert(0, current);

            // Trim history if needed
            if hist.len() > self.config.max_history {
                hist.truncate(self.config.max_history);
            }

            self.history.set(hist);
        }

        // Update current route - this automatically triggers UI re-render
        self.current_route.set(route_id);
        Ok(())
    }

    /// Get the current route ID
    pub fn current_route_id(&self) -> RouteId {
        self.current_route.read().clone()
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        !self.history.read().is_empty()
    }

    /// Go back to the previous route
    pub fn go_back(&mut self) -> bool {
        let mut hist = self.history.read().clone();
        if let Some(previous_route) = hist.first().cloned() {
            hist.remove(0);
            self.history.set(hist);
            self.current_route.set(previous_route);
            true
        } else {
            false
        }
    }

    /// Get the current route state for reactive access
    pub fn current_route_state(&self) -> State<RouteId> {
        self.current_route
    }
}

/// Router context for sharing router handle with child components
#[derive(Clone)]
pub struct RouterContext {
    pub handle: ReactiveRouterHandle,
}

/// Get current route from handle (simplified version)
/// For full reactive navigation, use ReactiveRouterHandle directly
pub fn use_router(handle: &ReactiveRouterHandle) -> RouteId {
    handle.current_route_id()
}

/// Hook for child components to access the router handle for navigation
/// Returns the ReactiveRouterHandle from the current context
pub fn use_router_handle(hooks: &mut Hooks) -> ReactiveRouterHandle {
    let context = hooks.use_context::<RouterContext>();
    context.handle.clone()
}

/// Hook for child components to access the current route ID reactively
/// Returns the current route state that automatically updates UI when changed
pub fn use_route(hooks: &mut Hooks) -> State<RouteId> {
    let context = hooks.use_context::<RouterContext>();
    context.handle.current_route_state()
}

/// UI router component properties
#[derive(Props)]
pub struct UIRouterProps {
    /// Router configuration
    pub config: ReactiveRouterConfig,
    /// Map of route IDs to their corresponding page renderers
    pub pages: HashMap<RouteId, PageRenderer>,
    /// Optional fallback page for unknown routes
    pub fallback_page: Option<PageRenderer>,
}

impl Default for UIRouterProps {
    fn default() -> Self {
        // Create a minimal default configuration for testing
        let router_config = RouterConfig::new()
            .add_route(Route::new("default", "Default"))
            .with_default_route("default".into());

        let config = ReactiveRouterConfig::new(router_config)
            .expect("Failed to create default reactive router config");

        Self {
            config,
            pages: HashMap::new(),
            fallback_page: None,
        }
    }
}

/// UI router component that renders different pages based on current route
/// Now uses reactive state management for automatic UI updates
#[component]
pub fn UIRouter(mut hooks: Hooks, props: &UIRouterProps) -> impl Into<AnyElement<'static>> {
    // Create the router handle with proper hooks context
    let handle = ReactiveRouterHandle::new_with_hooks(&mut hooks, props.config.clone());

    // Get the reactive current route state - this automatically triggers re-renders
    let current_route_state = handle.current_route_state();
    let current_route_id = current_route_state.read().clone();

    // Create router context to share with child components
    let router_context = RouterContext {
        handle: handle.clone(),
    };

    element! {
        ContextProvider(value: Context::owned(router_context)) {
            View(
                key: "router-container",
                width: 100pct,
                height: 100pct,
            ) {
                // Use conditional rendering to show the correct page
                #(if let Some(page_renderer) = props.pages.get(&current_route_id) {
                    Some(page_renderer(hooks))
                } else if let Some(fallback_renderer) = &props.fallback_page {
                    Some(fallback_renderer(hooks))
                } else {
                    Some(element! {
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
                    }.into())
                })
            }
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

    /// Build the UI router props
    pub fn build(self) -> RouterResult<UIRouterBuildResult> {
        let config = ReactiveRouterConfig::new(self.config)?;

        Ok(UIRouterBuildResult {
            props: UIRouterProps {
                config: config.clone(),
                pages: self.pages,
                fallback_page: self.fallback_page,
            },
            config,
        })
    }
}

/// Result of building a UI router
pub struct UIRouterBuildResult {
    /// UI router props for the component
    pub props: UIRouterProps,
    /// Router configuration for external access
    pub config: ReactiveRouterConfig,
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

        assert_eq!(build_result.config.initial_route().0, "home");
        assert_eq!(build_result.props.pages.len(), 2);
    }

    #[test]
    fn test_default_router_props() {
        let props = UIRouterProps::default();
        assert_eq!(props.config.initial_route().0, "default");
        assert!(props.pages.is_empty());
        assert!(props.fallback_page.is_none());
    }

    #[test]
    fn test_reactive_router_config() {
        let router_config = RouterConfig::new()
            .add_route(Route::new("home", "Home"))
            .add_route(Route::new("about", "About"))
            .with_default_route("home".into());

        let config = ReactiveRouterConfig::new(router_config).expect("Failed to create config");

        // Test initial route
        assert_eq!(config.initial_route().0, "home");
        assert_eq!(config.max_history, 50);
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

        assert_eq!(build_result.config.initial_route().0, "home");
        assert_eq!(build_result.props.pages.len(), 2);
    }

    #[test]
    fn test_router_error_types() {
        use crate::interactive::router::core::RouterError;

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
    fn test_reactive_router_compatibility() {
        // Test that new reactive API works correctly
        let router_config = RouterConfig::new()
            .add_route(Route::new("home", "Home"))
            .with_default_route("home".into());

        let config = ReactiveRouterConfig::new(router_config).expect("Failed to create config");

        // Test initial route
        assert_eq!(config.initial_route().0, "home");
    }
}
