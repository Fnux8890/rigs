# Rigs 🛠️

> Rate-limit-aware multi-agent LLM orchestration

Rigs is a command-line tool for intelligently orchestrating work across multiple LLM providers (Claude, Codex, Gemini) while respecting rate limits and optimizing costs.

## Features

- **Multi-Provider Orchestration**: Route tasks to Claude, Codex, or Gemini based on task type and current capacity
- **Rate Limit Awareness**: Track token usage across providers, defer work when limits are hit
- **Intelligent Routing**: Automatically choose the best provider for each task type
- **Goal Decomposition**: Break high-level goals into structured tasks using local LLMs
- **Prompt Optimization**: Optimize prompts before execution to reduce token usage
- **Quality Gates**: Review outputs before marking complete

## Installation

### From Source

```bash
git clone https://github.com/yourusername/rigs.git
cd rigs
cargo install --path .
```

### Prerequisites

- Rust 1.75+ 
- SQLite
- [Ollama](https://ollama.com) (optional, for local LLM optimization)

## Quick Start

```bash
# Initialize workspace
rigs init

# Check provider status
rigs tank list

# Create a goal (decompose and execute)
rigs goal execute "Add user authentication with OAuth2"

# Monitor progress
rigs status
```

## Architecture

Rigs uses a "Gas Town" inspired architecture:

```
┌─────────────────────────────────────────────────────────┐
│                       FOREMAN                            │
│              (Orchestration Loop)                        │
└─────────────────────┬───────────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
        ▼             ▼             ▼
   ┌─────────┐   ┌─────────┐   ┌─────────┐
   │ POLECAT │   │ POLECAT │   │ POLECAT │
   │ (Claude)│   │ (Codex) │   │ (Gemini)│
   └─────────┘   └─────────┘   └─────────┘
```

### Components

- **Tank**: Tracks rate limit state for each provider
- **Refinery**: Manages all tanks, refreshes from provider APIs
- **Depot**: Priority queue for pending beads
- **Dispatch**: Routes beads to providers based on affinity and capacity
- **Assayer**: Optimizes prompts using local LLMs (FREE!)
- **Foreman**: Main orchestration loop
- **Polecat**: Worker that executes beads on a provider

## Configuration

```toml
# ~/.rigs/config.toml

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
model = "gemini-2.5-pro"
api_key_env = "GEMINI_API_KEY"

[providers.deepseek]
enabled = true
model = "deepseek-chat"
api_key_env = "DEEPSEEK_API_KEY"

[assayer]
planner_model = "deepseek-r1:7b"
optimizer_model = "qwen3:8b"
estimator_model = "llama3.2:3b"
use_ollama = true

[routing]
strategy = "balanced"  # conservative, balanced, aggressive
```

## Commands

```bash
# Initialization
rigs init [--git]              # Initialize workspace

# Provider Management
rigs provider list             # List configured providers
rigs provider add <name>       # Add a provider
rigs provider test <name>      # Test connectivity

# Tank Management
rigs tank list                 # Show all tank statuses
rigs tank status <provider>    # Detailed provider status
rigs tank refresh              # Force refresh all

# Bead Management
rigs bead create <desc>        # Create a task
rigs bead list [--status X]    # List tasks
rigs bead show <id>            # Show task details

# Convoy Management
rigs convoy list               # List batches
rigs convoy show <id>          # Show batch progress

# Goal Processing
rigs goal plan "<goal>"        # Decompose goal (dry run)
rigs goal execute "<goal>"     # Decompose and execute

# Foreman Control
rigs foreman start             # Start daemon
rigs foreman stop              # Stop daemon
rigs foreman status            # Show status
rigs foreman attach            # Interactive TUI

# Status
rigs status                    # Show system overview
```

## Cost Optimization

Rigs uses a two-tier approach to minimize costs:

1. **Assayer Layer (FREE)**: Uses local Ollama models or cheap DeepSeek API
   - Planning: DeepSeek-R1 7B
   - Optimization: Qwen3 8B
   - Estimation: Llama3.2 3B

2. **Execution Layer (Paid)**: Uses your subscription tokens
   - Claude Max 5x: Complex implementation
   - Codex: Code review
   - Gemini: Research

Estimated savings: 20-40% token reduction through optimization.

## Development

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=rigs=debug cargo run -- status

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

MIT License - see [LICENSE](LICENSE)

## Acknowledgments

- Inspired by the [Gas Town](https://github.com/anthropics/anthropic-cookbook) concept
- Built for use with Claude Code, Codex, and Gemini
