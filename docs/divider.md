# Divider 使用说明

## Props
- `dashed: bool`：虚线样式，对应 antd `dashed`。
- `plain: bool`：文字采用简洁样式，与 antd `plain` 行为一致。
- `vertical: bool`：纵向分割线，相当于 antd `type="vertical"`。
- `orientation: DividerOrientation`：`Left/Center/Right`，仅在存在内容时生效。
- `orientation_margin: Option<String>`：控制文字两侧 margin，默认 `16px`。
- `content: Option<Element>` / `children: Option<Element>`：分割线中展示的内容（与 antd `children` 一致）。

## 行为说明
- 当 `vertical=true` 时，为 `role="separator" aria-orientation="vertical"`；横向默认为 `horizontal`。
- 当存在内容时，会自动添加 `.adui-divider-left|center|right`，并在内部渲染 `.adui-divider-inner-text`。
- `orientation_margin` 允许传入任意 CSS 长度字符串，例如 `"32px"`、`"2rem"`。

## 示例
```rust
Divider { dashed: true }
Divider {
    orientation: DividerOrientation::Left,
    orientation_margin: Some("24px".into()),
    plain: true,
    content: Some(rsx!("带文字"))
}
Divider {
    vertical: true,
}
```

运行 `dx serve --example layout_demo` 可查看多种 Divider 效果（方向、虚线、带文字、纵向等）。
