# Masonry 使用说明

## Props
- `columns: u16`：默认列数（xs 断点下生效）。
- `responsive: Option<MasonryResponsive>`：根据断点（xs/sm/md/lg/xl/xxl）设置列数，自动生成 `@media (min-width)` 样式。
- `gap: Option<f32>`：列间距（px），同时作为默认的行间距。
- `row_gap: Option<f32>`：可单独控制行间距（默认为 `gap`）。
- `min_column_width: Option<f32>`：设置 `column-width`，用于固定每列最小宽度。
- `class` / `style`：透传外层样式。

## 示例
```rust
Masonry {
    columns: 3,
    responsive: Some(MasonryResponsive {
        xs: Some(1),
        md: Some(2),
        lg: Some(3),
        xl: Some(4),
        ..Default::default()
    }),
    gap: Some(12.0),
    row_gap: Some(24.0),
    min_column_width: Some(180.0),
    {(0..6).map(|i| rsx!(div { class: "card", "Card {i}" }))}
}
```

- `gap` 会映射为 `column-gap`，同时写入 `--adui-masonry-gap` 方便子元素使用；`row_gap` 则通过 `--adui-masonry-row-gap` 控制子元素 `margin-bottom`。
- 当设置 `responsive` 时，会按断点优先级覆盖 `column-count`，未设置的断点沿用默认列数。
- 仍使用 CSS columns 实现瀑布流，子元素自动 `break-inside: avoid`。

可运行 `dx serve --example layout_demo` 查看更新后的 Masonry 区块。