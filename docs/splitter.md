# Splitter 使用说明

## Props
- `orientation`: `Horizontal`（默认）或 `Vertical`。
- `split: Option<f32>`：受控模式传入当前分割比（0-1），不传则使用 `default_split`。
- `default_split: f32`：非受控初始值，默认 `0.5`。
- `on_change`：拖拽过程中及释放后回调最新分割比。
- `on_moving` / `on_release`：分别在拖动过程中与释放时额外触发，可用于展示实时状态或保存尺寸。
- `min_primary` / `min_secondary`：主、次 Pane 的最小尺寸（px），默认 80。
- `gutter_aria_label`：为中间拖拽条提供无障碍标签。

## 行为
- 组件支持受控/非受控两种模式，内部会 clamp 到 0.05~0.95 范围并遵守最小宽度限制。
- Gutter 使用 Pointer 事件以支持鼠标与触屏操作，并自动设置/释放 Pointer Capture。
- `on_moving` 与 `on_change` 均在拖动时触发，若只需最终结果可使用 `on_release`。

## 示例
```rust
let split = use_signal(|| 0.4f32);

Splitter {
    orientation: SplitterOrientation::Horizontal,
    split: Some(*split.read()),
    on_change: move |value| split.set(value),
    min_primary: Some(120.0),
    min_secondary: Some(120.0),
    gutter_aria_label: Some("调整左右面板宽度".into()),
    first: rsx!({sample_bar("Pane A")}),
    second: rsx!({sample_bar("Pane B")}),
}
```

更多用法可运行 `dx serve --example layout_demo`。