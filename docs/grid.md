# Grid 使用说明

- ## Row
- `gutter`: 基础水平间距（px），等同 antd `[gutter, *]` 的第一个值。
- `gutter_vertical`: 可选的垂直间距，对应 `[horizontal, vertical]` 的第二项。
- `gutter_spec`: 一个 `RowGutter`，用于一次性传入 `Uniform(f32)` / `Pair(h, v)` / `Responsive(ResponsiveGutter)`，与 antd 的 `gutter={[16, 8]}` 或 `gutter={{ xs: 8, md: 24 }}` 行为对齐。若设置该字段，其余 `gutter*` 将被覆盖。
- `responsive_gutter`: 传入 `ResponsiveGutter`（由 `horizontal: ResponsiveValue` + 可选 `vertical` 组成）即可针对 xs/sm/md/lg/xl/xxl 分别覆盖 gutter；内部通过 `--adui-row-gutter-x/y` CSS 变量驱动。
- `justify` / `align`: 映射到 Flex 行主轴/交叉轴，与 antd 语义一致。

示例：
```rust
Row {
    gutter_spec: Some(RowGutter::Pair(24.0, 16.0)),
    responsive_gutter: Some(ResponsiveGutter {
        horizontal: ResponsiveValue { xs: Some(8.0), md: Some(24.0), ..Default::default() },
        vertical: Some(ResponsiveValue { md: Some(16.0), ..Default::default() }),
    }),
    // ...
}
```

## Col
- `span`/`offset`：默认 24 栅格，`span=12` 等价于占据一半宽度。
- `push`/`pull`：通过 `position: relative; left/right` 实现偏移，适合制作左右互换的排序。
- `order`: 设置 flex order。
- `responsive`: `ColResponsive` 包含 xs~xxl 的 `ColSize`，可分别指定 `span/offset/push/pull/order/flex`。

示例：
```rust
Col {
    span: 24,
    responsive: Some(ColResponsive {
        md: Some(ColSize { span: Some(12), ..Default::default() }),
        lg: Some(ColSize { span: Some(8), order: Some(1), ..Default::default() }),
        ..Default::default()
    }),
    // ...
}
```

更多参考见 `examples/grid_demo.rs`（`dx serve --example grid_demo`）。
