//! Convoy (batch) types
//!
//! A Convoy is a collection of related beads, typically created from
//! decomposing a high-level goal.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::bead::{BeadId, BeadStatus};

/// Unique identifier for a convoy
pub type ConvoyId = String;

/// Status of a convoy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConvoyStatus {
    /// Being decomposed by Planner Assayer
    Planning,
    /// Ready to execute
    Queued,
    /// Some beads in progress
    InProgress,
    /// Paused (e.g., all providers exhausted)
    Paused,
    /// All beads completed successfully
    Completed,
    /// One or more beads failed
    Failed,
}

impl ConvoyStatus {
    /// Check if this is a terminal status
    pub fn is_terminal(&self) -> bool {
        matches!(self, ConvoyStatus::Completed | ConvoyStatus::Failed)
    }
}

/// A batch of related beads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convoy {
    /// Unique identifier
    pub id: ConvoyId,
    /// Human-readable name
    pub name: String,
    /// Original goal (if created from decomposition)
    pub goal: Option<String>,
    /// Beads in this convoy (ordered by execution)
    pub beads: Vec<BeadId>,
    /// Current status
    pub status: ConvoyStatus,
    /// When created
    pub created_at: DateTime<Utc>,
    /// When completed (if finished)
    pub completed_at: Option<DateTime<Utc>>,
    /// Arbitrary metadata
    pub metadata: HashMap<String, String>,
}

impl Convoy {
    /// Create a new convoy
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            goal: None,
            beads: vec![],
            status: ConvoyStatus::Planning,
            created_at: Utc::now(),
            completed_at: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a convoy from a decomposed goal
    pub fn from_goal(name: impl Into<String>, goal: impl Into<String>, beads: Vec<BeadId>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            goal: Some(goal.into()),
            beads,
            status: ConvoyStatus::Queued,
            created_at: Utc::now(),
            completed_at: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a bead to the convoy
    pub fn add_bead(&mut self, bead_id: BeadId) {
        if !self.beads.contains(&bead_id) {
            self.beads.push(bead_id);
        }
    }

    /// Calculate progress (0.0 to 1.0)
    pub fn progress(&self, bead_statuses: &HashMap<BeadId, BeadStatus>) -> f32 {
        if self.beads.is_empty() {
            return 0.0;
        }

        let completed = self
            .beads
            .iter()
            .filter(|id| {
                bead_statuses
                    .get(*id)
                    .map(|s| *s == BeadStatus::Completed)
                    .unwrap_or(false)
            })
            .count();

        completed as f32 / self.beads.len() as f32
    }

    /// Count beads by status
    pub fn status_counts(&self, bead_statuses: &HashMap<BeadId, BeadStatus>) -> StatusCounts {
        let mut counts = StatusCounts::default();
        
        for id in &self.beads {
            match bead_statuses.get(id) {
                Some(BeadStatus::Completed) => counts.completed += 1,
                Some(BeadStatus::Failed) => counts.failed += 1,
                Some(BeadStatus::InProgress | BeadStatus::Assigned | BeadStatus::Reviewing) => {
                    counts.in_progress += 1
                }
                Some(BeadStatus::Deferred) => counts.deferred += 1,
                _ => counts.pending += 1,
            }
        }
        
        counts
    }

    /// Check if all beads are complete
    pub fn is_complete(&self, bead_statuses: &HashMap<BeadId, BeadStatus>) -> bool {
        self.beads.iter().all(|id| {
            bead_statuses
                .get(id)
                .map(|s| s.is_terminal())
                .unwrap_or(false)
        })
    }

    /// Check if any bead failed
    pub fn has_failures(&self, bead_statuses: &HashMap<BeadId, BeadStatus>) -> bool {
        self.beads.iter().any(|id| {
            bead_statuses
                .get(id)
                .map(|s| *s == BeadStatus::Failed)
                .unwrap_or(false)
        })
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

/// Counts of beads by status in a convoy
#[derive(Debug, Clone, Default)]
pub struct StatusCounts {
    pub pending: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub failed: usize,
    pub deferred: usize,
}

impl StatusCounts {
    pub fn total(&self) -> usize {
        self.pending + self.in_progress + self.completed + self.failed + self.deferred
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convoy_creation() {
        let convoy = Convoy::new("Test convoy");
        assert!(!convoy.id.is_empty());
        assert_eq!(convoy.name, "Test convoy");
        assert_eq!(convoy.status, ConvoyStatus::Planning);
    }

    #[test]
    fn test_convoy_progress() {
        let mut convoy = Convoy::new("Test");
        let id1 = BeadId::new();
        let id2 = BeadId::new();
        convoy.beads = vec![id1.clone(), id2.clone()];

        let mut statuses = HashMap::new();
        statuses.insert(id1.clone(), BeadStatus::Completed);
        statuses.insert(id2.clone(), BeadStatus::Pending);

        assert!((convoy.progress(&statuses) - 0.5).abs() < 0.001);
    }
}
