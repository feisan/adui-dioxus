# Watermark

## Overview

The Watermark component adds a watermark layer over its children. It supports both text and image watermarks with customizable appearance.

## API Reference

### WatermarkProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `z_index` | `i32` | `9` | Z-index of the watermark layer |
| `rotate` | `f32` | `-22.0` | Rotation angle in degrees |
| `width` | `Option<f32>` | `None` | Width of the watermark (auto-calculated if not provided) |
| `height` | `Option<f32>` | `None` | Height of the watermark (auto-calculated if not provided) |
| `image` | `Option<String>` | `None` | Image URL for image watermarks (takes precedence over content) |
| `content` | `Option<Vec<String>>` | `None` | Text content for the watermark (can be multiple lines) |
| `font` | `Option<WatermarkFont>` | `None` | Font configuration for text watermarks |
| `gap` | `Option<[f32; 2]>` | `None` | Gap between watermarks as `[horizontal, vertical]` (defaults to `[100, 100]`) |
| `offset` | `Option<[f32; 2]>` | `None` | Offset of the watermark from top-left as `[left, top]` |
| `class` | `Option<String>` | `None` | Extra class name for wrapper |
| `root_class` | `Option<String>` | `None` | Extra class name for root element |
| `style` | `Option<String>` | `None` | Inline style for wrapper |
| `inherit` | `bool` | `true` | Whether nested watermark contexts should inherit this watermark |
| `children` | `Element` | - | Content to be watermarked (required) |

### WatermarkFont

| Field | Type | Description |
|-------|------|-------------|
| `color` | `String` | Font color (defaults to `rgba(0, 0, 0, 0.15)`) |
| `font_size` | `f32` | Font size in pixels (defaults to 16) |
| `font_weight` | `String` | Font weight (e.g., "normal", "bold", or numeric like 400, 700) |
| `font_style` | `String` | Font style (e.g., "normal", "italic") |
| `font_family` | `String` | Font family (defaults to "sans-serif") |
| `text_align` | `String` | Text alignment (defaults to "center") |

## Usage Examples

### Basic Text Watermark

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        content: Some(vec!["Confidential".to_string()]),
        div {
            "Protected content here"
        }
    }
}
```

### Multi-line Text Watermark

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        content: Some(vec![
            "Confidential".to_string(),
            "Do not distribute".to_string(),
        ]),
        div {
            "Protected content"
        }
    }
}
```

### Image Watermark

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        image: Some("https://example.com/logo.png".to_string()),
        div {
            "Protected content"
        }
    }
}
```

### Custom Font

```rust
use adui_dioxus::{Watermark, WatermarkFont};

rsx! {
    Watermark {
        content: Some(vec!["Confidential".to_string()]),
        font: Some(WatermarkFont {
            color: "rgba(255, 0, 0, 0.3)".to_string(),
            font_size: 20.0,
            font_weight: "bold".to_string(),
            font_style: "normal".to_string(),
            font_family: "Arial".to_string(),
            text_align: "center".to_string(),
        }),
        div {
            "Protected content"
        }
    }
}
```

### Custom Gap and Offset

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        content: Some(vec!["Confidential".to_string()]),
        gap: Some([150.0, 150.0]),
        offset: Some([50.0, 50.0]),
        div {
            "Protected content"
        }
    }
}
```

## Use Cases

- **Document Protection**: Add watermarks to protect documents
- **Copyright**: Display copyright information
- **Branding**: Add brand watermarks to content
- **Confidential Content**: Mark confidential content

## Differences from Ant Design 6.0.0

- ✅ Text and image watermarks
- ✅ Custom fonts
- ✅ Custom gaps and offsets
- ✅ Nested watermark support
- ⚠️ Some advanced features may differ

