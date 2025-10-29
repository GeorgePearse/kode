# kode

&ensp;

<p align="center">
  <img src="docs/logo.png" alt="Kode Logo" width="400">
</p>

&ensp;

**kode** is a fast, local coding agent for your terminal. It's a community-driven fork of `openai/codex` focused on real developer ergonomics: Browser integration, multi-agents, theming, and reasoning control — all while staying compatible with upstream.

&ensp;
## What's new in v0.4.0 (October 26, 2025)

- **Auto Drive upgraded** – hand `/auto` a task and it now plans, coordinates agents, reruns checks, and recovers from hiccups without babysitting.
- **Unified settings** – `/settings` centralizes limits, model routing, themes, and CLI integrations so you can audit configuration in one place.
- **Card-based activity** – Agents, browser sessions, web search, and Auto Drive render as compact cards with drill-down overlays for full logs.
- **Turbocharged performance** – History rendering and streaming were optimized to stay smooth even during long multi-agent sessions.
- **Smarter agents** – Mix and match orchestrator CLIs (Claude, Gemini, GPT-5, Qwen, and more) per `/plan`, `/code`, or `/solve` run.

Read the full notes in `release-notes/RELEASE_NOTES.md`.

&ensp;
## Why kode

- 🚀 **Auto Drive orchestration** – Multi-agent automation that now self-heals and ships complete tasks.
- 🌐 **Browser Integration** – CDP support, headless browsing, screenshots captured inline.
- 🤖 **Multi-agent commands** – `/plan`, `/code` and `/solve` coordinate multiple CLI agents.
- 🧭 **Unified settings hub** – `/settings` overlay for limits, theming, approvals, and provider wiring.
- 🎨 **Theme system** – Switch between accessible presets, customize accents, and preview live via `/themes`.
- 🔌 **MCP support** – Extend with filesystem, DBs, APIs, or your own tools.
- 🔒 **Safety modes** – Read-only, approvals, and workspace sandboxing.

&ensp;
## Related Tools & Comparison

kode is part of an evolving ecosystem of AI coding agents. Here's how it compares to other tools:

### CLI Tools

