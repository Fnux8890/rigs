//! Bead (work unit) types
//!
//! A Bead is the fundamental unit of work in Rigs, inspired by Gas Town terminology.
//! Each bead represents a single task to be executed by an LLM provider.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::provider::Provider;

/// Unique identifier for a bead
/// Format: "gt-xxxxx" (5 alphanumeric characters)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BeadId(String);

impl BeadId {
    /// Generate a new unique BeadId
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let suffix: String = (0..5)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'a' + idx - 10) as char
                }
            })
            .collect();
        BeadId(format!("gt-{}", suffix))
    }

    /// Parse a BeadId from a string
    pub fn parse(s: &str) -> Result<Self, InvalidBeadId> {
        if s.starts_with("gt-")
            && s.len() == 8
            && s[3..].chars().all(|c| c.is_ascii_alphanumeric())
        {
            Ok(BeadId(s.to_lowercase()))
        } else {
            Err(InvalidBeadId(s.to_string()))
        }
    }

    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for BeadId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BeadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for BeadId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Error for invalid BeadId format
#[derive(Debug, Clone)]
pub struct InvalidBeadId(pub String);

impl fmt::Display for InvalidBeadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid bead ID '{}': must be 'gt-' followed by 5 alphanumeric characters",
            self.0
        )
    }
}

impl std::error::Error for InvalidBeadId {}

/// Type of task for routing and optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    /// Write new code or features
    Implementation,
    /// Review existing code
    Review,
    /// Research a topic or solution
    Research,
    /// Refactor existing code
    Refactor,
    /// Write or fix tests
    Test,
    /// Write documentation
    Documentation,
    /// Debug an issue
    Debug,
    /// Design architecture or API
    Design,
}

impl TaskType {
    /// Get the best provider for this task type
    pub fn preferred_provider(&self) -> Provider {
        match self {
            TaskType::Implementation => Provider::Claude,
            TaskType::Review => Provider::Codex,
            TaskType::Research => Provider::Gemini,
            TaskType::Refactor => Provider::Claude,
            TaskType::Test => Provider::Codex,
            TaskType::Documentation => Provider::Claude,
            TaskType::Debug => Provider::Codex,
            TaskType::Design => Provider::Claude,
        }
    }

    /// Get all providers ranked by affinity for this task type
    pub fn provider_affinities(&self) -> Vec<(Provider, f32)> {
        match self {
            TaskType::Implementation => vec![
                (Provider::Claude, 1.0),
                (Provider::Codex, 0.7),
                (Provider::Gemini, 0.5),
            ],
            TaskType::Review => vec![
                (Provider::Codex, 1.0),
                (Provider::Claude, 0.8),
                (Provider::Gemini, 0.5),
            ],
            TaskType::Research => vec![
                (Provider::Gemini, 1.0),
                (Provider::Claude, 0.6),
                (Provider::Codex, 0.4),
            ],
            TaskType::Refactor => vec![
                (Provider::Claude, 1.0),
                (Provider::Codex, 0.8),
                (Provider::Gemini, 0.4),
            ],
            TaskType::Test => vec![
                (Provider::Codex, 1.0),
                (Provider::Claude, 0.7),
                (Provider::Gemini, 0.4),
            ],
            TaskType::Documentation => vec![
                (Provider::Claude, 1.0),
                (Provider::Gemini, 0.7),
                (Provider::Codex, 0.5),
            ],
            TaskType::Debug => vec![
                (Provider::Codex, 1.0),
                (Provider::Claude, 0.9),
                (Provider::Gemini, 0.4),
            ],
            TaskType::Design => vec![
                (Provider::Claude, 1.0),
                (Provider::Gemini, 0.6),
                (Provider::Codex, 0.4),
            ],
        }
    }
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TaskType::Implementation => "implementation",
            TaskType::Review => "review",
            TaskType::Research => "research",
            TaskType::Refactor => "refactor",
            TaskType::Test => "test",
            TaskType::Documentation => "documentation",
            TaskType::Debug => "debug",
            TaskType::Design => "design",
        };
        write!(f, "{}", s)
    }
}

/// Priority level for a bead
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, clap::ValueEnum,
)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Priority::Low => "low",
            Priority::Normal => "normal",
            Priority::High => "high",
            Priority::Critical => "critical",
        };
        write!(f, "{}", s)
    }
}

/// Status of a bead in the pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum BeadStatus {
    /// Waiting in queue, not yet processed
    Pending,
    /// Being processed by Assayer (optimization)
    Optimizing,
    /// Ready for execution, waiting for capacity
    Queued,
    /// Assigned to a Polecat worker
    Assigned,
    /// Currently being executed by provider
    InProgress,
    /// Waiting for rate limit reset
    Deferred,
    /// Output being reviewed by Quality Gate
    Reviewing,
    /// Successfully completed
    Completed,
    /// Failed after retries
    Failed,
    /// Cancelled by user
    Cancelled,
}

impl BeadStatus {
    /// Check if this is a terminal status
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            BeadStatus::Completed | BeadStatus::Failed | BeadStatus::Cancelled
        )
    }

    /// Check if this bead is actively being worked on
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            BeadStatus::Optimizing
                | BeadStatus::Assigned
                | BeadStatus::InProgress
                | BeadStatus::Reviewing
        )
    }
}

