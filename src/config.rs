//! Configuration loading and management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::core::{Provider, Result, RigsError};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub providers: ProvidersConfig,
    #[serde(default)]
    pub assayer: AssayerConfig,
    #[serde(default)]
    pub routing: RoutingConfig,
    #[serde(default)]
    pub foreman: ForemanConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_workspace")]
    pub workspace: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_workspace() -> String {
    "~/.rigs".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            workspace: default_workspace(),
            log_level: default_log_level(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProvidersConfig {
    #[serde(default)]
    pub claude: ProviderEntry,
    #[serde(default)]
    pub codex: ProviderEntry,
    #[serde(default)]
    pub gemini: ProviderEntry,
    #[serde(default)]
    pub deepseek: ProviderEntry,
    #[serde(default)]
    pub ollama: OllamaEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderEntry {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub model: String,
    #[serde(default = "default_threshold_yellow")]
    pub threshold_yellow: f32,
    #[serde(default = "default_threshold_red")]
    pub threshold_red: f32,
    #[serde(default)]
    pub fallback_model: Option<String>,
    #[serde(default)]
    pub api_key_env: Option<String>,
}

fn default_true() -> bool {
    true
}

fn default_threshold_yellow() -> f32 {
    0.5
}

fn default_threshold_red() -> f32 {
    0.2
}

impl Default for ProviderEntry {
    fn default() -> Self {
        Self {
            enabled: true,
            model: String::new(),
            threshold_yellow: default_threshold_yellow(),
            threshold_red: default_threshold_red(),
            fallback_model: None,
            api_key_env: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaEntry {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_ollama_url")]
    pub base_url: String,
    #[serde(default = "default_ollama_model")]
    pub model: String,
    #[serde(default)]
    pub fallback_model: Option<String>,
}

fn default_ollama_url() -> String {
    "http://localhost:11434".to_string()
}

fn default_ollama_model() -> String {
    "deepseek-r1:7b".to_string()
}

impl Default for OllamaEntry {
    fn default() -> Self {
        Self {
            enabled: true,
            base_url: default_ollama_url(),
            model: default_ollama_model(),
            fallback_model: Some("llama3.2:3b".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssayerConfig {
    #[serde(default = "default_true")]
    pub use_ollama: bool,
    #[serde(default = "default_planner_model")]
    pub planner_model: String,
    #[serde(default = "default_optimizer_model")]
    pub optimizer_model: String,
    #[serde(default = "default_estimator_model")]
    pub estimator_model: String,
    #[serde(default = "default_quality_model")]
    pub quality_model: String,
    #[serde(default = "default_true")]
    pub fallback_to_api: bool,
}

fn default_planner_model() -> String {
    "deepseek-r1:7b".to_string()
}

fn default_optimizer_model() -> String {
    "qwen3:8b".to_string()
}

fn default_estimator_model() -> String {
    "llama3.2:3b".to_string()
}

fn default_quality_model() -> String {
    "llama3.2:3b".to_string()
}

impl Default for AssayerConfig {
    fn default() -> Self {
        Self {
            use_ollama: true,
            planner_model: default_planner_model(),
            optimizer_model: default_optimizer_model(),
            estimator_model: default_estimator_model(),
            quality_model: default_quality_model(),
            fallback_to_api: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    #[serde(default = "default_strategy")]
    pub strategy: String,
    #[serde(default)]
    pub affinity: HashMap<String, HashMap<String, f32>>,
}

fn default_strategy() -> String {
    "balanced".to_string()
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            strategy: default_strategy(),
            affinity: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForemanConfig {
    #[serde(default = "default_poll_interval")]
    pub poll_interval: u64,
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: u32,
    #[serde(default)]
    pub auto_start: bool,
}

fn default_poll_interval() -> u64 {
    5
}

fn default_max_concurrent() -> u32 {
    1
}

impl Default for ForemanConfig {
    fn default() -> Self {
        Self {
            poll_interval: default_poll_interval(),
            max_concurrent: default_max_concurrent(),
            auto_start: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_path")]
    pub path: String,
    #[serde(default = "default_true")]
    pub wal_mode: bool,
}

fn default_db_path() -> String {
    "~/.rigs/db/rigs.db".to_string()
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: default_db_path(),
            wal_mode: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            providers: ProvidersConfig::default(),
            assayer: AssayerConfig::default(),
            routing: RoutingConfig::default(),
            foreman: ForemanConfig::default(),
            database: DatabaseConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from file, with fallback to defaults
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let config_path = match path {
            Some(p) => p.to_path_buf(),
            None => Self::default_config_path()?,
        };

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| RigsError::InvalidConfig(format!("{}: {}", config_path.display(), e)))?;
            Ok(config)
        } else {
            // Return defaults if no config file
            Ok(Config::default())
        }
    }

    /// Get the default config file path
    pub fn default_config_path() -> Result<PathBuf> {
        let home = directories::BaseDirs::new()
            .ok_or_else(|| RigsError::ConfigError("Cannot determine home directory".to_string()))?;
        Ok(home.home_dir().join(".rigs").join("config.toml"))
    }

    /// Expand ~ in paths to actual home directory
    pub fn expand_path(&self, path: &str) -> PathBuf {
        if path.starts_with("~/") {
            if let Some(home) = directories::BaseDirs::new() {
                return home.home_dir().join(&path[2..]);
            }
        }
        PathBuf::from(path)
    }

    /// Get the workspace directory
    pub fn workspace_dir(&self) -> PathBuf {
        self.expand_path(&self.general.workspace)
    }

    /// Get the database path
    pub fn database_path(&self) -> PathBuf {
        self.expand_path(&self.database.path)
    }

    /// Check if a provider is enabled
    pub fn is_provider_enabled(&self, provider: Provider) -> bool {
        match provider {
            Provider::Claude => self.providers.claude.enabled,
            Provider::Codex => self.providers.codex.enabled,
            Provider::Gemini => self.providers.gemini.enabled,
            Provider::DeepSeek => self.providers.deepseek.enabled,
            Provider::Ollama => self.providers.ollama.enabled,
        }
    }

    /// Get model for a provider
    pub fn get_model(&self, provider: Provider) -> &str {
        match provider {
            Provider::Claude => &self.providers.claude.model,
            Provider::Codex => &self.providers.codex.model,
            Provider::Gemini => &self.providers.gemini.model,
            Provider::DeepSeek => &self.providers.deepseek.model,
            Provider::Ollama => &self.providers.ollama.model,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.workspace, "~/.rigs");
        assert_eq!(config.general.log_level, "info");
        assert!(config.providers.claude.enabled);
        assert_eq!(config.routing.strategy, "balanced");
    }

    #[test]
    fn test_parse_config() {
        let toml = r#"
            [general]
            workspace = "/custom/path"
            log_level = "debug"

            [providers.claude]
            enabled = false
            model = "claude-opus-4"
        "#;

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.general.workspace, "/custom/path");
        assert_eq!(config.general.log_level, "debug");
        assert!(!config.providers.claude.enabled);
        assert_eq!(config.providers.claude.model, "claude-opus-4");
    }

    #[test]
    fn test_expand_path() {
        let config = Config::default();
        let expanded = config.expand_path("~/.rigs/db/test.db");
        assert!(!expanded.to_string_lossy().starts_with("~"));
    }
}
