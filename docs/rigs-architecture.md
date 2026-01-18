# Rigs: Rate-Limit-Aware Multi-Agent LLM Orchestration

> *"Gas Town for the frugal — where every token counts"*

## Executive Summary

**Rigs** is a multi-agent orchestration system that extends Gas Town's paradigm with intelligent rate limit management across multiple LLM providers. Designed for developers on Pro-tier subscriptions (not $200/month Max plans), it ensures you never exhaust your quotas while maximizing productivity across Claude, Gemini, and ChatGPT/Codex.

---

## 1. Problem Statement

### The Challenge
You have access to multiple LLM Pro subscriptions:
- **Claude Pro** (Claude Code Max 5x): 5-hour rolling windows, weekly caps
- **ChatGPT Pro** (Codex CLI): Rate limits on GPT-5-Codex
- **Gemini Pro**: Requests/minute and tokens/day limits

Each provider has different:
- Rate limit structures (tokens, requests, time windows)
- Strengths (Claude for implementation, Codex for reviews, Gemini for research)
- Reset mechanisms (rolling windows vs. fixed windows)

**Without orchestration**: Hit limits → work stops → context lost → frustration

### The Goal
Build a system that:
1. **Tracks** rate limits across all providers in real-time
2. **Routes** tasks to the optimal provider based on capacity + task type
3. **Queues** work when limits approach (proactive, not reactive)
4. **Persists** state so agents can resume after limit windows reset
5. **Uses Gas Town terminology** for familiarity and fun

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              RIGS CLI                                    │
│                         (Rust + Tokio + Clap)                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐              │
│  │   REFINERY   │    │    DEPOT     │    │   DISPATCH   │              │
│  │  (Rate Limit │    │ (Task Queue  │    │  (Provider   │              │
│  │   Tracker)   │    │  + Priority) │    │   Router)    │              │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘              │
│         │                   │                   │                        │
│         └───────────────────┼───────────────────┘                        │
│                             │                                            │
│                    ┌────────▼────────┐                                   │
│                    │    FOREMAN      │                                   │
│                    │  (Orchestrator) │                                   │
│                    └────────┬────────┘                                   │
│                             │                                            │
│         ┌───────────────────┼───────────────────┐                        │
│         │                   │                   │                        │
│  ┌──────▼──────┐    ┌──────▼──────┐    ┌──────▼──────┐                 │
│  │   POLECAT   │    │   POLECAT   │    │   POLECAT   │                 │
│  │   (Claude)  │    │   (Codex)   │    │   (Gemini)  │                 │
│  └─────────────┘    └─────────────┘    └─────────────┘                 │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    │               │               │
             ┌──────▼──────┐ ┌─────▼─────┐ ┌──────▼──────┐
             │ Claude API  │ │ Codex CLI │ │ Gemini API  │
             │  (Anthropic)│ │ (OpenAI)  │ │  (Google)   │
             └─────────────┘ └───────────┘ └─────────────┘
```

---

## 3. Core Components (Gas Town Terminology Extended)

### 3.1 Refinery — Rate Limit Tracker
The **Refinery** monitors and tracks rate limit "fuel levels" for each provider.

```rust
struct Refinery {
    tanks: HashMap<Provider, Tank>,
}

struct Tank {
    provider: Provider,
    capacity: TokenBucket,
    current_level: u64,        // Tokens/requests remaining
    refill_rate: u64,          // Tokens per second
    window_type: WindowType,   // Rolling5Hour | Fixed | Daily
    next_reset: DateTime<Utc>,
    health: TankHealth,        // Green | Yellow | Red | Empty
}

enum TankHealth {
    Green,   // >50% capacity — route freely
    Yellow,  // 20-50% capacity — prefer other providers
    Red,     // <20% capacity — emergency only
    Empty,   // 0% — provider locked until reset
}
```

**Data Sources:**
- `ccusage --json` for Claude Code
- Codex CLI output parsing for OpenAI
- Gemini API response headers
- Local SQLite for persistence

### 3.2 Depot — Task Queue with Priority
The **Depot** holds incoming work items (beads) and manages priority scheduling.

```rust
struct Depot {
    incoming: PriorityQueue<Bead>,
    deferred: Vec<DeferredBead>,  // Waiting for rate limit reset
    in_progress: HashMap<BeadId, Assignment>,
}

struct Bead {
    id: BeadId,
    title: String,
    description: String,
    task_type: TaskType,
    estimated_tokens: u64,
    priority: Priority,
    preferred_provider: Option<Provider>,
    created_at: DateTime<Utc>,
    constraints: Vec<Constraint>,
}

enum TaskType {
    Implementation,  // Complex coding → Claude (Opus)
    Review,          // Code review → Codex
    Research,        // Web search, docs → Gemini
    Refactor,        // Medium complexity → Any available
    Test,            // Test writing → Codex or Claude
    Documentation,   // Docs generation → Any available
}

enum Priority {
    Critical,  // Route immediately, use best available
    High,      // Route soon, prefer optimal provider
    Normal,    // Route when capacity available
    Low,       // Defer to off-peak or idle time
}
```

### 3.3 Dispatch — Provider Router
**Dispatch** decides which provider handles each bead based on:
1. Task type affinity
2. Current rate limit status
3. Provider health
4. Cost considerations

```rust
struct Dispatch {
    routing_rules: Vec<RoutingRule>,
    affinity_map: HashMap<TaskType, Vec<(Provider, f32)>>,  // Provider + weight
}

