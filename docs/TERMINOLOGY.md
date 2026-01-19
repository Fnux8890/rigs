# Rigs Terminology

Rigs uses a "Gas Town" inspired naming convention. This document maps terms to their meanings.

## Core Components

| Term | Description | Analogy |
|------|-------------|---------|
| **Foreman** | Main orchestration loop that coordinates all work | Factory floor manager |
| **Polecat** | Worker that executes tasks on a specific provider | Individual worker |
| **Tank** | Rate limit state for a single provider | Fuel tank gauge |
| **Refinery** | Manages all tanks, refreshes rate limit data | Fuel depot |
| **Depot** | Priority queue for pending beads | Loading dock |
| **Dispatch** | Routes beads to providers based on capacity/affinity | Traffic controller |
| **Assayer** | Optimizes prompts using local LLMs | Quality inspector |

## Work Units

| Term | Description |
|------|-------------|
| **Bead** | Single unit of work (a task/prompt for an LLM) |
| **Convoy** | Batch of related beads (from goal decomposition) |
| **Goal** | High-level objective decomposed into beads |

## Bead Lifecycle

```
Pending → Optimizing → Queued → Assigned → InProgress → Reviewing → Completed
                  ↓                              ↓
              Deferred ←────────────────── (rate limit)
                  ↓
               Failed/Cancelled
```

## Tank Health States

| State | Meaning | Action |
|-------|---------|--------|
| **Green** | >50% capacity | Normal operation |
| **Yellow** | 20-50% capacity | Consider deferring non-urgent |
| **Red** | <20% capacity | Defer all but critical |
| **Empty** | 0% capacity | Locked until reset |

## Providers

| Provider | Purpose | Rate Model |
|----------|---------|------------|
| **Claude** | Complex implementation, design | 5-hour rolling window |
| **Codex** | Code review, debugging, tests | 5-hour rolling window |
| **Gemini** | Research, documentation | Daily limit |
| **DeepSeek** | Assayer (cheap API) | Pay-as-you-go |
| **Ollama** | Assayer (free local) | Unlimited |

## Task Types & Provider Affinity

| Task Type | Best Provider | Secondary | Fallback |
|-----------|--------------|-----------|----------|
| Implementation | Claude | Codex | Gemini |
| Review | Codex | Claude | Gemini |
| Research | Gemini | Claude | - |
| Refactor | Claude | Codex | - |
| Test | Codex | Claude | Gemini |
| Documentation | Claude | Gemini | Codex |
| Debug | Codex | Claude | - |
| Design | Claude | Gemini | - |

## Assayer Pipeline

The Assayer uses local/cheap LLMs to optimize work before sending to expensive providers:

1. **Planner** - Decomposes goals into structured beads
2. **Optimizer** - Refines prompts for clarity and token efficiency
3. **Estimator** - Predicts token usage for routing decisions
4. **Quality Gate** - Reviews outputs against acceptance criteria

## Routing Strategies

| Strategy | Behavior |
|----------|----------|
| **Conservative** | Defer early, preserve capacity for critical work |
| **Balanced** | Default, good mix of throughput and preservation |
| **Aggressive** | Use capacity fully, defer only when empty |

## ID Formats

- **BeadId**: `gt-xxxxx` (e.g., `gt-abc12`)
- **ConvoyId**: UUID (e.g., `550e8400-e29b-41d4-a716-446655440000`)
