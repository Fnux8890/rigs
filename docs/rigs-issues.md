# Rigs Project Plan: Comprehensive Issue Breakdown

> **Project**: Rigs — Rate-Limit-Aware Multi-Agent LLM Orchestration
> **Target**: 8 iterations over 8-10 weeks
> **Stack**: Rust + Tokio + Clap (core), Ollama (local LLM), SQLite (persistence)

---

## Overview Map

```
Iteration 0: Foundation ──────────────────────────────────────────── Week 1
    ├── Phase 0.1: Project Initialization (6 issues)
    ├── Phase 0.2: CLI Skeleton (9 issues)
    └── Phase 0.3: Documentation & Planning (4 issues)

Iteration 1: Data Layer ──────────────────────────────────────────── Week 2
    ├── Phase 1.1: Core Domain Types (12 issues)
    ├── Phase 1.2: SQLite Schema & Migrations (9 issues)
    └── Phase 1.3: Repository Layer (6 issues)

Iteration 2: Refinery (Rate Limits) ──────────────────────────────── Week 3
    ├── Phase 2.1: Token Bucket Algorithm (7 issues)
    ├── Phase 2.2: Refinery Core (9 issues)
    ├── Phase 2.3: Provider Data Sources (5 issues)
    └── Phase 2.4: CLI Tank Commands (5 issues)

Iteration 3: Depot (Task Queue) ──────────────────────────────────── Week 4
    ├── Phase 3.1: Depot Core (8 issues)
    ├── Phase 3.2: Convoy Management (7 issues)
    ├── Phase 3.3: CLI Bead Commands (5 issues)
    └── Phase 3.4: CLI Convoy Commands (4 issues)

Iteration 4: Dispatch (Routing) ──────────────────────────────────── Week 5
    ├── Phase 4.1: Routing Rules (7 issues)
    ├── Phase 4.2: Dispatch Engine (7 issues)
    └── Phase 4.3: Dispatch CLI (3 issues)

Iteration 5: Ollama Integration ──────────────────────────────────── Week 5-6
    ├── Phase 5.1: Ollama Client (8 issues)
    ├── Phase 5.2: Alternative Providers (5 issues)
    └── Phase 5.3: Assayer Foundation (5 issues)

Iteration 6: Assayer (Full Pipeline) ─────────────────────────────── Week 6-7
    ├── Phase 6.1: Planner Assayer (7 issues)
    ├── Phase 6.2: Optimizer Assayer (7 issues)
    ├── Phase 6.3: Estimator Assayer (8 issues)
    ├── Phase 6.4: Quality Gate Assayer (7 issues)
    └── Phase 6.5: Pipeline Integration (5 issues)

Iteration 7: Foreman & Polecats ──────────────────────────────────── Week 7-8
    ├── Phase 7.1: Polecat Workers (9 issues)
    ├── Phase 7.2: Foreman Orchestrator (9 issues)
    ├── Phase 7.3: CLI Foreman Commands (5 issues)
    └── Phase 7.4: Goal Processing CLI (4 issues)

Iteration 8: Polish & Release ────────────────────────────────────── Week 8-9
    ├── Phase 8.1: Interactive TUI (8 issues)
    ├── Phase 8.2: Gas Town Integration (6 issues)
    ├── Phase 8.3: Provider Management (4 issues)
    ├── Phase 8.4: Init Command (4 issues)
    └── Phase 8.5: Documentation & Release (7 issues)
```

---

# Iteration 0: Foundation & Scaffolding
**Duration**: 3-4 days | **Issues**: 19
**Goal**: Project setup, tooling, and basic CLI skeleton

---

## Phase 0.1: Project Initialization

### RIGS-001: Initialize Rust Workspace
```yaml
type: setup
priority: critical
complexity: low
dependencies: []
```
**Description**: Create the Rust workspace with proper crate structure.

**Tasks**:
- [ ] Run `cargo new rigs --bin`
- [ ] Create workspace Cargo.toml structure
- [ ] Set edition = "2021"
- [ ] Configure default-run

