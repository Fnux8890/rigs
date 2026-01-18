//! Core types and traits for Rigs
//!
//! This module contains the fundamental data structures used throughout
//! the Rigs orchestration system.

pub mod bead;
pub mod convoy;
pub mod error;
pub mod provider;
pub mod tank;

pub use bead::{Bead, BeadId, BeadStatus, Priority, TaskType};
pub use convoy::{Convoy, ConvoyId, ConvoyStatus};
pub use error::{Result, RigsError};
pub use provider::{Provider, ProviderConfig, ProviderLimits};
pub use tank::{Tank, TankHealth};
