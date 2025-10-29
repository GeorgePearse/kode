# Installation

## Prerequisites

- **Rust 1.75+** - [Install Rust](https://rustup.rs/)
- **Cargo** - Comes with Rust
- **An LLM API** - OpenAI, Anthropic, or any provider with MARS support

## Adding to Your Project

### Via Cargo.toml

Add MARS to your `Cargo.toml`:

```toml
[dependencies]
code-mars = { path = "../code-rs/code-mars" }
code-core = { path = "../code-rs/code-core" }  # For ModelClient
tokio = { version = "1", features = ["full"] }
```

Or if using from a published crate:

```toml
[dependencies]
code-mars = "0.1.0"
code-core = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Setup

### 1. Create a Rust Project

```bash
cargo new my_mars_project
cd my_mars_project
```

### 2. Add Dependencies

```bash
cargo add code-mars code-core tokio --features tokio/full
```

### 3. Create Your First Program

Create `src/main.rs`:

```rust
use code_mars::{MarsCoordinator, MarsConfig};
use code_core::ModelClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a ModelClient (uses OPENAI_API_KEY env var)
    let client = ModelClient::default();

    // Create MARS coordinator with default settings
    let config = MarsConfig::default();
    let mut coordinator = MarsCoordinator::new(config);

    // Ask a question
    let query = "Solve: If 3x + 5 = 20, what is x?";
    let result = coordinator.run_with_client(query, &client).await?;

    println!("Question: {}", query);
    println!("Answer: {}", result.answer);
    println!("Method: {:?}", result.selection_method);
    println!("Tokens used: {}", result.total_tokens);

    Ok(())
}
```

### 4. Set Your API Key

```bash
export OPENAI_API_KEY=sk-...
```

### 5. Run It

```bash
cargo run --release
```

## Environment Setup

### OpenAI API

```bash
export OPENAI_API_KEY=sk-your-key-here
```

### Multi-Model Setup (litellm-rs)

```bash
# Configure provider routing
export MARS_PROVIDER_ROUTING='{
  "default_provider": "openai",
  "providers": {
    "openai": {
      "model": "gpt-4",
      "base_url": "https://api.openai.com/v1"
    }
  }
}'
```

### Debug Mode

```bash
export MARS_DEBUG=1
```

## Verifying Installation

Run the test suite to verify everything is working:

```bash
cd code-rs/code-mars
cargo test --lib
cargo test --test '*'
```

Expected output:
```
test result: ok. 43 passed; 0 failed; 2 ignored
running 29 tests
test result: ok. 29 passed; 0 failed
```

## Troubleshooting

### `Error: could not find crate 'code_mars'`

Make sure you're using the correct path or published version in `Cargo.toml`.

### `Error: OPENAI_API_KEY not set`

Set your API key:
```bash
export OPENAI_API_KEY=sk-...
```

### `Timeout errors`

Increase the timeout in configuration:
```rust
let config = MarsConfig::default();
// Timeout is 300 seconds by default
```

### `Out of memory`

MARS can use significant memory with many iterations and solutions. Reduce:
- `max_iterations` (default: 5)
- `num_agents` (default: 3)
- Use `lightweight()` config

## Next Steps

- [Quick Start Guide](quick-start.md)
- [Basic Usage Examples](../usage/basic.md)
- [Advanced Configuration](../configuration/advanced.md)

## Support

For issues or questions:
1. Check [Troubleshooting](#troubleshooting) above
2. Review [API Reference](../api/types.md)
3. Look at [Examples](../usage/basic.md)
4. Open an issue on [GitHub](https://github.com/GeorgePearse/code/issues)
