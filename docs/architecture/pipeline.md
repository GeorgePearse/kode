# 5-Phase Pipeline

MARS executes in 5 distinct phases to solve complex reasoning problems.

## Phase 1: Multi-Agent Exploration

**Goal**: Generate diverse initial solutions

- Spawn N agents (default: 3)
- Each agent has different temperature for diverse exploration
- Temperatures: [0.3, 0.6, 1.0]
- All agents run LLM in parallel
- Store all solutions in workspace

**Output**: N solutions with varying approaches

## Phase 2a: Solution Aggregation

**Goal**: Synthesize and refine solutions

Choose one aggregation method:

- **MOA**: Generate diverse completions, critique, synthesize
- **MCTS**: Tree exploration with UCB selection and simulations
- **RSA**: Maintain population, iteratively refine

**Output**: Enhanced solutions from aggregation

## Phase 2b: Strategy Network

**Goal**: Share successful strategies across agents

- Extract reasoning patterns from verified solutions
- Identify what worked well
- Share insights across agents
- Generate enhanced solutions using peer strategies

**Output**: Strategy-enhanced solutions

## Phase 3: Verification

**Goal**: Verify solution correctness

- Cross-agent verification
- Each solution verified by multiple agents
- Consensus requirement: 2 consecutive "CORRECT" assessments
- Generate detailed feedback on weaknesses
- Score solutions (0.0-1.0)

**Output**: Verified solutions with feedback

## Phase 4: Iterative Improvement

**Goal**: Improve unverified solutions

Loop up to max_iterations (default: 5):

1. Identify unverified solutions
2. Target for enhancement based on feedback
3. Generate improved versions
4. Re-verify
5. Stop if verified or max iterations reached

**Output**: Improved solutions, some now verified

## Phase 5: Final Synthesis

**Goal**: Select and synthesize final answer

1. **Try Majority Voting**: If 2+ agents agree, use that answer
2. **Try Best Verified**: If verified solutions exist, use highest-scored
3. **Try Synthesis**: Otherwise, synthesize from top 3 solutions
4. **Clean up**: Extract clean answer from thinking tags

**Output**: Final answer with selection method

## Phase Control Flow

```
Phase 1: Exploration
  ↓ (3 parallel agents)
  ├─ Agent 1 (T=0.3): Solution A1
  ├─ Agent 2 (T=0.6): Solution A2
  └─ Agent 3 (T=1.0): Solution A3

Phase 2a: Aggregation
  ├─ MOA: Generate + Critique + Synthesize
  ├─ MCTS: Tree Search with UCB
  └─ RSA: Population Refinement
  ↓
  Generates N enhanced solutions

Phase 2b: Strategy Network (optional)
  ↓
  Extracts and shares strategies

Phase 3: Verification
  ├─ Verify all solutions (2-pass consensus)
  ├─ Score: 0.0-1.0
  └─ Generate feedback

Phase 4: Improvement (up to 5 iterations)
  ├─ Target unverified
  ├─ Enhance based on feedback
  └─ Re-verify

Phase 5: Synthesis
  ├─ Majority voting?
  ├─ Best verified?
  ├─ Synthesize?
  └─ Clean answer

Final Answer
```

## Performance by Phase

| Phase | Time | Parallelism | LLM Calls |
|-------|------|-------------|-----------|
| 1 | Fast | 3 agents | 3 |
| 2a (MOA) | Medium | Completions parallel | 2N+1 |
| 2a (MCTS) | Slow | Sequential | ~100 |
| 2a (RSA) | Medium | Selections parallel | K*T |
| 2b | Medium | Agents parallel | N |
| 3 | Fast | Solutions parallel | 2N |
| 4 | Slow | Improvements parallel | 3*I |
| 5 | Instant | None | 0-1 |

## Configuration by Phase

```rust
// Phase 1 config
.with_num_agents(3)
.with_temperatures(vec![0.3, 0.6, 1.0])

// Phase 2a config
.with_aggregation(true)
.with_aggregation_method(AggregationMethod::MOA)

// Phase 2b config
.with_strategy_network(true)

// Phase 4 config
.with_max_iterations(5)

// Consensus config
.with_consensus_threshold(2)
```

See [Configuration Guide](../configuration/config.md) for details.
