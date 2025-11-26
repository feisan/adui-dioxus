# Icon 使用说明

`Icon` 提供简单的内置图标集，并允许传入自定义 SVG 内容。

## Props
- `kind: IconKind`：内置图标（plus/minus/check/close/info/question/arrow/search/copy/edit/loading）。
- `size: f32`：像素尺寸，默认 20。
- `color: Option<String>`：自定义颜色（默认 `currentColor`）。
- `rotate: Option<f32>`：旋转角度，单位度。
- `spin: bool`：是否添加旋转动画（`Loading` 类型默认启用）。
- `aria_label: Option<String>`：无障碍标签，未传时使用 `kind` 名称。
- `view_box: Option<String>`：自定义 SVG viewBox。
- `custom: Option<Element>`：自定义 SVG 内容，传入后将忽略内置路径。

## 示例
```rust
Icon { kind: IconKind::Plus, size: 16.0 }
Icon { kind: IconKind::Loading, spin: true, color: Some("#1677ff".into()) }
Icon {
    size: 24.0,
    view_box: Some("0 0 16 16".into()),
    aria_label: Some("自定义".into()),
    custom: Some(rsx!(path { d: "M1 1h14v14H1z" }))
}
```

Icon 主要用于按钮、浮动面板等组件，所有属性均可配合 Dioxus 的 `class`/`style` 进一步扩展。
