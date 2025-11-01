# In-Memory Edits Architecture

## Overview

This document proposes a full implementation of in-memory edits for coding agents. Instead of agents manipulating files directly on disk, they would operate on a rich in-memory representation of the codebase, providing significant performance and architectural benefits.

## Core Motivation

Current file-based coding agent architectures have inherent performance limitations:
- **I/O Overhead**: File read/write operations are orders of magnitude slower than memory access (microseconds vs nanoseconds)
- **Repeated Parsing**: Files get parsed multiple times for each agent operation
- **Limited Context**: No persistent semantic model between agent calls
- **Filesystem Friction**: Multiple grep/ast-grep passes for cross-file navigation

In-memory architectures (like Rust Analyzer and IntelliJ IDEA) maintain rich semantic models that enable instant lookups and sophisticated analysis. A similar approach for coding agents would unlock:
- **O(1) symbol lookups** instead of O(n) grep passes
- **Instant cross-file references** via dependency graphs
- **Cached type information** for instant validation
- **Transactional batch edits** with holistic validation
- **Persistent agent context** between calls

## Architecture

### 1. In-Memory Representation Layer

```
CodeSession (container)
├── FileBuffer (per file)
│   ├── Raw content
│   ├── Parsed AST
│   ├── Language-specific representation
│   └── Change tracking
├── SemanticIndex
│   ├── Symbol table
│   ├── Type information
│   ├── Dependency graph
│   └── Reference index
└── ChangeLog
    ├── Sequential edits
    └── Rollback/replay capability
```

**CodeSession**: Main container holding all code state for a session
- Maintains parsed AST/semantic models for each file
- Tracks original vs. current state
- Builds and maintains dependency graphs
- Manages language-specific parsers

**FileBuffer**: Per-file representation
- Raw content + parsed representation (AST, tokens)
- Change tracking (character-level diffs)
- Language-specific AST nodes
- Support for Python (via `ast`), TypeScript (via `ts-parser`), Go, Rust, etc.

**SemanticIndex**: Cross-file semantic information
- Symbol table: maps identifiers to definitions
- Type information: inferred/annotated types
- Dependency graph: file and module dependencies
- Reference index: all usages of each symbol

### 2. Modification Tracking

**ChangeLog**: Immutable record of all edits
- Sequential edits with metadata (timestamp, agent, context)
- Enables rollback/replay of changes
- Allows undo/redo operations
- Provides audit trail of agent decisions

**Dirty Files**: Track which files have been modified
- Used for batch writing to disk
- Enables partial sync strategies

**Semantic Validation**: After each edit
- Validate syntax remains valid
- Type checking (language-specific)
- Dependency consistency
- No broken intermediate states

### 3. Diff Management System

**GitDiffBranch**: Parallel git branch tracking modifications
- Auto-update after each batch of edits
- Can be reviewed independently before PR
- SHA stored in CodeSession for easy reference
- Enables "review" mode before merging

**DiffGenerator**: Efficient diffing
- Show exact changes made by agents
- Useful for understanding agent decisions
- Supports multiple diff formats (unified, structured, etc.)

### 4. Agent Integration Interface

```python
class CodeAgent:
    def read_file(self, path: str) -> str:
        """Returns current in-memory content"""

    def edit_file(self, path: str, old_str: str, new_str: str) -> bool:
        """Apply edit in memory, validate, return success"""

    def get_symbols(self, file: str = None) -> List[Symbol]:
        """Instant symbol lookup, optionally filtered by file"""

    def find_references(self, symbol: str) -> List[Location]:
        """O(1) lookup of all usages vs O(n) grep"""

    def validate(self) -> List[Error]:
        """Instant type/syntax checking without external tools"""

    def batch_edits(self, edits: List[Edit]) -> bool:
        """Queue edits, validate holistically, commit atomically"""

    def get_diff(self) -> str:
        """Current diff from original state"""
```

**Key Properties**:
- Agents operate entirely on in-memory representations
- No filesystem access needed for reads (only for initial load and final sync)
- Batch operations validate transactionally
- Instant feedback on validity of changes

### 5. Sync Strategy

**Three modes of operation**:

1. **On-Demand Sync**: Write files back when:
   - Agent completes a logical unit of work
   - Session is checkpointed
   - Before validation/testing

2. **Write-Through**: Keep filesystem in sync with in-memory state
   - Higher I/O but maximum safety
   - Files always inspectable

3. **Lazy Write**: Batch all writes until session end
   - Minimizes I/O
   - Risk if process crashes

**Rollback Safety**: Can revert to last known-good file state

### 6. Git Integration Workflow

```
Worktree (original state)
    ↓ (load)
CodeSession (in-memory edits)
    ↓ (syncs to disk, run_in_background)
Working directory files
    ↓ (git add/commit)
Feature branch (staged diffs)
    ↓ (parallel track, auto-update)
in-memory-edits branch (review reference)
    ↓ (shows exact agent changes)
PR with full context
```