**Acceptance Criteria**:
- [ ] `cargo build` succeeds
- [ ] `cargo run -- --help` prints placeholder

---

### RIGS-002: Configure Core Dependencies
```yaml
type: setup
priority: critical
complexity: low
dependencies: [RIGS-001]
```
**Description**: Add all required dependencies to Cargo.toml.

**Dependencies to add**:
```toml
tokio = { version = "1.47", features = ["full"] }
clap = { version = "4.5", features = ["derive", "color"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio", "migrate"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "2.0"
anyhow = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
async-trait = "0.1"
reqwest = { version = "0.12", features = ["json"] }
toml = "0.8"
directories = "5.0"
```

**Acceptance Criteria**:
- [ ] All dependencies resolve
- [ ] No version conflicts

---

### RIGS-003: Set Up Git Repository
```yaml
type: setup
priority: high
complexity: low
dependencies: [RIGS-001]
```
**Description**: Initialize git with proper .gitignore and hooks.

**Tasks**:
- [ ] Create comprehensive .gitignore (Rust, IDE, SQLite)
- [ ] Set up pre-commit hook for `cargo fmt`
- [ ] Create initial commit

**Acceptance Criteria**:
- [ ] Git repo initialized
- [ ] Pre-commit hooks work

---

### RIGS-004: Create Directory Structure
```yaml
type: setup
priority: high
complexity: low
dependencies: [RIGS-001]
```
**Description**: Establish the project's module structure.

**Structure**:
```
src/
├── main.rs
├── cli/
│   ├── mod.rs
│   ├── tank.rs
│   ├── bead.rs
│   ├── convoy.rs
│   └── foreman.rs
├── core/
│   ├── mod.rs
│   ├── types.rs
│   ├── bead.rs
│   ├── convoy.rs
│   └── tank.rs
├── refinery/
│   ├── mod.rs
│   ├── token_bucket.rs
│   └── providers/
├── depot/
│   ├── mod.rs
│   └── queue.rs
├── dispatch/
│   ├── mod.rs
│   └── router.rs
├── assayer/
│   ├── mod.rs
│   ├── planner.rs
│   ├── optimizer.rs
│   ├── estimator.rs
│   └── quality.rs
├── foreman/
│   ├── mod.rs
│   └── polecat.rs
└── db/
    ├── mod.rs
    └── repository.rs
```

**Acceptance Criteria**:
- [ ] All directories created
- [ ] mod.rs files in place

---

### RIGS-005: Configure Linting & Formatting
```yaml
type: setup
priority: medium
complexity: low
dependencies: [RIGS-001]
```
**Description**: Set up rustfmt and clippy with project settings.

**Tasks**:
- [ ] Create rustfmt.toml with settings
- [ ] Create clippy.toml for lint rules
- [ ] Add `just lint` command
- [ ] Add `just fmt` command

**Acceptance Criteria**:
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy` passes with warnings as errors

---

### RIGS-006: Set Up CI Pipeline
```yaml
type: setup
priority: medium
complexity: medium
dependencies: [RIGS-003, RIGS-005]
```
**Description**: Create GitHub Actions workflow for CI.

**Workflow jobs**:
- Check formatting
- Run clippy
- Run tests
- Build release

**Acceptance Criteria**:
- [ ] CI runs on every push
- [ ] CI runs on PRs
- [ ] Badge in README

---

## Phase 0.2: CLI Skeleton

### RIGS-010: Implement Main CLI with Clap
```yaml
type: implementation
priority: critical
complexity: low
dependencies: [RIGS-002, RIGS-004]
```
**Description**: Create the main CLI entry point using clap derive macros.

**Code**:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rigs")]
#[command(version, about = "Rate-limit-aware multi-agent LLM orchestration")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Config file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
    
    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Init { ... },
    Provider { ... },
    Tank { ... },
    Bead { ... },
    Convoy { ... },
    Foreman { ... },
    Goal { ... },
}
```

**Acceptance Criteria**:
- [ ] `rigs --help` shows all commands
- [ ] `rigs --version` shows version
- [ ] Global flags work

---

