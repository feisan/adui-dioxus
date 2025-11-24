# Repository Guidelines

## 项目结构与模块组织
- `src/`: 核心组件与样式实现，按组件域分模块（如 `button/`, `layout/`）。公共类型与主题变量放在 `src/foundation/`。
- `examples/`: 交互示例与演示页面，便于验证 API。每个示例用独立子目录，入口命名 `main.rs`。
- `tests/`: 集成/端到端测试；单元测试优先放在对应源码模块的 `#[cfg(test)]` 中。
- `assets/`: 设计资源、图标与设计令牌(JSON/TOML)。尽量以变量形式引用，避免硬编码。
- `docs/`: 面向贡献者的补充文档与架构说明（如设计原则、可访问性指南）。新增文档请在此更新目录。

## 构建、测试与本地开发
- `cargo build`: 构建全部 crate，默认使用本机目标。新增依赖后请先运行。
- `cargo check`: 快速类型检查，提交前必跑。
- `cargo test`: 运行所有单元与集成测试；长耗时测试请打标签并在 CI 中配置过滤。
- `cargo fmt --all`: 按 `rustfmt` 统一格式，提交前必须通过。
- `cargo clippy --all-targets --all-features`: 静态检查，修复或显式 `allow` 并给出理由。
- Web 目标（如 Dioxus Web）可用 `cargo build --target wasm32-unknown-unknown`；示例调试可在 `examples/` 下运行 `cargo run --example <name>`。

## 代码风格与命名规范
- 缩进 4 空格，保持 `rustfmt` 默认配置；长链式调用考虑换行并分段。
- 类型/结构体使用 PascalCase，函数与变量使用 snake_case，常量使用 SCREAMING_SNAKE_CASE。
- 模块与文件名保持短小、语义化（如 `button_group.rs`）；公共接口需文档注释，解释语义与边界条件。
- 避免魔法数，优先使用常量或主题变量；错误信息保持可操作性，包含期望与实际值。

## 测试准则
- 单元测试紧邻实现；集成测试放 `tests/`。命名采用 `*_test.rs`，用 `mod name_tests` 分组。
- 优先覆盖状态边界、无效输入、可访问性属性（如 ARIA）与交互事件；新组件需至少一个渲染快照或行为测试。
- 若依赖异步/定时，请使用 `#[tokio::test]` 或等效宏；确保测试可重复，不依赖外部网络。

## Commit 与 PR 要求
- Commit 消息使用英语祈使句，推荐 Conventional Commits（如 `feat: add button loading state`，`fix: guard empty children`）。
- 每个 PR 需包含变更摘要、测试结果（命令与输出概述）与关联 Issue 编号；涉及 UI 变更请附前后截图或动图。
- 保持 PR 聚焦单一主题；若引入破坏性变更，需在描述中明确说明并给出迁移步骤。
- 在提交前确保通过 `cargo fmt`, `cargo clippy`, `cargo test`，并检查新增公开 API 是否有文档与示例。
