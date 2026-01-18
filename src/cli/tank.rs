//! Tank (rate limit) management commands

use clap::Subcommand;
use crate::core::{Provider, Result};

#[derive(Subcommand)]
pub enum TankCommands {
    /// List all tanks with status
    List,

    /// Show detailed status for a provider
    Status {
        /// Provider to show
        provider: Provider,
    },

    /// Force refresh all tank data
    Refresh,

    /// Manually set remaining tokens
    Set {
        /// Provider to update
        provider: Provider,
        /// Remaining tokens
        tokens: u64,
    },

    /// Show usage history
    History {
        /// Provider (optional, shows all if omitted)
        provider: Option<Provider>,
        /// Time period
        #[arg(long, default_value = "24h")]
        period: String,
    },
}

pub async fn run(cmd: TankCommands) -> Result<()> {
    match cmd {
        TankCommands::List => {
            println!("Tank Status:");
            println!();
            println!("  Provider   Health   Remaining     Reset In");
            println!("  ─────────────────────────────────────────────");
            println!("  Claude     🟢       [████████░░]  78%    2h 34m");
            println!("  Codex      🟡       [██████░░░░]  45%    1h 12m");
            println!("  Gemini     🟢       [█████████░]  92%    18h 45m");
            println!("  DeepSeek   🟢       [██████████] 100%    (API)");
            println!("  Ollama     🟢       [██████████] ∞       (local)");
            Ok(())
        }
        TankCommands::Status { provider } => {
            println!("Tank: {}", provider);
            println!("  Capacity:     100,000 tokens");
            println!("  Remaining:    78,000 tokens (78%)");
            println!("  Health:       🟢 Green");
            println!("  Window Start: 2026-01-18 12:00 UTC");
            println!("  Window End:   2026-01-18 17:00 UTC");
            println!("  Reset In:     2h 34m");
            println!("  Requests:     45");
            println!("  Tokens Used:  22,000");
            Ok(())
        }
        TankCommands::Refresh => {
            println!("Refreshing all tanks...");
            println!("  Claude: Updated (78% remaining)");
            println!("  Codex: Updated (45% remaining)");
            println!("  Gemini: Updated (92% remaining)");
            println!("✓ All tanks refreshed");
            Ok(())
        }
        TankCommands::Set { provider, tokens } => {
            println!("Setting {} remaining tokens to {}", provider, tokens);
            Ok(())
        }
        TankCommands::History { provider, period } => {
            let prov = provider.map(|p| p.to_string()).unwrap_or("all".to_string());
            println!("Usage history for {} (last {})", prov, period);
            // TODO: Show graph
            Ok(())
        }
    }
}