### RIGS-011: Add `rigs init` Command
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-010]
```
**Description**: Create workspace initialization command.

```rust
/// Initialize a new Rigs workspace
Init {
    /// Path to workspace directory
    #[arg(default_value = "~/.rigs")]
    path: PathBuf,
    
    /// Initialize git repository
    #[arg(long)]
    git: bool,
}
```

**Acceptance Criteria**:
- [ ] `rigs init` creates directory
- [ ] `rigs init --git` initializes git

---

### RIGS-012: Add Provider Subcommand Group
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-010]
```
```rust
Provider {
    #[command(subcommand)]
    action: ProviderCommands,
}

enum ProviderCommands {
    Add { name: String },
    Remove { name: String },
    List,
    Test { name: String },
}
```

---

### RIGS-013: Add Tank Subcommand Group
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-010]
```
```rust
Tank {
    #[command(subcommand)]
    action: TankCommands,
}

enum TankCommands {
    List,
    Status { provider: String },
    Refresh,
    Set { provider: String, tokens: u64 },
}
```

---

### RIGS-014: Add Bead Subcommand Group
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-010]
```
```rust
Bead {
    #[command(subcommand)]
    action: BeadCommands,
}

enum BeadCommands {
    Create { description: String, #[arg(short, long)] task_type: TaskType },
    List { #[arg(long)] status: Option<BeadStatus> },
    Show { id: String },
    Edit { id: String },
    Cancel { id: String },
}
```

---

### RIGS-015: Add Convoy Subcommand Group
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-010]
```
```rust
Convoy {
    #[command(subcommand)]
    action: ConvoyCommands,
}

enum ConvoyCommands {
    Create { name: String },
    List,
    Show { id: String },
    Add { convoy_id: String, bead_id: String },
}
```

---

### RIGS-016: Add Foreman Subcommand Group
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-010]
```
```rust
Foreman {
    #[command(subcommand)]
    action: ForemanCommands,
}

enum ForemanCommands {
    Start { #[arg(long)] foreground: bool },
    Stop,
    Status,
    Attach,
}
```

---

### RIGS-017: Implement Config File Loading
```yaml
type: implementation
priority: high
complexity: medium
dependencies: [RIGS-010]
```
**Description**: Load configuration from TOML file.

**Config structure**:
```toml
[general]
workspace = "~/.rigs"
log_level = "info"

[providers.claude]
enabled = true
model = "claude-sonnet-4-20250514"
threshold_yellow = 0.5
threshold_red = 0.2

[providers.codex]
enabled = true
threshold_yellow = 0.4
threshold_red = 0.15

[providers.gemini]
enabled = true
threshold_yellow = 0.3
threshold_red = 0.1

[assayer]
model = "deepseek-r1:7b"
optimize_model = "qwen3:8b"

[routing]
strategy = "balanced"  # conservative, balanced, aggressive
```

**Acceptance Criteria**:
- [ ] Loads from ~/.rigs/config.toml
- [ ] Supports --config override
- [ ] Sensible defaults if missing

---

### RIGS-018: Add Logging Infrastructure
```yaml
type: implementation
priority: medium
complexity: low
dependencies: [RIGS-002]
```
**Description**: Set up tracing with configurable log levels.

**Acceptance Criteria**:
- [ ] Logs to stderr
- [ ] Respects RUST_LOG env
- [ ] --verbose increases level

---

## Phase 0.3: Documentation & Planning

### RIGS-020: Write Initial README
```yaml
type: documentation
priority: medium
complexity: low
dependencies: []
```
**Contents**:
- Project description
- Features list
- Quick start
- Gas Town relationship
- License

---

### RIGS-021: Document Terminology
```yaml
type: documentation
priority: medium
complexity: low
dependencies: []
```
**Description**: Create glossary mapping Gas Town terms to Rigs extensions.

| Gas Town | Rigs | Description |
|----------|------|-------------|
| Mayor | Foreman | Orchestrator with rate awareness |
| Bead | Bead | Work unit (+ token estimates) |
| Convoy | Convoy | Batch of beads |
| - | Tank | Per-provider rate limit state |
| - | Refinery | Rate limit tracker |
| - | Depot | Priority queue |
| - | Dispatch | Routing engine |
| - | Assayer | LLM optimizer |

