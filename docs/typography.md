# Typography

## Overview

The Typography component provides text components including Title, Paragraph, and Text. It supports various text styles, ellipsis, copyable text, and inline editing.

## API Reference

### TitleProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `level` | `TitleLevel` | `TitleLevel::H1` | Heading level (H1-H5) |
| `type` | `TextType` | `TextType::Default` | Text tone |
| `strong` | `bool` | `false` | Bold text |
| `italic` | `bool` | `false` | Italic text |
| `underline` | `bool` | `false` | Underlined text |
| `delete` | `bool` | `false` | Strikethrough text |
| `code` | `bool` | `false` | Code style |
| `mark` | `bool` | `false` | Highlighted text |
| `copyable` | `Option<TypographyCopyable>` | `None` | Copyable configuration |
| `ellipsis` | `bool` | `false` | Enable ellipsis |
| `ellipsis_config` | `Option<TypographyEllipsis>` | `None` | Ellipsis configuration |
| `editable` | `Option<TypographyEditable>` | `None` | Inline editing configuration |
| `disabled` | `bool` | `false` | Disable text |
| `on_copy` | `Option<EventHandler<String>>` | `None` | Called when text is copied |
| `on_edit` | `Option<EventHandler<String>>` | `None` | Called when editing finishes |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Title content (required) |

### TextProps

Similar to TitleProps but for text elements.

### ParagraphProps

Similar to TitleProps but for paragraph elements.

### TitleLevel

- `H1` - Heading 1 (default)
- `H2` - Heading 2
- `H3` - Heading 3
- `H4` - Heading 4
- `H5` - Heading 5

### TextType

- `Default` - Default text (default)
- `Secondary` - Secondary text
- `Success` - Success text (green)
- `Warning` - Warning text (orange)
- `Danger` - Danger text (red)
- `Disabled` - Disabled text

### TypographyCopyable

| Field | Type | Description |
|-------|------|-------------|
| `text` | `String` | Text to copy |
| `icon` | `Option<Element>` | Custom copy icon |
| `copied_icon` | `Option<Element>` | Custom copied icon |
| `tooltips` | `Option<(String, String)>` | Tooltip texts (before, after) |

### TypographyEllipsis

| Field | Type | Description |
|-------|------|-------------|
| `rows` | `Option<u16>` | Number of rows before ellipsis |
| `expandable` | `bool` | Whether text can be expanded |
| `expand_text` | `Option<String>` | Expand button text |
| `collapse_text` | `Option<String>` | Collapse button text |
| `tooltip` | `Option<String>` | Tooltip text |

### TypographyEditable

| Field | Type | Description |
|-------|------|-------------|
| `text` | `Option<String>` | Editable text value |
| `placeholder` | `Option<String>` | Placeholder text |
| `auto_focus` | `bool` | Auto focus when editing |
| `max_length` | `Option<usize>` | Maximum length |
| `enter_icon` | `Option<Element>` | Enter icon |
| `cancel_icon` | `Option<Element>` | Cancel icon |

## Usage Examples

### Basic Title

```rust
use adui_dioxus::{Title, TitleLevel};

rsx! {
    Title {
        level: TitleLevel::H1,
        "Main Title"
    }
}
```

### Text with Styles

```rust
use adui_dioxus::{Text, TextType};

rsx! {
    div {
        Text { r#type: TextType::Default, "Default text" }
        Text { r#type: TextType::Success, "Success text" }
        Text { r#type: TextType::Warning, "Warning text" }
        Text { r#type: TextType::Danger, "Danger text" }
    }
}
```

### Copyable Text

```rust
use adui_dioxus::{Text, TypographyCopyable};

rsx! {
    Text {
        copyable: Some(TypographyCopyable::new("Copy this text")),
        "This text can be copied"
    }
}
```

### Ellipsis Text

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
        "Long text that will be truncated with ellipsis when it exceeds the specified number of rows..."
    }
}
```

### Editable Text

```rust
use adui_dioxus::{Text, TypographyEditable};

rsx! {
    Text {
        editable: Some(TypographyEditable {
            text: Some("Editable text".to_string()),
            placeholder: Some("Enter text".to_string()),
            ..Default::default()
        }),
        "Click to edit"
    }
}
```

## Use Cases

- **Headings**: Display page and section headings
- **Body Text**: Display body text with various styles
- **Code**: Display code snippets
- **Copyable Content**: Make text easily copyable
- **Truncated Text**: Display truncated text with expand option

## Differences from Ant Design 6.0.0

- ✅ Title, Text, and Paragraph components
- ✅ Text tones and styles
- ✅ Copyable functionality
- ✅ Ellipsis with expand/collapse
- ✅ Inline editing
- ⚠️ Some advanced styling options may differ

