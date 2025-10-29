# RSA: Reward-Seeking Aggregation

**Reward-Seeking Aggregation (RSA)** maintains a population of solutions, iteratively selects the best ones for refinement, and gradually improves the population through multiple refinement cycles.

## Algorithm

### Step 1: Initialize Population
Start with N solutions (default: 6) from initial agent exploration.

```
Population: [S1, S2, S3, S4, S5, S6]
```

### Step 2: Select Diverse Solutions
Randomly select K solutions (default: 3) for refinement:

```
From Population: [S1, S2, S3, S4, S5, S6]
    ↓
Randomly select K=3: [S2, S4, S6]
```

### Step 3: Refine Selected Solutions
Use LLM to improve each selected solution:

```
For each selected solution:
    ↓
Prompt: "Improve this solution: ..."
    ↓
Refined solution
```

### Step 4: Add Back to Population
Insert refined solutions into population:

```
Population + Refined = [S1, S2, S3, S4, S5, S6, S2', S4', S6']
```

### Step 5: Repeat
Loop steps 2-4 for T iterations (default: 3):

```
Iteration 1: Select & refine
    ↓
Iteration 2: Select & refine
    ↓
Iteration 3: Select & refine
    ↓
Final Population
```

## When to Use RSA

### ✅ Good For:
- **Iterative improvement** of solutions
- **Refinement problems** requiring multiple passes
- **Balanced approach** between quality and time
- **Stable improvement** with predictable trajectory
- **Large solution spaces** where diversity matters

### ❌ Avoid For:
- Simple problems (just use single agent)
- Very complex multi-step reasoning (use MCTS)
- Time-critical applications
- Problems with single correct approach

## Configuration

```rust
use code_mars::MarsConfig;

// Enable RSA (enabled by default with advanced features)
let config = MarsConfig::new()
    .with_advanced_features()
    .with_aggregation(true);  // RSA is the default

let mut coordinator = MarsCoordinator::new(config);
```

### Configuration Options

```rust
pub struct MarsConfig {
    // Size of solution population
    pub aggregation_population_size: usize,        // Default: 6

    // Number of solutions to select per loop
    pub aggregation_selection_size: usize,         // Default: 3

    // Number of aggregation loops
    pub aggregation_loops: usize,                  // Default: 3
}
```

## Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ModelClient::default();

    // Configure RSA for iterative improvement
    let config = MarsConfig::new()
        .with_advanced_features()
        .with_aggregation(true)
        .with_num_agents(3)
        .with_max_iterations(7);

    let mut coordinator = MarsCoordinator::new(config);

    let query = "Write a comprehensive essay on the French Revolution";
    let result = coordinator.run_with_client(query, &client).await?;

    println!("Final Essay:\n{}", result.reasoning);
    println!("Summary: {}", result.answer);

    Ok(())
}
```

## How RSA Works

### Population-Based Approach
Unlike MOA (parallel exploration) or MCTS (tree exploration), RSA maintains a **population** that evolves over time:

```
Generation 0: [S1, S2, S3, S4, S5, S6]
    ↓ Select K=3, refine, add back
