//! Simple CLI configuration loader for coro-code
//!
//! Implements single-source priority loading with flag overrides:
//! 1. --config file/dir (highest priority)
//! 2. Current working directory: ./coro.json or ./.coro/config.json
//! 3. Git repository root: <repo_root>/.coro/config.json
//! 4. XDG config: $XDG_CONFIG_HOME/coro/config.json or ~/.config/coro/config.json
//! 5. Environment variables only (no files)

use anyhow::{anyhow, Context, Result};
use coro_core::{ModelParams, Protocol, ResolvedLlmConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Raw configuration file format (simple single-file schema)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawConfig {
    /// Protocol to use
    pub protocol: String,
    /// API key (can be "env:VAR_NAME" for environment variable)
    pub api_key: String,
    /// Base URL (optional, uses protocol default if not specified)
    pub base_url: Option<String>,
    /// Model name
    pub model: String,
    /// Model parameters (optional)
    #[serde(default)]
    pub params: ModelParams,
    /// Additional headers (optional)
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

/// CLI configuration loader
pub struct CliConfigLoader {
    /// Override config file/directory path
    config_override: Option<PathBuf>,
    /// Flag overrides
    protocol_override: Option<String>,
    api_key_override: Option<String>,
    base_url_override: Option<String>,
    model_override: Option<String>,
}

impl CliConfigLoader {
    /// Create a new loader
    pub fn new() -> Self {
        Self {
            config_override: None,
            protocol_override: None,
            api_key_override: None,
            base_url_override: None,
            model_override: None,
        }
    }

    /// Set config file/directory override
    pub fn with_config_override(mut self, path: PathBuf) -> Self {
        self.config_override = Some(path);
        self
    }

    /// Set protocol override
    pub fn with_protocol_override(mut self, protocol: String) -> Self {
        self.protocol_override = Some(protocol);
        self
    }

    /// Set API key override
    pub fn with_api_key_override(mut self, api_key: String) -> Self {
        self.api_key_override = Some(api_key);
        self
    }

    /// Set base URL override
    pub fn with_base_url_override(mut self, base_url: String) -> Self {
        self.base_url_override = Some(base_url);
        self
    }

    /// Set model override
    pub fn with_model_override(mut self, model: String) -> Self {
        self.model_override = Some(model);
        self
    }

    /// Load and resolve configuration
    pub async fn load(&self) -> Result<ResolvedLlmConfig> {
        // Step 1: Find and load base configuration
        let mut config = if let Some(override_path) = &self.config_override {
            // Use explicit config override
            self.load_from_path(override_path).await.with_context(|| {
                format!(
                    "Failed to load config from override path: {}",
                    override_path.display()
                )
            })?
        } else {
            // Search in priority order
            self.search_and_load().await?
        };

        // Step 2: Apply flag overrides
        if let Some(protocol) = &self.protocol_override {
            config.protocol = protocol.clone();
        }
        if let Some(api_key) = &self.api_key_override {
            config.api_key = api_key.clone();
        }
        if let Some(base_url) = &self.base_url_override {
            config.base_url = Some(base_url.clone());
        }
        if let Some(model) = &self.model_override {
            config.model = model.clone();
        }

        // Step 3: Resolve to final LLM config
        self.resolve_config(config).await
    }

    /// Search for config in priority order
    async fn search_and_load(&self) -> Result<RawConfig> {
        // 1. Current working directory
        if let Some(config) = self.try_load_cwd().await? {
            return Ok(config);
        }

        // 2. Git repository root
        if let Some(config) = self.try_load_git_root().await? {
            return Ok(config);
        }

        // 3. XDG config directory
        if let Some(config) = self.try_load_xdg().await? {
            return Ok(config);
        }

        // 4. Environment variables only
        self.try_load_env_only().await
    }

    /// Try loading from current working directory
    async fn try_load_cwd(&self) -> Result<Option<RawConfig>> {
        let cwd = std::env::current_dir()?;

        // Try ./coro.json first
        let coro_json = cwd.join("coro.json");
        if coro_json.exists() {
            return Ok(Some(self.load_file(&coro_json).await?));
        }

        // Try ./.coro/config.json
        let coro_dir_config = cwd.join(".coro").join("config.json");
        if coro_dir_config.exists() {
            return Ok(Some(self.load_file(&coro_dir_config).await?));
        }

        Ok(None)
    }

    /// Try loading from git repository root
    async fn try_load_git_root(&self) -> Result<Option<RawConfig>> {
        if let Some(git_root) = self.find_git_root()? {
            let config_path = git_root.join(".coro").join("config.json");
            if config_path.exists() {
                return Ok(Some(self.load_file(&config_path).await?));
            }
        }
        Ok(None)
    }

    /// Try loading from XDG config directory
    async fn try_load_xdg(&self) -> Result<Option<RawConfig>> {
        if let Some(config_dir) = self.get_xdg_config_dir() {
            let config_path = config_dir.join("coro").join("config.json");
            if config_path.exists() {
                return Ok(Some(self.load_file(&config_path).await?));
            }
        }
        Ok(None)
    }

    /// Try loading from environment variables only
    async fn try_load_env_only(&self) -> Result<RawConfig> {
        // Check for common API keys
        let openai_key = std::env::var("OPENAI_API_KEY").ok();
        let anthropic_key = std::env::var("ANTHROPIC_API_KEY").ok();
        let google_key = std::env::var("GOOGLE_API_KEY").ok();
        let azure_key = std::env::var("AZURE_OPENAI_API_KEY").ok();

        // Check for base URL environment variables
        let openai_base_url = std::env::var("OPENAI_BASE_URL").ok();
        let anthropic_base_url = std::env::var("ANTHROPIC_BASE_URL").ok();
        let google_base_url = std::env::var("GOOGLE_BASE_URL").ok();
        let azure_base_url = std::env::var("AZURE_OPENAI_BASE_URL").ok();

        // Check for model environment variables
        let openai_model = std::env::var("OPENAI_MODEL").ok();
        let anthropic_model = std::env::var("ANTHROPIC_MODEL").ok();
        let google_model = std::env::var("GOOGLE_MODEL").ok();
        let azure_model = std::env::var("AZURE_OPENAI_MODEL").ok();

        // Check for generic overrides
        let coro_base_url = std::env::var("CORO_BASE_URL").ok();
        let coro_model = std::env::var("CORO_MODEL").ok();

        let available_keys: Vec<_> = [
            openai_key.as_ref().map(|_| "openai"),
            anthropic_key.as_ref().map(|_| "anthropic"),
            google_key.as_ref().map(|_| "google_ai"),
            azure_key.as_ref().map(|_| "azure_openai"),
        ]
        .into_iter()
        .flatten()
        .collect();

        // Check if we have a protocol override or CORO_PROTOCOL env var
        let env_protocol = std::env::var("CORO_PROTOCOL").ok();
        let protocol_preference = self.protocol_override.as_ref().or(env_protocol.as_ref());

        let protocol = if let Some(preferred_protocol) = protocol_preference {
            // Use the specified protocol if we have the corresponding API key
            match preferred_protocol.as_str() {
                "openai" if openai_key.is_some() => "openai",
                "anthropic" if anthropic_key.is_some() => "anthropic",
                "google_ai" if google_key.is_some() => "google_ai",
                "azure_openai" if azure_key.is_some() => "azure_openai",
                _ => return Err(anyhow!(
                    "Protocol '{}' specified but no corresponding API key found. Available keys: {}",
                    preferred_protocol,
                    available_keys.join(", ")
                )),
            }
        } else {
            // No protocol preference, use auto-detection logic
            match available_keys.len() {
                0 => return Err(anyhow!("No configuration found. Please create a coro.json file or set environment variables like OPENAI_API_KEY")),
                1 => available_keys[0],
                _ => return Err(anyhow!(
                    "Multiple API keys detected: {}. Please specify which protocol to use with CORO_PROTOCOL or --protocol",
                    available_keys.join(", ")
                )),
            }
        };

        let (api_key, default_model, base_url, model_env) = match protocol {
            "openai" => (
                openai_key.unwrap(),
                "gpt-4o",
                openai_base_url.or_else(|| coro_base_url.clone()),
                openai_model,
            ),
            "anthropic" => (
                anthropic_key.unwrap(),
                "claude-3-5-sonnet-20241022",
                anthropic_base_url.or_else(|| coro_base_url.clone()),
                anthropic_model,
            ),
            "google_ai" => (
                google_key.unwrap(),
                "gemini-pro",
                google_base_url.or_else(|| coro_base_url.clone()),
                google_model,
            ),
            "azure_openai" => (
                azure_key.unwrap(),
                "gpt-4",
                azure_base_url.or_else(|| coro_base_url.clone()),
                azure_model,
            ),
            _ => unreachable!(),
        };

        // Determine model with priority: protocol-specific env > generic env > default
        let model = model_env
            .or_else(|| coro_model.clone())
            .unwrap_or_else(|| default_model.to_string());

        Ok(RawConfig {
            protocol: protocol.to_string(),
            api_key,
            base_url, // Use environment variable if available, otherwise protocol default
            model,
            params: ModelParams::default(),
            headers: HashMap::new(),
        })
    }

    /// Load configuration from a specific path (file or directory)
    async fn load_from_path(&self, path: &Path) -> Result<RawConfig> {
        if path.is_file() {
            self.load_file(path).await
        } else if path.is_dir() {
            // Try config.json in the directory
            let config_file = path.join("config.json");
            if config_file.exists() {
                self.load_file(&config_file).await
            } else {
                Err(anyhow!(
                    "No config.json found in directory: {}",
                    path.display()
                ))
            }
        } else {
            Err(anyhow!("Config path does not exist: {}", path.display()))
        }
    }

    /// Load a single config file
    async fn load_file(&self, path: &Path) -> Result<RawConfig> {
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))
    }

    /// Find git repository root
    fn find_git_root(&self) -> Result<Option<PathBuf>> {
        let mut current = std::env::current_dir()?;

        loop {
            if current.join(".git").exists() {
                return Ok(Some(current));
            }

            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        Ok(None)
    }

    /// Get XDG config directory
    fn get_xdg_config_dir(&self) -> Option<PathBuf> {
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            Some(PathBuf::from(xdg_config))
        } else if let Ok(home) = std::env::var("HOME") {
            Some(PathBuf::from(home).join(".config"))
        } else {
            None
        }
    }

    /// Resolve raw config to ResolvedLlmConfig
    async fn resolve_config(&self, config: RawConfig) -> Result<ResolvedLlmConfig> {
        // Parse protocol
        let protocol = match config.protocol.as_str() {
            "openai" => Protocol::OpenAICompat,
            "anthropic" => Protocol::Anthropic,
            "google_ai" => Protocol::GoogleAI,
            "azure_openai" => Protocol::AzureOpenAI,
            custom => Protocol::Custom(custom.to_string()),
        };

        // Resolve API key (handle env: prefix)
        let api_key = if config.api_key.starts_with("env:") {
            let var_name = &config.api_key[4..];
            std::env::var(var_name)
                .with_context(|| format!("Environment variable not found: {}", var_name))?
        } else {
            config.api_key
        };

        // Resolve base URL
        let base_url = config.base_url.unwrap_or_else(|| {
            protocol
                .default_base_url()
                .unwrap_or("https://api.example.com")
                .to_string()
        });

        // Create resolved config
        let resolved = ResolvedLlmConfig::new(protocol, base_url, api_key, config.model)
            .with_params(config.params)
            .with_headers(config.headers);

        // Validate
        resolved
            .validate()
            .map_err(|e| anyhow!("Configuration validation failed: {}", e))?;

        Ok(resolved)
    }
}

impl Default for CliConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}
