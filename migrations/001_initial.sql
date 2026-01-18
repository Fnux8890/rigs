-- Initial Rigs schema
-- Migration: 001_initial

-- Providers and their rate limit state
CREATE TABLE IF NOT EXISTS tanks (
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

-- Work units (beads)
CREATE TABLE IF NOT EXISTS beads (
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

-- Indexes for bead queries
CREATE INDEX IF NOT EXISTS idx_beads_status ON beads(status);
CREATE INDEX IF NOT EXISTS idx_beads_priority ON beads(priority DESC, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_beads_deferred ON beads(deferred_until) WHERE status = 'deferred';
CREATE INDEX IF NOT EXISTS idx_beads_convoy ON beads(convoy_id);

-- Batches of beads (convoys)
CREATE TABLE IF NOT EXISTS convoys (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    goal TEXT,
    status TEXT NOT NULL DEFAULT 'planning',
    created_at TEXT NOT NULL,
    completed_at TEXT,
    metadata TEXT NOT NULL DEFAULT '{}'
);

-- Execution history
CREATE TABLE IF NOT EXISTS completions (
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

CREATE INDEX IF NOT EXISTS idx_completions_bead ON completions(bead_id);
CREATE INDEX IF NOT EXISTS idx_completions_provider ON completions(provider, completed_at);

-- Assayer optimization traces (for learning)
CREATE TABLE IF NOT EXISTS optimization_traces (
    id TEXT PRIMARY KEY,
    task_type TEXT NOT NULL,
    original_prompt TEXT NOT NULL,
    optimized_prompt TEXT NOT NULL,
    estimated_tokens INTEGER NOT NULL,
    actual_tokens INTEGER,
    quality_score REAL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_traces_task_type ON optimization_traces(task_type, quality_score DESC);

-- Configuration storage
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
