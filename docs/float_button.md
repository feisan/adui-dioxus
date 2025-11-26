# FloatButton / BackTop 使用说明

## FloatButton
- `type` (`Default/Primary`) 与 `shape` (`Circle/Square`) 控制基色与外观，`danger` 会应用危险色调。若提供 `Group` 上下文则自动继承。
- `icon` + `content/description` 组成主体区域；仅 icon 时会打上 `.adui-float-btn-icon-only`，Square 形态下 `content` 会垂直排列。
- `badge` (`BadgeConfig`) 支持文本或 `dot`，位置与 antd 一致；`tooltip` 会同步到 `title`/`aria-label`。
- 定位通过 `right/left/top/bottom/z_index` 控制，默认挂载在右下角（24px / 72px）。单独使用时会追加 `.adui-float-btn-individual` 方便覆写。

## FloatButtonGroup
- Props：`shape`、`type`、`gap`、`pure`、`right/left/top/bottom/z_index`。默认竖向堆叠，`pure=true` 时仅渲染容器，无绝对定位，可嵌进任意布局。
- 组内按钮自动继承 `shape/type`，并通过 CSS 变量控制间距。
- 示例：
```rust
FloatButtonGroup {
    right: Some(24.0),
    bottom: Some(120.0),
    FloatButton { r#type: FloatButtonType::Primary, icon: rsx!(span { "＋" }), tooltip: Some("快速创建".into()) }
    FloatButton { shape: FloatButtonShape::Square, content: Some("帮助".into()), badge: Some(BadgeConfig::text("New")) }
}
```

## FloatButtonPurePanel
- `FloatButtonPurePanel` 是 `FloatButtonGroup` 的精简包装（`pure=true`、无定位），适合在卡片/抽屉内展示快捷操作面板。
- Props：`shape`、`type`、`gap`、`class`、`style`、`children`。其余定位属性会被忽略。
- 示例：
```rust
FloatButtonPurePanel {
    FloatButton { icon: rsx!(span { "?" }), tooltip: Some("帮助中心".into()) }
    FloatButton { icon: rsx!(span { "!" }), danger: true, tooltip: Some("告警".into()) }
}
```

## BackTop
- 封装 `FloatButton` + 滚动逻辑，新增 `type/shape/danger/content/description/badge/right/left/top/bottom/z_index` Props，便于与主按钮统一样式。
- `onclick` 仍可监听，触发顺序为：用户回调 -> `window.scroll_to(0,0)`；若需异步滚动可在外部覆盖。
- 示例（来自 `examples/float_button_demo.rs`）：
```rust
BackTop {
    tooltip: Some("返回顶部".into()),
    content: Some("TOP".into()),
    shape: FloatButtonShape::Square,
    right: Some(24.0),
    bottom: Some(24.0),
}
```

## Demo
`dx serve --example float_button_demo` 展示：主题切换、浮动按钮组、Badge/Tooltip、BackTop 自定义位置与方形文字形态。
