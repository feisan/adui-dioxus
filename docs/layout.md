# Layout / Sider 使用说明

## Layout
- `Layout` 组件额外提供 `has_sider: Option<bool>`，当使用 `Sider` 时建议显式传入 `Some(true)` 以获得与 antd 一致的 `adui-layout-has-sider` 类名，用于正确的 Flex 渲染。
- `Header` / `Content` / `Footer` 继承 `LayoutProps`，保持 `class/style` 透传；Header/Footer 会根据主题 token 自动注入背景与文字颜色。

## Sider
- 新增 `collapsible`、`collapsed`、`default_collapsed`、`collapsed_width`、`reverse_arrow`、`trigger`、`zero_width_trigger_style`、`theme` 等 props，默认主题为 `Dark`，可切换为 `Light`。
- `collapsed` 为受控模式，`default_collapsed` 为非受控初始值；触发展开/收起时会调用 `on_collapse: EventHandler<bool>`。
- `width` 与 `collapsed_width` 以 `px` 表示（浮点数），当 `collapsed_width=0` 时会渲染 Zero Width Trigger。
- 若未提供 `trigger`，会内置箭头图标（使用 `IconKind::ArrowLeft/Right`），可通过 `reverse_arrow` 调整方向。
- 背景与文字颜色根据 `SiderTheme` 自动推导，边框由 `has_border` 控制。

示例：
```rust
let collapsed = use_signal(|| false);
Sider {
    collapsible: true,
    collapsed: Some(*collapsed.read()),
    on_collapse: move |next| collapsed.set(next),
    width: Some(220.0),
    collapsed_width: Some(64.0),
    theme: SiderTheme::Dark,
    { /* menu content */ }
}
```

## Demo
- `dx serve --example layout_demo` 展示 Sider 折叠、Zero Width Trigger、Header/Footer 的配色与 `has_sider` 用法。
- `dx serve --example layout_navigation_demo` 展示典型中后台导航布局：左侧 `Menu`、顶部 `Breadcrumb`、内容区列表 + `Pagination` 联动。
- `dx serve --example data_view_demo` 展示结合 Layout + Breadcrumb + Table/List + Pagination + Empty/Spin/Skeleton 的用户列表页模板。
- `dx serve --example dashboard_demo` 展示 Layout + Menu + Breadcrumb + Tabs + Card + Tag + Badge + Avatar + List/Pagination 的综合仪表盘页面。
- `dx serve --example flow_feedback_demo` 展示 Layout + Steps + Form + Progress + Alert + Result/Statistic 的典型流程反馈页面，用于多步向导与结果页场景。
- 代码位于 `examples/layout_demo.rs`、`examples/layout_navigation_demo.rs`、`examples/data_view_demo.rs`、`examples/dashboard_demo.rs` 与 `examples/flow_feedback_demo.rs`，README 中已补充运行方式。
