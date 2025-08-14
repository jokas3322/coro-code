//! Route definitions and utilities
//!
//! This module defines the core route types and functionality
//! for the routing system.

use std::collections::HashMap;
use std::fmt;

/// Unique identifier for a route
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RouteId(pub String);

impl From<&str> for RouteId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<String> for RouteId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl fmt::Display for RouteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for RouteId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for RouteId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Route definition containing metadata and configuration
#[derive(Debug, Clone)]
pub struct Route {
    /// Unique identifier for this route
    pub id: RouteId,
    /// Human-readable name for this route
    pub name: String,
    /// Optional description of what this route displays
    pub description: Option<String>,
    /// Whether this route is the default route
    pub is_default: bool,
    /// Additional metadata for the route
    pub metadata: HashMap<String, String>,
}

impl Route {
    /// Create a new route with the given ID and name
    pub fn new(id: impl Into<RouteId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            is_default: false,
            metadata: HashMap::new(),
        }
    }
}
