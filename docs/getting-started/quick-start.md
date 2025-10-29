# Quick Start

Get MARS up and running in 5 minutes.

## Step 1: Install

```bash
# Clone the repo
git clone https://github.com/GeorgePearse/code.git
cd code/code-rs/code-mars

# Verify installation
cargo test --lib
```

## Step 2: Set API Key

```bash
export OPENAI_API_KEY=sk-your-key-here
```

## Step 3: Create Your First Program

Create `examples/solve_math.rs`:

```rust
use code_mars::{MarsCoordinator, MarsConfig};
use code_core::ModelClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = ModelClient::default();

    // Create MARS with default settings
    let config = MarsConfig::default();
    let mut coordinator = MarsCoordinator::new(config);

    // Solve a problem
    let query = "What is the square root of 144?";
    let result = coordinator.run_with_client(query, &client).await?;

    println!("✓ Answer: {}", result.answer);
    Ok(())
}
```

## Step 4: Run It

```bash
cargo run --example solve_math
```

## Common Examples

### Example 1: Basic Question

```rust
let query = "What is 25 * 4?";
let result = coordinator.run_with_client(query, &client).await?;
println!("Answer: {}", result.answer);
```

### Example 2: With Advanced Features

```rust
let config = MarsConfig::default()
    .with_advanced_features()
    .with_moa_aggregation()
    .with_num_agents(4)
    .with_max_iterations(7);

let mut coordinator = MarsCoordinator::new(config);
let result = coordinator.run_with_client(query, &client).await?;
```

### Example 3: Using MCTS for Deep Reasoning

```rust
use code_mars::types::AggregationMethod;

let config = MarsConfig::new()
    .with_advanced_features()
    .with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)
    .with_mcts_simulation_depth(2)
    .with_mcts_exploration_weight(0.3);

let mut coordinator = MarsCoordinator::new(config);
let result = coordinator.run_with_client(query, &client).await?;
```

### Example 4: Lightweight Mode (Faster)

```rust
let config = MarsConfig::default().lightweight();
let mut coordinator = MarsCoordinator::new(config);
let result = coordinator.run_with_client(query, &client).await?;
```

### Example 5: Debug Output

```rust
let config = MarsConfig::default()
    .with_debug(true);

let mut coordinator = MarsCoordinator::new(config);
// Will print detailed logs
let result = coordinator.run_with_client(query, &client).await?;
```

## Understanding the Output

```rust
let result = coordinator.run_with_client(query, &client).await?;

// Access the result
println!("Answer: {}", result.answer);                      // Final answer
println!("Method: {:?}", result.selection_method);          // How it was selected
println!("Tokens: {}", result.total_tokens);                // Total tokens used
println!("Iterations: {}", result.iterations);              // Improvement iterations
println!("Solution ID: {}", result.final_solution_id);      // Which solution won
println!("All solutions: {}", result.all_solutions.len());  // Total generated
```

## Configuration Cheat Sheet

### Temperature Control
```rust
.with_temperatures(vec![0.2, 0.5, 0.9])  // Custom temps
```

### Agent Count
```rust
.with_num_agents(4)  // Default: 3
```

### Aggregation
```rust
.with_moa_aggregation()                        // Horizontal diversity
.with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)  // Vertical depth
```

### Iteration Control
```rust
.with_max_iterations(10)  // Default: 5
```

### Token Budget
```rust
.get_token_budget(false)  // For complex reasoning: 64K
.get_token_budget(true)   // For lightweight: 4K
```

### Debug
```rust
.with_debug(true)
```

## Monitoring Progress

Use event streaming to track MARS progress:

```rust
use tokio::sync::mpsc;
use code_mars::types::MarsEvent;

let (tx, mut rx) = mpsc::channel::<MarsEvent>(100);

// Run MARS in background
tokio::spawn(async move {
    // coordinator.run_with_events(query, &client, tx).await
});

// Listen to events
while let Some(event) = rx.recv().await {
    match event {
        MarsEvent::ExplorationStarted { num_agents } => {
            println!("Starting with {} agents", num_agents);
        }
        MarsEvent::SolutionGenerated { solution_id, .. } => {
            println!("Generated solution: {}", solution_id);
        }
        MarsEvent::SolutionVerified { solution_id, is_correct, .. } => {
            println!("Verified {}: {}", solution_id, is_correct);
        }
        MarsEvent::AnswerSynthesized { answer } => {
            println!("Final answer: {}", answer);
        }
        _ => {}
    }
}
```

## Next Steps

1. **Run more examples** - Check `code-rs/code-mars/examples/`
2. **Learn configuration** - Read [Configuration Guide](../configuration/config.md)
3. **Understand architecture** - See [Architecture Overview](../architecture/overview.md)
4. **Explore aggregation methods** - Learn [MOA](../aggregation/moa.md), [MCTS](../aggregation/mcts.md)
5. **API reference** - Check [Core Types](../api/types.md)

## Troubleshooting

### Getting Timeout Errors?
```rust
let config = MarsConfig::default()
    .with_timeout_seconds(600);  // Increase timeout
```

### Token Usage Too High?
```rust
let config = MarsConfig::default()
    .lightweight();  // Use fewer agents and iterations
```

### Need Custom LLM Provider?
```rust
// Implement LLMProvider trait
struct MyCustomProvider;

#[async_trait::async_trait]
impl LLMProvider for MyCustomProvider {
    async fn complete(&self, prompt: &str, system: Option<&str>) -> Result<String> {
        // Your implementation
    }
    // ... implement other methods
}

let result = coordinator.run_with_provider(query, &my_provider).await?;
```

## Common Tasks

### Verify a Math Solution
```rust
let config = MarsConfig::default()
    .with_num_agents(4)
    .with_consensus_threshold(3);
let query = "Is 2^10 = 1024? Prove it.";
```

### Generate Code
```rust
let config = MarsConfig::default()
    .with_moa_aggregation()
    .with_moa_num_completions(5);
let query = "Write a Rust function that checks if a number is prime";
```

### Complex Logic Problem
```rust
let config = MarsConfig::default()
    .with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)
    .with_advanced_features();
let query = "Solve this logic puzzle: ...";
```

## Full Example Program

```rust
use code_mars::{MarsCoordinator, MarsConfig};
use code_core::ModelClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create client
    let client = ModelClient::default();

    // 2. Configure MARS
    let config = MarsConfig::default()
        .with_advanced_features()
        .with_num_agents(3)
        .with_max_iterations(5)
        .with_debug(true);

    // 3. Create coordinator
    let mut coordinator = MarsCoordinator::new(config);

    // 4. Ask question
    let query = "What is the derivative of x^3?";

    // 5. Get answer
    let result = coordinator.run_with_client(query, &client).await?;

    // 6. Display results
    println!("Question: {}", query);
    println!("Answer: {}", result.answer);
    println!("Selected via: {:?}", result.selection_method);
    println!("Tokens used: {}", result.total_tokens);
    println!("Iterations: {}", result.iterations);

    Ok(())
}
```

---

**Ready to go deeper?** Continue with [Configuration Guide](../configuration/config.md) or [Architecture Details](../architecture/overview.md).