impl fmt::Display for BeadStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BeadStatus::Pending => "pending",
            BeadStatus::Optimizing => "optimizing",
            BeadStatus::Queued => "queued",
            BeadStatus::Assigned => "assigned",
            BeadStatus::InProgress => "in_progress",
            BeadStatus::Deferred => "deferred",
            BeadStatus::Reviewing => "reviewing",
            BeadStatus::Completed => "completed",
            BeadStatus::Failed => "failed",
            BeadStatus::Cancelled => "cancelled",
        };
        write!(f, "{}", s)
    }
}

/// A work unit in the Rigs system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bead {
    /// Unique identifier
    pub id: BeadId,
    /// Short descriptive title
    pub title: String,
    /// Full task description / prompt
    pub description: String,
    /// Type of task for routing
    pub task_type: TaskType,
    /// Priority for scheduling
    pub priority: Priority,
    /// Current status
    pub status: BeadStatus,

    // Token tracking
    /// Estimated tokens (from Estimator Assayer)
    pub estimated_tokens: u64,
    /// Actual tokens used (after execution)
    pub actual_tokens: Option<u64>,

    // Provider assignment
    /// User-requested provider preference
    pub preferred_provider: Option<Provider>,
    /// Assigned provider (by Dispatch)
    pub assigned_provider: Option<Provider>,

    // Task details
    /// Criteria for success (for Quality Gate)
    pub acceptance_criteria: Vec<String>,
    /// Dependencies on other beads
    pub dependencies: Vec<BeadId>,
    /// Parent convoy (if part of a batch)
    pub convoy_id: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deferred_until: Option<DateTime<Utc>>,

    // Content
    /// Optimized prompt (from Optimizer Assayer)
    pub optimized_prompt: Option<String>,
    /// Execution output
    pub output: Option<String>,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl Bead {
    /// Create a new bead with minimal required fields
    pub fn new(title: impl Into<String>, description: impl Into<String>, task_type: TaskType) -> Self {
        Self {
            id: BeadId::new(),
            title: title.into(),
            description: description.into(),
            task_type,
            priority: Priority::default(),
            status: BeadStatus::Pending,
            estimated_tokens: 0,
            actual_tokens: None,
            preferred_provider: None,
            assigned_provider: None,
            acceptance_criteria: vec![],
            dependencies: vec![],
            convoy_id: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            deferred_until: None,
            optimized_prompt: None,
            output: None,
            error: None,
        }
    }

    /// Builder: set priority
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Builder: set preferred provider
    pub fn with_provider(mut self, provider: Provider) -> Self {
        self.preferred_provider = Some(provider);
        self
    }

    /// Builder: set acceptance criteria
    pub fn with_criteria(mut self, criteria: Vec<String>) -> Self {
        self.acceptance_criteria = criteria;
        self
    }

    /// Builder: set dependencies
    pub fn with_dependencies(mut self, deps: Vec<BeadId>) -> Self {
        self.dependencies = deps;
        self
    }

    /// Builder: set estimated tokens
    pub fn with_estimate(mut self, tokens: u64) -> Self {
        self.estimated_tokens = tokens;
        self
    }

    /// Get the prompt to use (optimized if available, else original)
    pub fn effective_prompt(&self) -> &str {
        self.optimized_prompt
            .as_deref()
            .unwrap_or(&self.description)
    }

    /// Check if all dependencies are complete
    pub fn dependencies_met(&self, completed: &std::collections::HashSet<BeadId>) -> bool {
        self.dependencies.iter().all(|dep| completed.contains(dep))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bead_id_generation() {
        let id1 = BeadId::new();
        let id2 = BeadId::new();
        assert_ne!(id1, id2);
        assert!(id1.as_str().starts_with("gt-"));
        assert_eq!(id1.as_str().len(), 8);
    }

    #[test]
    fn test_bead_id_parsing() {
        assert!(BeadId::parse("gt-abc12").is_ok());
        assert!(BeadId::parse("gt-ABC12").is_ok()); // Should lowercase
        assert!(BeadId::parse("invalid").is_err());
        assert!(BeadId::parse("gt-abc").is_err()); // Too short
        assert!(BeadId::parse("gt-abc123").is_err()); // Too long
    }

    #[test]
    fn test_bead_builder() {
        let bead = Bead::new("Test task", "Do the thing", TaskType::Implementation)
            .with_priority(Priority::High)
            .with_provider(Provider::Claude)
            .with_estimate(5000);

        assert_eq!(bead.title, "Test task");
        assert_eq!(bead.priority, Priority::High);
        assert_eq!(bead.preferred_provider, Some(Provider::Claude));
        assert_eq!(bead.estimated_tokens, 5000);
    }

    #[test]
    fn test_task_type_affinities() {
        let affinities = TaskType::Implementation.provider_affinities();
        assert_eq!(affinities[0].0, Provider::Claude);
        assert_eq!(affinities[0].1, 1.0);

        let research_affinities = TaskType::Research.provider_affinities();
        assert_eq!(research_affinities[0].0, Provider::Gemini);
    }
}
