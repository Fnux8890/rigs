//! Repository implementations for database operations

use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::core::{Bead, BeadId, BeadStatus, Convoy, Provider, Result, Tank};

/// Repository for bead operations
#[async_trait]
pub trait BeadRepository: Send + Sync {
    async fn create(&self, bead: &Bead) -> Result<()>;
    async fn get(&self, id: &BeadId) -> Result<Option<Bead>>;
    async fn update(&self, bead: &Bead) -> Result<()>;
    async fn delete(&self, id: &BeadId) -> Result<()>;
    async fn list_by_status(&self, status: BeadStatus) -> Result<Vec<Bead>>;
    async fn list_by_convoy(&self, convoy_id: &str) -> Result<Vec<Bead>>;
    async fn get_pending_ordered(&self) -> Result<Vec<Bead>>;
    async fn get_deferred_ready(&self) -> Result<Vec<Bead>>;
}

/// Repository for tank operations
#[async_trait]
pub trait TankRepository: Send + Sync {
    async fn get(&self, provider: Provider) -> Result<Option<Tank>>;
    async fn get_all(&self) -> Result<Vec<Tank>>;
    async fn upsert(&self, tank: &Tank) -> Result<()>;
}

/// Repository for convoy operations
#[async_trait]
pub trait ConvoyRepository: Send + Sync {
    async fn create(&self, convoy: &Convoy) -> Result<()>;
    async fn get(&self, id: &str) -> Result<Option<Convoy>>;
    async fn update(&self, convoy: &Convoy) -> Result<()>;
    async fn list_active(&self) -> Result<Vec<Convoy>>;
}

/// SQLite implementation of repositories
pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// TODO: Implement all repository traits for SqliteRepository
