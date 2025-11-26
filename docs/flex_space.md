# Flex / Space 使用说明

## Flex
- `FlexConfigProvider` 可透传 `class/style/vertical` 设置，子 `Flex` 组件无需重复填写。示例中 `demo-flex-shared` 会自动加到所有子 Flex 上。
- `component` 支持 `div/section/article/nav/span`，可根据语义选择 tag。
- `gap_size` (`Small/Middle/Large`) 走 CSS 变量，若传入 `gap/row_gap/column_gap` 则以自定义值为准。
- `wrap`、`justify`、`align`、`orientation/vertical` 都通过类名映射到 `adui-flex-*`，配合 `ThemeProvider` 注入的样式即可还原 antd 行为。

示例：`dx serve --example flex_space_demo`，重点片段：
```rust
FlexConfigProvider { value: flex_config.clone(),
    Flex {
        component: FlexComponent::Section,
        wrap: FlexWrap::Wrap,
        justify: FlexJustify::Between,
        gap_size: Some(FlexGap::Large),
        Card { title: "配置 1", body: "继承 ConfigProvider" }
        // ...
    }
}
```

## Space
- `size` 预设（`Small/Middle/Large`）会转换为 `.adui-space-size-*` 类，底层 gap 由 CSS 变量控制。
- `gap` / `gap_cross` 可覆盖预设值；`gap_cross` 会根据方向映射为 `row-gap` 或 `column-gap`。
- `wrap`（仅在 `direction=Horizontal` 时生效）与 `compact` 会添加额外类，方便复用 antd 的紧凑样式。
- `split` 支持传入任意 `Element`，会在子元素之间插入。

示例：
```rust
Space {
    direction: SpaceDirection::Horizontal,
    size: SpaceSize::Large,
    wrap: Some(true),
    split: Some(rsx!(span { "·" })),
    Card { title: "Large", body: "size=Large + split dot" }
    // ...
}
```

更多示例代码位于 `examples/flex_space_demo.rs`。