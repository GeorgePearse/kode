# Configuration Guide

MARS provides comprehensive configuration via the fluent builder pattern.

## Default Configuration

```rust
let config = MarsConfig::default();
// num_agents: 3
// temperatures: [0.3, 0.6, 1.0]
// max_iterations: 5
// enable_aggregation: false
// enable_strategy_network: false
// timeout_seconds: 300
```

## Builder Pattern

Configure MARS fluently:

```rust
let config = MarsConfig::new()
    .with_num_agents(4)
    .with_max_iterations(7)
    .with_advanced_features()
    .with_moa_aggregation()
    .with_debug(true);
```

## Configuration Options

### Exploration Phase

```rust
.with_num_agents(n)                    // Number of agents (default: 3)
.with_temperatures(vec![...])          // Temperature values (default: [0.3, 0.6, 1.0])
```

### Aggregation Phase

```rust
// MOA
.with_moa_aggregation()
.with_moa_num_completions(5)           // Default: 3
.with_moa_fallback_enabled(true)       // Default: true

// MCTS
.with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)
.with_mcts_simulation_depth(2)         // Default: 1
.with_mcts_exploration_weight(0.3)     // Default: 0.2
.with_mcts_num_simulations(4)          // Default: 2
.with_mcts_num_actions(3)              // Default: 3

// RSA
.with_aggregation(true)
.with_aggregation_population_size(8)   // Default: 6
.with_aggregation_selection_size(4)    // Default: 3
.with_aggregation_loops(5)             // Default: 3
```

### Verification Phase

```rust
.with_consensus_threshold(2)           // Default: 2 (2-pass consensus)
```

### Improvement Phase

```rust
.with_max_iterations(7)                // Default: 5
```

### General

```rust
.with_debug(true)                      // Enable debug output
.with_timeout_seconds(600)             // Request timeout (default: 300)

.lightweight()                         // Quick config: fewer agents, fewer iterations
.with_advanced_features()              // Enable aggregation + strategy network
```

## Quick Presets

### Lightweight Mode

```rust
MarsConfig::default().lightweight()
// 2 agents, 2 iterations, no aggregation
// Fast but lower quality
```

### Advanced Mode

```rust
MarsConfig::default().with_advanced_features()
// Enables aggregation + strategy network
// Slower but higher quality
```

### MOA Focus

```rust
MarsConfig::new()
    .with_advanced_features()
    .with_moa_aggregation()
    .with_moa_num_completions(7)
// Horizontal diversity with 7 parallel completions
```

### MCTS Focus

```rust
MarsConfig::new()
    .with_advanced_features()
    .with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)
    .with_mcts_simulation_depth(3)
    .with_mcts_num_simulations(10)
// Vertical exploration with deep tree search
```

See [Advanced Configuration](advanced.md) for more options.
