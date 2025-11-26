# adui-dioxus

## 介绍
adui-dioxus 是一个基于 Dioxus 的 UI 库，提供了丰富的组件和样式，帮助开发者快速构建跨平台的 Web 和移动应用。
它是从 ant design UI 库(https://github.com/ant-design/ant-design) 中提取出来的，旨在提供更简洁、更易用的组件和样式。它继承了 ant design 的设计理念和组件风格，同时结合了 Dioxus 的性能和灵活性，为开发者提供了更加高效和便捷的开发体验。

## 组件进度
- Theme：Ant Design 6.x 风格的令牌与主题上下文（明/暗预设，CSS 变量导出）
- Button：对齐 type/size/shape/danger/ghost/loading/block/icon/href
- FloatButton：悬浮按钮，支持圆/方形、primary/default、danger、tooltip、可配置位置
- Icon：内置常用图标集（plus/minus/check/close/info/question/search/arrow/loading），支持旋转、大小、颜色
- Typography：Title/Text/Paragraph，支持 tone（default/secondary/success/warning/danger/disabled）、strong/italic/underline/delete/code/mark、ellipsis（单/多行 + 展开）、copyable、editable、禁用态语义
- 布局：Divider/Flex/Grid（支持基础断点 span）/Layout（Sider 支持 collapsible/trigger/theme/手动 has_sider）/Masonry（列数可响应式 + gap/row_gap/min width）/Space/Splitter（可拖拽分栏），覆盖常用布局场景

## 本地运行
要求 Rust + Dioxus 0.7 生态（推荐安装 dioxus-cli）。

- 构建与检查：`cargo fmt && cargo clippy --all-targets --all-features && cargo test`
- 按钮示例（浏览器）：`dx serve --example button_demo`
- 悬浮按钮示例（浏览器）：`dx serve --example float_button_demo`
- 图标示例（浏览器）：`dx serve --example icon_demo`
- 排版示例（浏览器）：`dx serve --example typography_demo`
- 布局示例（浏览器）：`dx serve --example layout_demo`
- Flex/Space 示例（浏览器）：`dx serve --example flex_space_demo`
- Grid 示例（浏览器）：`dx serve --example grid_demo`
- 更多文档见 `docs/` 目录（如 `docs/button.md`、`docs/float_button.md`、`docs/layout.md`、`docs/masonry.md`、`docs/splitter.md`、`docs/icon.md` 等）。

## 示例功能概览
- `button_demo`：主题切换（Light/Dark）、主色预设、按钮 type/size/shape 及状态开关
- `float_button_demo`：浮动按钮主/副按钮，主题切换，位置与 tooltip 展示
- `icon_demo`：图标列表，主题切换，大小调节，主色切换，全局旋转开关
- `typography_demo`：Title/Text/Paragraph，展示 tone 切换、修饰、copyable、可展开 ellipsis、Inline 编辑
- `layout_demo`：展示 Divider、Flex、Space、Grid、Layout（含 Sider 折叠/Zero Trigger）、Masonry、Splitter（拖拽调整分栏比例）
- `grid_demo`：展示 Row 水平/垂直/响应式 gutter 以及 Col 的 span/offset/order/push/pull/flex 响应式配置
- `flex_space_demo`：演示 `FlexConfigProvider`、gap 预设、wrap、Space size/split/compact 等布局能力
