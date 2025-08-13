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

## 计划中的功能

- 首次进入配置管理：引导式向导（检测/创建 openai.json 或环境变量），校验 API Key，提供默认模型与示例
- 重构、优化配置加载逻辑：统一优先级（CLI 参数 > 环境变量 > JSON 文件）、更友好的错误提示与诊断、可选热加载
- Tool Call 权限系统：按工具/命令/目录白名单、交互确认、防越权与敏感操作提示
- 支持 LODE.md 自定义提示词：项目/子目录级覆盖、场景化模板（bugfix/重构/文档/测试）
- core 支持打包为 WASM：浏览器/插件环境可用，同构工具接口与最小运行时
- UI 布局优化与统一化：Header/Status/Input 风格统一、键位与交互一致性优化
- 多模型与自动路由：按任务类型自动选择模型，失败自动降级与重试策略
- 轨迹回放与导出：Trajectory 可视化、一键回放、导出为 JSON/Markdown
- 上下文优化与缓存：文件摘要缓存、重复引用去重、Token 预算控制
- MCP 扩展生态：常用 Provider 预设与模板，一键启停外部工具
- 跨平台增强：macOS/Linux/Windows/WSL 细节适配与稳定性提升
- 安全与速率限制：沙箱模式（受限 bash/网络开关）、并发与速率限制
- 插件化工具系统：第三方工具注册规范、版本与依赖声明
- 测试与基准：端到端测试样例、性能基准与对比报告

## 许可证

双许可证，任选其一：

- Apache-2.0（LICENSE-APACHE）
- MIT（LICENSE-MIT）

## 致谢

- Trae Agent：原始 Python 实现与规范
- iocraft：优秀的终端 UI 框架
- OpenAI、Anthropic、Google：模型与 API
- Rust 社区：出色的生态与工具