---

### RIGS-022: Create CONTRIBUTING Guide
```yaml
type: documentation
priority: low
complexity: low
dependencies: []
```

---

### RIGS-023: Design Database Schema
```yaml
type: documentation
priority: high
complexity: medium
dependencies: []
```
**Description**: ERD diagram and schema documentation.

---

# Iteration 1: Data Layer & Core Types
**Duration**: 4-5 days | **Issues**: 27
**Goal**: SQLite persistence, core domain types, CRUD operations

---

## Phase 1.1: Core Domain Types

### RIGS-100: Define Provider Enum
```yaml
type: implementation
priority: critical
complexity: low
dependencies: [RIGS-004]
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
pub enum Provider {
    Claude,
    Codex,
    Gemini,
    Ollama,  // For local tasks
}

impl Provider {
    pub fn all() -> impl Iterator<Item = Provider> {
        [Provider::Claude, Provider::Codex, Provider::Gemini].into_iter()
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Provider::Claude => "Claude",
            Provider::Codex => "Codex",
            Provider::Gemini => "Gemini",
            Provider::Ollama => "Ollama",
        }
    }
}
```

---

### RIGS-101: Define TaskType Enum
```yaml
type: implementation
priority: critical
complexity: low
dependencies: [RIGS-004]
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
pub enum TaskType {
    Implementation,
    Review,
    Research,
    Refactor,
    Test,
    Documentation,
    Debug,
    Design,
}
```

---

### RIGS-102: Define Priority Enum
```yaml
type: implementation
priority: critical
complexity: low
dependencies: [RIGS-004]
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, clap::ValueEnum)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for Priority {
    fn default() -> Self { Priority::Normal }
}
```

---

### RIGS-103: Define BeadStatus Enum
```yaml
type: implementation
priority: critical
complexity: low
dependencies: [RIGS-004]
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeadStatus {
    Pending,      // Waiting in queue
    Optimizing,   // Being processed by Assayer
    Queued,       // Ready for provider
    Assigned,     // Sent to Polecat
    InProgress,   // Provider working
    Deferred,     // Waiting for rate limit reset
    Reviewing,    // Quality gate check
    Completed,
    Failed,
    Cancelled,
}
```

---

### RIGS-104: Define TankHealth Enum
```yaml
type: implementation
priority: critical
complexity: low
dependencies: [RIGS-004]
```
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TankHealth {
    Green,   // >50% capacity
    Yellow,  // 20-50% capacity
    Red,     // <20% capacity
    Empty,   // 0% - locked
}

impl TankHealth {
    pub fn from_ratio(ratio: f32, yellow: f32, red: f32) -> Self {
        if ratio <= 0.0 { TankHealth::Empty }
        else if ratio < red { TankHealth::Red }
        else if ratio < yellow { TankHealth::Yellow }
        else { TankHealth::Green }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            TankHealth::Green => "green",
            TankHealth::Yellow => "yellow",
            TankHealth::Red => "red",
            TankHealth::Empty => "gray",
        }
    }
}
```

---

### RIGS-105: Implement BeadId Type
```yaml
type: implementation
priority: critical
complexity: medium
dependencies: [RIGS-004]
```
```rust
/// A unique identifier for beads in the format "gt-xxxxx"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BeadId(String);

impl BeadId {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let suffix: String = (0..5)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect::<String>()
            .to_lowercase();
        BeadId(format!("gt-{}", suffix))
    }
    
    pub fn from_str(s: &str) -> Result<Self, InvalidBeadId> {
        if s.starts_with("gt-") && s.len() == 8 && s[3..].chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(BeadId(s.to_string()))
        } else {
            Err(InvalidBeadId(s.to_string()))
        }
    }
}

impl Display for BeadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

---