Generation 1: [S1, S2, S3, S4, S5, S6, S2', S4', S6']
    ↓ Select K=3, refine, add back
Generation 2: [S1, S2, S3, S4, S5, S6, S2', S4', S6', S3'', S5'', S1'']
    ↓ Select K=3, refine, add back
Generation 3: [... evolved population ...]
```

### Random Selection for Diversity
- **Not** selecting only the best solutions
- **Random selection** ensures diversity in refinement targets
- **Prevents** premature convergence to local optima

## Algorithm Pseudocode

```rust
async fn aggregate_rsa(
    solutions: &[Solution],
    population_size: usize,
    selection_size: usize,
    num_loops: usize,
) -> Result<Vec<Solution>> {
    let mut aggregated = Vec::new();

    if solutions.is_empty() {
        return Ok(aggregated);
    }

    let mut population = solutions.to_vec();

    // Limit population to requested size
    if population.len() > population_size {
        population.truncate(population_size);
    }

    // Perform aggregation loops
    for loop_idx in 0..num_loops {
        // Step 2: Select diverse solutions
        let selected = select_diverse_solutions(&population, selection_size)?;

        // Step 3: Refine selected solutions
        if !selected.is_empty() {
            let refined = synthesize_solution(&selected, loop_idx)?;
            aggregated.push(refined.clone());

            // Step 4: Add back to population
            population.push(refined);
        }
    }

    Ok(aggregated)
}
```

## Performance

### Time Complexity
- **Per loop**: O(selection_size) refinements
- **Total**: O(num_loops × selection_size)
- **Typical**: Linear (much faster than MCTS)

### Space Complexity
- **Population**: O(population_size)
- **Typical**: Keep ~6-10 solutions

### Cost
- **Tokens**: 15K-45K per aggregation
- **Time**: 1-3 minutes (faster than MCTS)
- **Quality**: Steady improvement ~+5-15% per loop

## Comparison with MOA and MCTS

| Aspect | RSA | MOA | MCTS |
|--------|-----|-----|------|
| **Approach** | Population + refinement | Parallel diversity | Tree exploration |
| **Time** | Medium | Fast | Slow |
| **Tokens** | 15K-45K | 10K-30K | 20K-60K |
| **Best For** | Iterative improvement | Multiple perspectives | Strategic planning |
| **Quality** | Steady improvement | Broad coverage | Deep lookahead |
| **Complexity** | Simple | Medium | Complex |

## Tuning RSA

### For Higher Quality
```rust
.with_aggregation_population_size(8)   // Larger population
.with_aggregation_selection_size(4)    // Refine more per loop
.with_aggregation_loops(5)             // More iterations
```

### For Speed
```rust
.with_aggregation_population_size(4)   // Smaller population
.with_aggregation_selection_size(2)    // Refine fewer per loop
.with_aggregation_loops(2)             // Fewer iterations
```

### Balanced (Default)
```rust
.with_aggregation_population_size(6)   // Default
.with_aggregation_selection_size(3)    // Default
.with_aggregation_loops(3)             // Default
```

## Why Random Selection?

Instead of always selecting the **best** solutions:

1. **Preserves diversity** - Poor solutions might improve significantly
2. **Avoids local optima** - Exploiting low-quality solutions can reveal new insights
3. **Statistical validity** - Population sampling is more robust
4. **Faster convergence** - Random walk often beats greedy

Example:
```
Population: [S1(0.7), S2(0.5), S3(0.9), S4(0.4), S5(0.8), S6(0.6)]

Greedy selection: [S3(0.9), S5(0.8), S1(0.7)]
    → Refine only high-quality solutions
    → May miss improvement opportunities in weaker solutions

Random selection: [S2(0.5), S4(0.4), S6(0.6)]
    → Refine weaker solutions
    → Discover unexpected improvements
```

## Troubleshooting

### Quality Not Improving
- Increase `aggregation_loops` to 4-5
- Increase `aggregation_selection_size` to 4
- Check that refinement prompts are effective

### Takes Too Long
- Reduce `aggregation_loops` to 1-2
- Reduce `aggregation_selection_size` to 2
- Reduce population size

### Token Usage Too High
- Reduce `aggregation_loops`
- Reduce `aggregation_selection_size`
- Shorter solution descriptions

## Advanced: Hybrid Approaches

Combine RSA with other phases:

```rust
// Phase 2a: RSA aggregation
// Phase 2b: Strategy network (extract insights from population)
// Phase 4: Iterative improvement (improves unverified solutions)

// This creates a "double refinement" system
let config = MarsConfig::new()
    .with_advanced_features()
    .with_aggregation(true)           // RSA
    .with_strategy_network(true)       // Strategy extraction
    .with_max_iterations(7);           // More improvement loops
```

## See Also

- [Aggregation Overview](overview.md) - Compare with other methods
- [MOA](moa.md) - Parallel diversity approach
- [MCTS](mcts.md) - Tree-based exploration
- [Configuration Guide](../configuration/config.md) - Full options
