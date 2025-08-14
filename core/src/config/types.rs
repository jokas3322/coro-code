//! Minimal configuration types for Lode core
//!
//! Core only accepts fully resolved, validated configuration.
//! All discovery, loading, and merging happens in CLI layer.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported LLM protocols
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    /// OpenAI-compatible API (includes OpenAI, many proxies, local models)
    #[serde(rename = "openai_compat")]
    OpenAICompat,
    /// Anthropic Claude API
    #[serde(rename = "anthropic")]
    Anthropic,
    /// Google AI API (Gemini)
    #[serde(rename = "google_ai")]
    GoogleAI,
    /// Azure OpenAI API
    #[serde(rename = "azure_openai")]
    AzureOpenAI,
    /// Custom protocol
    #[serde(rename = "custom")]
    Custom(String),
}

impl Protocol {
    /// Get the protocol name as a string
    pub fn as_str(&self) -> &str {
        match self {
            Protocol::OpenAICompat => "openai_compat",
            Protocol::Anthropic => "anthropic",
            Protocol::GoogleAI => "google_ai",
            Protocol::AzureOpenAI => "azure_openai",
            Protocol::Custom(name) => name,
        }
    }

    /// Get the default base URL for this protocol
    pub fn default_base_url(&self) -> Option<&'static str> {
        match self {
            Protocol::OpenAICompat => Some("https://api.openai.com/v1"),
            Protocol::Anthropic => Some("https://api.anthropic.com"),
            Protocol::GoogleAI => Some("https://generativelanguage.googleapis.com/v1beta"),
            Protocol::AzureOpenAI => None, // Requires custom endpoint
            Protocol::Custom(_) => None,
        }
    }
}

/// Model parameters for LLM requests
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelParams {
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Temperature for sampling (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Top-p sampling parameter
    pub top_p: Option<f32>,
    /// Top-k sampling parameter (for compatible models)
    pub top_k: Option<u32>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
}

/// A fully resolved LLM configuration ready for use by core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedLlmConfig {
    /// The protocol to use
    pub protocol: Protocol,
    /// Base URL for the API
    pub base_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Model name/identifier
    pub model: String,
    /// Model parameters
    #[serde(default)]
    pub params: ModelParams,
    /// Additional headers for requests
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

impl ResolvedLlmConfig {
    /// Create a new resolved LLM config
    pub fn new(protocol: Protocol, base_url: String, api_key: String, model: String) -> Self {
        Self {
            protocol,
            base_url,
            api_key,
            model,
            params: ModelParams::default(),
            headers: HashMap::new(),
        }
    }

    /// Set model parameters
    pub fn with_params(mut self, params: ModelParams) -> Self {
        self.params = params;
        self
    }

    /// Add a header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Add multiple headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }

        if self.model.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }

        if self.base_url.is_empty() {
            return Err("Base URL cannot be empty".to_string());
        }

        // Validate URL format
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err("Base URL must start with http:// or https://".to_string());
        }

        // Validate temperature range
        if let Some(temp) = self.params.temperature {
            if temp < 0.0 || temp > 2.0 {
                return Err("Temperature must be between 0.0 and 2.0".to_string());
            }
        }

        // Validate top_p range
        if let Some(top_p) = self.params.top_p {
            if top_p < 0.0 || top_p > 1.0 {
                return Err("Top-p must be between 0.0 and 1.0".to_string());
            }
        }

        Ok(())
    }
}
