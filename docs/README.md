# 文档索引

## 已撰写的组件说明
- [App & ConfigProvider & 全局反馈](./app_config.md)：App/use_* API、ConfigProvider 全局配置、OverlayManager 抽象，Message/Notification/Modal/Drawer 的 MVP 设计。
- [Button](./button.md)：颜色/变体映射、ButtonGroup 上下文、auto insert space 说明。
- [FloatButton](./float_button.md)：FloatButton/Group/BackTop 的 props、定位与 demo。
- [Flex / Space](./flex_space.md)：FlexConfigProvider 继承方式与 Space gap/split。
- [Input / TextArea](./input.md)：受控/非受控模式、`allow_clear`、与 Form 集成示例。
- [Checkbox / Radio / Switch](./checkbox_radio_switch.md)：布尔控件与单选控件的 Group 行为与 Form 集成。
- [Select / TreeSelect 等选择器家族](./select_family.md)：统一的 Option/Value 模型、下拉基础设施与 TreeSelect 当前能力说明。
- [Grid](./grid.md)：Row/Col 响应式 gutter 及断点语义。
- [Typography](./typography.md)：Tone、copyable/ellipsis/editable 行为与示例。
- [Layout](./layout.md)：Sider 折叠、zero width trigger、内容区主题。
- [Divider](./divider.md)：水平/垂直/带文字/虚线的 API。
- [Masonry](./masonry.md)：列数、gap、响应式配置。
- [Splitter](./splitter.md)：拖拽/受控 split、ARIA 属性。
- [Icon](./icon.md)：内置图标、旋转/大小、自定义 SVG。

## 未覆盖的 Ant Design 能力与后续路线
- Button：暂未实现波纹效果、水波纹延迟、Loading delay tailwind cancel；未来考虑通过自定义动画或注入 `<style>` 来模拟。
- FloatButton：PurePanel/Tooltip `description` slot、漂浮拖拽自定义位置尚未实现；计划待 Sider 浮层交互稳定后补充。
- Layout：Sider breakpoint（`collapsedWidth` 自适应）、可折叠触发器 `tooltip` 自定义尚缺，需与浏览器 resize 监听配合。
- Grid/Flex：RTL 支持仅在样式层规划，尚未监听 `dir="rtl"` 调整 gutter；后续补充 `adui-layout-rtl` 类及 `Theme` 配置。
- Space：Compact 模式当前只包含类名，缺少子按钮之间的边框合并；下一步计划在 Button/Inputs 中添加针对 `.adui-space-compact` 的样式。
- Splitter：键盘操作目前仅提供 `tabindex`，未处理方向键/Enter 拖拽；后续需要添加键盘增量调整逻辑。
- Typography：`editable.max_length` 仅作为输入属性，未向调用方暴露告警；计划增加回调/提示。
- 表单控件扩展：`Input.Password` / `Input.Search`、`InputNumber`、`Select` 等扩展能力暂未实现，仅提供基础 `Input` / `TextArea` 与 Checkbox/Radio/Switch；Form 受控模式（`FormItem` 内由 FormStore 作为单一数据源）已基本对齐 Ant Design 6.x，后续将围绕此模型补充更多控件；扩展能力计划在 `plan/0003.md` 的 Phase 2 中补充。
- 主题：缺少 token 自定义指南（目前仅 README 提示），后续考虑新增 `docs/theme.md` 展示可覆盖字段与暗色示例。

> 以上缺口同步记录在 `plan/0001.md`，后续有新的组件或 API 时请更新此列表。