impl Dispatch {
    fn route(&self, bead: &Bead, refinery: &Refinery) -> RoutingDecision {
        // 1. Check if preferred provider has capacity
        if let Some(pref) = &bead.preferred_provider {
            if refinery.has_capacity(pref, bead.estimated_tokens) {
                return RoutingDecision::Route(*pref);
            }
        }
        
        // 2. Find best provider by affinity + capacity
        let candidates = self.affinity_map
            .get(&bead.task_type)
            .iter()
            .filter(|(provider, _)| {
                refinery.get_health(provider) != TankHealth::Empty
            })
            .sorted_by(|(p1, w1), (p2, w2)| {
                // Weight by: affinity * (capacity_remaining / total_capacity)
                let score1 = w1 * refinery.capacity_ratio(p1);
                let score2 = w2 * refinery.capacity_ratio(p2);
                score2.partial_cmp(&score1).unwrap()
            });
        
        match candidates.first() {
            Some((provider, _)) => RoutingDecision::Route(*provider),
            None => {
                // All providers exhausted — defer
                let next_reset = refinery.next_available_reset();
                RoutingDecision::Defer(next_reset)
            }
        }
    }
}
```

### 3.4 Foreman — Central Orchestrator
The **Foreman** coordinates everything, similar to Gas Town's Mayor but with rate-limit awareness.

```rust
struct Foreman {
    refinery: Arc<RwLock<Refinery>>,
    depot: Arc<RwLock<Depot>>,
    dispatch: Dispatch,
    polecats: HashMap<Provider, Polecat>,
    state_path: PathBuf,  // Git-backed state persistence
}

impl Foreman {
    async fn process_queue(&self) -> Result<()> {
        loop {
            // 1. Refresh rate limit data
            self.refinery.write().await.refresh_all().await?;
            
            // 2. Check for deferred beads that can now be processed
            self.promote_deferred_beads().await;
            
            // 3. Get next bead from depot
            let bead = match self.depot.write().await.pop() {
                Some(b) => b,
                None => {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
            
            // 4. Route the bead
            match self.dispatch.route(&bead, &*self.refinery.read().await) {
                RoutingDecision::Route(provider) => {
                    self.assign_to_polecat(provider, bead).await?;
                }
                RoutingDecision::Defer(until) => {
                    self.depot.write().await.defer(bead, until);
                    log::info!("Deferred {} until {}", bead.id, until);
                }
            }
            
            // 5. Persist state
            self.save_state().await?;
        }
    }
}
```

### 3.5 Polecat — Provider Worker
**Polecats** are the actual workers that interface with LLM providers.

```rust
struct Polecat {
    provider: Provider,
    config: ProviderConfig,
    current_task: Option<BeadId>,
    session: Option<SessionHandle>,  // For Claude Code / Codex sessions
}

impl Polecat {
    async fn execute(&mut self, bead: Bead) -> Result<BeadResult> {
        match self.provider {
            Provider::Claude => self.execute_claude(bead).await,
            Provider::Codex => self.execute_codex(bead).await,
            Provider::Gemini => self.execute_gemini(bead).await,
        }
    }
    
    async fn execute_claude(&mut self, bead: Bead) -> Result<BeadResult> {
        // Spawn claude code session or use API
        let output = Command::new("claude")
            .args(["--print", "--output-format", "json"])
            .arg(&bead.description)
            .output()
            .await?;
        
        // Parse response, track tokens used
        let response: ClaudeResponse = serde_json::from_slice(&output.stdout)?;
        
        Ok(BeadResult {
            bead_id: bead.id,
            provider: Provider::Claude,
            tokens_used: response.usage.total_tokens,
            output: response.content,
            duration: response.duration,
        })
    }
}
```

---

## 4. Rate Limiting Strategy

### 4.1 Token Bucket Algorithm (Per Provider)

Based on research, we use a **Token Bucket** algorithm because it:
- Allows burst capacity (using saved tokens)
- Enforces average rate over time
- Matches how most LLM providers actually rate limit

```rust
struct TokenBucket {
    capacity: u64,           // Maximum tokens
    tokens: f64,             // Current tokens (float for partial refills)
    refill_rate: f64,        // Tokens per second
    last_update: Instant,
}

impl TokenBucket {
    fn consume(&mut self, amount: u64) -> bool {
        self.refill();
        
        if self.tokens >= amount as f64 {
            self.tokens -= amount as f64;
            true
        } else {
            false  // Insufficient capacity
        }
    }
    
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity as f64);
        self.last_update = now;
    }
    
    fn time_until_available(&self, amount: u64) -> Duration {
        if self.tokens >= amount as f64 {
            Duration::ZERO
        } else {
            let needed = amount as f64 - self.tokens;
            Duration::from_secs_f64(needed / self.refill_rate)
        }
    }
}
```

### 4.2 Provider-Specific Configurations

```rust
fn default_provider_configs() -> HashMap<Provider, ProviderConfig> {
    hashmap! {
        Provider::Claude => ProviderConfig {
            name: "Claude Pro (Max 5x)".into(),
            // 5-hour rolling window, switches to Sonnet at 20%
            window: WindowConfig::Rolling { hours: 5 },
            limits: vec![
                Limit::TokensPerWindow { tokens: 35_000, window_hours: 5 },
                Limit::WeeklyCap { tokens: 500_000 },  // Approximate
            ],
            threshold_yellow: 0.50,  // Switch preference at 50%
            threshold_red: 0.20,     // Emergency only at 20%
            fallback_model: Some("sonnet".into()),
        },
        
        Provider::Codex => ProviderConfig {
            name: "Codex CLI (ChatGPT Pro)".into(),
            window: WindowConfig::Rolling { hours: 5 },
            limits: vec![
                Limit::TokensPerWindow { tokens: 30_000, window_hours: 5 },
                Limit::RequestsPerMinute { rpm: 60 },
            ],
            threshold_yellow: 0.40,
            threshold_red: 0.15,
            fallback_model: Some("codex-mini".into()),
        },
        
        Provider::Gemini => ProviderConfig {
            name: "Gemini Pro".into(),
            window: WindowConfig::Fixed { reset_daily: true },
            limits: vec![
                Limit::RequestsPerMinute { rpm: 15 },
                Limit::TokensPerDay { tokens: 1_000_000 },
            ],
            threshold_yellow: 0.30,
            threshold_red: 0.10,
            fallback_model: None,
        },
    }
}
```

### 4.3 Proactive Deferral Strategy

**Key Insight**: Don't wait for rate limit errors — proactively defer work.

```rust
enum DeferralStrategy {
    // Conservative: Defer when ANY provider hits yellow
    Conservative,
    
    // Balanced: Defer only when preferred provider hits red
    Balanced,
    
    // Aggressive: Only defer when all providers exhausted
    Aggressive,
}