### RIGS-106: Implement Bead Struct
```yaml
type: implementation
priority: critical
complexity: high
dependencies: [RIGS-100, RIGS-101, RIGS-102, RIGS-103, RIGS-105]
```
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bead {
    pub id: BeadId,
    pub title: String,
    pub description: String,
    pub task_type: TaskType,
    pub priority: Priority,
    pub status: BeadStatus,
    
    // Token tracking
    pub estimated_tokens: u64,
    pub actual_tokens: Option<u64>,
    
    // Provider assignment
    pub preferred_provider: Option<Provider>,
    pub assigned_provider: Option<Provider>,
    
    // Task details
    pub acceptance_criteria: Vec<String>,
    pub dependencies: Vec<BeadId>,
    pub convoy_id: Option<ConvoyId>,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deferred_until: Option<DateTime<Utc>>,
    
    // Content
    pub optimized_prompt: Option<String>,
    pub output: Option<String>,
    pub error: Option<String>,
}

impl Bead {
    pub fn new(title: String, description: String, task_type: TaskType) -> Self {
        Self {
            id: BeadId::new(),
            title,
            description,
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
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_criteria(mut self, criteria: Vec<String>) -> Self {
        self.acceptance_criteria = criteria;
        self
    }
}
```

---

### RIGS-107: Implement Convoy Struct
```yaml
type: implementation
priority: high
complexity: medium
dependencies: [RIGS-105, RIGS-106]
```
```rust
pub type ConvoyId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convoy {
    pub id: ConvoyId,
    pub name: String,
    pub goal: Option<String>,  // Original goal if created from decomposition
    pub beads: Vec<BeadId>,
    pub status: ConvoyStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConvoyStatus {
    Planning,
    Queued,
    InProgress,
    Paused,
    Completed,
    Failed,
}

impl Convoy {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            goal: None,
            beads: vec![],
            status: ConvoyStatus::Planning,
            created_at: Utc::now(),
            completed_at: None,
        }
    }
    
    pub fn progress(&self, bead_statuses: &HashMap<BeadId, BeadStatus>) -> f32 {
        let completed = self.beads.iter()
            .filter(|id| bead_statuses.get(*id) == Some(&BeadStatus::Completed))
            .count();
        completed as f32 / self.beads.len().max(1) as f32
    }
}
```

---

### RIGS-108: Implement Tank Struct
```yaml
type: implementation
priority: critical
complexity: medium
dependencies: [RIGS-100, RIGS-104]
```
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tank {
    pub provider: Provider,
    pub capacity: u64,
    pub remaining: u64,
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub health: TankHealth,
    pub last_request: Option<DateTime<Utc>>,
    pub requests_this_window: u32,
    pub tokens_this_window: u64,
    pub updated_at: DateTime<Utc>,
}

impl Tank {
    pub fn capacity_ratio(&self) -> f32 {
        if self.capacity == 0 { return 0.0; }
        self.remaining as f32 / self.capacity as f32
    }
    
    pub fn time_until_reset(&self) -> chrono::Duration {
        let now = Utc::now();
        if now >= self.window_end {
            chrono::Duration::zero()
        } else {
            self.window_end - now
        }
    }
    
    pub fn can_consume(&self, tokens: u64) -> bool {
        self.remaining >= tokens && self.health != TankHealth::Empty
    }
    
    pub fn consume(&mut self, tokens: u64) -> Result<(), InsufficientCapacity> {
        if !self.can_consume(tokens) {
            return Err(InsufficientCapacity { requested: tokens, available: self.remaining });
        }
        self.remaining -= tokens;
        self.tokens_this_window += tokens;
        self.requests_this_window += 1;
        self.last_request = Some(Utc::now());
        self.recalculate_health();
        Ok(())
    }
    
    fn recalculate_health(&mut self) {
        let ratio = self.capacity_ratio();
        self.health = TankHealth::from_ratio(ratio, 0.5, 0.2);
    }
}
```

---

### RIGS-109: Implement ProviderConfig
```yaml
type: implementation
priority: high
complexity: medium
dependencies: [RIGS-100]
```
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: Provider,
    pub enabled: bool,
    pub model: String,
    pub limits: ProviderLimits,
    pub threshold_yellow: f32,
    pub threshold_red: f32,
    pub fallback_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderLimits {
    pub tokens_per_window: u64,
    pub window_hours: u32,
    pub requests_per_minute: Option<u32>,
    pub weekly_cap: Option<u64>,
    pub daily_cap: Option<u64>,
}

impl ProviderConfig {
    pub fn claude_default() -> Self {
        Self {
            provider: Provider::Claude,
            enabled: true,
            model: "claude-sonnet-4-20250514".into(),
            limits: ProviderLimits {
                tokens_per_window: 35_000,
                window_hours: 5,
                requests_per_minute: None,
                weekly_cap: Some(500_000),
                daily_cap: None,
            },
            threshold_yellow: 0.5,
            threshold_red: 0.2,
            fallback_model: None,
        }
    }
    
