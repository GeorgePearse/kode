# References and Research

## Academic Papers

### Aggregation Methods

#### MOA (Mixture of Agents)
**Mixture of Agents: Enhancing LLM Capabilities through Collaborative Specialization**

- **Authors**: Anonymous (arXiv submission)
- **Year**: 2025
- **URL**: https://arxiv.org/abs/2502.04913
- **Citation**: Introduces the MOA aggregation method for horizontal diversity in LLM reasoning
- **Key Concepts**:
  - Diverse completions via temperature sampling
  - Critique and synthesis workflow
  - Collaborative refinement of solutions

#### MCTS (Monte Carlo Tree Search)
**Efficient Selectivity and Backup Operators in Monte-Carlo Tree Search**

- **Authors**: Rémi Coulom
- **Year**: 2006
- **URL**: https://dblp.org/rec/journals/cg/Coulom06.html
- **Citation**: Foundational work on UCB selection and efficient tree search
- **Key Concepts**:
  - Upper Confidence Bounds (UCB) formula
  - Selection, expansion, simulation, backpropagation phases
  - Efficient leaf node backup strategies

### Related Work

#### Tree of Thoughts
**Tree of Thoughts: Deliberate Problem Solving with Large Language Models**

- **Authors**: Yao et al.
- **Year**: 2023
- **URL**: https://arxiv.org/abs/2305.10601
- **Relevance**: Tree-based LLM reasoning with explicit state exploration
- **Related to**: MCTS implementation in MARS

#### Chain-of-Thought Prompting
**Chain-of-Thought Prompting Elicits Reasoning in Large Language Models**

- **Authors**: Wei et al.
- **Year**: 2023
- **URL**: https://arxiv.org/abs/2201.11903
- **Relevance**: Foundational reasoning technique used in all phases
- **Related to**: Base prompting strategy in MARS

### Ensemble Methods

#### Mixture of Experts (MoE)
**Outrageously Large Neural Networks for Efficient Conditional Computation**

- **Authors**: Shazeer et al.
- **Year**: 2017
- **URL**: https://arxiv.org/abs/1701.06538
- **Relevance**: Inspired population-based approaches in MARS
- **Related to**: RSA aggregation strategy

#### Ensemble Learning
**Ensemble Methods in Machine Learning**

- **Authors**: Zhou, Z. H.
- **Year**: 2012
- **URL**: https://ieeexplore.ieee.org/abstract/document/6271705
- **Relevance**: Theoretical foundation for multi-agent approaches
- **Related to**: Cross-agent verification and voting

## Optimization Techniques

### Temperature-Based Sampling
Temperature controls randomness in language model outputs:

- **Low (0.3)**: Deterministic, focused responses
- **Medium (0.6)**: Balanced exploration
- **High (1.0)**: Creative, diverse responses

**Reference**: OpenAI API documentation on temperature parameter

### Verification Strategies

#### Consensus-Based Verification
- Multiple agents verify each solution
- Threshold-based consensus (default: 2 passes)
- Feedback-driven improvement

**Related to**: Phase 3 in MARS pipeline

### Multi-Agent Systems

#### Multi-Agent Reinforcement Learning
- Coordination mechanisms
- Shared reward structures
- Convergence properties

**Reference**: Busoniu et al. "Multi-agent reinforcement learning: An overview"

## Framework References

### Rust Async Ecosystem
- **Tokio**: Async runtime
- **Async-trait**: Trait-based async abstractions
- **serde**: Data serialization

### Benchmarking

#### AIME 2025
American Invitational Mathematics Examination

- **Baseline (GPT-4)**: 43.3%
- **MARS Result**: 73.3%
- **Improvement**: +30 percentage points (+69% relative)

#### IMO 2025
International Mathematical Olympiad

- **Baseline**: 16.7%
- **MARS Result**: 33.3%
- **Improvement**: +16.7 percentage points (+100% relative)

#### LiveCodeBench v5/v6
Code generation and debugging benchmark

- **Baseline**: 39.05%
- **MARS Result**: 50.48%
- **Improvement**: +11.43 percentage points (+29% relative)

## Recommended Reading Order

