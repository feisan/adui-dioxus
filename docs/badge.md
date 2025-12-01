# Badge

## Overview

The Badge component displays a badge or notification indicator, typically used to show counts, status, or notifications on other components like buttons, avatars, or menu items.

## API Reference

### BadgeProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `count` | `Option<Element>` | `None` | Number or custom element to show in badge |
| `count_number` | `Option<u32>` | `None` | Numeric count (for backward compatibility) |
| `overflow_count` | `u32` | `99` | Max count before displaying "overflow+" |
| `dot` | `bool` | `false` | Whether to show red dot without number |
| `show_zero` | `bool` | `false` | Whether to show badge when count is zero |
| `status` | `Option<BadgeStatus>` | `None` | Optional semantic status |
| `color` | `Option<BadgeColor>` | `None` | Badge color (preset or custom) |
| `text` | `Option<String>` | `None` | Text shown next to status indicator |
| `size` | `BadgeSize` | `BadgeSize::Default` | Badge size |
| `offset` | `Option<(f32, f32)>` | `None` | Offset position [x, y] for badge placement |
| `title` | `Option<String>` | `None` | Title attribute for badge (tooltip) |
| `class` | `Option<String>` | `None` | Extra class on root element |
| `style` | `Option<String>` | `None` | Inline style for root element |
| `children` | `Option<Element>` | `None` | Wrapped element to display badge on |

### BadgeStatus

- `Default` - Default status
- `Success` - Success status (green)
- `Warning` - Warning status (orange)
- `Error` - Error status (red)

### BadgeColor

- `Preset(String)` - Preset color name
- `Custom(String)` - Custom color (hex, rgb, etc.)

### BadgeSize

- `Default` - Default size
- `Small` - Small size

## Usage Examples

### Basic Badge with Count

```rust
use adui_dioxus::{Badge, Button, ButtonType};

rsx! {
    Badge {
        count_number: Some(5),
        children: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "Messages"
            }
        }),
    }
}
```

### Dot Badge

```rust
use adui_dioxus::{Badge, Avatar};

rsx! {
    Badge {
        dot: true,
        children: Some(rsx! {
            Avatar { children: Some(rsx!("U")) }
        }),
    }
}
```

### Status Badge

```rust
use adui_dioxus::{Badge, BadgeStatus};

rsx! {
    Badge {
        status: Some(BadgeStatus::Success),
        text: Some("Online".to_string()),
    }
}
```

### Custom Color Badge

```rust
use adui_dioxus::{Badge, BadgeColor};

rsx! {
    Badge {
        count_number: Some(99),
        color: Some(BadgeColor::Custom("#ff4d4f".to_string())),
        children: Some(rsx!(div { "Notifications" })),
    }
}
```

## Use Cases

- **Notifications**: Show unread message counts
- **Status Indicators**: Display online/offline status
- **Counters**: Show item counts on buttons or icons
- **Alerts**: Indicate new items or updates

## Differences from Ant Design 6.0.0

- ✅ Count badges with overflow handling
- ✅ Dot badges
- ✅ Status badges
- ✅ Custom colors
- ✅ Offset positioning
- ⚠️ Some advanced styling options may differ

