# MOA: Mixture of Agents

**Mixture of Agents (MOA)** generates diverse completions in parallel, critiques each for strengths and weaknesses, then synthesizes a final answer incorporating all insights.

## Paper

[Mixture of Agents: Enhancing LLM Capabilities through Collaborative Specialization](https://arxiv.org/abs/2502.04913)

## Algorithm

### Step 1: Generate Diverse Completions
Use high temperature (1.0) to generate N different completions of the same prompt.

```
Input: "Solve: 3x + 5 = 20"
    ↓
Generate 5 completions (temp=1.0)
    ├─ Solution 1: Step-by-step algebraic approach
    ├─ Solution 2: Visual/graphical reasoning
    ├─ Solution 3: Substitution method
    ├─ Solution 4: Alternative verification
    └─ Solution 5: Conceptual explanation
```

### Step 2: Critique Each Completion
Analyze each solution for:
- **Correctness** - Is the answer right?
- **Completeness** - Does it address all aspects?
- **Clarity** - Is it easy to understand?
- **Rigor** - Is the reasoning sound?

```
For each solution:
    ↓
Critique prompt: "Analyze this solution's strengths and weaknesses"
    ↓
Generate critique
    ├─ Strengths: [list]
    └─ Weaknesses: [list]
```

### Step 3: Synthesize Final Answer
Combine all solutions and critiques into optimal final answer.

```
All Solutions + Critiques
    ↓
Synthesis prompt: "Synthesize the best answer"
    ↓
Final Answer
```

## When to Use MOA

### ✅ Good For:
- **Math problems** with multiple solution approaches
- **Code generation** with multiple algorithmic solutions
- **Essay writing** combining multiple perspectives
- **Analysis** requiring comprehensive coverage
- **Proof writing** showing different proof techniques

### ❌ Avoid For:
- Simple factual questions (overkill)
- Real-time applications (MOA is slow)
- Very short answers
- Tasks where all approaches are equivalent

## Configuration

```rust
use code_mars::{MarsConfig, MarsCoordinator};

// Enable MOA aggregation
let config = MarsConfig::new()
    .with_advanced_features()
    .with_moa_aggregation()
    .with_moa_num_completions(5);  // Default: 3

let mut coordinator = MarsCoordinator::new(config);
```

### Configuration Options

```rust
MarsConfig {
    // Number of completions to generate
    moa_num_completions: 5,  // Default: 3

    // Enable fallback if n parameter not supported by model
    moa_fallback_enabled: true,  // Default: true
}
```

## Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ModelClient::default();

    // Configure MOA for maximum diversity
    let config = MarsConfig::new()
        .with_advanced_features()
        .with_moa_aggregation()
        .with_moa_num_completions(7);

    let mut coordinator = MarsCoordinator::new(config);

    let query = "Prove that the sum of squares of first n integers is n(n+1)(2n+1)/6";
    let result = coordinator.run_with_client(query, &client).await?;

    println!("Proof:\n{}", result.reasoning);
    println!("Final Answer: {}", result.answer);

    Ok(())
}
```

## How MOA Differs

### vs Single Agent
- **Single**: One attempt, one perspective
- **MOA**: Multiple perspectives synthesized

### vs Ensemble Voting
- **Ensemble**: Simple majority voting
- **MOA**: Intelligent synthesis of critiques

### vs MCTS
- **MOA**: Horizontal diversity (many approaches in parallel)
- **MCTS**: Vertical depth (strategic exploration of one path)

## Performance

### Time Complexity
- **Generate completions**: O(n) parallel (n = num_completions)
- **Critique each**: O(n) parallel
- **Synthesize**: O(1)
- **Total**: O(n) with parallelization

### Space Complexity
- Store all completions: O(n × completion_length)
- Typical: ~5K-50K tokens for 5 completions

### Cost
- **Tokens**: 10K-30K per aggregation
- **Time**: 30-60 seconds (with parallel API calls)
- **Success rate**: +10-30% on most tasks

## Algorithm Pseudocode

```rust
async fn aggregate_moa(
    query: &str,
    system_prompt: &str,
    num_completions: usize,
    provider: &dyn LLMProvider,
) -> Result<Solution> {
    // Step 1: Generate diverse completions
    let mut completions = Vec::new();
    for i in 0..num_completions {
        let completion = provider.complete(
            &query,
            Some(system_prompt)
        ).await?;
        completions.push(completion);
    }

    // Step 2: Critique each completion
    let mut critiques = Vec::new();
    for completion in &completions {
        let critique_prompt = format!(
            "Analyze this solution's strengths and weaknesses:\n{}",
            completion
        );
        let critique = provider.complete(
            &critique_prompt,
            Some(system_prompt)
        ).await?;
        critiques.push(critique);
    }

    // Step 3: Synthesize final answer
    let synthesis_prompt = format!(
        "Given these solutions and critiques, synthesize the best answer:\n\
         Solutions: {:?}\n\
         Critiques: {:?}",
        completions, critiques
    );
    let final_answer = provider.complete(
        &synthesis_prompt,
        Some(system_prompt)
    ).await?;

    Ok(Solution::new(
        "moa-aggregator".to_string(),
        format!("Completions: {:?}\nCritiques: {:?}", completions, critiques),
        final_answer,
        0.5,
        token_count,
    ))
}
```

## Advanced Configuration

### High-Quality Mode
```rust
let config = MarsConfig::new()
    .with_moa_aggregation()
    .with_moa_num_completions(10)  // More completions
    .with_num_agents(5);             // More initial agents
```

### Fast Mode
```rust
let config = MarsConfig::new()
    .with_moa_aggregation()
    .with_moa_num_completions(3)   // Fewer completions
    .lightweight();                  // Fewer agents overall
```

## Troubleshooting

### Quality Too Low
- Increase `moa_num_completions` to 5-7
- Ensure system prompt is clear
- Check model temperature is appropriate

### Takes Too Long
- Reduce `moa_num_completions` to 3-4
- Use lighter model (gpt-3.5-turbo)
- Disable other phases (strategy network)

### Token Usage Too High
- Reduce `moa_num_completions`
- Use lighter model
- Shorten query and system prompt

## See Also

- [Aggregation Overview](overview.md) - Compare with other methods
- [MCTS](mcts.md) - Vertical exploration alternative
- [Configuration Guide](../configuration/config.md) - Full config options
