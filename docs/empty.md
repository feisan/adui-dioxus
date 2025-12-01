# Empty

## Overview

The Empty component displays an empty state when there is no data to show. It's commonly used in lists, tables, and other data display components to provide user feedback when content is unavailable.

## API Reference

### EmptyProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `description` | `Option<String>` | `None` | Optional description text shown under the image (defaults to "暂无数据") |
| `image` | `Option<EmptyImage>` | `None` | Image preset or custom image |
| `class` | `Option<String>` | `None` | Extra class name applied to the root |
| `style` | `Option<String>` | `None` | Inline style applied to the root |
| `footer` | `Option<Element>` | `None` | Optional footer content rendered below the description (e.g. action buttons) |

### EmptyImage

- `Default` - Default illustration (SVG)
- `Simple` - Minimal/simple illustration
- `Small` - Smaller footprint variant
- `Custom(String)` - Custom image URL

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Empty;

rsx! {
    Empty {}
}
```

### With Custom Description

```rust
use adui_dioxus::Empty;

rsx! {
    Empty {
        description: Some("No data available".to_string()),
    }
}
```

### With Custom Image

```rust
use adui_dioxus::{Empty, EmptyImage};

rsx! {
    Empty {
        image: Some(EmptyImage::Custom("https://example.com/empty.png".to_string())),
        description: Some("Custom empty state".to_string()),
    }
}
```

### With Footer Actions

```rust
use adui_dioxus::{Empty, Button, ButtonType};

rsx! {
    Empty {
        description: Some("No items found".to_string()),
        footer: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "Create Item"
            }
        }),
    }
}
```

## Use Cases

- **Empty Lists**: Display when a list has no items
- **Empty Tables**: Show when a table has no data
- **Empty Search Results**: Indicate no search results found
- **Empty States**: General empty state for any data container

## Differences from Ant Design 6.0.0

- ✅ All basic empty image presets supported
- ✅ Custom image support
- ✅ Footer content support
- ✅ Custom description support
- ⚠️ Some advanced styling options may differ

