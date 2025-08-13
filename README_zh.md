## Lode

语言: [English](README.md) | [中文](README_zh.md)

Lode 是一个用 Rust 编写的高性能 AI 编码代理，带有丰富的终端界面。此前名为 Trae Agent Rust，现已更名并聚焦于速度、稳定性与优秀的使用体验，同时保持与原始工具规范的兼容性。

![demo](./images/demo.gif)

### 亮点

- 纯 Rust 内核与简洁清晰的架构，输出层抽象良好
- 基于 iocraft 的交互式终端 UI，实时状态与动画
- 强大的工具系统：bash、edit、json_edit、thinking、task_done、ckg、mcp
- 模型提供商：已支持 OpenAI；Anthropic 与 Google 即将到来
- 智能文件搜索：@path 语法、感知 Git、极速匹配

## 快速开始

### 前置

- Rust 稳定版（1.70+）
- 模型 API Key（推荐 OpenAI；Anthropic/Google 即将支持）

### 安装

```bash
cargo install --git https://github.com/Blushyes/trae-agent-rs --bin trae-rs
```

### 运行

```bash
# 交互模式（推荐）
trae-rs interactive

# 或直接
trae-rs

# 单次任务
trae-rs run "Fix the bug in main.rs"
```

## 最简配置

可用环境变量或 JSON 文件进行配置。

- 方案 A：环境变量

```bash
# OpenAI
export OPENAI_API_KEY="your_openai_api_key"
export OPENAI_MODEL="gpt-4o"
```

- 方案 B：工作目录中的 JSON 文件

```bash
# openai.json
{
  "api_key": "your_openai_api_key",
  "model": "gpt-4o"
}
```

支持（当前/计划）：

- OpenAI：gpt-4o、gpt-4o-mini
- Anthropic：claude-3.5 系列（计划）
- Google：gemini 1.5 系列（计划）

## 许可证

双许可证，任选其一：

- Apache-2.0（LICENSE-APACHE）
- MIT（LICENSE-MIT）

## 致谢

- Trae Agent：原始 Python 实现与规范
- iocraft：优秀的终端 UI 框架
- OpenAI、Anthropic、Google：模型与 API
- Rust 社区：出色的生态与工具
