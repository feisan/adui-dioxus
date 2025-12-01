# Affix

## Overview

The Affix component wraps content and pins it to the viewport when scrolling past a threshold. It's commonly used for sticky navigation, toolbars, or action buttons.

## API Reference

### AffixProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `offset_top` | `Option<f64>` | `None` | Offset from top when to start affixing (pixels, defaults to 0 if neither offset is set) |
| `offset_bottom` | `Option<f64>` | `None` | Offset from bottom when to start affixing (pixels) |
| `on_change` | `Option<EventHandler<bool>>` | `None` | Callback fired when affix state changes |
| `class` | `Option<String>` | `None` | Additional CSS class for wrapper |
| `style` | `Option<String>` | `None` | Additional inline styles for wrapper |
| `children` | `Element` | - | Content to be affixed (required) |

## Usage Examples

### Basic Affix

```rust
use adui_dioxus::Affix;

rsx! {
    Affix {
        offset_top: Some(10.0),
        div {
            "This content will stick to the top when scrolled"
        }
    }
}
```

### Affix to Bottom

```rust
use adui_dioxus::Affix;

rsx! {
    Affix {
        offset_bottom: Some(20.0),
        Button {
            r#type: ButtonType::Primary,
            "Sticky Button"
        }
    }
}
```

### With Change Handler

```rust
use adui_dioxus::Affix;
use dioxus::prelude::*;

let is_affixed = use_signal(|| false);

rsx! {
    Affix {
        offset_top: Some(0.0),
        on_change: Some(move |affixed| {
            is_affixed.set(affixed);
        }),
        div {
            if *is_affixed.read() {
                "Affixed!"
            } else {
                "Not affixed"
            }
        }
    }
}
```

## Use Cases

- **Sticky Navigation**: Create sticky navigation bars
- **Action Buttons**: Pin action buttons to viewport
- **Toolbars**: Create sticky toolbars
- **Sidebars**: Pin sidebars when scrolling

## Differences from Ant Design 6.0.0

- ✅ Top and bottom affixing
- ✅ Custom offsets
- ✅ Change callbacks
- ⚠️ Container targeting not yet implemented
- ⚠️ Some advanced features may differ

