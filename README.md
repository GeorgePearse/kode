# Zeus

```
 ________                    __
/_  __/ /_  ___  _________  / /____
 / / / __ \/ _ \/ ___/ __ \/ / ___/
/ / / / / /  __/ /  / /_/ / (__  )
/_/ /_/ /_/\___/_/   \____/_/____/

           /\
          /  \
         / /\ \
        / /  \ \
       /_/   / /
          / /
         / /
        / /
       / /
      /_/
```

**Zeus** is a fast, local coding agent for your terminal. It's a community-driven fork of `openai/codex` focused on real developer ergonomics: Browser integration, multi-agents, theming, and reasoning control — all while staying compatible with upstream.

Interactions with git take fucking ages, no idea why they're so slow, should maybe automatically swap to a smaller model for that

<img width="460" height="464" alt="image" src="https://github.com/user-attachments/assets/bd152896-ccb7-4dd1-b1aa-768f9b1840d3" />


&ensp;
## What's new in v0.4.0 (October 26, 2025)

- **Auto Drive upgraded** – hand `/auto` a task and it now plans, coordinates agents, reruns checks, and recovers from hiccups without babysitting.
- **Unified settings** – `/settings` centralizes limits, model routing, themes, and CLI integrations so you can audit configuration in one place.
- **Card-based activity** – Agents, browser sessions, web search, and Auto Drive render as compact cards with drill-down overlays for full logs.
- **Turbocharged performance** – History rendering and streaming were optimized to stay smooth even during long multi-agent sessions.
- **Smarter agents** – Mix and match orchestrator CLIs (Claude, Gemini, GPT-5, Qwen, and more) per `/plan`, `/code`, or `/solve` run.

Read the full notes in `release-notes/RELEASE_NOTES.md`.

&ensp;
## Why Zeus

- 🚀 **Auto Drive orchestration** – Multi-agent automation that now self-heals and ships complete tasks.
- 🌐 **Browser Integration** – CDP support, headless browsing, screenshots captured inline.
- 🤖 **Multi-agent commands** – `/plan`, `/code` and `/solve` coordinate multiple CLI agents.
- 🧭 **Unified settings hub** – `/settings` overlay for limits, theming, approvals, and provider wiring.
- 🎨 **Theme system** – Switch between accessible presets, customize accents, and preview live via `/themes`.
- 🔌 **MCP support** – Extend with filesystem, DBs, APIs, or your own tools.
- 🔒 **Safety modes** – Read-only, approvals, and workspace sandboxing.

&ensp;
## Related Tools & Comparison

Zeus is part of an evolving ecosystem of AI coding agents. Here's how it compares to other tools:

### CLI Tools

