//! Error types for Rigs

use chrono::{DateTime, Utc};
use thiserror::Error;

use super::bead::{BeadId, BeadStatus};
use super::provider::Provider;

/// Result type alias for Rigs operations
pub type Result<T> = std::result::Result<T, RigsError>;

/// Main error type for Rigs
#[derive(Error, Debug)]
pub enum RigsError {
    // Provider errors
    #[error("Provider {0} is not configured")]
    ProviderNotConfigured(Provider),

    #[error("Provider {0} is disabled")]
    ProviderDisabled(Provider),

    #[error("Rate limit exceeded for {provider}: {remaining} tokens remaining, need {requested}")]
    RateLimitExceeded {
        provider: Provider,
        remaining: u64,
        requested: u64,
    },

    #[error("All providers exhausted, next reset at {0}")]
    AllProvidersExhausted(DateTime<Utc>),

    #[error("Provider {0} API error: {1}")]
    ProviderApiError(Provider, String),

    // Bead errors
    #[error("Bead {0} not found")]
    BeadNotFound(BeadId),

    #[error("Invalid bead ID: {0}")]
    InvalidBeadId(String),

    #[error("Invalid state transition: {from:?} -> {to:?}")]
    InvalidStateTransition { from: BeadStatus, to: BeadStatus },

    #[error("Bead {0} has unmet dependencies")]
    UnmetDependencies(BeadId),

    // Convoy errors
    #[error("Convoy {0} not found")]
    ConvoyNotFound(String),

    #[error("Dependency cycle detected: {0:?}")]
    DependencyCycle(Vec<BeadId>),

    // Assayer errors
    #[error("Assayer error: {0}")]
    AssayerError(String),

    #[error("Ollama not available: {0}")]
    OllamaNotAvailable(String),

    #[error("Failed to parse LLM response: {0}")]
    LlmParseError(String),

    // Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Workspace not initialized. Run `rigs init` first.")]
    WorkspaceNotInitialized,

    #[error("Invalid configuration file: {0}")]
    InvalidConfig(String),

    // Database errors
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    // IO errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    // Serialization errors
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),

    // HTTP errors
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    // Generic errors
    #[error("{0}")]
    Other(String),
}

impl RigsError {
    /// Check if this error is recoverable (can retry)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            RigsError::RateLimitExceeded { .. }
                | RigsError::AllProvidersExhausted(_)
                | RigsError::OllamaNotAvailable(_)
                | RigsError::HttpError(_)
        )
    }

    /// Check if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(
            self,
            RigsError::RateLimitExceeded { .. } | RigsError::AllProvidersExhausted(_)
        )
    }

    /// Get suggested wait time for recoverable errors
    pub fn suggested_wait(&self) -> Option<std::time::Duration> {
        match self {
            RigsError::AllProvidersExhausted(reset_time) => {
                let now = Utc::now();
                if *reset_time > now {
                    let duration = *reset_time - now;
                    Some(duration.to_std().unwrap_or(std::time::Duration::from_secs(60)))
                } else {
                    Some(std::time::Duration::from_secs(60))
                }
            }
            RigsError::RateLimitExceeded { .. } => Some(std::time::Duration::from_secs(300)), // 5 min
            RigsError::OllamaNotAvailable(_) => Some(std::time::Duration::from_secs(10)),
            RigsError::HttpError(_) => Some(std::time::Duration::from_secs(5)),
            _ => None,
        }
    }
}

/// Extension trait for adding context to errors
pub trait ResultExt<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;
}

impl<T, E: std::error::Error> ResultExt<T> for std::result::Result<T, E> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| RigsError::Other(format!("{}: {}", msg.into(), e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_is_recoverable() {
        let rate_limit = RigsError::RateLimitExceeded {
            provider: Provider::Claude,
            remaining: 0,
            requested: 1000,
        };
        assert!(rate_limit.is_recoverable());
        assert!(rate_limit.is_rate_limit());

        let not_found = RigsError::BeadNotFound(super::super::bead::BeadId::new());
        assert!(!not_found.is_recoverable());
    }

    #[test]
    fn test_error_display() {
        let err = RigsError::ProviderNotConfigured(Provider::Claude);
        assert!(err.to_string().contains("Claude"));
    }
}
