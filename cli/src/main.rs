//! # coro CLI
//!
//! Command-line interface for coro-code - a high-performance AI coding agent.
//!
//! ## Usage
//!
//! - `coro` - Start interactive mode
//! - `coro "task description"` - Execute a single task
//! - `coro tools` - Show available tools
//! - `coro test` - Run basic tests
//!
//! This CLI provides both single-shot task execution and interactive modes,
//! with a beautiful terminal UI powered by iocraft.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod interactive;
mod output;
mod tools;
mod ui;

use commands::{interactive_command, run_command, test_command, tools_command};
use config::CliConfigLoader;

/// coro - A high-performance AI coding agent
#[derive(Parser)]
#[command(name = "coro")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A high-performance AI coding agent written in Rust")]
#[command(long_about = None)]
struct Cli {
    /// Configuration file or directory path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Protocol to use (openai, anthropic, google_ai, azure_openai)
    #[arg(long)]
    protocol: Option<String>,

    /// API key override
    #[arg(long)]
    api_key: Option<String>,

    /// Base URL override
    #[arg(long)]
    base_url: Option<String>,

    /// Model name override
    #[arg(long)]
    model: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug output mode (default is normal mode)
    #[arg(short = 'd', long = "debug")]
    debug_output: bool,

    /// Working directory
    #[arg(long)]
    working_dir: Option<PathBuf>,

    /// Maximum number of steps (for run mode)
    #[arg(long)]
    max_steps: Option<usize>,

    /// Output trajectory file
    #[arg(long)]
    trajectory_file: Option<PathBuf>,

    /// Must create a patch file (for run mode)
    #[arg(long)]
    must_patch: bool,

    /// Patch output file (for run mode)
    #[arg(long, default_value = "changes.patch")]
    patch_path: PathBuf,

    /// The task to execute (if provided, runs in single-task mode)
    task: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show available tools
    Tools,

    /// Run basic tests
    Test,
}

/// Build a configuration loader from CLI arguments
fn build_config_loader(cli: &Cli) -> CliConfigLoader {
    let mut loader = CliConfigLoader::new();

    if let Some(config_path) = &cli.config {
        loader = loader.with_config_override(config_path.clone());
    }

    if let Some(protocol) = &cli.protocol {
        loader = loader.with_protocol_override(protocol.clone());
    }

    if let Some(api_key) = &cli.api_key {
        loader = loader.with_api_key_override(api_key.clone());
    }

    if let Some(base_url) = &cli.base_url {
        loader = loader.with_base_url_override(base_url.clone());
    }

    if let Some(model) = &cli.model {
        loader = loader.with_model_override(model.clone());
    }

    loader
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let filter = if cli.verbose || cli.debug_output {
        "debug"
    } else {
        "info"
    };

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(filter))
        .init();

    // Change working directory if specified
    if let Some(working_dir) = &cli.working_dir {
        std::env::set_current_dir(working_dir)?;
    }

    // Build configuration loader
    let config_loader = build_config_loader(&cli);

    match (cli.task, cli.command) {
        // If task is provided, run in single-task mode
        (Some(task), None) => {
            run_command(
                task,
                config_loader,
                cli.max_steps,
                cli.trajectory_file,
                cli.must_patch,
                cli.patch_path,
                cli.working_dir,
                cli.debug_output,
            )
            .await
        }
        // If task is provided with a subcommand, that's an error
        (Some(_), Some(_)) => {
            tracing::error!("Error: Cannot specify both a task and a subcommand");
            std::process::exit(1);
        }
        // Handle subcommands
        (None, Some(Commands::Tools)) => tools_command().await,
        (None, Some(Commands::Test)) => test_command().await,
        // Default to interactive mode
        (None, None) => {
            interactive_command(config_loader, cli.trajectory_file, cli.debug_output).await
        }
    }
}
