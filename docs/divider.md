# Divider

## Overview

The Divider component is used to separate content sections with a horizontal or vertical line. It can include optional text content and supports different orientations and styles.

## API Reference

### DividerProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `dashed` | `bool` | `false` | Whether the divider is dashed |
| `plain` | `bool` | `false` | Whether the divider is plain (no border) |
| `vertical` | `bool` | `false` | Whether the divider is vertical |
| `orientation` | `DividerOrientation` | `DividerOrientation::Center` | Text orientation: `Left`, `Center`, `Right` |
| `orientation_margin` | `Option<String>` | `None` | Margin for orientation (defaults to "16px") |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `content` | `Option<Element>` | `None` | Optional content/text to display |

### DividerOrientation

- `Left` - Text aligned to the left
- `Center` - Text centered (default)
- `Right` - Text aligned to the right

## Usage Examples

### Basic Horizontal Divider

```rust
use adui_dioxus::Divider;

rsx! {
    div {
        p { "Content above" }
        Divider {}
        p { "Content below" }
    }
}
```

### Divider with Text

```rust
use adui_dioxus::{Divider, DividerOrientation};

rsx! {
    Divider {
        content: Some(rsx!("OR")),
        orientation: DividerOrientation::Center,
    }
}
```

### Dashed Divider

```rust
use adui_dioxus::Divider;

rsx! {
    Divider {
        dashed: true,
    }
}
```

### Vertical Divider

```rust
use adui_dioxus::Divider;

rsx! {
    div {
        style: "display: flex;",
        span { "Left" }
        Divider { vertical: true }
        span { "Right" }
    }
}
```

## Use Cases

- **Content Separation**: Separate different sections of content
- **Form Sections**: Divide form fields into logical groups
- **List Items**: Separate items in lists
- **Vertical Layouts**: Create vertical separators in horizontal layouts

## Differences from Ant Design 6.0.0

- ✅ All basic divider styles supported
- ✅ Horizontal and vertical orientations
- ✅ Text content with orientation control
- ✅ Dashed and plain variants
- ⚠️ Some advanced styling options may differ