impl Foreman {
    fn should_defer(&self, bead: &Bead, strategy: DeferralStrategy) -> Option<DateTime<Utc>> {
        let refinery = self.refinery.read().await;
        
        match strategy {
            DeferralStrategy::Conservative => {
                // If optimal provider is yellow, wait for green
                let optimal = self.dispatch.get_optimal_provider(&bead.task_type);
                if refinery.get_health(&optimal) == TankHealth::Yellow {
                    Some(refinery.time_until_green(&optimal))
                } else {
                    None
                }
            }
            
            DeferralStrategy::Balanced => {
                // Route to any available, defer only if all red/empty
                let any_available = Provider::all()
                    .any(|p| matches!(
                        refinery.get_health(&p),
                        TankHealth::Green | TankHealth::Yellow
                    ));
                
                if any_available { None } 
                else { Some(refinery.next_available_reset()) }
            }
            
            DeferralStrategy::Aggressive => {
                // Only defer if literally no capacity anywhere
                if refinery.total_capacity() > bead.estimated_tokens {
                    None
                } else {
                    Some(refinery.next_available_reset())
                }
            }
        }
    }
}
```

---

## 5. Task Routing Matrix

### 5.1 Provider Affinity by Task Type

| Task Type | Claude (Opus) | Codex (GPT-5) | Gemini Pro | Notes |
|-----------|---------------|---------------|------------|-------|
| Implementation | **1.0** | 0.6 | 0.4 | Claude excels at complex code |
| Code Review | 0.7 | **1.0** | 0.5 | Codex `/review` is purpose-built |
| Refactor | 0.8 | 0.8 | 0.5 | Either Claude or Codex works |
| Research | 0.6 | 0.5 | **1.0** | Gemini has better web access |
| Test Writing | 0.7 | **0.9** | 0.4 | Codex excellent for tests |
| Documentation | 0.8 | 0.7 | 0.8 | All roughly equal |
| Architecture | **1.0** | 0.7 | 0.6 | Claude best for high-level design |

### 5.2 Routing Decision Flow

```
┌─────────────────┐
│ Incoming Bead   │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────┐
│ Check preferred provider capacity   │
└────────┬───────────────┬────────────┘
         │               │
      Has capacity    No capacity
         │               │
         ▼               ▼
┌─────────────┐  ┌─────────────────────┐
│ Route to    │  │ Find best available │
│ preferred   │  │ by affinity × cap   │
└─────────────┘  └──────────┬──────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
        Found provider              No providers
              │                           │
              ▼                           ▼
       ┌─────────────┐           ┌─────────────┐
       │ Route to    │           │ Defer until │
       │ alternative │           │ next reset  │
       └─────────────┘           └─────────────┘
```

---

## 6. Patterns & Anti-Patterns

### 6.1 Patterns to Follow ✅

#### Pattern 1: Role Specialization
> "Reserve general, high-level reasoning for the Coordinator only"

Each polecat (worker) has a **single responsibility**:
- Claude Polecat → Implementation tasks only
- Codex Polecat → Review tasks only
- Gemini Polecat → Research tasks only

The Foreman handles all routing decisions.

#### Pattern 2: Sense-Think-Act Loop
Every agent follows:
1. **Sense**: Ingest task from depot
2. **Think**: Process with LLM
3. **Act**: Return result, update state

#### Pattern 3: Git-Native Persistence
Like Gas Town, all state persists in git:
```
~/.rigs/
├── state.json          # Current depot, refinery state
├── history/            # Completed beads
├── logs/               # Execution logs
└── .git/               # Version everything
```

#### Pattern 4: Circuit Breaker for Providers
If a provider returns 3+ consecutive errors:
1. Mark as `Unhealthy`
2. Stop routing to it
3. Retry after exponential backoff (1min → 2min → 4min)

#### Pattern 5: Token Estimation Before Routing
Estimate tokens BEFORE sending to provider:
```rust
fn estimate_tokens(bead: &Bead) -> u64 {
    // Rough heuristic: 1 token ≈ 4 characters
    let input_tokens = bead.description.len() as u64 / 4;
    let output_estimate = match bead.task_type {
        TaskType::Implementation => input_tokens * 3,  // Code is verbose
        TaskType::Review => input_tokens * 2,
        TaskType::Documentation => input_tokens * 4,
        _ => input_tokens * 2,
    };
    input_tokens + output_estimate
}
```

### 6.2 Anti-Patterns to Avoid ❌

#### Anti-Pattern 1: The Swiss Army Knife Agent
> "Every agent is prompted to be a smart assistant capable of doing anything"

**Problem**: No accountability, debugging nightmare
**Solution**: Strict role definitions per polecat

#### Anti-Pattern 2: Prompt Entanglement
> "Cramming every instruction into a single enormous prompt"

**Problem**: Bloated context, unpredictable behavior
**Solution**: Task-specific prompts per task type

#### Anti-Pattern 3: Circular Dependencies
**Problem**: Agent A waits for B, B waits for A
**Solution**: DAG-based task dependencies only

#### Anti-Pattern 4: Reactive Rate Limiting
**Problem**: Only respond to 429 errors after they happen
**Solution**: Track capacity proactively, defer before exhaustion

#### Anti-Pattern 5: Stateless Reasoning
**Problem**: Agent forgets context between calls
**Solution**: Persist conversation state, use memory/context injection

#### Anti-Pattern 6: Over-Complex Routing
> "You do not need a fancy AI to decide which AI to use"

**Problem**: Routing logic becomes a bottleneck
**Solution**: Simple rules first (task type → provider), add complexity only when needed

---

## 7. Technology Stack

### 7.1 Core Runtime: Rust + Tokio

**Why Rust?**
- Fast CLI startup (no JIT warmup like Node/Bun)
- Excellent async story with Tokio
- Strong typing prevents runtime errors
- Low memory footprint for long-running daemon
- `clap` is the best CLI framework available

**Crates:**
```toml
[dependencies]
tokio = { version = "1.47", features = ["full"] }
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.12", features = ["json"] }
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

### 7.2 Alternative: Bun + TypeScript

If rapid iteration > raw performance:

**Why Bun?**
- Fastest JS runtime
- Native TypeScript support
- Good for scripting/glue code
- Easier to modify routing rules

