# MARS - Multi-Agent Reasoning System

A production-ready Rust implementation of advanced LLM optimization techniques, achieving **69% improvement on AIME 2025** benchmarks.

![Benchmark Results](../images/benchmarks.png)

## Why MARS?

Complex reasoning tasks require more than a single pass through a language model. MARS coordinates multiple agents with different exploration strategies to:

- **Explore different solution paths** - Temperature diversity (0.3, 0.6, 1.0) encourages varied approaches
- **Verify and improve solutions** - Cross-agent verification catches errors and identifies weak points
- **Aggregate diverse insights** - Combine perspectives using MOA (horizontal) or MCTS (vertical) strategies
- **Synthesize optimal answers** - Select best solution via majority voting, verification scores, or synthesis

## Key Features

✅ **5-Phase Pipeline** - Exploration → Aggregation → Verification → Improvement → Synthesis

✅ **Multiple Aggregation Strategies** - MOA for diversity, MCTS for deep exploration, RSA for iteration

✅ **Cross-Model Support** - Works with ModelClient, litellm-rs, or any custom LLM provider

✅ **Comprehensive Verification** - 2-pass consensus verification with detailed feedback

✅ **Production Ready** - Async/await, error handling, event streaming, configurability

## Benchmark Results

| Benchmark | Baseline | MARS | Improvement |
|-----------|----------|------|-------------|
| **AIME 2025** | 43.3% | 73.3% | **+30 pp (+69%)** |
| **IMO 2025** | 16.7% | 33.3% | **+16.7 pp (+100%)** |
| **LiveCodeBench** | 39.05% | 50.48% | **+11.43 pp (+29%)** |

## Quick Start

```rust
use code_mars::{MarsCoordinator, MarsConfig};
use code_core::ModelClient;

// Create coordinator
let config = MarsConfig::default()
    .with_advanced_features()
    .with_moa_aggregation();

let mut coordinator = MarsCoordinator::new(config);
let client = ModelClient::new(...);

// Run MARS
let result = coordinator.run_with_client(query, &client).await?;
println!("Answer: {}", result.answer);
```

## What's Different?

### vs Single-Shot LLM
- **MARS**: Multiple agents explore different paths, verify answers, iteratively improve
- **Single-Shot**: One attempt, no verification, no improvement loop

### vs Simple Ensemble
- **MARS**: Sophisticated aggregation (MOA for diversity, MCTS for depth)
- **Simple**: Basic voting without specialized exploration strategies

### vs Tree-of-Thought
- **MARS**: Uses multiple reasoning strategies (MOA + MCTS) coordinated by verification
- **Tree-of-Thought**: Single strategy without cross-agent verification

## Architecture at a Glance

```
Phase 1: Multi-Agent Exploration
    ↓ (3 agents × 3 temperatures)
Phase 2a: Solution Aggregation (MOA/MCTS/RSA)
    ↓ (synthesize improved solutions)
Phase 2b: Strategy Network
    ↓ (extract and share insights)
Phase 3: Cross-Agent Verification
    ↓ (2-pass consensus)
Phase 4: Iterative Improvement
    ↓ (max 5 iterations on unverified)
Phase 5: Final Synthesis
    ↓ (majority voting → best verified → synthesized)
Answer
```

## Documentation Structure

- **[Getting Started](getting-started/overview.md)** - Installation and quick start
- **[Architecture](architecture/overview.md)** - Detailed system design
- **[Aggregation Methods](aggregation/overview.md)** - MOA, MCTS, RSA strategies
- **[Configuration](configuration/config.md)** - How to customize MARS
- **[Usage Examples](usage/basic.md)** - Code samples and patterns
- **[Performance](performance/benchmarks.md)** - Optimization and costs
- **[API Reference](api/types.md)** - Complete API documentation

## Use Cases

### Mathematical Problem Solving
Complex proofs, geometry, algebra - use **MOA** for diverse approaches

### Multi-Step Reasoning
Chains of thought, logic puzzles - use **MCTS** for strategic exploration

### Creative Writing
Stories, essays - use **RSA** for iterative refinement

### Code Generation
Complex programs - use **MOA** for parallel implementations

## Implementation Details

- **Language**: Rust 1.75+
- **Async Runtime**: Tokio
- **Test Coverage**: 43 unit + 29 integration tests
- **Code Size**: ~3,500 LOC
- **Dependencies**: Minimal (workspace deps)

## Citation

If you use MARS in your research, please cite:

```bibtex
@software{mars-2025,
  title={MARS: Multi-Agent Reasoning System},
  author={Code Project},
  year={2025},
  url={https://github.com/GeorgePearse/code}
}
```

## Next Steps

1. [Install MARS](getting-started/installation.md)
2. [Read the Quick Start](getting-started/quick-start.md)
3. [Explore Architecture](architecture/overview.md)
4. [Learn Aggregation Methods](aggregation/overview.md)
5. [Run Examples](usage/basic.md)

---

**Questions?** Check the [references](references.md) for papers and resources.
