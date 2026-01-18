//! Bead (task) management commands

use clap::Subcommand;
use crate::core::{BeadStatus, Priority, Provider, Result, TaskType};

#[derive(Subcommand)]
pub enum BeadCommands {
    /// Create a new bead
    Create {
        /// Task description
        description: String,
        /// Task type
        #[arg(short, long)]
        task_type: TaskType,
        /// Priority
        #[arg(short, long, default_value = "normal")]
        priority: Priority,
        /// Preferred provider
        #[arg(long)]
        provider: Option<Provider>,
    },

    /// List beads
    List {
        /// Filter by status
        #[arg(long)]
        status: Option<BeadStatus>,
        /// Filter by convoy
        #[arg(long)]
        convoy: Option<String>,
        /// Maximum results
        #[arg(long, default_value = "20")]
        limit: u32,
    },

    /// Show bead details
    Show {
        /// Bead ID
        id: String,
    },

    /// Edit a bead
    Edit {
        /// Bead ID
        id: String,
    },

    /// Cancel a bead
    Cancel {
        /// Bead ID
        id: String,
    },

    /// Retry a failed bead
    Retry {
        /// Bead ID
        id: String,
    },
}

pub async fn run(cmd: BeadCommands) -> Result<()> {
    match cmd {
        BeadCommands::Create { description, task_type, priority, provider } => {
            let id = "gt-abc12"; // TODO: Generate real ID
            println!("Created bead: {}", id);
            println!("  Type:     {}", task_type);
            println!("  Priority: {}", priority);
            if let Some(p) = provider {
                println!("  Provider: {}", p);
            }
            println!("  Description: {}", description);
            Ok(())
        }
        BeadCommands::List { status, convoy: _, limit } => {
            println!("Beads (showing {} of {}):", limit, 42);
            println!();
            println!("  ID         Status      Type           Priority  Provider");
            println!("  ─────────────────────────────────────────────────────────");
            println!("  gt-abc12   completed   implementation high      Claude");
            println!("  gt-def34   in_progress review         normal    Codex");
            println!("  gt-ghi56   pending     research       normal    -");
            if let Some(s) = status {
                println!("\n  (filtered by status: {})", s);
            }
            Ok(())
        }
        BeadCommands::Show { id } => {
            println!("Bead: {}", id);
            println!("  Title:       Implement user authentication");
            println!("  Description: Add OAuth2 authentication flow...");
            println!("  Type:        implementation");
            println!("  Priority:    high");
            println!("  Status:      in_progress");
            println!("  Provider:    Claude");
            println!("  Est. Tokens: 5,000");
            println!("  Created:     2026-01-18 10:00 UTC");
            println!("  Started:     2026-01-18 10:05 UTC");
            Ok(())
        }
        BeadCommands::Edit { id } => {
            println!("Editing bead: {}", id);
            // TODO: Open editor
            Ok(())
        }
        BeadCommands::Cancel { id } => {
            println!("Cancelled bead: {}", id);
            Ok(())
        }
        BeadCommands::Retry { id } => {
            println!("Retrying bead: {}", id);
            Ok(())
        }
    }
}
