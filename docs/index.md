# kode

**kode** is a fast, local coding agent for your terminal. It's a community-driven fork of `openai/codex` focused on real developer ergonomics: Browser integration, multi-agents, theming, and reasoning control — all while staying compatible with upstream.

## What's New in v0.4.0 (October 26, 2025)

- **Auto Drive upgraded** – hand `/auto` a task and it now plans, coordinates agents, reruns checks, and recovers from hiccups without babysitting.
- **Unified settings** – `/settings` centralizes limits, model routing, themes, and CLI integrations so you can audit configuration in one place.
- **Card-based activity** – Agents, browser sessions, web search, and Auto Drive render as compact cards with drill-down overlays for full logs.
- **Turbocharged performance** – History rendering and streaming were optimized to stay smooth even during long multi-agent sessions.
- **Smarter agents** – Mix and match orchestrator CLIs (Claude, Gemini, GPT-5, Qwen, and more) per `/plan`, `/code`, or `/solve` run.

## Why kode

- 🚀 **Auto Drive orchestration** – Multi-agent automation that now self-heals and ships complete tasks.
- 🌐 **Browser Integration** – CDP support, headless browsing, screenshots captured inline.
- 🤖 **Multi-agent commands** – `/plan`, `/code` and `/solve` coordinate multiple CLI agents.
- 🧭 **Unified settings hub** – `/settings` overlay for limits, theming, approvals, and provider wiring.
- 🎨 **Theme system** – Switch between accessible presets, customize accents, and preview live via `/themes`.
- 🔌 **MCP support** – Extend with filesystem, DBs, APIs, or your own tools.
- 🔒 **Safety modes** – Read-only, approvals, and workspace sandboxing.

## Quick Start

### Run
```bash
npx -y @just-every/code
```

### Install & Run
```bash
npm install -g @just-every/code
code // or `coder` if you're using VS Code
```

**Authenticate** (one of the following):
- **Sign in with ChatGPT** (Plus/Pro/Team; uses models available to your plan)
  - Run `code` and pick "Sign in with ChatGPT"
- **API key** (usage-based)
  - Set `export OPENAI_API_KEY=xyz` and run `code`

## Key Features

### Multi-Agent Commands
- `/plan` - Plan code changes (Claude, Gemini and GPT-5 consensus)
- `/solve` - Solve complex problems (fastest preferred)
- `/code` - Write code! (consensus approach)

### Auto Drive
- Hand off multi-step tasks; Auto Drive coordinates agents and approvals
- `/auto "Refactor the auth flow and add device login"`

### Browser Integration
- Connect to external Chrome browser via CDP
- Use internal headless browser mode
- Screenshots captured inline for visual context

### Settings & Configuration
- Centralized `/settings` for all configuration
- Theme system with accessible presets
- Model routing and reasoning controls

## Next Steps

- [Installation Guide](install.md)
- [Getting Started](getting-started.md)
- [Command Reference](slash-commands.md)
- [Configuration Guide](config.md)

## Contributing

We welcome contributions! This fork maintains compatibility with upstream while adding community-requested features.

See our [Contributing Guide](contributing.md) for details.

## License

Apache 2.0 - See [LICENSE](license.md) file for details.

This project is a community fork of the original Codex CLI. We maintain compatibility while adding enhanced features requested by the developer community.

---

Need help? Open an issue on [GitHub](https://github.com/GeorgePearse/kode/issues) or check our documentation.
