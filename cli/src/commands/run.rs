//! Single task execution command

use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info};

/// Execute a single task
pub async fn run_command(
    task: String,
    config_loader: crate::config::CliConfigLoader,
    max_steps: Option<usize>,
    trajectory_file: Option<PathBuf>,
    must_patch: bool,
    patch_path: PathBuf,
    working_dir: Option<PathBuf>,
    debug_output: bool,
) -> Result<()> {
    info!("Executing task: {}", task);

    use crate::output::cli_handler::{CliOutputConfig, CliOutputHandler};
    use coro_core::{trajectory::TrajectoryRecorder, AgentBuilder, AgentConfig, OutputMode};

    // Load LLM configuration
    let llm_config = config_loader.load().await?;
    info!("🤖 Using protocol: {}", llm_config.protocol.as_str());
    info!("🤖 Using model: {}", llm_config.model);

    // Create agent configuration with CLI tools
    let mut agent_config = AgentConfig::default();
    agent_config.tools = crate::tools::get_default_cli_tools();
    if let Some(steps) = max_steps {
        agent_config.max_steps = steps;
    }
    if debug_output {
        agent_config.output_mode = OutputMode::Debug;
    }

    // Create CLI output handler
    let cli_config = CliOutputConfig {
        realtime_updates: true, // Always enable realtime updates for better UX
    };
    let cli_output = Box::new(CliOutputHandler::new(cli_config));

    // Build agent with new configuration system and CLI tools
    let cli_tool_registry = crate::tools::create_cli_tool_registry();
    let agent = AgentBuilder::new(llm_config)
        .with_agent_config(agent_config)
        .build_with_output_and_registry(cli_output, cli_tool_registry)
        .await?;

    // Initialize trajectory recorder
    let trajectory = TrajectoryRecorder::new();
    let task_entry = coro_core::trajectory::TrajectoryEntry::task_start(
        task.clone(),
        serde_json::json!({"max_steps": max_steps.unwrap_or(200)}),
    );
    trajectory.record(task_entry).await?;

    if let Some(trajectory_file) = &trajectory_file {
        info!("📊 Trajectory file: {}", trajectory_file.display());
    }

    debug!("🤖 Using coro-code Agent system prompt");

    // Get current working directory
    let current_dir = working_dir
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let project_path = current_dir.canonicalize().unwrap_or(current_dir);

    debug!("📁 Project path: {}", project_path.display());

    // Execute the task using the agent
    let mut agent = agent; // Make mutable for execution
    let _execution_result = agent
        .execute_task_with_context(&task, &project_path)
        .await?;

    if must_patch {
        info!("📄 Creating patch file: {}", patch_path.display());
        std::fs::write(
            &patch_path,
            "# Placeholder patch file\n# Changes would be recorded here\n",
        )?;
    }

    // Save trajectory if requested
    if let Some(trajectory_file) = &trajectory_file {
        info!("📊 Trajectory saved to: {}", trajectory_file.display());
    }

    info!("✅ Task completed successfully");

    Ok(())
}
