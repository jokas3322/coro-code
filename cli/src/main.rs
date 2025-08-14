//! # coro CLI
//!
//! Command-line interface for coro-code - a high-performance AI coding agent.
//!
//! This CLI provides both single-shot task execution and interactive modes,
//! with a beautiful terminal UI powered by iocraft.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

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

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a single task
    Run {
        /// The task to execute
        task: String,

        /// Maximum number of steps
        #[arg(long)]
        max_steps: Option<usize>,

        /// Output trajectory file
        #[arg(long)]
        trajectory_file: Option<PathBuf>,

        /// Must create a patch file
        #[arg(long)]
        must_patch: bool,

        /// Patch output file
        #[arg(long, default_value = "changes.patch")]
        patch_path: PathBuf,
    },

    /// Start interactive mode
    Interactive {
        /// Output trajectory file
        #[arg(long)]
        trajectory_file: Option<PathBuf>,

        /// Enable debug output mode
        #[arg(short = 'd', long = "debug")]
        debug_output: bool,
    },

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

    info!("Starting Trae Agent CLI v{}", env!("CARGO_PKG_VERSION"));

    // Change working directory if specified
    if let Some(working_dir) = &cli.working_dir {
        std::env::set_current_dir(working_dir)?;
        info!("Changed working directory to: {}", working_dir.display());
    }

    // Build configuration loader
    let config_loader = build_config_loader(&cli);

    match cli.command {
        Some(Commands::Run {
            task,
            max_steps,
            trajectory_file,
            must_patch,
            patch_path,
        }) => {
            run_command(
                task,
                config_loader,
                max_steps,
                trajectory_file,
                must_patch,
                patch_path,
                cli.working_dir,
                cli.debug_output,
            )
            .await
        }
        Some(Commands::Interactive {
            trajectory_file,
            debug_output,
        }) => interactive_command(config_loader, trajectory_file, debug_output).await,
        Some(Commands::Tools) => tools_command().await,
        Some(Commands::Test) => test_command().await,
        None => {
            // Default to interactive mode
            interactive_command(config_loader, None, cli.debug_output).await
        }
    }
}
