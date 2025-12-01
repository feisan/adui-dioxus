# Image

## Overview

The Image component displays images with loading states, fallback support, and an interactive preview modal with zoom and navigation capabilities.

## API Reference

### ImageProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `src` | `String` | - | Image source URL (required) |
| `alt` | `Option<String>` | `None` | Alt text for the image |
| `width` | `Option<String>` | `None` | Width of the image |
| `height` | `Option<String>` | `None` | Height of the image |
| `fallback` | `Option<String>` | `None` | Fallback image source if main fails |
| `placeholder` | `Option<Element>` | `None` | Placeholder element shown while loading |
| `preview` | `bool` | `true` | Whether to enable preview on click |
| `preview_config` | `Option<PreviewConfig>` | `None` | Preview configuration |
| `on_load` | `Option<EventHandler<()>>` | `None` | Called when image loads successfully |
| `on_error` | `Option<EventHandler<()>>` | `None` | Called when image fails to load |
| `class` | `Option<String>` | `None` | Extra class for root element |
| `style` | `Option<String>` | `None` | Inline style for root element |
| `image_class` | `Option<String>` | `None` | Extra class for image element |
| `image_style` | `Option<String>` | `None` | Inline style for image element |

### PreviewConfig

| Field | Type | Description |
|-------|------|-------------|
| `visible` | `bool` | Whether preview is enabled |
| `mask` | `Option<String>` | Mask element or text to show on hover |
| `close_icon` | `Option<Element>` | Custom close icon |
| `scale` | `f32` | Initial scale for preview |
| `min_scale` | `f32` | Minimum scale |
| `max_scale` | `f32` | Maximum scale |

### ImageStatus

- `Loading` - Image is loading (default)
- `Loaded` - Image loaded successfully
- `Error` - Image failed to load

## Usage Examples

### Basic Image

```rust
use adui_dioxus::Image;

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        alt: Some("Example image".to_string()),
    }
}
```

### With Fallback

```rust
use adui_dioxus::Image;

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        fallback: Some("https://example.com/fallback.jpg".to_string()),
    }
}
```

### With Placeholder

```rust
use adui_dioxus::{Image, Spin};

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        placeholder: Some(rsx!(Spin {})),
    }
}
```

### Disable Preview

```rust
use adui_dioxus::Image;

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        preview: false,
    }
}
```

### Custom Preview Config

```rust
use adui_dioxus::{Image, PreviewConfig};

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        preview_config: Some(PreviewConfig::new()
            .with_mask("Click to preview")
            .scale(1.5)
            .min_scale(0.5)
            .max_scale(4.0)),
    }
}
```

## Use Cases

- **Image Galleries**: Display images in galleries
- **Product Images**: Show product images with preview
- **User Avatars**: Display user avatars with fallback
- **Content Images**: Show content images with loading states

## Differences from Ant Design 6.0.0

- ✅ Loading states
- ✅ Fallback images
- ✅ Preview modal with zoom
- ✅ Custom preview configuration
- ⚠️ Some advanced features may differ