**Packages:**
```json
{
  "dependencies": {
    "@anthropic-ai/sdk": "^0.32.0",
    "openai": "^4.73.0",
    "@google/generative-ai": "^0.21.0",
    "commander": "^12.0.0",
    "better-sqlite3": "^11.0.0"
  }
}
```

### 7.3 Recommended: Hybrid Approach

```
Rust Core (rigs-daemon)
├── Rate limit tracking (needs precision)
├── Token bucket algorithm (needs speed)
├── Task queue management (needs persistence)
└── Provider health monitoring (needs reliability)

TypeScript/Bun Plugins (rigs-plugins/)
├── Custom routing rules (needs flexibility)
├── Provider adapters (needs rapid iteration)
├── Prompt templates (needs easy editing)
└── Dashboard UI (needs quick prototyping)
```

The Rust daemon exposes a local HTTP API that Bun plugins call.

---

## 8. Data Persistence

### 8.1 SQLite Schema

```sql
-- Provider rate limit state
CREATE TABLE tanks (
    provider TEXT PRIMARY KEY,
    tokens_remaining INTEGER NOT NULL,
    window_start_at TEXT NOT NULL,
    last_request_at TEXT,
    health TEXT NOT NULL DEFAULT 'green',
    updated_at TEXT NOT NULL
);

-- Task queue
CREATE TABLE beads (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    task_type TEXT NOT NULL,
    estimated_tokens INTEGER NOT NULL,
    priority INTEGER NOT NULL DEFAULT 2,
    preferred_provider TEXT,
    status TEXT NOT NULL DEFAULT 'pending',  -- pending, assigned, deferred, completed
    assigned_to TEXT,
    deferred_until TEXT,
    created_at TEXT NOT NULL,
    completed_at TEXT
);

-- Execution history
CREATE TABLE completions (
    id TEXT PRIMARY KEY,
    bead_id TEXT NOT NULL REFERENCES beads(id),
    provider TEXT NOT NULL,
    tokens_used INTEGER NOT NULL,
    duration_ms INTEGER NOT NULL,
    success INTEGER NOT NULL,
    error_message TEXT,
    completed_at TEXT NOT NULL
);

-- Deferred beads index
CREATE INDEX idx_beads_deferred ON beads(deferred_until) 
    WHERE status = 'deferred';

-- Pending beads by priority
CREATE INDEX idx_beads_pending ON beads(priority, created_at) 
    WHERE status = 'pending';
```

### 8.2 Git Integration

Like Gas Town, we version state in git:

```bash
# Auto-commit on state changes
rigs commit "Completed bead gt-abc12"

# Time travel for debugging
rigs history --bead gt-abc12

# Recover from bad state
rigs restore --to 2h-ago
```

---

## 9. CLI Design

### 9.1 Commands (Gas Town Compatible)

```bash
# Initialize rigs workspace
rigs init ~/rigs --git

# Configure providers
rigs provider add claude --config ~/.claude/config.json
rigs provider add codex --credentials ~/.codex/credentials
rigs provider add gemini --api-key $GEMINI_API_KEY

# Tank (rate limit) management
rigs tank list                    # Show all provider capacities
rigs tank status claude           # Detailed Claude status
rigs tank refresh                 # Force refresh all limits

# Bead (task) management
rigs bead create "Implement auth system" --type implementation --priority high
rigs bead list --status pending
rigs bead show gt-abc12

# Convoy (batch) management  
rigs convoy create "Sprint 42" gt-abc12 gt-def34 gt-ghi56
rigs convoy list
rigs convoy status sprint-42

# Dispatch (routing)
rigs dispatch --strategy balanced  # Run the routing loop
rigs dispatch --dry-run           # Preview routing decisions

# Foreman (orchestrator)
rigs foreman start                # Start daemon
rigs foreman stop
rigs foreman status

# Integration with Gas Town
rigs gt-import                    # Import beads from Gas Town
rigs gt-sync                      # Bidirectional sync
```

### 9.2 Interactive Mode

```bash
$ rigs foreman attach

🛢️  RIGS Foreman v0.1.0
   Claude: ████████░░ 78% (2,450 tokens remaining)
   Codex:  ██████████ 95% (4,200 tokens remaining)  
   Gemini: ███░░░░░░░ 28% (280k tokens remaining)

Pending: 5 beads | In Progress: 2 | Deferred: 1

> bead create "Fix the auth bug in login.rs" --type implementation
Created bead gt-x7k2m (est. 1,200 tokens)
Routing to Claude (optimal for implementation, 78% capacity)...
Assigned to Claude Polecat

> tank status
Provider  │ Health │ Capacity │ Resets In
──────────┼────────┼──────────┼──────────
Claude    │ 🟡     │ 78%      │ 3h 42m
Codex     │ 🟢     │ 95%      │ 4h 58m
Gemini    │ 🔴     │ 28%      │ 18h 12m

> convoy list
ID        │ Beads │ Progress │ Providers Used
──────────┼───────┼──────────┼───────────────
sprint-42 │ 3/5   │ 60%      │ Claude, Codex
```

---

## 10. Integration Points

### 10.1 ccusage Integration (Claude)

```rust
async fn refresh_claude_tank() -> Result<TankState> {
    let output = Command::new("ccusage")
        .args(["blocks", "--json"])
        .output()
        .await?;
    
    let usage: CcusageOutput = serde_json::from_slice(&output.stdout)?;
    
    Ok(TankState {
        tokens_remaining: usage.remaining_tokens,
        window_ends_at: usage.window_reset_time,
        health: calculate_health(usage.remaining_tokens, usage.total_tokens),
    })
}
```

### 10.2 Codex CLI Integration

```rust
async fn refresh_codex_tank() -> Result<TankState> {
    // Codex exposes limits in API response headers
    // or via the codex CLI status command
    let output = Command::new("codex")
        .args(["status", "--json"])
        .output()
        .await?;
    
    // Parse and return
}
```

### 10.3 Gas Town Sync

```rust
async fn sync_with_gastown(gt_path: &Path) -> Result<()> {
    // Read Gas Town beads
    let gt_beads = read_gastown_beads(gt_path).await?;
    
    // Import new beads that don't exist in Rigs
    for bead in gt_beads {
        if !self.depot.contains(&bead.id) {
            self.depot.insert(bead.into());
        }
    }
    
    // Export completed beads back to Gas Town
    for completed in self.depot.get_completed() {
        write_gastown_completion(gt_path, &completed).await?;
    }
    
    Ok(())
}
```