    pub fn codex_default() -> Self {
        Self {
            provider: Provider::Codex,
            enabled: true,
            model: "codex".into(),
            limits: ProviderLimits {
                tokens_per_window: 30_000,
                window_hours: 5,
                requests_per_minute: Some(60),
                weekly_cap: None,
                daily_cap: None,
            },
            threshold_yellow: 0.4,
            threshold_red: 0.15,
            fallback_model: None,
        }
    }
    
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
            fallback_model: None,
        }
    }
}
```

---

### RIGS-110: Define Error Types
```yaml
type: implementation
priority: high
complexity: medium
dependencies: [RIGS-100, RIGS-103, RIGS-105]
```
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RigsError {
    #[error("Provider {0:?} is not configured")]
    ProviderNotConfigured(Provider),
    
    #[error("Rate limit exceeded for {provider:?}: {remaining} remaining, need {requested}")]
    RateLimitExceeded {
        provider: Provider,
        remaining: u64,
        requested: u64,
    },
    
    #[error("Bead {0} not found")]
    BeadNotFound(BeadId),
    
    #[error("Convoy {0} not found")]
    ConvoyNotFound(String),
    
    #[error("Invalid bead ID: {0}")]
    InvalidBeadId(String),
    
    #[error("Invalid state transition: {from:?} -> {to:?}")]
    InvalidStateTransition {
        from: BeadStatus,
        to: BeadStatus,
    },
    
    #[error("All providers exhausted, next reset at {0}")]
    AllProvidersExhausted(DateTime<Utc>),
    
    #[error("Dependency cycle detected: {0:?}")]
    DependencyCycle(Vec<BeadId>),
    
    #[error("Assayer error: {0}")]
    AssayerError(String),
    
    #[error("Ollama not available: {0}")]
    OllamaNotAvailable(String),
    
    #[error("Provider API error: {0}")]
    ProviderApiError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Config error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, RigsError>;
```

---

### RIGS-111: Write Unit Tests for Core Types
```yaml
type: test
priority: high
complexity: medium
dependencies: [RIGS-100..RIGS-110]
```
**Test cases**:
- BeadId parsing (valid/invalid)
- BeadId uniqueness
- TankHealth::from_ratio thresholds
- Bead builder pattern
- Convoy progress calculation
- Tank consumption logic
- Serialization roundtrips

---

## Phase 1.2: SQLite Schema & Migrations

### RIGS-120: Set Up SQLx
```yaml
type: implementation
priority: critical
complexity: medium
dependencies: [RIGS-002]
```
**Tasks**:
- Configure SQLx for SQLite
- Set up connection pooling
- Configure compile-time checking
- Create migrations directory

---

### RIGS-121: Create Tanks Table
```yaml
type: implementation
priority: critical
complexity: low
dependencies: [RIGS-120]
```
```sql
-- migrations/001_tanks.sql
CREATE TABLE tanks (
    provider TEXT PRIMARY KEY,
    capacity INTEGER NOT NULL,
    remaining INTEGER NOT NULL,
    window_start TEXT NOT NULL,
    window_end TEXT NOT NULL,
    health TEXT NOT NULL DEFAULT 'green',
    last_request TEXT,
    requests_this_window INTEGER NOT NULL DEFAULT 0,
    tokens_this_window INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL
);
```

---