- **[Aider](https://aider.chat/)** – Git-aware AI pair programmer with code editing capabilities
- **[OpenCode](https://github.com/sst/opencode)** – Computer use-enabled coding agent
- **[Cline](https://github.com/cline/cline)** – VS Code extension for agentic coding workflows
- **[Goose](https://github.com/block/goose)** – Extensible coding agent with custom tool support

### Web IDE & Chat

- **[Cursor](https://www.cursor.com/)** – VS Code-based IDE with integrated Claude autocomplete and chat
- **[Anthropic's Claude](https://claude.ai/)** – Web-based chat with programming capabilities
- **[GitHub Copilot](https://github.com/features/copilot)** – IDE plugin for code completion (no agentic reasoning)

### Related Agent Frameworks

- **[LangGraph](https://langchain-ai.github.io/langgraph/)** – LLM frameworks for agentic workflows with state management
- **[SWE-agent](https://github.com/princeton-nlp/SWE-agent)** – Automated agent for software engineering tasks
- **[Qdrant Supervisor](https://qdrant.tech/)** – Multi-agent framework for autonomous systems

### Code Understanding

- **[Serena](https://github.com/oraios/serena)** – Semantic code analysis toolkit (planned Zeus integration)
- **[Brokk](https://github.com/BrokkAi/brokk)** – Code understanding and analysis platform
- **[Octocode](https://github.com/bgauryy/octocode-mcp)** - Octocode for code/semantic understanding

## Implementation Patterns

Zeus demonstrates key patterns in modern AI agent design:

- **Multi-model support** – Switch between Claude, Gemini, GPT-5, Qwen without rewriting
- **MCP integration** – Standardized protocol for extending agent capabilities
- **Browser automation** – Screenshot + navigation for web-based workflows
- **Multi-agent orchestration** – Coordinate specialized agents for complex tasks
- **Session persistence** – Memory and context across conversations
- **Rich output** – Streaming, tables, formatted code, terminal UI

&ensp;
## Features Roadmap

### Phase 1: Foundation (Current - v0.4.0)

- ✅ CLI chat interface with terminal UI
- ✅ Multi-model support (Claude, GPT-4, Gemini, Qwen)
- ✅ Browser integration (CDP)
- ✅ Basic MCP support
- ✅ Read-only and approval modes
- ✅ Auto Drive orchestration
- ✅ Settings hub

### Phase 2: Intelligence & Observability (Planning)

**Smart context management**
- Relevant code caching for faster responses
- Semantic search across project files
- Intelligent token counting and model routing
- Adaptive context window management

**Observability**
Comprehensive trace capture and visualization for agent execution flows:

- Full request/response logging with structured formatting
- Performance metrics (latency, token usage, cost, concurrency)
- Agent execution DAG visualization
- Error tracking and categorization by severity
- Export traces for external analysis or compliance reporting

**Implementation approach:**
- OpenTelemetry instrumentation throughout Zeus for standardized trace collection
- Optional backend configuration (SigNoz, SkyWalking, OneUptime, or local)
- Inline trace inspection via `/traces` command with filters and search
- Automatic trace linking between parent and child agent tasks

### Phase 3: Advanced Workflows

**Multi-session context management**
- Cross-session memory and learning
- Persistent agent personas
- Collaborative workflows (multiple team members)

**Agentic patterns**
- Specialized micro-agents for specific domains
- Reflection and self-correction patterns
- Multi-turn planning and refinement

**Enterprise features**
- Audit logging and compliance hooks
- Custom model provider support
- Advanced permission and workspace management

&ensp;
## Usage

### Run

```bash
npx -y @just-every/zeus
```

### Install & Run

```bash
npm install -g @just-every/zeus
zeus // or `zeusr` if you're using VS Code
```

Note: If another tool already provides a `zeus` command (e.g. VS Code), our CLI is also installed as `zeusr`. Use `zeusr` to avoid conflicts.

**Authenticate** (one of the following):
- **Sign in with ChatGPT** (Plus/Pro/Team; uses models available to your plan)
  - Run `zeus` and pick "Sign in with ChatGPT"
- **API key** (usage-based)
  - Set `export OPENAI_API_KEY=xyz` and run `zeus`

### Install Claude & Gemini (optional)

Zeus supports orchestrating other AI CLI tools. Install these and config to use alongside Zeus.

```bash
# Ensure Node.js 20+ is available locally (installs into ~/.n)
n install 20

# Install Claude CLI (requires Anthropic API key)
npm install -g @anthropic-ai/claude-cli
claude --version

# Install Gemini CLI
npm install -g @google/generative-ai-cli
gemini --version

# Install Qwen CLI (requires Alibaba Cloud API key)
npm install -g @alibaba-cloud/qwen-cli
qwen --version
```

### Browser
```bash
# Connect Zeus to external Chrome browser (running CDP)
/chrome        # Connect with auto-detect port
/chrome 9222   # Connect to specific port

# Browser sessions are available in command selection dropdown or `/solve` when working with web apps
```

&ensp;
## CLI reference

```shell
zeus [options] [prompt]

Options:
  --model <name>        Override the model (gpt-5, claude-opus, etc.)
  --no-approval         Auto-approve file changes (caution!)
  --read-only           Prevent modifications (analysis only)
  --auto                Start in Auto Drive mode
  --help                Show help message
```

&ensp;
## Memory & project docs

Zeus can remember context across sessions:

1. **Create an `AGENTS.md` or `CLAUDE.md` file** in your project root:
```markdown
# Your project name

This is a React TypeScript application with:
- `/client/` - React components and UI
- `/server/` - Backend services
```

2. **Session memory**: Zeus maintains conversation history
3. **Codebase analysis**: Automatically understands project structure

&ensp;
## Automation & CI/CD

For automation and CI/CD:

```shell
# Run a specific task
zeus --no-approval "run tests and fix any failures"

# Generate reports
zeus --read-only "analyze code quality and generate report"

# Batch processing
zeus --config output_format=json "list all TODO comments"
```

&ensp;
## Model Context Protocol (MCP)

Zeus supports MCP for extended capabilities:

- **File operations**: Advanced file system access
- **Database connections**: Query and modify databases
- **API integrations**: Connect to external services
- **Custom tools**: Build your own extensions

Configure MCP in `~/.zeus/config.toml` Define each server under a named table like `[mcp_servers.<name>]` (this maps to the JSON `mcpServers` object used by other clients):

```toml
[mcp_servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/project"]
```

&ensp;
## Configuration

Configuration is stored in `~/.zeus/config.toml`:

```toml
[general]
default_model = "claude-opus"
approval_mode = "review"  # "review", "auto", or "manual"

[themes]
current = "dark"
accent_color = "blue"

[mcp_servers]
# Your MCP server configurations here
```

&ensp;
## Architecture

### Core Components

- **TUI** – Terminal user interface (Rust using Ratatui)
- **Core Agent** – Multi-model orchestration
- **Tool Runner** – File operations, browser control, MCP
- **Model Provider** – Abstraction for LLM APIs

### Key Patterns

- **Browser integration** – Chrome DevTools Protocol (CDP) for automation
- **Streaming** – Real-time token streaming from all supported LLMs
- **Stateful UI** – Maintains context across tool executions
- **MCP** – Pluggable tool system for extending capabilities

&ensp;
## Development

### Local Development

```bash
# Clone the repo
git clone https://github.com/just-every/zeus.git
cd zeus

# Install dependencies
npm install

# Run in development
npm run dev

# Run tests
npm test
```

### Building from Source

```bash
# Build for your platform
npm run build

# Build for all platforms (requires platform-specific tools)
npm run build:all
```

&ensp;
## Inspirations & References

Zeus's design and roadmap are informed by proven patterns from leading open-source projects:

### Agent Orchestration & Multi-Agent Workflows
- **[ModelScope ms-agent](https://github.com/modelscope/ms-agent)** – Agent-centric orchestration with async/await patterns, chain-based multi-agent workflows, pluggable tool/plugin system, and configuration-driven lifecycle management. Demonstrates clear callback-based observability patterns.
- **[AutoGen](https://github.com/microsoft/autogen)** – Multi-agent conversations, role-based agents, and complex conversation flows with human-in-the-loop patterns. Reference for orchestration state management.
- **[Smolagents](https://github.com/huggingface/smolagents)** – Minimal agent framework with composable tools and LLM-based planning. Shows alternative to hierarchical agent design.

### Terminal UI & Developer Experience
- **[Charm Bubble Tea](https://github.com/charmbracelet/bubbletea)** – Elegant TUI framework (Go) inspiring our streaming and state model
- **[Hurl](https://hurl.dev/)** – CLI tool with great UX design patterns
- **[GitHub CLI](https://github.com/cli/cli)** – Example of well-structured command parsing and output formatting

### Browser Automation & Web Integration
- **[Playwright](https://playwright.dev/)** – Cross-browser automation reference; demonstrates resilient navigation patterns
- **[Puppeteer](https://pptr.dev/)** – Chrome DevTools Protocol patterns and high-level API design

### Configuration & State Management
- **[Nix](https://nixos.org/)** – Declarative system configuration patterns
- **[Just](https://github.com/casey/just)** – Command runner with clear, readable syntax; influenced our `/` command design

### Streaming & Real-time Interaction
- **[Anthropic's Streaming](https://docs.anthropic.com/claude/reference/streaming)** – Streaming response patterns for real-time user feedback
- **[Ollama](https://ollama.ai/)** – Local model management and API reference design

### Safety & Approval Systems
- **[Mozilla's Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)** – Approval sandboxing patterns
- **[Temporal Workflows](https://temporal.io/)** – Durable execution and approval workflow patterns

&ensp;
## Community

Contributions are welcome! Areas we're actively developing:

- New model provider integrations
- MCP server implementations
- TUI improvements and themes
- Documentation and examples
- Bug fixes and performance

&ensp;
## License

MIT – See `LICENSE` file
