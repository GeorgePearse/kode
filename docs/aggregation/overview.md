# Aggregation Methods Overview

Aggregation is Phase 2a of the MARS pipeline. After Phase 1 generates N initial solutions, aggregation synthesizes and refines them into improved solutions.

## Three Strategies

MARS provides three complementary aggregation strategies:

### 1. **MOA (Mixture of Agents)** - Horizontal Diversity
Explores different solution approaches in parallel.

- **Best For**: Mathematical proofs, multiple solution paths, diverse perspectives
- **Paper**: [Mixture of Agents: Enhancing LLM Capabilities through Collaborative Specialization](https://arxiv.org/abs/2502.04913)
- **Speed**: Fast (parallel completions)
- **Quality**: High diversity of approaches

### 2. **MCTS (Monte Carlo Tree Search)** - Vertical Exploration
Explores deep reasoning chains with strategic selection.

- **Best For**: Complex multi-step problems, strategic decision making
- **Paper**: [Efficient Selectivity and Backup Operators in Monte-Carlo Tree Search](https://dblp.org/rec/journals/cg/Coulom06.html)
- **Speed**: Slower (sequential tree exploration)
- **Quality**: High depth and strategic coverage

### 3. **RSA (Reward-Seeking Aggregation)** - Iterative Refinement
Maintains and improves a population of solutions.

- **Best For**: Gradual improvement, population-based optimization
- **Speed**: Medium (iterative loops)
- **Quality**: Steady improvement trajectory

## Comparison Table

| Aspect | MOA | MCTS | RSA |
|--------|-----|------|-----|
| **Exploration Type** | Horizontal (parallel) | Vertical (sequential) | Iterative (refinement) |
| **Core Algorithm** | Generate → Critique → Synthesize | Select → Expand → Simulate → Backprop | Select → Refine → Loop |
| **Best For** | Diverse approaches | Deep reasoning | Iterative improvement |
| **Time Complexity** | O(n completions) | O(depth × simulations) | O(loops × selections) |
| **Quality** | Breadth | Depth | Consistency |
| **Paper** | MOA 2025 | Coulom 2006 | MARS 2025 |

## How They Work

### MOA (Horizontal)
```
Initial Solutions (3)
    ↓
Generate N Completions (high temp)
    ↓
Critique Each Completion
    ↓
Synthesize Final Answer
    ↓
Improved Solution
```

### MCTS (Vertical)
```
Initial State (root node)
    ↓
Select Most Promising Path (UCB formula)
    ↓
Expand: Generate Actions
    ↓
Simulate: Rollout to Depth
    ↓
Backpropagate: Update Values
    ↓ (repeat many times)
Best Path
```

### RSA (Iterative)
```
Population: [S1, S2, S3, S4, S5, S6]
    ↓
Select K Solutions
    ↓
Refine Each
    ↓
Add Back to Population
    ↓ (repeat T times)
Improved Population
```

## Decision Matrix

Choose your aggregation method based on your problem:

| Problem Type | Recommended | Reason |
|--------------|-------------|--------|
| Math proofs with multiple approaches | MOA | Capture different solution methods |
| Complex logic with many decision points | MCTS | Strategic path exploration |
| Problems needing gradual refinement | RSA | Iterative improvement loop |
| Creative writing or generation | MOA | Diverse styles and perspectives |
| Code generation (multiple implementations) | MOA | Different algorithmic approaches |
| Strategy/planning problems | MCTS | Tree-based exploration |
| Scientific hypotheses | RSA | Hypothesis refinement |

## Configuration Examples

### Using MOA

```rust
let config = MarsConfig::new()
    .with_advanced_features()
    .with_moa_aggregation()
    .with_moa_num_completions(5);

let mut coordinator = MarsCoordinator::new(config);
```

### Using MCTS

```rust
use code_mars::types::AggregationMethod;

let config = MarsConfig::new()
    .with_advanced_features()
    .with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)
    .with_mcts_simulation_depth(2)
    .with_mcts_exploration_weight(0.3)
    .with_mcts_num_simulations(4);

let mut coordinator = MarsCoordinator::new(config);
```

### Using RSA (Default)

```rust
let config = MarsConfig::new()
    .with_advanced_features()
    .with_aggregation(true);  // RSA is default

let mut coordinator = MarsCoordinator::new(config);
```

## Performance Characteristics

### MOA
- **Time**: Fast (~30 seconds for 5 completions)
- **Tokens**: 10K-30K per aggregation
- **Parallelizable**: Yes (completions in parallel)
- **Best For**: Time-constrained tasks

### MCTS
- **Time**: Slow (~2-5 minutes for deep exploration)
- **Tokens**: 20K-60K per aggregation
- **Parallelizable**: Limited (sequential tree)
- **Best For**: Accuracy-first tasks

### RSA
- **Time**: Medium (~1-3 minutes for 3 loops)
- **Tokens**: 15K-45K per aggregation
- **Parallelizable**: Yes (loop iterations parallel)
- **Best For**: Balanced approach

## Combining Methods

You can also combine strategies for even better results:

```rust
// Phase 2a: Start with MOA for diversity
// Then Phase 2b: Use strategy network to share insights
// Then improve further with RSA-style refinement

let config = MarsConfig::new()
    .with_advanced_features()
    .with_moa_aggregation()
    .with_strategy_network(true);
```

## Advanced Topics

- [MOA Details](moa.md) - Deep dive into Mixture of Agents
- [MCTS Details](mcts.md) - Understanding Monte Carlo Tree Search
- [RSA Details](rsa.md) - Reward-Seeking Aggregation internals

## Next Steps

1. Read [MOA in Detail](moa.md)
2. Understand [MCTS Algorithm](mcts.md)
3. Learn [RSA Strategy](rsa.md)
4. Try [Configuration Guide](../configuration/config.md)