### RIGS-122: Create Beads Table
```yaml
type: implementation
priority: critical
complexity: medium
dependencies: [RIGS-120]
```
```sql
-- migrations/002_beads.sql
CREATE TABLE beads (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    task_type TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'pending',
    estimated_tokens INTEGER NOT NULL DEFAULT 0,
    actual_tokens INTEGER,
    preferred_provider TEXT,
    assigned_provider TEXT,
    acceptance_criteria TEXT NOT NULL DEFAULT '[]',
    dependencies TEXT NOT NULL DEFAULT '[]',
    convoy_id TEXT,
    created_at TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    deferred_until TEXT,
    optimized_prompt TEXT,
    output TEXT,
    error TEXT
);

CREATE INDEX idx_beads_status ON beads(status);
CREATE INDEX idx_beads_priority ON beads(priority DESC, created_at ASC);
CREATE INDEX idx_beads_deferred ON beads(deferred_until) WHERE status = 'deferred';
CREATE INDEX idx_beads_convoy ON beads(convoy_id);
```

---

### RIGS-123: Create Convoys Table
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-120]
```
```sql
-- migrations/003_convoys.sql
CREATE TABLE convoys (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    goal TEXT,
    status TEXT NOT NULL DEFAULT 'planning',
    created_at TEXT NOT NULL,
    completed_at TEXT
);
```

---

### RIGS-124: Create Completions Table
```yaml
type: implementation
priority: high
complexity: low
dependencies: [RIGS-120]
```
```sql
-- migrations/004_completions.sql
CREATE TABLE completions (
    id TEXT PRIMARY KEY,
    bead_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    estimated_tokens INTEGER NOT NULL,
    actual_tokens INTEGER NOT NULL,
    duration_ms INTEGER NOT NULL,
    success INTEGER NOT NULL,
    quality_score REAL,
    original_prompt TEXT,
    optimized_prompt TEXT,
    error_message TEXT,
    completed_at TEXT NOT NULL
);