This workflow:
- Loads files once at session start
- Agents work entirely in memory
- Syncs to disk periodically for inspection/testing
- Maintains parallel diff branch for visibility
- Final PR shows all agent changes

### 7. Performance Optimizations

**Memory Efficiency**:
- **Lazy Parsing**: Only parse files when accessed
- **Incremental Updates**: AST updates for specific changes, not re-parsing
- **Caching**: Cache symbol tables, type information, call graphs
- **Memory Management**: LRU eviction for very large codebases
- **Batched Writes**: Aggregate file writes to reduce I/O

**Lookup Performance**:
- Symbol resolution: hash table O(1)
- Reference finding: indexed list O(1)
- Type inference: cached O(1)
- Dependency graph navigation: adjacency list O(degree)

## Implementation Phases

### Phase 1: Core Engine
- [ ] CodeSession and FileBuffer classes
- [ ] Language-specific parsers (start with Python)
- [ ] Change tracking and validation
- [ ] File sync back to disk
- [ ] Basic edit operations

### Phase 2: Agent Interface
- [ ] CodeAgent API wrapper
- [ ] File read/edit operations in memory
- [ ] Symbol resolution (no filesystem access)
- [ ] Batch edit validation
- [ ] Integration with existing agents

### Phase 3: Git Integration
- [ ] Auto-update diff branch with edits
- [ ] Diff generation and review interface
- [ ] Rollback mechanisms
- [ ] Branch management helpers

### Phase 4: Testing & Validation
- [ ] Type checking without writing files
- [ ] Running tests against in-memory edits
- [ ] Syntax validation
- [ ] Performance benchmarks vs file-based

### Phase 5: Advanced Features
- [ ] Multi-file refactoring atomicity
- [ ] Concurrent agent sessions
- [ ] Semantic caching (call graphs, dependency trees)
- [ ] Cross-language type information

## Performance Comparison

| Operation | File-Based | In-Memory |
|-----------|-----------|-----------|
| Find all usages of symbol | O(n) grep passes | O(1) lookup |
| Cross-file navigation | Filesystem traversal | Graph traversal |
| Type checking | External tool invocation | Cached model |
| Batch edits validation | Run tests repeatedly | Single holistic check |
| Agent context persistence | Lost between calls | Full semantic model |
| File reads per agent call | Multiple (parse per read) | Single load, in-memory reuse |

**Estimated speedups** (on large codebases):
- **Symbol lookups**: 10-100x faster
- **Reference finding**: 50-1000x faster
- **Batch validation**: 5-20x faster
- **Agent session time**: 2-5x faster for multi-file changes

## Key Design Decisions

### 1. Syntax Trees vs Pure Text
- **Decision**: Maintain both
- **Rationale**: Pure text for agents to manipulate, ASTs for validation/analysis
- **Trade-off**: 2x memory vs instant semantic information

### 2. Language Support Strategy
- **Decision**: Plugin system for language-specific implementations
- **Start with**: Python (most common for agents)
- **Add gradually**: TypeScript, Go, Rust, Java

### 3. Sync Frequency
- **Decision**: On-demand + checkpoint-based
- **Rationale**: Balance between safety and performance
- **Default**: Write after each logical unit, before validation

### 4. Validation Strictness
- **Decision**: Fail-fast on syntax errors, warn on type issues
- **Rationale**: Prevent broken intermediate states
- **Allow override**: For experimental/WIP edits

## Configuration

```yaml
# session.yaml
project:
  root: /path/to/worktree
  languages: [python, typescript]

memory:
  lazy_parse: true
  cache_symbols: true
  max_memory_mb: 1024
  lru_eviction: true

git:
  diff_branch: in-memory-edits
  auto_commit: true
  sync_frequency: on_demand

agents:
  features:
    instant_symbol_lookup: true
    batch_validation: true
    persistent_context: true

validation:
  fail_on_syntax_error: true
  warn_on_type_error: true
  run_tests_before_sync: false
```

## Open Questions

1. **Serialization**: How to persist CodeSession state between long-running agent calls?
2. **Conflict Resolution**: What if user edits files while session is active?
3. **Testing Integration**: How do agents run tests against in-memory code?
4. **Scale Limits**: What's the max codebase size before memory becomes an issue?
5. **IDE Compatibility**: Can IDEs read the diff branch for real-time feedback?
6. **Language Coverage**: Which languages to support first, and in what order?

## References

- Rust Analyzer: Rich in-memory semantic model for Rust
- IntelliJ IDEA: High-performance semantic indexing
- Language Servers: How LSPs maintain in-memory state
- Modern Build Systems: Virtual file systems and lazy evaluation

## Next Steps

1. Implement Phase 1 (Core Engine) with Python support
2. Measure performance improvements on typical agent tasks
3. Create agent interface and integrate with existing agents
4. Implement git integration for visibility
5. Expand language support based on usage patterns
