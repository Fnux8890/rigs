use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

mod cli;
mod core;
mod db;

use crate::cli::{bead, convoy, foreman, goal, provider, tank};
use crate::core::error::Result;

#[derive(Parser)]
#[command(name = "rigs")]
#[command(version, about = "Rate-limit-aware multi-agent LLM orchestration")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to config file
    #[arg(short, long, global = true, env = "RIGS_CONFIG")]
    config: Option<PathBuf>,

    /// Verbose output (-v, -vv, -vvv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Output format (text, json)
    #[arg(long, global = true, default_value = "text")]
    format: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Rigs workspace
    Init {
        /// Path to workspace directory
        #[arg(default_value = "~/.rigs")]
        path: PathBuf,

        /// Initialize git repository
        #[arg(long)]
        git: bool,
    },

    /// Manage LLM providers (Claude, Codex, Gemini)
    Provider {
        #[command(subcommand)]
        action: provider::ProviderCommands,
    },

    /// View and manage rate limit tanks
    Tank {
        #[command(subcommand)]
        action: tank::TankCommands,
    },

    /// Manage work items (beads)
    Bead {
        #[command(subcommand)]
        action: bead::BeadCommands,
    },

    /// Manage convoys (batches of beads)
    Convoy {
        #[command(subcommand)]
        action: convoy::ConvoyCommands,
    },

    /// Control the orchestration daemon
    Foreman {
        #[command(subcommand)]
        action: foreman::ForemanCommands,
    },

    /// Work with goals (decompose, plan, execute)
    Goal {
        #[command(subcommand)]
        action: goal::GoalCommands,
    },

    /// Show system status overview
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose);

    info!("Rigs v{} starting", env!("CARGO_PKG_VERSION"));

    match cli.command {
        Commands::Init { path, git } => {
            cli::init::run(path, git).await?;
        }
        Commands::Provider { action } => {
            provider::run(action).await?;
        }
        Commands::Tank { action } => {
            tank::run(action).await?;
        }
        Commands::Bead { action } => {
            bead::run(action).await?;
        }
        Commands::Convoy { action } => {
            convoy::run(action).await?;
        }
        Commands::Foreman { action } => {
            foreman::run(action).await?;
        }
        Commands::Goal { action } => {
            goal::run(action).await?;
        }
        Commands::Status => {
            cli::status::run().await?;
        }
    }

    Ok(())
}

fn init_logging(verbosity: u8) {
    let level = match verbosity {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    let filter = format!("rigs={},sqlx=warn", level);

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| filter.into()),
        )
        .with_target(false)
        .init();
}
