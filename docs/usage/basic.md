# Basic Usage

Get started with MARS in minutes.

## Simple Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create client
    let client = ModelClient::default();

    // 2. Create coordinator
    let config = MarsConfig::default();
    let mut coordinator = MarsCoordinator::new(config);

    // 3. Ask a question
    let query = "What is 2 + 2?";

    // 4. Get answer
    let result = coordinator.run_with_client(query, &client).await?;

    // 5. Display result
    println!("Answer: {}", result.answer);

    Ok(())
}
```

## With Advanced Features

```rust
let config = MarsConfig::default()
    .with_advanced_features()
    .with_moa_aggregation()
    .with_num_agents(4);

let mut coordinator = MarsCoordinator::new(config);
let result = coordinator.run_with_client(query, &client).await?;

println!("Answer: {}", result.answer);
println!("Reasoning: {}", result.reasoning);
println!("Method: {:?}", result.selection_method);
println!("Tokens: {}", result.total_tokens);
```

See [Advanced Examples](advanced.md) for more patterns.
