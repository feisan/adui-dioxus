# adui-dioxus

## 介绍

adui-dioxus 是一个基于 Dioxus 的 UI 组件库，提供了丰富的组件和样式，帮助开发者快速构建跨平台的 Web 和移动应用。

这是一个**实验性项目**，使用 **Vibe Coding** 将 Ant Design 6.0.0 移植到 Dioxus。它从 Ant Design UI 库（https://github.com/ant-design/ant-design）中提取组件，并适配到 Rust/Dioxus 生态系统。该库继承了 Ant Design 的设计理念和组件风格，同时结合了 Dioxus 的性能和灵活性，为开发者提供了更加高效和便捷的开发体验。

## 项目状态

这是 Ant Design 6.0.0 到 Dioxus 的实验性移植。该库包括：

- **Theme**：Ant Design 6.x 风格的令牌与主题上下文（明/暗预设，CSS 变量导出）
- **Button**：对齐 type/size/shape/danger/ghost/loading/block/icon/href
- **FloatButton**：悬浮按钮，支持圆/方形、primary/default、danger、tooltip、可配置位置
- **Icon**：内置常用图标集（plus/minus/check/close/info/question/search/arrow/loading），支持旋转、大小、颜色
- **Typography**：Title/Text/Paragraph，支持 tone（default/secondary/success/warning/danger/disabled）、strong/italic/underline/delete/code/mark、ellipsis（单/多行 + 展开）、copyable、editable、禁用态语义
- **Form**：`Form`/`FormItem`/`use_form_item_control`，支持 required/min/max/pattern/custom rule、布局控制、必填标记、上下文 Hook
- **Upload**：点击选择/拖拽上传、列表（text/picture/picture-card）、`before_upload`、XHR 上传进度/abort、受控/非受控列表
- **布局**：Divider/Flex/Grid（支持基础断点 span）/Layout（Sider 支持 collapsible/trigger/theme/手动 has_sider）/Masonry（列数可响应式 + gap/row_gap/min width）/Space/Splitter（可拖拽分栏），覆盖常用布局场景

以及更多组件...

## 文档

完整的文档位于 `docs/` 目录：

- **[English Documentation Index](docs/README.md)** - 完整的英文组件文档
- **[中文文档索引](docs/README_CN.md)** - 完整的中文组件文档

每个组件都有详细的文档，包括：
- API 参考（所有属性、事件和方法）
- 使用示例
- 使用场景
- 与 Ant Design 6.0.0 的差异

## 本地运行

要求 Rust + Dioxus 0.7 生态（推荐安装 dioxus-cli）。

### 构建与检查

```bash
cargo fmt && cargo clippy --all-targets --all-features && cargo test
```

### 运行示例

- 按钮示例（浏览器）：`dx serve --example button_demo`
- 悬浮按钮示例（浏览器）：`dx serve --example float_button_demo`
- 图标示例（浏览器）：`dx serve --example icon_demo`
- 排版示例（浏览器）：`dx serve --example typography_demo`
- 布局示例（浏览器）：`dx serve --example layout_demo`
- Flex/Space 示例（浏览器）：`dx serve --example flex_space_demo`
- Grid 示例（浏览器）：`dx serve --example grid_demo`
- Form 示例（浏览器）：`dx serve --example form_demo`
- Upload 示例（浏览器）：`dx serve --example upload_demo`

## 示例功能概览

- `button_demo`：主题切换（Light/Dark）、主色预设、按钮 type/size/shape 及状态开关
- `float_button_demo`：浮动按钮主/副按钮，主题切换，位置与 tooltip 展示
- `icon_demo`：图标列表，主题切换，大小调节，主色切换，全局旋转开关
- `typography_demo`：Title/Text/Paragraph，展示 tone 切换、修饰、copyable、可展开 ellipsis、Inline 编辑
- `layout_demo`：展示 Divider、Flex、Space、Grid、Layout（含 Sider 折叠/Zero Trigger）、Masonry、Splitter（拖拽调整分栏比例）
- `grid_demo`：展示 Row 水平/垂直/响应式 gutter 以及 Col 的 span/offset/order/push/pull/flex 响应式配置
- `flex_space_demo`：演示 `FlexConfigProvider`、gap 预设、wrap、Space size/split/compact 等布局能力
- `form_demo`：展示 `Form` 基本校验、布局、重置/提交回调与自定义控件接入
- `upload_demo`：展示基础上传、图片列表、dragger 拖拽区域，验证 `before_upload`、列表移除与上传日志

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 注意

这是一个实验性项目。某些功能可能与原始 Ant Design 实现有所不同，API 可能会随着项目的成熟而演变。请参考组件文档了解具体的差异和限制。