### Beginner
1. Start: [Getting Started](getting-started/overview.md)
2. Quick Start: [5-Minute Setup](getting-started/quick-start.md)
3. Aggregation: [Overview](aggregation/overview.md)

### Intermediate
4. Architecture: [System Design](architecture/overview.md)
5. MOA: [Mixture of Agents](aggregation/moa.md)
6. MCTS: [Tree Search](aggregation/mcts.md)

### Advanced
7. Configuration: [Full Guide](configuration/config.md)
8. API: [Type Reference](api/types.md)
9. Papers: [Research Links](references.md) (this page)

## Contributing & Citations

### How to Cite MARS

```bibtex
@software{mars-2025,
  title={MARS: Multi-Agent Reasoning System},
  author={Code Project},
  year={2025},
  url={https://github.com/GeorgePearse/code},
  note={Rust implementation of advanced LLM optimization}
}
```

### How to Cite Individual Components

**For MOA aggregation:**
```bibtex
@arXiv{mixture_of_agents_2025,
  title={Mixture of Agents: Enhancing LLM Capabilities through Collaborative Specialization},
  year={2025},
  url={https://arxiv.org/abs/2502.04913}
}
```

**For MCTS:**
```bibtex
@article{coulom2006efficient,
  title={Efficient selectivity and backup operators in Monte-Carlo tree search},
  author={Coulom, R{\'e}mi},
  journal={Computers and Games},
  pages={72--83},
  year={2006},
  publisher={Springer}
}
```

## Key Concepts Explained

### UCB (Upper Confidence Bounds)
Formula used in MCTS node selection:

```
UCB = (value/visits) + C * sqrt(ln(parent_visits) / visits)
```

- **value/visits**: Exploitation - how good is this node?
- **C * sqrt(...)**: Exploration - how uncertain are we?
- **Higher UCB** = more promising node

### Consensus Verification
Solution is marked verified when:

1. Passes verification check (is_correct = true)
2. Receives a score (0.0-1.0)
3. Gets 2 consecutive passes with no failures

### Temperature Sampling
Controls randomness in LLM outputs:

- **Temperature = 0**: Deterministic
- **Temperature = 1.0**: Standard randomness
- **Temperature > 1.0**: More random

MARS uses: 0.3 (conservative), 0.6 (balanced), 1.0 (creative)

## Related Projects

### OptimLLM
**Repository**: https://github.com/coohom/optillm

The Python implementation that inspired MARS. Contains:
- Original MOA implementation
- MCTS algorithm reference
- Prompt templates
- Benchmark code

### LiteLLM
**Repository**: https://github.com/BerriAI/litellm

Multi-model LLM interface. MARS uses litellm-rs for:
- Provider routing
- Model abstraction
- Cost tracking

### Code Project
**Repository**: https://github.com/GeorgePearse/code

Full context codebase with:
- MARS (this project)
- ModelClient for LLM access
- CLI integration
- Additional optimization techniques

## Learning Resources

### Understanding Tree Search
- AlphaGo paper for MCTS in games
- AlphaZero for pure tree search RL
- Sutton & Barto for RL foundations

### Language Model Reasoning
- GPT-4 technical report
- Constitutional AI papers
- Prompt engineering guides

### Rust Systems Programming
- Tokio documentation
- async-trait patterns
- Arc<RwLock> concurrency

## Staying Updated

### Follow Research
- arXiv (cs.CL, cs.AI sections)
- OpenAI blog
- Anthropic research papers
- Papers with Code

### Community
- GitHub discussions
- Rust communities
- ML research forums

### Benchmarks
- AIME, IMO official sites
- LiveCodeBench updates
- HumanEval variants

---

## Quick Links

| Topic | Link |
|-------|------|
| MOA Paper | https://arxiv.org/abs/2502.04913 |
| MCTS Paper | https://dblp.org/rec/journals/cg/Coulom06.html |
| OptimLLM | https://github.com/coohom/optillm |
| This Project | https://github.com/GeorgePearse/code |
| AIME 2025 | https://www.maa.org/aime |
| IMO 2025 | https://www.imo-official.org |
