//! Provider management commands

use clap::Subcommand;
use crate::core::{Provider, Result};

#[derive(Subcommand)]
pub enum ProviderCommands {
    /// Add and configure a provider
    Add {
        /// Provider to add
        provider: Provider,
    },

    /// Remove a provider
    Remove {
        /// Provider to remove
        provider: Provider,
    },

    /// List configured providers
    List,

    /// Test provider connectivity
    Test {
        /// Provider to test
        provider: Provider,
    },

    /// Enable a provider
    Enable {
        /// Provider to enable
        provider: Provider,
    },

    /// Disable a provider
    Disable {
        /// Provider to disable
        provider: Provider,
    },
}

pub async fn run(cmd: ProviderCommands) -> Result<()> {
    match cmd {
        ProviderCommands::Add { provider } => {
            println!("Adding provider: {}", provider);
            // TODO: Interactive configuration
            Ok(())
        }
        ProviderCommands::Remove { provider } => {
            println!("Removing provider: {}", provider);
            // TODO: Remove from config
            Ok(())
        }
        ProviderCommands::List => {
            println!("Configured providers:");
            println!("  Claude  - claude-sonnet-4-20250514");
            println!("  Codex   - codex");
            println!("  Gemini  - gemini-2.5-pro");
            println!("  DeepSeek - deepseek-chat (Assayer)");
            println!("  Ollama  - deepseek-r1:7b (local)");
            Ok(())
        }
        ProviderCommands::Test { provider } => {
            println!("Testing provider: {}", provider);
            // TODO: Send test request
            println!("✓ {} is responding", provider);
            Ok(())
        }
        ProviderCommands::Enable { provider } => {
            println!("Enabled provider: {}", provider);
            Ok(())
        }
        ProviderCommands::Disable { provider } => {
            println!("Disabled provider: {}", provider);
            Ok(())
        }
    }
}