---

## 11. Monitoring & Observability

### 11.1 Metrics to Track

```rust
struct RigsMetrics {
    // Rate limit metrics
    tokens_consumed_total: Counter,
    tokens_remaining_gauge: Gauge,
    rate_limit_hits_total: Counter,
    deferrals_total: Counter,
    
    // Routing metrics
    routing_decisions_total: Counter,
    routing_latency_histogram: Histogram,
    provider_utilization_gauge: Gauge,
    
    // Task metrics
    beads_completed_total: Counter,
    bead_duration_histogram: Histogram,
    queue_depth_gauge: Gauge,
}
```

### 11.2 Dashboard (Optional Bun UI)

```
┌─────────────────────────────────────────────────────────────────┐
│  RIGS Dashboard                                    [Auto-refresh]│
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Provider Health                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Claude  [████████░░] 78%  │ Resets in 3h 42m            │  │
│  │ Codex   [██████████] 95%  │ Resets in 4h 58m            │  │
│  │ Gemini  [███░░░░░░░] 28%  │ Resets in 18h 12m           │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│  Queue Status                                                    │
│  ┌────────────────┬────────────────┬────────────────┐          │
│  │ Pending: 5     │ In Progress: 2 │ Deferred: 1    │          │
│  └────────────────┴────────────────┴────────────────┘          │
│                                                                  │
│  Recent Activity                                                 │
│  ─────────────────────────────────────────────────────────────  │
│  14:32  gt-x7k2m  Completed  Claude   1,180 tokens   2m 34s    │
│  14:28  gt-p9n4q  Assigned   Codex    (in progress)            │
│  14:25  gt-abc12  Deferred   -        Reset: 18:00             │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 12. Implementation Roadmap

### Phase 1: Core (Week 1-2)
- [ ] Rust CLI skeleton with clap
- [ ] SQLite schema and migrations
- [ ] Token bucket implementation
- [ ] Basic depot (queue) operations
- [ ] ccusage integration for Claude

### Phase 2: Routing (Week 2-3)
- [ ] Dispatch routing logic
- [ ] Provider affinity configuration
- [ ] Proactive deferral system
- [ ] Codex CLI integration
- [ ] Gemini API integration

### Phase 3: Orchestration (Week 3-4)
- [ ] Foreman daemon mode
- [ ] Polecat workers (Claude, Codex, Gemini)
- [ ] Git persistence layer
- [ ] Gas Town import/export

### Phase 4: Polish (Week 4+)
- [ ] Interactive TUI
- [ ] Metrics and observability
- [ ] Bun dashboard (optional)
- [ ] Documentation
- [ ] Tests

---

## 13. Configuration Example

```toml
# ~/.rigs/config.toml

[general]
workspace = "~/rigs"
strategy = "balanced"  # conservative | balanced | aggressive
log_level = "info"

[providers.claude]
enabled = true
config_path = "~/.claude/config.json"
threshold_yellow = 0.50
threshold_red = 0.20
preferred_for = ["implementation", "architecture"]

[providers.codex]
enabled = true
credentials_path = "~/.codex/credentials"
threshold_yellow = 0.40
threshold_red = 0.15
preferred_for = ["review", "test"]

[providers.gemini]
enabled = true
api_key_env = "GEMINI_API_KEY"
threshold_yellow = 0.30
threshold_red = 0.10
preferred_for = ["research"]

[routing]
# Fallback chain when preferred provider exhausted
fallback_order = ["codex", "claude", "gemini"]

# Task type to provider affinity weights
[routing.affinity]
implementation = { claude = 1.0, codex = 0.6, gemini = 0.4 }
review = { claude = 0.7, codex = 1.0, gemini = 0.5 }
research = { claude = 0.6, codex = 0.5, gemini = 1.0 }
refactor = { claude = 0.8, codex = 0.8, gemini = 0.5 }

[gastown]
enabled = true
path = "~/gt"
sync_interval = "5m"
```

---

## 14. The Assayer: LLM-Powered Pre-Optimization

This is where it gets clever. Before burning precious tokens on Claude/Codex/Gemini, we use **cheap or local LLMs** to optimize the work. This doesn't count against your Pro-tier rate limits.

### 14.1 The Assayer Component

In mining terminology, an **Assayer** tests ore samples to determine their value before committing to full extraction. Our Assayer does the same — it evaluates and refines beads before they consume rate-limited tokens.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         RIGS OPTIMIZATION LAYER                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐              │
│  │   ASSAYER    │    │   ASSAYER    │    │   ASSAYER    │              │
│  │  (Planner)   │───▶│ (Optimizer)  │───▶│  (Estimator) │              │
│  └──────────────┘    └──────────────┘    └──────────────┘              │
│         │                   │                   │                        │
│         │    LOCAL / CHEAP LLM CALLS ONLY      │                        │
│         │    (Ollama, Groq, DeepSeek-R1)       │                        │
│         │                   │                   │                        │
│         └───────────────────┴───────────────────┘                        │
│                             │                                            │
│                    ┌────────▼────────┐                                   │
│                    │  OPTIMIZED BEAD │                                   │
│                    └────────┬────────┘                                   │
│                             │                                            │
│                             ▼                                            │
│                    ┌────────────────┐                                    │
│                    │    FOREMAN     │                                    │
│                    │ (Routes to Pro │                                    │
│                    │   providers)   │                                    │
│                    └────────────────┘                                    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 14.2 Three Assayer Functions

#### Function 1: Planner (Task Decomposition)

Takes high-level goals and decomposes them into well-structured beads.

```rust
struct PlannerAssayer {
    model: LocalLLM,  // Ollama (llama3.2), Groq (llama-3.3-70b), or DeepSeek-R1
}

