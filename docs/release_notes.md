# Release Notes（Plan 0001 对齐）

## 主要变更
- 主题系统：扩展 `ThemeTokens`，拆分 `THEME_BASE_STYLE`，支持更多颜色/尺寸/阴影变量，`ThemeProvider` 输出完整 CSS 变量。
- 组件重构：
  - Button：color + variant API、ButtonGroup、loading delay、自定义 icon、ghost、ARIA。
  - FloatButton：Group、BackTop、PurePanel、Badge/Tooltip、Square 文案。
  - Flex/Space/Grid/Typography/Layout/Divider/Masonry/Splitter/Icon：全面对齐 antd 6 行为，新增响应式、copyable/ellipsis、Sider 折叠、Splitter 拖拽等。
  - Form/Upload：新增组件与 demo，支持同步校验、拖拽上传、XHR 进度。
- Demo：`button_demo`、`float_button_demo`、`flex_space_demo`、`grid_demo`、`typography_demo`、`layout_demo`、`form_demo`、`upload_demo` 覆盖核心能力。
- 文档：在 `docs/` 中为主要组件补充 API/示例/差异说明，并在 README 更新运行指引。

## 兼容性说明
- Button 旧 `type`/`size`/`shape` 属性仍可使用，会映射到新 API；如需精准控制请使用 `color/variant/icon_placement`。
- FloatButton 位置 props 默认跟随 AntD（右下角 24px / 72px），可通过 `FloatButtonGroup`/`FloatButtonPurePanel` 自定义布局。
- 上传组件仅在 Web 目标下执行真实网络请求，桌面/SSR 场景会提示“仅支持 Web”。
- 部分 Clippy warnings（如 clone_on_copy/collapsible_if）暂未修复，不影响构建但建议后续清理。

## 已知问题 & 后续待办
- 自动化测试：缺少针对 gap/gutter/ellipsis 的单元测试及关键渲染快照。
- 键盘/ARIA：FloatButton/Button 的键盘行为已基本覆盖，但尚未在 CI 中校验，后续应补充 E2E 测试。
- Upload：未暴露 `customRequest`/`onPreview` 等高阶特性；非 Web 平台仍无法上传。
- Form：暂不支持数组路径/依赖项，需要 `Form.List` 等能力补充。
- 文档：`docs/README.md` 中列出的部分 antd 能力（如 Layout RTL、Grid SSR 媒体查询优化）仍在 TODO。
