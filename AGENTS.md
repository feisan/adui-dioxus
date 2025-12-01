# Repository Guidelines

## 项目概述

本项目是 Ant Design 6.0.0 到 Dioxus 的实验性移植，使用 Vibe Coding 创建。项目基于 Dioxus 0.7+ 构建，提供丰富的 UI 组件库，继承 Ant Design 的设计语言和模式。

## 项目结构与模块组织
- `src/`: 核心组件与样式实现，按组件域分模块（如 `button.rs`, `layout.rs`）。公共类型与主题变量放在 `src/foundation/`。
- `src/components/`: 所有组件实现，每个组件一个文件。组件应导出 Props 结构体和组件函数。
- `src/foundation/`: 基础类型、语义化变量和主题相关的公共代码。
- `src/theme.rs`: 主题系统实现，包括 Ant Design 6.x 风格令牌和主题上下文。
- `examples/`: 交互示例与演示页面，便于验证 API。每个示例用独立文件，命名 `*_demo.rs`，使用 `dx serve --example <name>` 运行。
- `tests/`: 集成/端到端测试；单元测试优先放在对应源码模块的 `#[cfg(test)]` 中。
- `docs/`: 组件文档，每个组件有英文（`*.md`）和中文（`*_CN.md`）两个版本。包含 API 参考、使用示例、用例和与 Ant Design 的差异说明。

## 构建、测试与本地开发

### 环境要求
- Rust 工具链（推荐最新稳定版）
- Dioxus CLI (`cargo install dioxus-cli` 或使用 `dx` 命令)
- 用于 WASM 目标的 `wasm32-unknown-unknown` target（`rustup target add wasm32-unknown-unknown`）

### 常用命令
- `cargo build`: 构建全部 crate，默认使用本机目标。新增依赖后请先运行。
- `cargo check`: 快速类型检查，提交前必跑。
- `cargo test`: 运行所有单元与集成测试；长耗时测试请打标签并在 CI 中配置过滤。
- `cargo fmt --all`: 按 `rustfmt` 统一格式，提交前必须通过。
- `cargo clippy --all-targets --all-features`: 静态检查，修复或显式 `allow` 并给出理由。
- `cargo build --target wasm32-unknown-unknown`: 构建 WASM 目标（用于 Web 平台）。
- `dx serve --example <name>`: 在浏览器中运行示例（如 `dx serve --example button_demo`）。
- `cargo run --example <name>`: 直接运行示例（非 Web 目标）。

### 开发工作流
提交前必须执行：
```bash
cargo fmt && cargo clippy --all-targets --all-features && cargo test
```

## 代码风格与命名规范
- 缩进 4 空格，保持 `rustfmt` 默认配置；长链式调用考虑换行并分段。
- 类型/结构体使用 PascalCase，函数与变量使用 snake_case，常量使用 SCREAMING_SNAKE_CASE。
- 模块与文件名保持短小、语义化（如 `button_group.rs`）；公共接口需文档注释，解释语义与边界条件。
- 避免魔法数，优先使用常量或主题变量；错误信息保持可操作性，包含期望与实际值。
- 组件 Props 结构体命名遵循 `<Component>Props` 模式（如 `ButtonProps`）。
- 组件函数使用 PascalCase，与组件名一致（如 `Button`）。

## 组件开发指南

### 新增组件
1. 在 `src/components/` 下创建新文件 `<component>.rs`。
2. 实现 Props 结构体（派生 `Clone`, `PartialEq`，必要时实现 `Default`）。
3. 实现组件函数，使用 `#[component]` 宏（Dioxus 0.7）。
4. 在 `src/components/mod.rs` 中导出组件和 Props。
5. 在 `src/lib.rs` 中重新导出，便于用户使用。
6. 创建示例文件 `examples/<component>_demo.rs`。
7. 编写文档 `docs/<component>.md` 和 `docs/<component>_CN.md`。

### 组件设计原则
- 遵循 Ant Design 6.0.0 的 API 设计，但适配 Rust/Dioxus 的类型系统。
- 使用 Dioxus Signals 进行状态管理。
- 支持主题系统（通过 `use_config` hook 访问主题上下文）。
- 确保组件支持可访问性（ARIA 属性、键盘导航等）。
- 组件应支持受控和非受控两种模式（通过 `value`/`default_value` 和 `on_change`）。

## 测试准则
- 单元测试紧邻实现；集成测试放 `tests/`。命名采用 `*_test.rs`，用 `mod name_tests` 分组。
- 优先覆盖状态边界、无效输入、可访问性属性（如 ARIA）与交互事件；新组件需至少一个渲染快照或行为测试。
- 若依赖异步/定时，请使用 `#[tokio::test]` 或等效宏；确保测试可重复，不依赖外部网络。
- 测试应验证组件在不同主题（light/dark）下的表现。

## 文档要求
- 每个组件必须有完整的文档，包括：
  - API 参考（所有 Props、事件、方法）
  - 使用示例（代码片段）
  - 常见用例
  - 与 Ant Design 6.0.0 的差异说明（如有）
- 文档同时提供英文和中文版本。
- 示例代码应可直接运行，展示组件的核心功能。

## Commit 与 PR 要求
- Commit 消息使用英语祈使句，推荐 Conventional Commits（如 `feat(button): add loading state`，`fix(form): guard empty children`）。
- 每个 PR 需包含变更摘要、测试结果（命令与输出概述）与关联 Issue 编号；涉及 UI 变更请附前后截图或动图。
- 保持 PR 聚焦单一主题；若引入破坏性变更，需在描述中明确说明并给出迁移步骤。
- 在提交前确保通过 `cargo fmt`, `cargo clippy`, `cargo test`，并检查新增公开 API 是否有文档与示例。
- 新增组件必须包含示例和文档才能合并。