- **[Aider](https://aider.chat/)** – Git-aware AI pair programmer with code editing capabilities
- **[OpenCode](https://github.com/sst/opencode)** – Computer use-enabled coding agent
- **[Cline](https://github.com/cline/cline)** – VS Code extension for agentic coding workflows
- **[Goose](https://github.com/block/goose)** – Extensible coding agent with custom tool support
- **[Plandex](https://plandex.ai/)** – Task-driven planning and execution agent
- **[Gemini CLI](https://github.com/google/gemini-cli)** – Google's command-line AI agent
- **[AIChat](https://github.com/sigoden/aichat)** – Lightweight terminal AI chat with code support

### IDE Extensions

- **[Continue](https://continue.dev/)** – Open-source VS Code/JetBrains copilot alternative
- **[Roo Code](https://github.com/RooVetGit/Roo-Cline)** – Enhanced agentic coding in your IDE
- **[Tabby](https://www.tabby.ai/)** – Self-hosted code completion and chat

### Local Infrastructure

- **[Ollama](https://ollama.ai/)** – Run large language models locally
- **[LM Studio](https://lmstudio.ai/)** – User-friendly local LLM management

### Code Understanding

- **[Serena](https://github.com/oraios/serena)** – Semantic code analysis toolkit (planned kode integration)
- **[Brokk](https://github.com/BrokkAi/brokk)** – Code understanding and analysis platform

## Implementation Patterns

kode demonstrates key patterns in modern AI agent design:

- **Multi-model support** – Switch between Claude, Gemini, GPT-5, Qwen without rewriting
- **MCP integration** – Standardized protocol for extending agent capabilities
- **Context management** – Intelligent codebase analysis and memory across sessions
- **Sandboxed execution** – Safe code execution with approval workflows
- **CLI/Extension duality** – Works as standalone CLI or alongside IDE tools
- **Provider independence** – Not locked to single AI vendor or provider
- **Client/server architecture** – Browser CDP, agent coordination, modular design
- **Self-hosted options** – Run locally or point to custom endpoints
- **Custom commands** – `/plan`, `/code`, `/solve` provide semantic task decomposition
- **Plan/Act separation** – Reasoning and execution as distinct phases

&ensp;
## AI Videos

&ensp;
<p align="center">
  <a href="https://youtu.be/UOASHZPruQk">
    <img src="docs/screenshots/video-auto-drive-new-play.jpg" alt="Play Introducing Auto Drive video" width="100%">
  </a><br>
  <strong>Auto Drive Overview</strong>
</p>

&ensp;
<p align="center">
  <a href="https://youtu.be/sV317OhiysQ">
    <img src="docs/screenshots/video-v03-play.jpg" alt="Play Multi-Agent Support video" width="100%">
  </a><br>
  <strong>Multi-Agent Promo</strong>
</p>



&ensp;
## Roadmap & Future Plans

### Visual Agent Forking (Planned)

A file finder-style TUI interface for visually managing agent task branches and orchestration:

**Concept:**
- **File Finder Layout** – Hierarchical tree view of agent forks and task variants, similar to VS Code's file explorer or FZF preview pane
- **Task Branching** – Visually fork agent tasks to explore multiple solution paths in parallel
- **Real-time Comparison** – Side-by-side diff view of different agent implementations
- **Branch Selection** – Quick navigation and selection between agent variants using arrow keys, fuzzy search, and preview
- **Convergence Control** – Choose which forked branch to keep, merge, or discard before consolidating back into the main conversation

**Use Cases:**
- Compare how different agents (`/code` with Claude vs Gemini vs GPT-5) would solve the same task
- Explore multiple architectural approaches without losing context
- Test risky refactorings in isolated branches before committing
- Build decision trees for complex multi-step problems where each agent explores different options

**Implementation approach:**
- Tree-based rendering with collapsible branches
- Integrated with Auto Drive for seamless orchestration
- Keyboard-driven navigation (arrows, Enter to select, 'd' to delete branch, 'm' to merge)
- Status indicators: running, completed, failed, pending user review

### Distributed Tracing & Observability (Planned)

Comprehensive trace capture and visualization for agent execution flows:

**Concept:**
- **Distributed Tracing** – Full request/response traces across agent orchestration, API calls, and tool execution
- **Trace Storage** – Persistent storage of execution traces for audit, debugging, and performance analysis
- **Timeline Visualization** – Waterfall-style trace visualization showing dependencies and durations
- **Root Cause Analysis** – Drill-down into specific traces to understand failures and performance bottlenecks

**Integration Options:**
- **SigNoz** – Open-source, self-hosted observability platform with metrics, traces, and logs in a single dashboard (similar UX to Datadog)
- **Apache SkyWalking** – APM-focused platform with strong multi-language support and cloud-native optimizations
- **OneUptime** – Self-hosted alternative combining traces, logs, uptime monitoring, and incident management
- **Local Storage** – Optional built-in trace storage using SQLite or PostgreSQL for teams preferring on-device data

**Use Cases:**
- Debug why an agent fork took longer than expected
- Trace API latency issues across multiple agent calls
- Audit compliance: store and review all agent decisions and tool invocations
- Performance profiling of `/auto` multi-step orchestrations
- Export traces for external analysis or compliance reporting

**Implementation approach:**
- OpenTelemetry instrumentation throughout kode for standardized trace collection
- Optional backend configuration (SigNoz, SkyWalking, OneUptime, or local)
- Inline trace inspection via `/traces` command with filters and search
- Automatic trace linking between parent and child agent tasks
- Export as JSON, OTLP, or integration-specific formats

### Performance Optimization: In-Memory File Cache (Planned)

An in-memory cache layer for frequently read or edited files during a session:

**Concept:**
- **Session-scoped cache** – Store file contents and edits in memory during active development to avoid redundant disk reads
- **Edit tracking** – Track modifications made during the session to detect conflicts or stale reads
- **Smart invalidation** – Automatically invalidate cache when files are modified externally or by other agents
- **Stats and monitoring** – Inspect cache hit rates and memory usage via `/cache` command

**Benefits:**
- Faster repeated file reads during multi-agent planning and code execution
- Reduced disk I/O during intensive refactoring or analysis tasks
- Better performance for large codebases where file operations are a bottleneck
- Session consistency when agents read and edit the same files

**Implementation approach:**
- LRU (Least Recently Used) cache with configurable max size
- Track file timestamps to detect external modifications
- Expose cache management via CLI (inspect, clear, stats)
- Optional persistent cache across sessions for large stable files

**Inspiration:**
- [rust-analyzer metrics dashboard](https://rust-analyzer.github.io/metrics/) – Real-world example of performance monitoring and acceleration tracking across time periods. Demonstrates effective visualization of code acceleration metrics and performance trends.

&ensp;
## Inspirations & References

kode's design and roadmap are informed by proven patterns from leading open-source projects:

### Agent Orchestration & Multi-Agent Workflows
- **[ModelScope ms-agent](https://github.com/modelscope/ms-agent)** – Agent-centric orchestration with async/await patterns, chain-based multi-agent workflows, pluggable tool/plugin system, and configuration-driven lifecycle management. Demonstrates clear callback-based observability patterns.

### Metrics, Benchmarking & Performance Tracking
- **[rust-analyzer](https://github.com/rust-lang/rust-analyzer)** – Comprehensive metrics collection via xtask, JSONL-based historical data storage, and GitHub Pages visualization with Plotly.js. Architecture shows how to track multiple dimensions (build time, inference time, memory usage) across projects with minimal infrastructure overhead.
- **[PyKEEN](https://github.com/pykeen/pykeen)** – Multi-backend result tracking abstraction (W&B, TensorBoard, MLflow, CSV, JSON), hierarchical metric system with 20+ metric variants, pluggable callback system, and automatic memory optimization through binary search. Demonstrates reproducible benchmarking pipelines.

### Code Understanding & Codebase Navigation
- **[Aider repomap](https://aider.chat/2023/10/22/repomap.html)** – Tree-sitter based repository mapping for compressed codebase context, semantic ranking of code entities through graph algorithms, and token-efficient code representation for LLMs. Shows how to balance context size with information density.
- **[sombra](https://github.com/maskdotdev/sombra)** – Multi-layer graph database architecture with property graphs, hybrid indexing strategy (BTree + Label + Property), and traversal-first code understanding. Demonstrates how to expose code structure as queryable tools for LLM agents.
- **[IndraDB](https://github.com/indradb/indradb)** – Layered query abstraction with composable pipes, indexed property-based filtering for fast lookups, pluggable datastore abstraction (Memory, RocksDB, PostgreSQL), and bi-directional edge traversal. Applicable to building queryable code dependency graphs.

### Context Management & Codebase Representation
- **[code2prompt](https://github.com/mufeedvh/code2prompt)** – Layered context pipeline with intelligent file filtering, per-file token counting, template-driven output flexibility, and Git context integration. Shows how to incrementally load codebase context with budget constraints rather than materializing full trees.

&ensp;
## Quickstart

### Run

```bash
npx -y @just-every/kode
```

### Install & Run

```bash
npm install -g @just-every/kode
kode // or `koder` if you're using VS Code
```

Note: If another tool already provides a `kode` command (e.g. VS Code), our CLI is also installed as `koder`. Use `koder` to avoid conflicts.

**Authenticate** (one of the following):
- **Sign in with ChatGPT** (Plus/Pro/Team; uses models available to your plan)
  - Run `kode` and pick "Sign in with ChatGPT"
- **API key** (usage-based)
  - Set `export OPENAI_API_KEY=xyz` and run `kode`

### Install Claude & Gemini (optional)

kode supports orchestrating other AI CLI tools. Install these and config to use alongside kode.

```bash
# Ensure Node.js 20+ is available locally (installs into ~/.n)
npm install -g n
export N_PREFIX="$HOME/.n"
export PATH="$N_PREFIX/bin:$PATH"
n 20.18.1

# Install the companion CLIs
export npm_config_prefix="${npm_config_prefix:-$HOME/.npm-global}"
mkdir -p "$npm_config_prefix/bin"
export PATH="$npm_config_prefix/bin:$PATH"
npm install -g @anthropic-ai/claude-code @google/gemini-cli @qwen-code/qwen-code

# Quick smoke tests
claude --version
gemini --version
qwen --version
```

> ℹ️ Add `export N_PREFIX="$HOME/.n"` and `export PATH="$N_PREFIX/bin:$PATH"` (plus the `npm_config_prefix` bin path) to your shell profile so the CLIs stay on `PATH` in future sessions.

&ensp;
## Commands

### Browser
```bash
# Connect code to external Chrome browser (running CDP)
/chrome        # Connect with auto-detect port
/chrome 9222   # Connect to specific port

# Switch to internal browser mode
/browser       # Use internal headless browser
/browser https://example.com  # Open URL in internal browser
```

### Agents
```bash
# Plan code changes (Claude, Gemini and GPT-5 consensus)
# All agents review task and create a consolidated plan
/plan "Stop the AI from ordering pizza at 3AM"

# Solve complex problems (Claude, Gemini and GPT-5 race)
# Fastest preferred (see https://arxiv.org/abs/2505.17813)
/solve "Why does deleting one user drop the whole database?"

# Write code! (Claude, Gemini and GPT-5 consensus)
# Creates multiple worktrees then implements the optimal solution
/code "Show dark mode when I feel cranky"
```

### Auto Drive
```bash
# Hand off a multi-step task; Auto Drive will coordinate agents and approvals
/auto "Refactor the auth flow and add device login"

# Resume or inspect an active Auto Drive run
/auto status
```

### General
```bash
# Try a new theme!
/themes

# Change reasoning level
/reasoning low|medium|high

# Switch models or effort presets
/model

# Start new conversation
/new
```

## CLI reference

```shell
kode [options] [prompt]

Options:
  --model <name>        Override the model (gpt-5, claude-opus, etc.)
  --read-only          Prevent file modifications
  --no-approval        Skip approval prompts (use with caution)
  --config <key=val>   Override config values
  --oss                Use local open source models
  --sandbox <mode>     Set sandbox level (read-only, workspace-write, etc.)
  --help              Show help information
  --debug             Log API requests and responses to file
  --version           Show version number
```

&ensp;
## Memory & project docs

kode can remember context across sessions:

1. **Create an `AGENTS.md` or `CLAUDE.md` file** in your project root:
```markdown
# Project Context
This is a React TypeScript application with:
- Authentication via JWT
- PostgreSQL database
- Express.js backend

## Key files:
- `/src/auth/` - Authentication logic
- `/src/api/` - API client code
- `/server/` - Backend services
```

2. **Session memory**: kode maintains conversation history
3. **Codebase analysis**: Automatically understands project structure

&ensp;
## Non-interactive / CI mode

For automation and CI/CD:

```shell
# Run a specific task
kode --no-approval "run tests and fix any failures"

# Generate reports
kode --read-only "analyze code quality and generate report"

# Batch processing
kode --config output_format=json "list all TODO comments"
```

&ensp;
## Model Context Protocol (MCP)

kode supports MCP for extended capabilities:

- **File operations**: Advanced file system access
- **Database connections**: Query and modify databases
- **API integrations**: Connect to external services
- **Custom tools**: Build your own extensions

Configure MCP in `~/.kode/config.toml` Define each server under a named table like `[mcp_servers.<name>]` (this maps to the JSON `mcpServers` object used by other clients):

```toml
[mcp_servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/project"]
```

&ensp;
## Configuration

Main config file: `~/.kode/config.toml`

> [!NOTE]
> kode reads from both `~/.kode/` and `~/.codex/` for backwards compatibility, but it only writes updates to `~/.kode/`. If you switch back to Codex and it fails to start, remove `~/.codex/config.toml`. If kode appears to miss settings after upgrading, copy your legacy `~/.codex/config.toml` into `~/.kode/`.

```toml
# Model settings
model = "gpt-5"
model_provider = "openai"

# Behavior
approval_policy = "on-request"  # untrusted | on-failure | on-request | never
model_reasoning_effort = "medium" # low | medium | high
sandbox_mode = "workspace-write"

# UI preferences see THEME_CONFIG.md
[tui.theme]
name = "light-photon"

# Add config for specific models
[profiles.gpt-5]
model = "gpt-5"
model_provider = "openai"
approval_policy = "never"
model_reasoning_effort = "high"
model_reasoning_summary = "detailed"
```

### Environment variables

- `KODE_HOME`: Override config directory location
- `OPENAI_API_KEY`: Use API key instead of ChatGPT auth
- `OPENAI_BASE_URL`: Use alternative API endpoints
- `OPENAI_WIRE_API`: Force the built-in OpenAI provider to use `chat` or `responses` wiring

&ensp;
## FAQ

**How is this different from the original?**
> This fork adds browser integration, multi-agent commands (`/plan`, `/solve`, `/code`), theme system, and enhanced reasoning controls while maintaining full compatibility.

**Can I use my existing Codex configuration?**
> Yes. kode reads from both `~/.kode/` (primary) and legacy `~/.codex/` directories. We only write to `~/.kode/`, so Codex will keep running if you switch back; copy or remove legacy files if you notice conflicts.

**Does this work with ChatGPT Plus?**
> Absolutely. Use the same "Sign in with ChatGPT" flow as the original.

**Is my data secure?**
> Yes. Authentication stays on your machine, and we don't proxy your credentials or conversations.

&ensp;
## Contributing

We welcome contributions! This fork maintains compatibility with upstream while adding community-requested features.

### Development workflow

```bash
# Clone and setup
git clone https://github.com/just-every/kode.git
cd kode
npm install

# Build (use fast build for development)
./build-fast.sh

# Run locally
./code-rs/target/dev-fast/code
```

### Opening a pull request

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Build successfully: `./build-fast.sh`
6. Submit a pull request


&ensp;
## Legal & Use

### License & attribution
- This project is a community fork of `openai/codex` under **Apache-2.0**. We preserve upstream LICENSE and NOTICE files.
- **kode** is **not** affiliated with, sponsored by, or endorsed by OpenAI.

### Your responsibilities
Using OpenAI, Anthropic or Google services through kode means you agree to **their Terms and policies**. In particular:
- **Don't** programmatically scrape/extract content outside intended flows.
- **Don't** bypass or interfere with rate limits, quotas, or safety mitigations.
- Use your **own** account; don't share or rotate accounts to evade limits.
- If you configure other model providers, you're responsible for their terms.

### Privacy
- Your auth file lives at `~/.kode/auth.json`
- Inputs/outputs you send to AI providers are handled under their Terms and Privacy Policy; consult those documents (and any org-level data-sharing settings).

### Subject to change
AI providers can change eligibility, limits, models, or authentication flows. kode supports **both** ChatGPT sign-in and API-key modes so you can pick what fits (local/hobby vs CI/automation).

&ensp;
## License

Apache 2.0 - See [LICENSE](LICENSE) file for details.

This project is a community fork of the original Codex CLI. We maintain compatibility while adding enhanced features requested by the developer community.

&ensp;
---
**Need help?** Open an issue on [GitHub](https://github.com/just-every/kode/issues) or check our documentation.