CREATE INDEX idx_completions_bead ON completions(bead_id);
CREATE INDEX idx_completions_provider ON completions(provider, completed_at);
```

---

### RIGS-125: Create Optimization Traces Table
```yaml
type: implementation
priority: medium
complexity: low
dependencies: [RIGS-120]
```
```sql
-- migrations/005_optimization_traces.sql
CREATE TABLE optimization_traces (
    id TEXT PRIMARY KEY,
    task_type TEXT NOT NULL,
    original_prompt TEXT NOT NULL,
    optimized_prompt TEXT NOT NULL,
    estimated_tokens INTEGER NOT NULL,
    actual_tokens INTEGER,
    quality_score REAL,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_traces_task_type ON optimization_traces(task_type, quality_score DESC);
```

---

### RIGS-126: Create Config Table
```yaml
type: implementation
priority: medium
complexity: low
dependencies: [RIGS-120]
```
```sql
-- migrations/006_config.sql
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

---

### RIGS-127: Implement Migration Runner
```yaml
type: implementation
priority: high
complexity: medium
dependencies: [RIGS-121..RIGS-126]
```
**Description**: Run migrations on init or when outdated.

---

## Phase 1.3: Repository Layer

### RIGS-130: Implement TankRepository
```yaml
type: implementation
priority: critical
complexity: medium
dependencies: [RIGS-108, RIGS-121]
```
```rust
#[async_trait]
pub trait TankRepository: Send + Sync {
    async fn get(&self, provider: Provider) -> Result<Option<Tank>>;
    async fn get_all(&self) -> Result<Vec<Tank>>;
    async fn upsert(&self, tank: &Tank) -> Result<()>;
}
```

---

### RIGS-131: Implement BeadRepository
```yaml
type: implementation
priority: critical
complexity: high
dependencies: [RIGS-106, RIGS-122]
```
```rust
#[async_trait]
pub trait BeadRepository: Send + Sync {
    async fn create(&self, bead: &Bead) -> Result<()>;
    async fn get(&self, id: &BeadId) -> Result<Option<Bead>>;
    async fn update(&self, bead: &Bead) -> Result<()>;
    async fn delete(&self, id: &BeadId) -> Result<()>;
    
    async fn list_by_status(&self, status: BeadStatus) -> Result<Vec<Bead>>;
    async fn list_by_convoy(&self, convoy_id: &str) -> Result<Vec<Bead>>;
    async fn get_pending_ordered(&self) -> Result<Vec<Bead>>;
    async fn get_deferred_ready(&self, now: DateTime<Utc>) -> Result<Vec<Bead>>;
}
```

---

### RIGS-132: Implement ConvoyRepository
```yaml
type: implementation
priority: high
complexity: medium
dependencies: [RIGS-107, RIGS-123]
```

---

### RIGS-133: Implement CompletionRepository
```yaml
type: implementation
priority: medium
complexity: medium
dependencies: [RIGS-124]
```

---

### RIGS-134: Implement TraceRepository
```yaml
type: implementation
priority: medium
complexity: medium
dependencies: [RIGS-125]
```

---

### RIGS-135: Write Repository Integration Tests
```yaml
type: test
priority: high
complexity: high
dependencies: [RIGS-130..RIGS-134]
```
**Description**: Test all CRUD operations with temporary database.

---

# Iteration 2-8: Continue with Same Detail...

[Document continues with same level of detail for remaining iterations]

---

## Issue Count Summary

| Iteration | Phases | Issues | Complexity |
|-----------|--------|--------|------------|
| 0: Foundation | 3 | 19 | Low |
| 1: Data Layer | 3 | 27 | Medium |
| 2: Refinery | 4 | 26 | High |
| 3: Depot | 4 | 24 | Medium |
| 4: Dispatch | 3 | 17 | High |
| 5: Ollama | 3 | 18 | High |
| 6: Assayer | 5 | 34 | Very High |
| 7: Foreman | 4 | 27 | Very High |
| 8: Polish | 5 | 29 | Medium |
| **Total** | **34** | **221** | - |

---

## Critical Path

```
[Iteration 0] Foundation
       │
       ▼
[Iteration 1] Data Layer
       │
       ├────────────┬────────────┐
       ▼            ▼            ▼
[Iter 2]       [Iter 3]      [Iter 5]
Refinery       Depot         Ollama
       │            │            │
       └────────┬───┘            │
                ▼                │
           [Iter 4]              │
           Dispatch              │
                │                │
                └────────┬───────┘
                         ▼
                    [Iter 6]
                    Assayer (Full)
                         │
                         ▼
                    [Iter 7]
                    Foreman
                         │
                         ▼
                    [Iter 8]
                    Polish
```

---

## Milestone Checkpoints

**MVP 1 (End of Iteration 2)**:
- [ ] Can view tank status for all providers
- [ ] Rate limits refresh from ccusage/CLI
- [ ] CLI commands work

**MVP 2 (End of Iteration 4)**:
- [ ] Can add beads to queue
- [ ] Routing decisions work
- [ ] Deferral works

**MVP 3 (End of Iteration 6)**:
- [ ] Goals decompose into beads
- [ ] Prompts are optimized
- [ ] Quality gate runs

**MVP 4 (End of Iteration 7)**:
- [ ] Full end-to-end execution
- [ ] Tasks route to Claude/Codex/Gemini
- [ ] Results returned

**Release (End of Iteration 8)**:
- [ ] TUI works
- [ ] Gas Town sync works
- [ ] Documentation complete

---

## Quick Reference: Gas Town Bead Format

Each issue can be converted to a Gas Town bead:

```json
{
  "id": "rigs-106",
  "title": "Implement Bead Struct",
  "description": "Implement the core Bead struct with all fields...",
  "task_type": "implementation",
  "priority": "critical",
  "estimated_tokens": 1500,
  "acceptance_criteria": [
    "Bead struct has all required fields",
    "Builder pattern works",
    "Serde serialization works",
    "Unit tests pass"
  ],
  "dependencies": ["rigs-100", "rigs-101", "rigs-102", "rigs-103", "rigs-105"],
  "labels": ["iteration-1", "phase-1.1", "core-types"]
}
```
