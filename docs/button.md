# Button 使用说明

## Props 重点
- `color` + `variant`：拆分 tone 与表现层，`ButtonType` 仍可用于兼容旧 API（会映射到 `variant`）。常用组合：`Primary/Solid`、`Default/Outlined`、`Text`、`Link`、`Dashed`。
- `shape` (`Default/Round/Circle`) 与 `size` (`Small/Middle/Large`) 会共同决定半径、高度、字重，底层依赖 theme tokens 生成 CSS 变量。
- `icon` 与 `icon_placement` 控制图标位置，`label`/`auto_insert_space` 可确保单个汉字自动插入空格；若只渲染图标可通过 `icon_only` 或自动侦测获得 `.adui-btn-icon-only` 类。
- `loading` 支持 `loading_delay`（毫秒）与 `loading_icon` 自定义节点；`href` 会渲染 `<a>` 并自动阻断 disabled/Loading 状态的点击。
- `class_names_*` 与 `styles_root` 便于按块覆写 class/style，`onclick` 为标准 `EventHandler<MouseEvent>`。

## ButtonGroup
- `ButtonGroup` 会注入上下文以统一 `size/shape/color/variant`，同时渲染 `.adui-btn-group` 容器，内部按钮可继承圆角与边界控制。
- 用法示例（摘自 `examples/button_demo.rs`）：
```rust
ButtonGroup {
    size: Some(ButtonSize::Small),
    variant: Some(ButtonVariant::Solid),
    color: Some(ButtonColor::Primary),
    Button { label: Some("上一页".into()), icon: rsx!(span { "←" }) }
    Button { label: Some("刷新".into()), icon: rsx!(span { "↻" }) }
    Button { label: Some("下一页".into()), icon_placement: ButtonIconPlacement::End, icon: rsx!(span { "→" }) }
}
```
- 组内按钮未显式传入 `size/variant/color` 时会使用上方配置；也可继续设置 `ButtonType` 来兼容旧语义。

## 示例
- 运行 `dx serve --example button_demo` 即可查看主题切换、Ghost/Danger、Loading/Block、Group 等状态。
- 常见样式问题（如 `ghost` 在暗色模式下的对比）可通过调整 `ThemeProvider` tokens 来验证。