impl PlannerAssayer {
    async fn decompose(&self, goal: &str) -> Vec<Bead> {
        let prompt = format!(r#"
You are a task planner for a software development team.

GOAL: {goal}

Decompose this goal into 3-7 concrete, actionable tasks.
Each task should be:
- Completable in one focused session (< 30 min of LLM work)
- Have clear acceptance criteria
- Be assignable to one of: implementation, review, research, refactor, test, documentation

Output as JSON array:
[
  {{
    "title": "Short descriptive title",
    "description": "Detailed description with context and requirements",
    "task_type": "implementation|review|research|refactor|test|documentation",
    "acceptance_criteria": ["criterion 1", "criterion 2"],
    "dependencies": ["bead-id if depends on another task"],
    "estimated_complexity": "low|medium|high"
  }}
]
"#);

        let response = self.model.complete(&prompt).await?;
        parse_beads_from_json(&response)
    }
}
```

**Example:**

Input goal:
> "Add OAuth2 authentication to the API with Google and GitHub providers"

Decomposed beads:
1. **Research OAuth2 libraries** (research) → Find best Rust OAuth2 crate
2. **Design auth flow** (documentation) → Document the token flow
3. **Implement OAuth2 client** (implementation) → Core OAuth2 logic
4. **Add Google provider** (implementation) → Google-specific config
5. **Add GitHub provider** (implementation) → GitHub-specific config
6. **Write integration tests** (test) → Test both providers
7. **Code review** (review) → Review the implementation

#### Function 2: Optimizer (Prompt Refinement)

Uses DSPy-style optimization to improve prompts before sending to expensive models.

```rust
struct OptimizerAssayer {
    model: LocalLLM,
    optimization_history: Vec<OptimizationTrace>,  // Learn from past successes
}

impl OptimizerAssayer {
    async fn optimize_prompt(&self, bead: &Bead) -> OptimizedPrompt {
        // Step 1: Analyze the task
        let analysis = self.analyze_task(&bead).await?;
        
        // Step 2: Apply task-type-specific templates
        let template = self.get_template_for_task_type(&bead.task_type);
        
        // Step 3: Inject successful examples (few-shot learning)
        let examples = self.get_similar_successful_examples(&bead).await?;
        
        // Step 4: Refine the prompt using the cheap LLM
        let refined = self.refine_with_llm(&bead, &template, &examples).await?;
        
        // Step 5: Estimate token usage after optimization
        let estimated_tokens = self.estimate_tokens(&refined);
        
        OptimizedPrompt {
            original: bead.description.clone(),
            optimized: refined,
            estimated_input_tokens: estimated_tokens.input,
            estimated_output_tokens: estimated_tokens.output,
            optimization_notes: analysis.notes,
        }
    }
    
