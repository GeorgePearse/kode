# MARS Overview

MARS (Multi-Agent Reasoning System) is a Rust framework for advanced reasoning with large language models. It coordinates multiple agents to explore different solution paths, verify answers, and synthesize optimal results.

## Core Concept

Instead of asking an LLM a question once, MARS:

1. **Explores** - Spawn multiple agents with different exploration parameters (temperatures)
2. **Aggregates** - Combine insights using specialized strategies (MOA for diversity, MCTS for depth)
3. **Verifies** - Cross-check solutions with multiple agents to catch errors
4. **Improves** - Target weak solutions for enhancement based on verification feedback
5. **Synthesizes** - Select the best answer via majority voting, consensus, or synthesis

## Why Multiple Agents?

Language models exhibit **stochasticity** - the same prompt produces different outputs at different temperatures:

- **Low Temperature (0.3)** - Focused, deterministic responses
- **Medium Temperature (0.6)** - Balanced exploration
- **High Temperature (1.0)** - Creative, diverse responses

By running all three in parallel, MARS captures different reasoning paths and can pick the best one.

## The 5-Phase Pipeline

### Phase 1: Multi-Agent Exploration
- Spawn N agents (default: 3)
- Each explores with different temperature
- Generate initial solutions in parallel
- Store all solutions in shared workspace

### Phase 2a: Solution Aggregation
Choose a strategy to combine and refine solutions:
- **MOA** - Generate completions, critique, synthesize (horizontal diversity)
- **MCTS** - Tree exploration with UCB selection (vertical depth)
- **RSA** - Population-based iterative refinement

### Phase 2b: Strategy Network
- Extract successful strategies from solutions
- Identify patterns that work well
- Share insights across agents
- Generate enhanced solutions

### Phase 3: Verification
- Cross-agent verification of all solutions
- Consensus requirement: 2 consecutive "CORRECT" assessments
- Detailed feedback on strengths and weaknesses

### Phase 4: Iterative Improvement
- Target unverified solutions
- Use feedback to guide improvements
- Re-verify improved solutions
- Repeat up to max iterations (default: 5)

### Phase 5: Final Synthesis
- **Majority Voting** - If 2+ agents agree, use that answer
- **Best Verified** - Otherwise use highest-scoring verified solution
- **Synthesis** - If no consensus, synthesize from top 3
- Clean up answer and return result

## When to Use MARS

### ✅ Good For:
- Complex reasoning tasks (math, logic, proofs)
- Multi-step problem solving
- Tasks where accuracy > speed
- Problems with multiple solution approaches
- Tasks requiring verification and improvement

### ❌ Not Ideal For:
- Simple factual questions (single-agent LLM sufficient)
- Real-time applications (MARS takes 2-5 minutes)
- Tasks with fixed output format (verification becomes complex)
- Cost-sensitive applications (MARS uses 50K-200K tokens)

## Architecture Diagram

```
Input Query
    ↓
┌─────────────────────────────────┐
│  Phase 1: Exploration           │
│  3 Agents × 3 Temperatures      │
│  (temp: 0.3, 0.6, 1.0)          │
└─────────────────────────────────┘
    ↓ (3 solutions)
┌─────────────────────────────────┐
│  Phase 2a: Aggregation          │
│  MOA / MCTS / RSA               │
│  (synthesize improved solutions) │
└─────────────────────────────────┘
    ↓ (6+ solutions)
┌─────────────────────────────────┐
│  Phase 2b: Strategy Network     │
│  (extract and share strategies) │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Phase 3: Verification          │
│  Cross-agent verification       │
│  (2-pass consensus)             │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Phase 4: Improvement           │
│  (max 5 iterations)             │
│  Re-verify improved solutions   │
└─────────────────────────────────┘
    ↓
┌─────────────────────────────────┐
│  Phase 5: Final Synthesis       │
│  • Majority voting              │
│  • Best verified                │
│  • Synthesized answer           │
└─────────────────────────────────┘
    ↓
Final Answer
```

## Key Metrics

| Metric | Value |
|--------|-------|
| **Test Coverage** | 43 unit + 29 integration tests |
| **Code Size** | ~3,500 LOC |
| **Typical Runtime** | 2-5 minutes |
| **Token Usage** | 50K-200K per task |
| **Temperature Variants** | 3 (0.3, 0.6, 1.0) |
| **Max Iterations** | 5 (improvement phase) |
| **Verification Threshold** | 2-pass consensus |

## Comparison with Alternatives

| Feature | MARS | Ensemble | Tree-of-Thought | Single-Agent |
|---------|------|----------|-----------------|--------------|
| Multiple agents | ✅ | ✅ | ❌ | ❌ |
| Temperature diversity | ✅ | ✅ | ❌ | ❌ |
| Verification | ✅ | ❌ | ❌ | ❌ |
| Iterative improvement | ✅ | ❌ | ✅ | ❌ |
| Specialized aggregation | ✅ | ❌ | ✅ | ❌ |
| Strategy sharing | ✅ | ❌ | ❌ | ❌ |
| Production ready | ✅ | ✅ | ❌ | ✅ |

## Next Steps

- [Installation](installation.md) - Set up MARS in your project
- [Quick Start](quick-start.md) - Run your first example
- [Architecture Details](../architecture/overview.md) - Deep dive into the system
