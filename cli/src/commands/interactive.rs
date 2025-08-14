//! Interactive mode command

use crate::interactive::app::run_rich_interactive;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info};

/// Start interactive mode
pub async fn interactive_command(
    config_loader: crate::config::CliConfigLoader,
    trajectory_file: Option<PathBuf>,
    debug_output: bool,
) -> Result<()> {
    info!("Starting interactive mode");

    if let Some(trajectory_file) = &trajectory_file {
        tracing::debug!("📊 Trajectory file: {}", trajectory_file.display());
    }

    // Load LLM configuration
    let llm_config = config_loader.load().await?;
    info!("🤖 Using protocol: {}", llm_config.protocol.as_str());
    info!("🤖 Using model: {}", llm_config.model);

    // Get current working directory
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let project_path = current_dir.canonicalize().unwrap_or(current_dir);

    debug!("📁 Project path: {}", project_path.display());

    // Run the interactive mode (always use rich mode)
    // TODO: Update run_rich_interactive to use ResolvedLlmConfig
    run_rich_interactive(llm_config, project_path).await
}
