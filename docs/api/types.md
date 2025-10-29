# Core Types

Key types in the MARS API.

## Solution

```rust
pub struct Solution {
    pub id: String,
    pub agent_id: String,
    pub reasoning: String,
    pub answer: String,
    pub temperature: f32,
    pub token_count: usize,
    pub verification_passes: usize,
    pub is_verified: bool,
    pub verification_score: f32,
}
```

## MarsOutput

```rust
pub struct MarsOutput {
    pub answer: String,
    pub reasoning: String,
    pub all_solutions: Vec<Solution>,
    pub final_solution_id: String,
    pub selection_method: SelectionMethod,
    pub iterations: usize,
    pub total_tokens: usize,
}
```

## MarsEvent

```rust
pub enum MarsEvent {
    ExplorationStarted { num_agents: usize },
    SolutionGenerated { solution_id: String, agent_id: String },
    VerificationStarted,
    SolutionVerified { solution_id: String, is_correct: bool, score: f32 },
    ImprovementStarted { iteration: usize },
    AnswerSynthesized { answer: String },
    Completed { final_answer: String, method: String },
    Error { message: String },
}
```

## AggregationMethod

```rust
pub enum AggregationMethod {
    MOA,                    // Mixture of Agents
    MCTS,                   // Monte Carlo Tree Search
    RSA,                    // Reward-Seeking Aggregation
    MajorityVoting,
    BestOfN,
}
```

See Architecture docs for detailed type reference.
