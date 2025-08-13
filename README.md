# 🚀 Lode

<div align="center">

**Language:** [English](README.md) | [中文](README_zh.md)

_A high-performance AI coding agent written in Rust with a rich terminal UI_

![demo](./images/demo.gif)

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

</div>

---

Lode is a high-performance AI coding agent written in Rust with a rich terminal UI. Formerly known as **Trae Agent Rust**, it remains compatible with the original tool spec while focusing on speed, reliability, and great UX.

## ✨ Highlights

- 🦀 **Fast Rust core** and clean architecture with an output abstraction layer
- 🎨 **Interactive TUI** built on iocraft with real-time status and animations
- 🛠️ **Powerful tool system**: bash, edit, json_edit, thinking, task_done, ckg, mcp
- 🤖 **Providers**: OpenAI ready; Anthropic and Google in progress
- 🔍 **Smart file search** with @path syntax, Git-aware, and blazing fast

## 🚀 Quick Start

### 📋 Prerequisites

- 🦀 Rust stable (1.70+)
- 🔑 An API key (OpenAI recommended; Anthropic/Google coming soon)

### 📦 Install

```bash
cargo install --git https://github.com/Blushyes/Lode --bin lode
```

### ▶️ Run

```bash
# Interactive mode (recommended)
lode interactive

# Or simply
lode

# Single task
lode run "Fix the bug in main.rs"
```

## ⚙️ Minimal Config

You can configure via environment variables or simple JSON files.

**Option A:** Environment variables

```bash
# OpenAI
export OPENAI_API_KEY="your_openai_api_key"
export OPENAI_MODEL="gpt-4o"
```

**Option B:** JSON files in your working directory

```json
{
  "base_url": "https://api.deepseek.com",
  "api_key": "your-api-key",
  "model": "deepseek-chat",
  "max_token": 8192
}
```

### 🤖 Supported Models

| Provider         | Models                  | Status    |
| ---------------- | ----------------------- | --------- |
| 🟢 **OpenAI**    | `gpt-4o`, `gpt-4o-mini` | ✅ Ready  |
| 🟡 **Anthropic** | `claude-3.5` family     | 🚧 Coming |
| 🔵 **Google**    | `gemini-1.5` family     | 🚧 Coming |

## 🗺️ Roadmap

<details>
<summary><strong>🚀 Phase 1: Core Experience</strong></summary>

| Priority | Feature                                  | Description                                                                                  |
| -------- | ---------------------------------------- | -------------------------------------------------------------------------------------------- |
| 🔥 High  | **First-run config onboarding**          | Guided wizard (detect/create openai.json or env vars), API key validation, sensible defaults |
| 🔥 High  | **Refactor and optimize config loading** | Unified precedence (CLI args > env > JSON), clearer errors/diagnostics, optional hot-reload  |
| 🔥 High  | **Tool Call permission system**          | Allowlist by tool/command/dir, interactive confirmations, sensitive-operation guardrails     |

</details>

<details>
<summary><strong>🎨 Phase 2: Enhanced UX</strong></summary>

| Priority  | Feature                      | Description                                                                  |
| --------- | ---------------------------- | ---------------------------------------------------------------------------- |
| 🟡 Medium | **LODE.md custom prompts**   | Project/dir-level overrides, scenario templates (bugfix/refactor/docs/tests) |
| 🟡 Medium | **UI layout unification**    | Consistent Header/Status/Input, keyboard/interaction coherence               |
| 🟡 Medium | **Trajectory replay/export** | Visualization, one-click replay, export to JSON/Markdown                     |
| 🎨 Low    | **Need a cli LOGO**          | Like gemini-cli's style                                                      |

</details>

<details>
<summary><strong>🤖 Phase 3: Intelligence & Performance</strong></summary>

| Priority  | Feature                              | Description                                                        |
| --------- | ------------------------------------ | ------------------------------------------------------------------ |
| 🟡 Medium | **Multi-model and auto-routing**     | Pick model per task type, graceful fallback and retry strategies   |
| 🟡 Medium | **Context optimization and caching** | File summary cache, dedup repeated refs, token budget control      |
| 🔵 Low    | **MCP ecosystem**                    | Presets/templates for common providers, easy on/off external tools |

</details>

<details>
<summary><strong>🌐 Phase 4: Platform & Ecosystem</strong></summary>

| Priority | Feature                   | Description                                                                         |
| -------- | ------------------------- | ----------------------------------------------------------------------------------- |
| 🔵 Low   | **Core as WASM**          | Run in browser/plug-in contexts with isomorphic tool interfaces and minimal runtime |
| 🔵 Low   | **Cross-platform polish** | macOS/Linux/Windows/WSL nuances and stability                                       |
| 🔵 Low   | **Pluggable tool system** | Spec for third-party tools, versioning and dependency declaration                   |

</details>

<details>
<summary><strong>🛡️ Phase 5: Safety & Quality</strong></summary>

| Priority  | Feature                      | Description                                                              |
| --------- | ---------------------------- | ------------------------------------------------------------------------ |
| 🟡 Medium | **Safety and rate limiting** | Sandbox mode (restricted bash/network toggle), concurrency and rate caps |
| 🔵 Low    | **Testing and benchmarking** | E2e samples, performance baselines and comparison reports                |

</details>

## 📄 License

Dual-licensed at your option:

- **Apache-2.0** ([LICENSE-APACHE](LICENSE-APACHE))
- **MIT** ([LICENSE-MIT](LICENSE-MIT))

## 🙏 Acknowledgments

- **[Trae Agent](https://github.com/bytedance/trae-agent)** for the original Python implementation and spec
- **[iocraft](https://github.com/ccbrown/iocraft)** for the beautiful terminal UI framework
- **OpenAI, Anthropic, and Google** for model APIs
- **Rust community** for the amazing ecosystem

---

<div align="center">

Made with ❤️ in Rust

</div>
