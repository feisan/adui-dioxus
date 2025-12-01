# Drawer

## Overview

The Drawer component displays a panel that slides in from the edge of the screen. It's commonly used for settings, filters, or additional content.

## API Reference

### DrawerProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `bool` | - | Whether the drawer is visible (required) |
| `title` | `Option<String>` | `None` | Optional title displayed in header |
| `on_close` | `Option<EventHandler<()>>` | `None` | Called when drawer is closed |
| `mask_closable` | `bool` | `true` | Whether clicking mask closes drawer |
| `closable` | `bool` | `true` | Show close button |
| `destroy_on_close` | `bool` | `false` | Destroy contents when closed |
| `placement` | `DrawerPlacement` | `DrawerPlacement::Right` | Drawer side |
| `size` | `Option<f32>` | `None` | Logical size (width for left/right, height for top/bottom, defaults to 378) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Drawer content (required) |

### DrawerPlacement

- `Left` - Slide from left
- `Right` - Slide from right (default)
- `Top` - Slide from top
- `Bottom` - Slide from bottom

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Drawer, Button, ButtonType};
use dioxus::prelude::*;

let is_open = use_signal(|| false);

rsx! {
    div {
        Button {
            onclick: move |_| {
                is_open.set(true);
            },
            "Open Drawer"
        }
        Drawer {
            open: *is_open.read(),
            title: Some("Drawer Title".to_string()),
            on_close: Some(move |_| {
                is_open.set(false);
            }),
            "Drawer content goes here"
        }
    }
}
```

### Left Placement

```rust
use adui_dioxus::{Drawer, DrawerPlacement};

rsx! {
    Drawer {
        open: true,
        placement: DrawerPlacement::Left,
        title: Some("Left Drawer".to_string()),
        "Content from left"
    }
}
```

### Custom Size

```rust
use adui_dioxus::Drawer;

rsx! {
    Drawer {
        open: true,
        size: Some(500.0),
        title: Some("Wide Drawer".to_string()),
        "Wide content"
    }
}
```

## Use Cases

- **Settings Panels**: Display settings
- **Filters**: Show filter options
- **Details**: Display detailed information
- **Navigation**: Side navigation drawers

## Differences from Ant Design 6.0.0

- ✅ Basic drawer functionality
- ✅ Multiple placements
- ✅ Custom sizes
- ✅ Mask and close button support
- ⚠️ Some advanced features may differ
- ⚠️ Footer support not yet implemented

