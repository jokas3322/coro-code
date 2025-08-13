## Lode

Language: [English](README.md) | [中文](README_zh.md)

Lode is a high-performance AI coding agent written in Rust with a rich terminal UI. Formerly known as Trae Agent Rust, it remains compatible with the original tool spec while focusing on speed, reliability, and great UX.

![demo](./images/demo.gif)

### Highlights

- Fast Rust core and clean architecture with an output abstraction layer
- Interactive TUI built on iocraft with real-time status and animations
- Powerful tool system: bash, edit, json_edit, thinking, task_done, ckg, mcp
- Providers: OpenAI ready; Anthropic and Google in progress
- Smart file search with @path syntax, Git-aware, and blazing fast

## Quick Start

### Prerequisites

- Rust stable (1.70+)
- An API key (OpenAI recommended; Anthropic/Google coming soon)

### Install

```bash
cargo install --git https://github.com/Blushyes/trae-agent-rs --bin trae-rs
```

### Run

```bash
# Interactive mode (recommended)
trae-rs interactive

# Or simply
trae-rs

# Single task
trae-rs run "Fix the bug in main.rs"
```

## Minimal Config

You can configure via environment variables or simple JSON files.

- Option A: Environment variables

```bash
# OpenAI
export OPENAI_API_KEY="your_openai_api_key"
export OPENAI_MODEL="gpt-4o"
```

- Option B: JSON files in your working directory

```bash
# openai.json
{
  "api_key": "your_openai_api_key",
  "model": "gpt-4o"
}
```

Supported (current/coming):

- OpenAI: gpt-4o, gpt-4o-mini
- Anthropic: claude-3.5 family (coming)
- Google: gemini 1.5 family (coming)

## Roadmap

- First-run config onboarding: guided wizard (detect/create openai.json or env vars), API key validation, sensible defaults
- Refactor and optimize config loading: unified precedence (CLI args > env > JSON), clearer errors/diagnostics, optional hot-reload
- Tool Call permission system: allowlist by tool/command/dir, interactive confirmations, sensitive-operation guardrails
- LODE.md custom prompts: project/dir-level overrides, scenario templates (bugfix/refactor/docs/tests)
- Core as WASM: run in browser/plug-in contexts with isomorphic tool interfaces and minimal runtime
- UI layout unification: consistent Header/Status/Input, keyboard/interaction coherence
- Multi-model and auto-routing: pick model per task type, graceful fallback and retry strategies
- Trajectory replay/export: visualization, one-click replay, export to JSON/Markdown
- Context optimization and caching: file summary cache, dedup repeated refs, token budget control
- MCP ecosystem: presets/templates for common providers, easy on/off external tools
- Cross-platform polish: macOS/Linux/Windows/WSL nuances and stability
- Safety and rate limiting: sandbox mode (restricted bash/network toggle), concurrency and rate caps
- Pluggable tool system: spec for third-party tools, versioning and dependency declaration
- Testing and benchmarking: e2e samples, performance baselines and comparison reports

## License

Dual-licensed at your option:

- Apache-2.0 (LICENSE-APACHE)
- MIT (LICENSE-MIT)

## Acknowledgments

- Trae Agent for the original Python implementation and spec
- iocraft for the beautiful terminal UI framework
- OpenAI, Anthropic, and Google for model APIs
- Rust community for the amazing ecosystem
