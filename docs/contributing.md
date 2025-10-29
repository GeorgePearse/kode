# Contributing to MARS

We welcome contributions! Here's how you can help.

## Development Setup

### 1. Clone the Repository

```bash
git clone https://github.com/GeorgePearse/code.git
cd code/code-rs/code-mars
```

### 2. Install Dependencies

```bash
rustup update
cargo build
```

### 3. Run Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# All tests with output
cargo test -- --nocapture
```

## Code Structure

```
code-rs/code-mars/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Public API
│   ├── coordinator.rs        # Phase orchestration
│   ├── agent.rs             # Individual agent
│   ├── workspace.rs         # Solution storage
│   ├── verifier.rs          # Verification system
│   ├── aggregator.rs        # Aggregation routing
│   ├── moa.rs               # MOA implementation
│   ├── mcts.rs              # MCTS implementation
│   ├── strategy.rs          # Strategy network
│   ├── types.rs             # Core types
│   ├── config.rs            # Configuration
│   ├── prompts.rs           # Prompt templates
│   └── ...
├── tests/
│   ├── mcts_integration.rs
│   └── multi_provider_integration.rs
├── examples/
│   └── ...
└── README.md
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

Follow Rust conventions and run tests:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --lib
cargo test --test '*'
```

### 3. Commit and Push

```bash
git add .
git commit -m "feat: description of change"
git push origin feature/your-feature-name
```

### 4. Create a Pull Request

Push to GitHub and create a PR with clear description.

## Testing Guidelines

All public functions should have tests. Use:

```rust
#[test]
fn test_feature() {
    let input = setup();
    let result = function_under_test(input);
    assert_eq!(result, expected);
}
```

For async code:

```rust
#[tokio::test]
async fn test_async_feature() {
    // Your test
}
```

## Code Style

- Functions: `snake_case`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`

## Questions?

Check [GitHub Issues](https://github.com/GeorgePearse/code/issues) or [Documentation](../index.md)

Thank you for contributing!
