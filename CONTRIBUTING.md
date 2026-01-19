# Contributing to Rigs

Thank you for considering contributing to Rigs!

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rigs.git
   cd rigs
   ```
3. Install dependencies:
   ```bash
   cargo build
   ```

## Development Setup

### Prerequisites

- Rust 1.75+ (`rustup update stable`)
- SQLite
- [Ollama](https://ollama.com) (optional, for testing Assayer)

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- --help
cargo run -- status
```

### Code Style

We use standard Rust formatting:

```bash
# Format code
cargo fmt

# Check lints
cargo clippy -- -D warnings
```

## Making Changes

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the code style

3. Add tests for new functionality

4. Ensure all tests pass:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. Commit with a clear message:
   ```bash
   git commit -m "Add feature: description of change"
   ```

6. Push and create a Pull Request

## Project Structure

```
src/
├── main.rs           # CLI entry point
├── config.rs         # Configuration loading
├── cli/              # CLI command implementations
├── core/             # Core types (Bead, Tank, etc.)
├── db/               # Database layer
├── refinery/         # Rate limit tracking (TODO)
├── depot/            # Task queue (TODO)
├── dispatch/         # Routing engine (TODO)
├── assayer/          # Prompt optimization (TODO)
└── foreman/          # Orchestration loop (TODO)
```

## Issue Labels

- `iteration-N`: Which development iteration
- `type:implementation`: Code implementation
- `type:setup`: Project setup
- `type:test`: Testing
- `type:documentation`: Documentation
- `priority:critical/high/medium/low`: Priority level

## Commit Message Format

```
<type>: <description>

[optional body]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance

## Questions?

Open an issue with the `question` label.
