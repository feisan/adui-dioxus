# TextArea

## Overview

The TextArea component provides a multi-line text input field. It supports character counting, max length, and various visual variants.

## API Reference

### TextAreaProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled value |
| `default_value` | `Option<String>` | `None` | Initial value in uncontrolled mode |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `rows` | `Option<u16>` | `None` | Number of rows (default: 3) |
| `disabled` | `bool` | `false` | Disable interactions |
| `size` | `Option<InputSize>` | `None` | Component size |
| `variant` | `Option<Variant>` | `None` | Visual variant |
| `status` | `Option<ControlStatus>` | `None` | Control status |
| `max_length` | `Option<usize>` | `None` | Maximum length |
| `show_count` | `bool` | `false` | Show character count |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `class_names` | `Option<InputClassNames>` | `None` | Semantic class names |
| `styles` | `Option<InputStyles>` | `None` | Semantic styles |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when value changes |

## Usage Examples

### Basic TextArea

```rust
use adui_dioxus::TextArea;
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    TextArea {
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("Enter text".to_string()),
    }
}
```

### With Character Count

```rust
use adui_dioxus::TextArea;

rsx! {
    TextArea {
        max_length: Some(100),
        show_count: true,
        placeholder: Some("Enter text".to_string()),
    }
}
```

### With Custom Rows

```rust
use adui_dioxus::TextArea;

rsx! {
    TextArea {
        rows: Some(5),
        placeholder: Some("Enter text".to_string()),
    }
}
```

## Use Cases

- **Forms**: Multi-line text input in forms
- **Comments**: Comment input fields
- **Descriptions**: Description input fields
- **Notes**: Note-taking input fields

## Differences from Ant Design 6.0.0

- ✅ Multi-line text input
- ✅ Character counting
- ✅ Max length support
- ✅ Visual variants
- ⚠️ Some advanced features may differ

