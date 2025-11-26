# Typography 使用说明

Typography 组件族由 `Text`、`Paragraph`、`Title` 组成，遵循 Ant Design 6.0.0 的 Tone / 可交互能力，并加入 Rust/Dioxus 语义化 Props。所有组件共享以下特性：

- Tone：`TextType`（`Default/Secondary/Success/Warning/Danger/Disabled`）会映射到主题 token `--adui-color-*`。
- 修饰：`strong/italic/underline/delete/code/mark` 与 antd 原始样式保持一致。
- Wrap：`wrap=false` 时添加 `.adui-text-nowrap` 并使用 `text-overflow`。
- Ellipsis：布尔开关兼容旧 API，或通过 `TypographyEllipsis` 控制多行省略、tooltip 与 `expand/collapse` 标签。
- Copyable/Editable：使用结构体 Props 传入配置，并自动生成 ARIA/键盘交互。

## API 摘要

| 属性 | 说明 |
| --- | --- |
| `TextProps::copyable` | `Option<TypographyCopyable>`，提供 `text/icon/copied_icon/tooltips`，默认展示内置 Copy/Icon | 
| `TextProps::ellipsis_config` | `Option<TypographyEllipsis>`，可设置 `rows`、`expandable`、`expand_text`、`collapse_text`、`tooltip` |
| `TextProps::editable` | `Option<TypographyEditable>`，包含 `text/placeholder/auto_focus/max_length/enter_icon/cancel_icon` | 
| `TextProps::on_copy` | 回调，在复制逻辑触发后同步通知外部 |
| `TextProps::on_edit` / `on_edit_cancel` / `on_edit_start` | 编辑模式进入/提交/取消时的回调 |

`ParagraphProps` 与 `TitleProps` 复用了上述字段，因此 Paragraph/Title 也支持 copy/ellipsis/编辑行为。`TypographyEditable` 默认在 Paragraph 中渲染 `textarea`，其余组件使用 `input`，并支持 `Enter/Escape` 键盘交互。

## Ellipsis 行为

```rust
Text {
    r#type: TextType::Secondary,
    ellipsis: true,
    ellipsis_config: Some(TypographyEllipsis {
        rows: Some(2),
        expandable: true,
        tooltip: Some("悬停展示完整内容".into()),
        expand_text: Some("展开".into()),
        collapse_text: Some("收起".into()),
    }),
    "多行文本"
}
```

- `rows=1` 时仍使用单行 `text-overflow: ellipsis`。
- `expandable=true` 时在尾部渲染 `.adui-typography-expand` 按钮，可切换展开/收起。

## Copyable

```rust
Text {
    copyable: Some(TypographyCopyable {
        tooltips: Some(("复制".into(), "已复制".into())),
        ..TypographyCopyable::new("复制内容")
    }),
    "复制示例"
}
```

- 默认图标为 `IconKind::Copy` / `IconKind::Check`。
- 在 Web 端自动调用 `navigator.clipboard.writeText`，其他端会触发回调但不改变剪贴板。
- `title` 根据复制状态在 `tooltips` 两个文案之间切换。

## Editable

```rust
let paragraph_text = use_signal(|| "可编辑段落".to_string());

Paragraph {
    editable: Some(TypographyEditable {
        text: Some(paragraph_text.read().clone()),
        placeholder: Some("输入新的内容".into()),
        auto_focus: true,
        ..Default::default()
    }),
    on_edit: move |value: String| paragraph_text.set(value),
    on_edit_cancel: move |_| log::info!("edit canceled"),
    "{paragraph_text.read().clone()}"
}
```

- 编辑状态下会显示输入框 + `确认/取消` 图标按钮（默认采用 `Check/Close`）。
- `Enter`（Text）或 `Ctrl+Enter`（Paragraph）提交，`Escape` 取消。

## 示例

可直接运行 `dx serve --example typography_demo` 查看 Tone、Copyable、Ellipsis、Editable 组合效果，对应源码位于 `examples/typography_demo.rs`。
