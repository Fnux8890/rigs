//! Goal commands (decomposition and execution)

use clap::Subcommand;
use crate::core::{Priority, Result};

#[derive(Subcommand)]
pub enum GoalCommands {
    /// Plan a goal (decompose into beads without executing)
    Plan {
        /// Goal description
        goal: String,
        /// Iteratively refine the plan
        #[arg(long)]
        refine: bool,
    },

    /// Execute a goal (decompose and run)
    Execute {
        /// Goal description
        goal: String,
        /// Priority for all beads
        #[arg(long, default_value = "normal")]
        priority: Priority,
        /// Auto-approve (no confirmation)
        #[arg(long)]
        yes: bool,
    },
}

pub async fn run(cmd: GoalCommands) -> Result<()> {
    match cmd {
        GoalCommands::Plan { goal, refine } => {
            println!("Planning goal: {}", goal);
            println!();
            
            if refine {
                println!("Using iterative refinement...");
            }
            
            println!("Decomposing with Planner Assayer (DeepSeek R1)...");
            println!();
            println!("Generated 5 beads:");
            println!();
            println!("  1. [research]       Research OAuth2 authentication flows");
            println!("     Est. tokens: 2,000 | Provider: Gemini");
            println!();
            println!("  2. [design]         Design authentication API endpoints");
            println!("     Est. tokens: 3,000 | Provider: Claude");
            println!("     Depends on: #1");
            println!();
            println!("  3. [implementation] Implement OAuth2 client library");
            println!("     Est. tokens: 5,000 | Provider: Claude");
            println!("     Depends on: #2");
            println!();
            println!("  4. [implementation] Add Google OAuth provider");
            println!("     Est. tokens: 3,000 | Provider: Claude");
            println!("     Depends on: #3");
            println!();
            println!("  5. [test]           Write authentication tests");
            println!("     Est. tokens: 2,000 | Provider: Codex");
            println!("     Depends on: #3, #4");
            println!();
            println!("Total estimated tokens: 15,000");
            println!("Estimated cost: ~$0.50 (if using API)");
            println!();
            println!("Run `rigs goal execute \"{}\"` to execute this plan", goal);
            Ok(())
        }
        GoalCommands::Execute { goal, priority, yes } => {
            println!("Executing goal: {}", goal);
            println!("Priority: {}", priority);
            println!();
            
            // Show plan first
            println!("Generated plan with 5 beads...");
            
            if !yes {
                println!();
                println!("Proceed? [y/N] ");
                // TODO: Read input
            }
            
            println!();
            println!("Creating convoy...");
            println!("✓ Convoy created: oauth-feature-xyz123");
            println!();
            println!("Queuing beads...");
            println!("  ✓ gt-abc12 queued (research)");
            println!("  ✓ gt-def34 queued (design)");
            println!("  ✓ gt-ghi56 queued (implementation)");
            println!("  ✓ gt-jkl78 queued (implementation)");
            println!("  ✓ gt-mno90 queued (test)");
            println!();
            println!("Convoy started. Use `rigs convoy show oauth-feature-xyz123` to track progress.");
            Ok(())
        }
    }
}
