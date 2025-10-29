# MCTS: Monte Carlo Tree Search

**Monte Carlo Tree Search (MCTS)** uses UCB-based node selection, rollout simulations, and backpropagation to strategically explore the tree of reasoning possibilities.

## Paper

[Efficient Selectivity and Backup Operators in Monte-Carlo Tree Search](https://dblp.org/rec/journals/cg/Coulom06.html)

## Algorithm

MCTS has four core phases that repeat:

### Phase 1: Selection
Use the UCB formula to select the most promising child node:

```
UCB = (value/visits) + C * sqrt(ln(parent_visits) / visits)
```

where:
- `value/visits` = exploitation (average quality of node)
- `C * sqrt(...)` = exploration (uncertainty bonus)
- Higher UCB = more promising node

### Phase 2: Expansion
When reaching an unvisited node, generate N possible actions:

```
Current State
    ↓
Generate N actions via LLM completion
    ├─ Action 1: Next step variation 1
    ├─ Action 2: Next step variation 2
    ├─ ...
    └─ Action N: Next step variation N
    ↓
Create N child nodes in tree
```

### Phase 3: Simulation
Rollout to depth D using random action selection:

```
From new node:
    ↓
Repeat D times:
    ├─ Randomly select action
    ├─ Apply to current state
    └─ Move forward
    ↓
Terminal state reached
    ↓
Evaluate state quality (0.0-1.0)
```

### Phase 4: Backpropagation
Update all ancestor nodes with simulation result:

```
Leaf node: value = 0.8, visits = 1
    ↓
Parent node: value += 0.8, visits += 1
    ↓
Grandparent node: value += 0.8, visits += 1
    ↓
... up to root
```

## Complete Algorithm Pseudocode

```
MCTS(initial_state, num_simulations):
    root = create_node(initial_state)

    for i in 1 to num_simulations:
        node = root

        // Phase 1: Select
        while not is_terminal(node.state):
            if node has unvisited children:
                break
            node = select_child_with_highest_ucb(node)

        // Phase 2: Expand
        if not is_terminal(node.state) and node.visits > 0:
            actions = generate_actions(node.state)
            node = create_random_child(node, actions)

        // Phase 3: Simulate
        state = node.state
        for depth in 1 to simulation_depth:
            if is_terminal(state):
                break
            actions = generate_actions(state)
            action = select_random(actions)
            state = apply_action(state, action)

        value = evaluate_state(state)

        // Phase 4: Backpropagate
        while node != null:
            node.visits += 1
            node.value += value
            node = node.parent

    return best_child(root)  // Most visited child
```

## When to Use MCTS

### ✅ Good For:
- **Multi-step reasoning** with complex decision trees
- **Strategy problems** requiring lookahead
- **Logic puzzles** with branching paths
- **Planning problems** with multiple options
- **Code design** with architectural choices
- **Scientific problems** with hypothesis testing

### ❌ Avoid For:
- Simple single-step questions
- Time-critical applications (MCTS is slow)
- Problems with few decision points
- Tasks where all paths are equivalent

## Configuration

```rust
use code_mars::{MarsConfig, MarsCoordinator, types::AggregationMethod};

// Enable MCTS aggregation
let config = MarsConfig::new()
    .with_advanced_features()
    .with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)
    .with_mcts_simulation_depth(2)
    .with_mcts_exploration_weight(0.3)
    .with_mcts_num_simulations(4)
    .with_mcts_num_actions(3);

let mut coordinator = MarsCoordinator::new(config);
```

### Configuration Options

```rust
pub struct MCTSConfig {
    // How deep to simulate (default: 1)
    pub simulation_depth: usize,

    // UCB exploration coefficient (default: 0.2)
    pub exploration_weight: f32,

    // Number of simulations per search (default: 2)
    pub num_simulations: usize,

    // Number of actions to generate per expansion (default: 3)
    pub num_actions: usize,

    // Temperature for action generation (default: 1.0)
    pub generation_temperature: f32,

    // Temperature for state evaluation (default: 0.1)
    pub evaluation_temperature: f32,

    // Maximum conversation history length (default: 10)
    pub max_history_length: usize,
}
```

## Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ModelClient::default();

    // Configure MCTS for deep reasoning
    let config = MarsConfig::new()
        .with_advanced_features()
        .with_aggregation_method(AggregationMethod::MonteCarloTreeSearch)
        .with_mcts_simulation_depth(3)
        .with_mcts_exploration_weight(0.5)
        .with_mcts_num_simulations(10);

    let mut coordinator = MarsCoordinator::new(config);

    let query = "Plan a 5-day trip to Japan focusing on cultural sites";
    let result = coordinator.run_with_client(query, &client).await?;

    println!("Reasoning:\n{}", result.reasoning);
    println!("Final Plan:\n{}", result.answer);

    Ok(())
}
```

## Understanding the Parameters

### `simulation_depth`
- **1**: Shallow exploration, faster but less lookahead
- **2-3**: Balanced depth and speed (recommended)
- **4+**: Deep exploration, slow but comprehensive

### `exploration_weight` (C in UCB formula)
- **0.1**: More exploitation, use known good paths
- **0.2**: Balanced (default)
- **0.5-1.0**: More exploration, try diverse paths

### `num_simulations`
- **2-4**: Quick exploration
- **4-8**: Balanced (default range)
- **10+**: Thorough exploration, slow

### `num_actions`
- **2-3**: Few options per step (default: 3)
- **4-5**: More diversity
- **6+**: Very diverse, slower

## How MCTS Differs

### vs MOA
- **MOA**: All approaches explored in parallel (horizontal)
- **MCTS**: Strategic exploration of tree (vertical)

### vs Simple Greedy
- **Greedy**: Pick highest-value action at each step (local optimization)
- **MCTS**: Consider future consequences via simulation (global optimization)

### vs Exhaustive Search
- **Exhaustive**: Try every possible path (exponential time)
- **MCTS**: Intelligently focus on promising paths (polynomial time)

## Performance

### Time Complexity
- **Per simulation**: O(simulation_depth × num_actions)
- **Total**: O(num_simulations × simulation_depth × num_actions)
- **Typical**: O(n³) where n = num_simulations

### Space Complexity
- **Tree nodes**: O(num_simulations × branching_factor)
- **Typical**: Store ~100-1000 nodes

### Cost
- **Tokens**: 20K-60K per search
- **Time**: 1-5 minutes for deep exploration
- **Quality**: +20-40% on multi-step problems

## Tuning for Your Problem

### For Accuracy (Ignore Speed)
```rust
.with_mcts_simulation_depth(4)
.with_mcts_num_simulations(20)
.with_mcts_exploration_weight(0.5)
```

### For Speed (Accept Lower Quality)
```rust
.with_mcts_simulation_depth(1)
.with_mcts_num_simulations(2)
.with_mcts_exploration_weight(0.1)
```

### Balanced
```rust
.with_mcts_simulation_depth(2)
.with_mcts_num_simulations(4)
.with_mcts_exploration_weight(0.3)
```

## Troubleshooting

### Takes Too Long
- Reduce `simulation_depth` to 1-2
- Reduce `num_simulations` to 2-4
- Reduce `num_actions` to 2

### Quality Too Low
- Increase `simulation_depth` to 3-4
- Increase `num_simulations` to 6-10
- Increase exploration_weight to 0.4-0.6

### Token Usage Too High
- Use fewer simulations
- Reduce simulation depth
- Shorten problem description

## See Also

- [Aggregation Overview](overview.md) - Compare with other methods
- [MOA](moa.md) - Horizontal diversity alternative
- [Configuration Guide](../configuration/config.md) - Full options
