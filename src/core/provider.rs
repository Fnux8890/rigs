//! Provider types and configuration

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported LLM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    /// Anthropic Claude (via Claude Code CLI)
    Claude,
    /// OpenAI Codex (via Codex CLI)  
    Codex,
    /// Google Gemini (via API)
    Gemini,
    /// DeepSeek (via API) - for Assayer
    DeepSeek,
    /// Local models via Ollama - for Assayer
    Ollama,
}

impl Provider {
    /// Returns all remote providers (excludes Ollama)
    pub fn remote() -> impl Iterator<Item = Provider> {
        [
            Provider::Claude,
            Provider::Codex,
            Provider::Gemini,
            Provider::DeepSeek,
        ]
        .into_iter()
    }

    /// Returns all execution providers (used by Foreman)
    pub fn execution() -> impl Iterator<Item = Provider> {
        [Provider::Claude, Provider::Codex, Provider::Gemini].into_iter()
    }

    /// Returns all assayer providers (used for optimization)
    pub fn assayer() -> impl Iterator<Item = Provider> {
        [Provider::DeepSeek, Provider::Ollama].into_iter()
    }

    /// Human-readable display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Provider::Claude => "Claude",
            Provider::Codex => "Codex",
            Provider::Gemini => "Gemini",
            Provider::DeepSeek => "DeepSeek",
            Provider::Ollama => "Ollama",
        }
    }

    /// Default model for this provider
    pub fn default_model(&self) -> &'static str {
        match self {
            Provider::Claude => "claude-sonnet-4-20250514",
            Provider::Codex => "codex",
            Provider::Gemini => "gemini-2.5-pro",
            Provider::DeepSeek => "deepseek-chat",
            Provider::Ollama => "deepseek-r1:7b",
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Configuration for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: Provider,
    pub enabled: bool,
    pub model: String,
    pub limits: ProviderLimits,
    pub threshold_yellow: f32,
    pub threshold_red: f32,
    pub fallback_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_env: Option<String>,
}

impl ProviderConfig {
    /// Create default config for Claude
    pub fn claude_default() -> Self {
        Self {
            provider: Provider::Claude,
            enabled: true,
            model: "claude-sonnet-4-20250514".into(),
            limits: ProviderLimits {
                tokens_per_window: 88_000, // Max 5x
                window_hours: 5,
                requests_per_minute: None,
                weekly_cap: Some(500_000),
                daily_cap: None,
            },
            threshold_yellow: 0.5,
            threshold_red: 0.2,
            fallback_model: Some("claude-haiku-4-20250514".into()),
            api_key_env: None, // Uses CLI auth
        }
    }

    /// Create default config for Codex
    pub fn codex_default() -> Self {
        Self {
            provider: Provider::Codex,
            enabled: true,
            model: "codex".into(),
            limits: ProviderLimits {
                tokens_per_window: 50_000,
                window_hours: 5,
                requests_per_minute: Some(60),
                weekly_cap: None,
                daily_cap: None,
            },
            threshold_yellow: 0.4,
            threshold_red: 0.15,
            fallback_model: None,
            api_key_env: None, // Uses CLI auth
        }
    }

    /// Create default config for Gemini
    pub fn gemini_default() -> Self {
        Self {
            provider: Provider::Gemini,
            enabled: true,
            model: "gemini-2.5-pro".into(),
            limits: ProviderLimits {
                tokens_per_window: 1_000_000,
                window_hours: 24,
                requests_per_minute: Some(15),
                weekly_cap: None,
                daily_cap: Some(1_000_000),
            },
            threshold_yellow: 0.3,
            threshold_red: 0.1,
            fallback_model: Some("gemini-2.5-flash".into()),
            api_key_env: Some("GEMINI_API_KEY".into()),
        }
    }

    /// Create default config for DeepSeek (Assayer)
    pub fn deepseek_default() -> Self {
        Self {
            provider: Provider::DeepSeek,
            enabled: true,
            model: "deepseek-chat".into(),
            limits: ProviderLimits {
                tokens_per_window: 10_000_000, // Very generous
                window_hours: 24,
                requests_per_minute: Some(60),
                weekly_cap: None,
                daily_cap: None,
            },
            threshold_yellow: 0.3,
            threshold_red: 0.1,
            fallback_model: Some("deepseek-coder".into()),
            api_key_env: Some("DEEPSEEK_API_KEY".into()),
        }
    }

    /// Create default config for Ollama (local)
    pub fn ollama_default() -> Self {
        Self {
            provider: Provider::Ollama,
            enabled: true,
            model: "deepseek-r1:7b".into(),
            limits: ProviderLimits {
                tokens_per_window: u64::MAX, // Unlimited (local)
                window_hours: 24,
                requests_per_minute: None,
                weekly_cap: None,
                daily_cap: None,
            },
            threshold_yellow: 0.0,
            threshold_red: 0.0,
            fallback_model: Some("llama3.2:3b".into()),
            api_key_env: None, // No auth needed
        }
    }
}

/// Rate limits for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderLimits {
    /// Maximum tokens per rolling window
    pub tokens_per_window: u64,
    /// Window duration in hours
    pub window_hours: u32,
    /// Requests per minute limit (if any)
    pub requests_per_minute: Option<u32>,
    /// Weekly token cap (if any)
    pub weekly_cap: Option<u64>,
    /// Daily token cap (if any)
    pub daily_cap: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_serialization() {
        let provider = Provider::Claude;
        let json = serde_json::to_string(&provider).unwrap();
        assert_eq!(json, r#""claude""#);

        let parsed: Provider = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Provider::Claude);
    }

    #[test]
    fn test_provider_display() {
        assert_eq!(Provider::Claude.display_name(), "Claude");
        assert_eq!(Provider::DeepSeek.display_name(), "DeepSeek");
    }

    #[test]
    fn test_default_configs() {
        let claude = ProviderConfig::claude_default();
        assert!(claude.enabled);
        assert_eq!(claude.limits.window_hours, 5);

        let deepseek = ProviderConfig::deepseek_default();
        assert!(deepseek.api_key_env.is_some());
    }
}