    async fn refine_with_llm(&self, bead: &Bead, template: &str, examples: &[Example]) -> String {
        let prompt = format!(r#"
You are a prompt engineer optimizing instructions for an AI coding assistant.

ORIGINAL TASK:
{description}

TASK TYPE: {task_type}

TEMPLATE FOR THIS TASK TYPE:
{template}

SUCCESSFUL EXAMPLES OF SIMILAR TASKS:
{examples}

Rewrite the task description to be:
1. Clear and unambiguous
2. Include relevant context the AI needs
3. Specify expected output format
4. Include edge cases to consider
5. Be concise (minimize tokens while maximizing clarity)

OPTIMIZED PROMPT:
"#, 
            description = bead.description,
            task_type = bead.task_type,
            template = template,
            examples = format_examples(examples),
        );

        self.model.complete(&prompt).await
    }
}
```

**DSPy-Style Patterns Applied:**

| Pattern | Implementation |
|---------|----------------|
| **COPRO** | Iteratively refine prompts with coordinate ascent |
| **BootstrapFewShot** | Include successful task completions as examples |
| **MIPROv2** | Data-aware instruction generation |
| **SIMBA** | Analyze failures to generate self-reflective rules |

#### Function 3: Estimator (Token Cost Prediction)

Accurately predicts token costs to enable smarter routing decisions.

```rust
struct EstimatorAssayer {
    model: LocalLLM,
    historical_data: HashMap<TaskType, TokenStats>,
}

impl EstimatorAssayer {
    async fn estimate(&self, bead: &Bead, optimized_prompt: &str) -> TokenEstimate {
        // Method 1: Character-based heuristic
        let input_chars = optimized_prompt.len();
        let heuristic_input = input_chars / 4;  // ~4 chars per token
        
        // Method 2: Historical average for task type
        let historical = self.historical_data
            .get(&bead.task_type)
            .map(|stats| stats.average_output_tokens)
            .unwrap_or(500);
        
        // Method 3: LLM-based estimation (more accurate but costs a local call)
        let llm_estimate = self.estimate_with_llm(bead, optimized_prompt).await?;
        
        // Combine estimates with confidence weighting
        TokenEstimate {
            input_tokens: heuristic_input,
            output_tokens: (historical + llm_estimate) / 2,
            confidence: calculate_confidence(&bead, &self.historical_data),
            breakdown: TokenBreakdown {
                prompt_template: 200,
                task_description: heuristic_input - 200,
                expected_code: llm_estimate * 0.7,
                expected_explanation: llm_estimate * 0.3,
            }
        }
    }
}
```

### 14.3 Local/Cheap LLM Options

The key is these models are **free or very cheap** compared to your Pro subscriptions:

| Provider | Model | Cost | Best For |
|----------|-------|------|----------|
| **Ollama** (local) | llama3.2:3b | Free | Quick planning, estimation |
| **Ollama** (local) | deepseek-r1:7b | Free | Complex decomposition |
| **Ollama** (local) | qwen3:8b | Free | Code-aware optimization |
| **Groq** (cloud) | llama-3.3-70b-versatile | Free tier | High-quality planning |
| **Groq** (cloud) | deepseek-r1-distill-llama-70b | Free tier | Strategic reasoning |
| **Together.ai** | Qwen3-30B-Thinking | ~$0.20/M tokens | Thinking-mode planning |

**Recommendation**: Run **Ollama locally** for zero-cost optimization. Deepseek-R1 7B or Qwen3 8B are excellent for task decomposition.

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull models for Rigs
ollama pull deepseek-r1:7b      # Best for planning
ollama pull qwen3:8b            # Best for code tasks
ollama pull llama3.2:3b         # Fast estimation
```

### 14.4 The Optimization Pipeline

```
┌───────────────────────────────────────────────────────────────────────┐
│                          USER INPUT                                    │
│  "Add OAuth2 authentication with Google and GitHub"                   │
└────────────────────────────────┬──────────────────────────────────────┘
                                 │
                                 ▼
┌───────────────────────────────────────────────────────────────────────┐
│  PLANNER ASSAYER (DeepSeek-R1 7B local)                               │
│  ─────────────────────────────────────                                │
│  Decompose into beads:                                                │
│  1. gt-a1: Research OAuth2 libraries (research, est. 800 tokens)      │
│  2. gt-a2: Design auth flow (documentation, est. 1200 tokens)         │
│  3. gt-a3: Implement OAuth2 client (implementation, est. 3000 tokens) │
│  4. gt-a4: Add Google provider (implementation, est. 1500 tokens)     │
│  5. gt-a5: Add GitHub provider (implementation, est. 1500 tokens)     │
│  6. gt-a6: Write integration tests (test, est. 2000 tokens)           │
│  7. gt-a7: Code review (review, est. 1000 tokens)                     │
│                                                                        │
│  Total estimated: ~11,000 tokens                                       │
└────────────────────────────────┬──────────────────────────────────────┘
                                 │
                                 ▼
┌───────────────────────────────────────────────────────────────────────┐
│  OPTIMIZER ASSAYER (Qwen3 8B local)                                   │
│  ─────────────────────────────────                                    │
│  For each bead, optimize the prompt:                                  │
│                                                                        │
│  BEFORE (gt-a3):                                                       │
│  "Implement OAuth2 client"                                             │
│                                                                        │
│  AFTER:                                                                │
│  "Implement an OAuth2 2.0 client in Rust using the `oauth2` crate.    │
│   Requirements:                                                        │
│   - Support authorization code flow with PKCE                          │
│   - Handle token refresh automatically                                 │
│   - Store tokens securely (use keyring crate)                         │
│   - Expose async methods: authorize(), refresh(), get_token()         │
│   Output: Single oauth2_client.rs file with tests                      │
│   Reference: RFC 6749, RFC 7636"                                       │
│                                                                        │
│  Token reduction: 4 tokens → 95 tokens input, but ~40% fewer output   │
│  tokens due to clearer instructions (net savings)                      │
└────────────────────────────────┬──────────────────────────────────────┘
                                 │
                                 ▼
┌───────────────────────────────────────────────────────────────────────┐
│  ESTIMATOR ASSAYER (Llama3.2 3B local)                                │
│  ─────────────────────────────────────                                │
│  Final token estimates per bead:                                       │
│                                                                        │
│  gt-a1: 800 tokens  → Route to Gemini (research)                      │
│  gt-a2: 1200 tokens → Route to Claude (docs)                          │
│  gt-a3: 2800 tokens → Route to Claude (implementation) ← savings!     │
│  gt-a4: 1400 tokens → Route to Claude (implementation)                │
│  gt-a5: 1400 tokens → Route to Codex (similar to gt-a4, good review)  │
│  gt-a6: 1900 tokens → Route to Codex (tests)                          │
│  gt-a7: 900 tokens  → Route to Codex (review)                         │
│                                                                        │
│  Revised total: ~10,400 tokens (5% savings from optimization)         │
└────────────────────────────────┬──────────────────────────────────────┘
                                 │
                                 ▼
┌───────────────────────────────────────────────────────────────────────┐
│  FOREMAN (Routes to Pro providers)                                     │
│  ─────────────────────────────────                                    │
│  Check tank levels:                                                    │
│  - Claude: 15,000 tokens remaining (can handle gt-a2, gt-a3, gt-a4)   │
│  - Codex: 20,000 tokens remaining (can handle gt-a5, gt-a6, gt-a7)    │
│  - Gemini: 500,000 tokens remaining (can handle gt-a1)                │
│                                                                        │
│  Routing plan:                                                         │
│  NOW:     gt-a1 → Gemini (research)                                   │
│  NOW:     gt-a2 → Claude (docs, runs parallel)                        │
│  QUEUED:  gt-a3 → Claude (depends on gt-a2)                           │
│  QUEUED:  gt-a4 → Claude (depends on gt-a3)                           │
│  QUEUED:  gt-a5 → Codex (depends on gt-a3)                            │
│  QUEUED:  gt-a6 → Codex (depends on gt-a4, gt-a5)                     │
│  QUEUED:  gt-a7 → Codex (depends on gt-a3, gt-a4, gt-a5)              │
└───────────────────────────────────────────────────────────────────────┘
```

### 14.5 Quality Gate (Post-Execution Assayer)

After a bead completes, the Assayer can also review the output before marking it done:

```rust
struct QualityGateAssayer {
    model: LocalLLM,
}

impl QualityGateAssayer {
    async fn review(&self, bead: &Bead, output: &str) -> QualityResult {
        let prompt = format!(r#"
You are a code reviewer checking if a task was completed correctly.

TASK: {title}
ACCEPTANCE CRITERIA:
{criteria}

OUTPUT:
{output}

Evaluate:
1. Does the output meet all acceptance criteria? (yes/no for each)
2. Are there obvious bugs or issues?
3. Is the code/documentation complete?

Respond with JSON:
{{
  "criteria_met": {{"criterion1": true, "criterion2": false}},
  "issues": ["issue1", "issue2"],
  "verdict": "pass|fail|needs_revision",
  "revision_suggestions": "..."
}}
"#,
            title = bead.title,
            criteria = bead.acceptance_criteria.join("\n- "),
            output = truncate(output, 4000),  // Keep it cheap
        );

        let response = self.model.complete(&prompt).await?;
        parse_quality_result(&response)
    }
}
```

### 14.6 Learning from History (DSPy-Style Optimization)

The Assayer improves over time by tracking what works:

```rust
struct OptimizationTrace {
    bead_id: BeadId,
    task_type: TaskType,
    original_prompt: String,
    optimized_prompt: String,
    estimated_tokens: u64,
    actual_tokens: u64,
    quality_score: f32,  // 0.0 - 1.0
    timestamp: DateTime<Utc>,
}

impl OptimizerAssayer {
    fn update_from_completion(&mut self, trace: OptimizationTrace) {
        // Store successful optimizations as few-shot examples
        if trace.quality_score > 0.8 {
            self.successful_examples.push(trace.clone());
        }
        
        // Update token estimation model
        self.historical_data
            .entry(trace.task_type)
            .or_default()
            .add_sample(trace.estimated_tokens, trace.actual_tokens);
        
        // If optimization made things worse, learn from it
        if trace.actual_tokens > trace.estimated_tokens * 1.5 {
            self.failure_patterns.push(FailurePattern {
                task_type: trace.task_type,
                prompt_pattern: extract_pattern(&trace.optimized_prompt),
                issue: "Underestimated complexity",
            });
        }
    }
}
```

### 14.7 Cost Analysis

| Component | Model | Tokens/Request | Cost |
|-----------|-------|----------------|------|
| Planner | DeepSeek-R1 7B (local) | ~2,000 | **$0.00** |
| Optimizer | Qwen3 8B (local) | ~1,500 | **$0.00** |
| Estimator | Llama3.2 3B (local) | ~500 | **$0.00** |
| Quality Gate | Llama3.2 3B (local) | ~1,000 | **$0.00** |

**Total pre-optimization cost: $0.00** (if using Ollama locally)

**Or with Groq free tier:**
- 14,400 requests/day
- 6,000 tokens/minute
- More than enough for Rigs optimization

### 14.8 Integration with Rigs Architecture

```rust
struct Rigs {
    // Core components (from earlier)
    refinery: Arc<RwLock<Refinery>>,
    depot: Arc<RwLock<Depot>>,
    dispatch: Dispatch,
    foreman: Foreman,
    polecats: HashMap<Provider, Polecat>,
    
    // NEW: Assayer components
    assayer: Assayer,
}

struct Assayer {
    planner: PlannerAssayer,
    optimizer: OptimizerAssayer,
    estimator: EstimatorAssayer,
    quality_gate: QualityGateAssayer,
    local_model: OllamaClient,  // Or GroqClient
}

impl Rigs {
    async fn process_goal(&self, goal: &str) -> Result<ConvoyId> {
        // Step 1: Decompose goal into beads (FREE - local LLM)
        let beads = self.assayer.planner.decompose(goal).await?;
        
        // Step 2: Optimize each bead (FREE - local LLM)
        let optimized_beads: Vec<OptimizedBead> = futures::future::join_all(
            beads.iter().map(|b| self.assayer.optimizer.optimize(b))
        ).await;
        
        // Step 3: Estimate tokens (FREE - local LLM)
        for bead in &mut optimized_beads {
            bead.estimated_tokens = self.assayer.estimator.estimate(&bead).await?;
        }
        
        // Step 4: Add to depot
        let convoy = self.depot.write().await.create_convoy(optimized_beads);
        
        // Step 5: Foreman routes based on real capacity (USES PRO PROVIDERS)
        self.foreman.schedule_convoy(&convoy).await?;
        
        Ok(convoy.id)
    }
    
    async fn on_bead_complete(&self, bead_id: BeadId, output: &str) -> Result<()> {
        // Quality gate check (FREE - local LLM)
        let quality = self.assayer.quality_gate.review(
            &self.depot.read().await.get(&bead_id)?,
            output
        ).await?;
        
        match quality.verdict {
            Verdict::Pass => {
                self.depot.write().await.mark_complete(bead_id);
            }
            Verdict::NeedsRevision => {
                // Re-queue with revision notes
                self.depot.write().await.requeue_with_notes(
                    bead_id,
                    &quality.revision_suggestions
                );
            }
            Verdict::Fail => {
                // Escalate or try different provider
                self.foreman.escalate(bead_id, &quality.issues).await?;
            }
        }
        
        // Learn from this execution
        self.assayer.optimizer.update_from_completion(...);
        
        Ok(())
    }
}
```

---

## 15. Summary

**Rigs** extends Gas Town with rate-limit-aware orchestration AND LLM-powered pre-optimization:

| Component | Gas Town | Rigs Extension |
|-----------|----------|----------------|
| Mayor | Orchestrator | **Foreman** with rate awareness |
| Rig | Project | Same |
| Bead | Work unit | Same + token estimates |
| Convoy | Batch | Same |
| Sling | Assign | Via **Dispatch** router |
| Polecat | Worker | Per-provider workers |
| - | - | **Refinery** (rate limit tracker) |
| - | - | **Depot** (priority queue) |
| - | - | **Tank** (per-provider capacity) |
| - | - | **Assayer** (LLM pre-optimizer) |

**Key Innovations:**
1. **Proactive deferral** — Don't hit limits, anticipate them
2. **Task-type routing** — Right tool for the job
3. **Capacity-weighted routing** — Balance load across providers
4. **Provider fallback chains** — Graceful degradation
5. **Git-native state** — Survives restarts, enables time travel
6. **🆕 LLM Pre-Optimization (Assayer)** — Use free local LLMs to:
   - Decompose goals into well-structured beads
   - Optimize prompts before sending to rate-limited providers
   - Estimate token costs accurately
   - Quality-gate outputs before marking complete
   - Learn from history (DSPy-style)

**The Assayer Advantage:**

| Without Assayer | With Assayer |
|-----------------|--------------|
| Vague task: "Add auth" | Decomposed: 7 specific beads with criteria |
| Wasted tokens on unclear prompts | Optimized prompts → 20-40% fewer output tokens |
| Guessed token estimates | Accurate estimates → better routing |
| Manual quality review | Automated quality gates |
| No learning | Improves over time |
| **Cost: 100% on Pro providers** | **Cost: 0% optimization + Pro execution** |

**Technology:**
- **Rust + Tokio** for the core daemon (precision, speed)
- **Bun + TypeScript** for plugins and UI (flexibility)
- **SQLite** for persistence (simplicity, portability)
- **Git** for state versioning (reliability, debugging)
- **Ollama** for local LLM optimization (free!)

**Local LLM Setup:**
```bash
# Zero-cost optimization with Ollama
ollama pull deepseek-r1:7b   # Planning
ollama pull qwen3:8b         # Code optimization  
ollama pull llama3.2:3b      # Fast estimation
```

This architecture ensures you can run 20-30 concurrent agents across Claude, Codex, and Gemini without ever hitting a rate limit wall — all while maintaining Gas Town's delightful terminology and adding intelligent pre-optimization that costs you nothing.
