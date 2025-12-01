# Typography 排版

## 概述

Typography 组件提供文本组件，包括 Title、Paragraph 和 Text。它支持各种文本样式、省略号、可复制文本和内联编辑。

## API 参考

### TitleProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `level` | `TitleLevel` | `TitleLevel::H1` | 标题级别（H1-H5） |
| `type` | `TextType` | `TextType::Default` | 文本色调 |
| `strong` | `bool` | `false` | 粗体文本 |
| `italic` | `bool` | `false` | 斜体文本 |
| `underline` | `bool` | `false` | 下划线文本 |
| `delete` | `bool` | `false` | 删除线文本 |
| `code` | `bool` | `false` | 代码样式 |
| `mark` | `bool` | `false` | 高亮文本 |
| `copyable` | `Option<TypographyCopyable>` | `None` | 可复制配置 |
| `ellipsis` | `bool` | `false` | 启用省略号 |
| `ellipsis_config` | `Option<TypographyEllipsis>` | `None` | 省略号配置 |
| `editable` | `Option<TypographyEditable>` | `None` | 内联编辑配置 |
| `disabled` | `bool` | `false` | 禁用文本 |
| `on_copy` | `Option<EventHandler<String>>` | `None` | 文本被复制时调用 |
| `on_edit` | `Option<EventHandler<String>>` | `None` | 编辑完成时调用 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 标题内容（必需） |

### TextProps

与 TitleProps 类似，但用于文本元素。

### ParagraphProps

与 TitleProps 类似，但用于段落元素。

### TitleLevel

- `H1` - 标题 1（默认）
- `H2` - 标题 2
- `H3` - 标题 3
- `H4` - 标题 4
- `H5` - 标题 5

### TextType

- `Default` - 默认文本（默认）
- `Secondary` - 次要文本
- `Success` - 成功文本（绿色）
- `Warning` - 警告文本（橙色）
- `Danger` - 危险文本（红色）
- `Disabled` - 禁用文本

### TypographyCopyable

| 字段 | 类型 | 说明 |
|------|------|------|
| `text` | `String` | 要复制的文本 |
| `icon` | `Option<Element>` | 自定义复制图标 |
| `copied_icon` | `Option<Element>` | 自定义已复制图标 |
| `tooltips` | `Option<(String, String)>` | 工具提示文本（之前，之后） |

### TypographyEllipsis

| 字段 | 类型 | 说明 |
|------|------|------|
| `rows` | `Option<u16>` | 省略号前的行数 |
| `expandable` | `bool` | 文本是否可以展开 |
| `expand_text` | `Option<String>` | 展开按钮文本 |
| `collapse_text` | `Option<String>` | 收起按钮文本 |
| `tooltip` | `Option<String>` | 工具提示文本 |

### TypographyEditable

| 字段 | 类型 | 说明 |
|------|------|------|
| `text` | `Option<String>` | 可编辑文本值 |
| `placeholder` | `Option<String>` | 占位符文本 |
| `auto_focus` | `bool` | 编辑时自动聚焦 |
| `max_length` | `Option<usize>` | 最大长度 |
| `enter_icon` | `Option<Element>` | 确认图标 |
| `cancel_icon` | `Option<Element>` | 取消图标 |

## 使用示例

### 基础标题

```rust
use adui_dioxus::{Title, TitleLevel};

rsx! {
    Title {
        level: TitleLevel::H1,
        "主标题"
    }
}
```

### 带样式的文本

```rust
use adui_dioxus::{Text, TextType};

rsx! {
    div {
        Text { r#type: TextType::Default, "默认文本" }
        Text { r#type: TextType::Success, "成功文本" }
        Text { r#type: TextType::Warning, "警告文本" }
        Text { r#type: TextType::Danger, "危险文本" }
    }
}
```

### 可复制文本

```rust
use adui_dioxus::{Text, TypographyCopyable};

rsx! {
    Text {
        copyable: Some(TypographyCopyable::new("复制此文本")),
        "此文本可以复制"
    }
}
```

### 省略文本

```rust
use adui_dioxus::{Paragraph, TypographyEllipsis};

rsx! {
    Paragraph {
        ellipsis: true,
        ellipsis_config: Some(TypographyEllipsis {
            rows: Some(2),
            expandable: true,
            expand_text: Some("展开".to_string()),
            collapse_text: Some("收起".to_string()),
            ..Default::default()
        }),
        "长文本，当超过指定行数时将被截断并显示省略号..."
    }
}
```

### 可编辑文本

```rust
use adui_dioxus::{Text, TypographyEditable};

rsx! {
    Text {
        editable: Some(TypographyEditable {
            text: Some("可编辑文本".to_string()),
            placeholder: Some("输入文本".to_string()),
            ..Default::default()
        }),
        "点击编辑"
    }
}
```

## 使用场景

- **标题**：显示页面和章节标题
- **正文**：显示各种样式的正文
- **代码**：显示代码片段
- **可复制内容**：使文本易于复制
- **截断文本**：显示带展开选项的截断文本

## 与 Ant Design 6.0.0 的差异

- ✅ Title、Text 和 Paragraph 组件
- ✅ 文本色调和样式
- ✅ 可复制功能
- ✅ 带展开/收起的省略号
- ✅ 内联编辑
- ⚠️ 某些高级样式选项可能有所不同

