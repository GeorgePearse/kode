# Architecture Overview

MARS is a five-phase multi-agent reasoning system designed to solve complex problems through exploration, verification, and iterative improvement.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Input Query                               │
└────────────────────────────┬────────────────────────────────┘
                             ↓
┌─────────────────────────────────────────────────────────────┐
│      Phase 1: Multi-Agent Exploration                       │
│  • 3 agents with temperatures [0.3, 0.6, 1.0]             │
│  • Parallel LLM calls                                      │
│  • Initial solutions generated                            │
└────────────────────────────┬────────────────────────────────┘
                             ↓
         ┌───────────────────────────────────────┐
         │   Phase 2a: Solution Aggregation      │
         │  ┌─────────┐  ┌─────────┐  ┌────────┐│
         │  │   MOA   │  │  MCTS   │  │  RSA   ││
         │  │Horizontal│ │Vertical │ │Iterative││
         │  └─────────┘  └─────────┘  └────────┘│
         └───────────────────┬───────────────────┘
                             ↓
         ┌───────────────────────────────────────┐
         │  Phase 2b: Strategy Network           │
         │  • Extract successful strategies      │
         │  • Share insights across agents       │
         └───────────────────┬───────────────────┘
                             ↓
         ┌───────────────────────────────────────┐
         │  Phase 3: Verification                │
         │  • Cross-agent verification           │
         │  • 2-pass consensus                   │
         │  • Detailed feedback                  │
         └───────────────────┬───────────────────┘
                             ↓
         ┌───────────────────────────────────────┐
         │  Phase 4: Iterative Improvement       │
         │  • Target unverified solutions        │
         │  • Enhance based on feedback          │
         │  • Re-verify (up to 5 iterations)     │
         └───────────────────┬───────────────────┘
                             ↓
         ┌───────────────────────────────────────┐
         │  Phase 5: Final Synthesis             │
         │  • Majority voting                    │
         │  • Best verified selection            │
         │  • Synthesize if no consensus         │
         └───────────────────┬───────────────────┘
                             ↓
┌─────────────────────────────────────────────────────────────┐
│                   Final Answer                              │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. **Coordinator** (`src/coordinator.rs`)
The orchestrator that manages all 5 phases and coordinates between components.

- **Responsibilities**:
  - Phase sequencing and flow control
  - Event emission for monitoring
  - Error handling and recovery
  - Configuration management

### 2. **Agent** (`src/agent.rs`)
Individual reasoning agents with temperature-based exploration.

- **Responsibilities**:
  - Generate solutions via LLM
  - Manage agent state
  - Track token usage
  - Support custom reasoning prompts

### 3. **Workspace** (`src/workspace.rs`)
Shared solution storage (Arc<RwLock> for thread-safe access).

- **Responsibilities**:
  - Store and retrieve solutions
  - Query solutions by various criteria
  - Concurrent access management
  - Solution lifecycle tracking

### 4. **Verifier** (`src/verifier.rs`)
Cross-agent verification system.

- **Responsibilities**:
  - Check solution correctness
  - Generate detailed feedback
  - Track verification scores
  - Manage consensus threshold

### 5. **Aggregator** (`src/aggregator.rs`)
Solution synthesis using specialized strategies.

- **Responsibilities**:
  - Route to appropriate aggregation method
  - Synthesize improved solutions
  - Combine multiple perspectives
  - Generate aggregation statistics

### 6. **MOA** (`src/moa.rs`)
Mixture of Agents aggregation (horizontal diversity).

- **Responsibilities**:
  - Generate diverse completions
  - Critique each completion
  - Synthesize final answer

### 7. **MCTS** (`src/mcts.rs`)
Monte Carlo Tree Search (vertical exploration).

- **Responsibilities**:
  - Build reasoning tree
  - Select via UCB formula
  - Simulate rollouts
  - Backpropagate values

### 8. **Strategy Network** (`src/strategy.rs`)
Cross-agent learning system.

- **Responsibilities**:
  - Extract strategies from solutions
  - Calculate success rates
  - Share insights across agents
  - Generate strategy-based solutions

### 9. **Types** (`src/types.rs`)
Core data structures and enums.

- **Responsibilities**:
  - Define Solution, VerificationResult
  - Define MarsEvent for streaming
  - Define AggregationMethod enum
  - Type safety across modules

### 10. **Config** (`src/config.rs`)
Configuration system with builder pattern.

- **Responsibilities**:
  - Parse and validate config
  - Provide sensible defaults
  - Support builder pattern
  - Generate phase-specific configs

## Data Flow

### Solution Flow

```
Agent 1 (temp=0.3) ──┐
Agent 2 (temp=0.6) ──┼──→ Workspace ──→ Aggregation ──→ Verification
Agent 3 (temp=1.0) ──┘                                         ↓
                                                          Improvement
                                                                ↓
                                                          Final Synthesis
```

### Event Flow

```
Coordinator
  ↓ emits
MarsEvent
  ├─ ExplorationStarted
  ├─ SolutionGenerated
  ├─ VerificationStarted
  ├─ SolutionVerified
  ├─ ImprovementStarted
  ├─ AnswerSynthesized
  └─ Completed
  ↓
Event Channel (mpsc)
  ↓
TUI / Monitoring
```

## Key Design Patterns

### 1. **Arc<RwLock> for Concurrency**
Allows lock-free reads during parallel phases:

```rust
workspace: Arc<RwLock<Vec<Solution>>>
```

### 2. **Builder Pattern for Configuration**
Fluent API for composable configuration:

```rust
config
    .with_advanced_features()
    .with_moa_aggregation()
    .with_num_agents(4)
```

### 3. **Trait-Based Abstraction**
LLMProvider trait supports any model:

```rust
pub trait LLMProvider: Send + Sync {
    async fn complete(&self, prompt: &str, system: Option<&str>) -> Result<String>;
    // ...
}
```

### 4. **Event Streaming**
Real-time progress tracking:

```rust
while let Some(event) = rx.recv().await {
    match event {
        MarsEvent::SolutionGenerated { solution_id, .. } => { /* ... */ }
        // ...
    }
}
```

### 5. **Error Handling**
Comprehensive error type:

```rust
pub enum MarsError {
    AgentError(String),
    VerificationError(String),
    AggregationError(String),
    // ... 10+ variants
}
```

## Configuration Hierarchy

```
MarsConfig (global settings)
  ├─ num_agents
  ├─ temperatures
  ├─ aggregation_method
  │   ├─ MOA: moa_num_completions, moa_fallback_enabled
  │   ├─ MCTS: mcts_simulation_depth, mcts_exploration_weight, etc.
  │   └─ RSA: aggregation_loops, aggregation_selection_size
  ├─ max_iterations
  ├─ verify settings
  └─ token budgets

↓ Phase-specific configs

MCTSConfig, MoaMetadata, StrategyConfig
```

## Thread Safety

All shared data structures use proper synchronization:

- **Arc<RwLock>** - Solutions in workspace
- **mpsc::channel** - Event distribution
- **tokio::spawn** - Agent parallelization
- **No unsafe code** - Type-safe Rust throughout

## Performance Characteristics

### Phase 1: Exploration
- **Parallelism**: 3 agents in parallel
- **Time**: O(latency of slowest LLM call)
- **Memory**: O(num_agents × max_tokens)

### Phase 2a: Aggregation
- **MOA**: O(num_completions) parallel
- **MCTS**: O(simulations × depth) sequential
- **RSA**: O(loops × selections) sequential

### Phase 3: Verification
- **Parallelism**: 2 verifiers per solution in parallel
- **Time**: O(num_solutions / num_verifiers)

### Phase 4: Improvement
- **Parallelism**: Improve multiple solutions in parallel
- **Time**: O(max_iterations × num_unverified / num_agents)

### Phase 5: Synthesis
- **Time**: O(1) - pure selection/synthesis

## Extensibility Points

### 1. Custom LLM Providers
Implement `LLMProvider` trait:

```rust
struct MyProvider;

#[async_trait]
impl LLMProvider for MyProvider {
    async fn complete(&self, prompt: &str, system: Option<&str>) -> Result<String> {
        // Your implementation
    }
    // ...
}
```

### 2. Custom Aggregation Methods
Add to `AggregationMethod` enum and implement in Aggregator.

### 3. Custom Verification Strategies
Modify Verifier to support domain-specific checks.

### 4. Custom Prompts
Update `src/prompts.rs` with domain-specific templates.

## Next Steps

- [5-Phase Pipeline Details](pipeline.md)
- [Core Components](components.md)
- [Configuration Guide](../configuration/config.md)
