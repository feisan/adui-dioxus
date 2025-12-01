# Tooltip

## Overview

The Tooltip component displays a small popup with additional information when users hover over or click an element. It's commonly used to provide context or help text.

## API Reference

### TooltipProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `Option<String>` | `None` | Simple text title shown inside tooltip |
| `content` | `Option<Element>` | `None` | Custom tooltip content node |
| `placement` | `Option<TooltipPlacement>` | `None` (defaults to `Top`) | Placement relative to trigger |
| `trigger` | `TooltipTrigger` | `TooltipTrigger::Hover` | How tooltip is triggered |
| `open` | `Option<bool>` | `None` | Controlled open state |
| `default_open` | `Option<bool>` | `None` | Initial open state in uncontrolled mode |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | Called when open state changes |
| `disabled` | `bool` | `false` | Disable user interaction |
| `class` | `Option<String>` | `None` | Extra class for trigger wrapper |
| `overlay_class` | `Option<String>` | `None` | Extra class for tooltip bubble |
| `overlay_style` | `Option<String>` | `None` | Inline styles for tooltip bubble |
| `children` | `Element` | - | Trigger element (required) |

### TooltipPlacement

- `Top` - Above the trigger (default)
- `Bottom` - Below the trigger
- `Left` - To the left of the trigger
- `Right` - To the right of the trigger

### TooltipTrigger

- `Hover` - Show on hover (default)
- `Click` - Show on click

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Tooltip, Button, ButtonType};

rsx! {
    Tooltip {
        title: Some("This is a tooltip".to_string()),
        Button {
            r#type: ButtonType::Primary,
            "Hover me"
        }
    }
}
```

### Custom Content

```rust
use adui_dioxus::Tooltip;

rsx! {
    Tooltip {
        content: Some(rsx! {
            div {
                "Custom tooltip content"
                br {}
                "With multiple lines"
            }
        }),
        span { "Hover for custom tooltip" }
    }
}
```

### Click Trigger

```rust
use adui_dioxus::{Tooltip, TooltipTrigger};

rsx! {
    Tooltip {
        title: Some("Click to show".to_string()),
        trigger: TooltipTrigger::Click,
        Button { "Click me" }
    }
}
```

### Controlled Tooltip

```rust
use adui_dioxus::Tooltip;
use dioxus::prelude::*;

let is_open = use_signal(|| false);

rsx! {
    Tooltip {
        open: Some(*is_open.read()),
        on_open_change: Some(move |open| {
            is_open.set(open);
        }),
        title: Some("Controlled tooltip".to_string()),
        Button { "Toggle tooltip" }
    }
}
```

## Use Cases

- **Help Text**: Provide additional information
- **Form Fields**: Show field descriptions
- **Icons**: Explain icon meanings
- **Disabled Elements**: Explain why something is disabled

## Differences from Ant Design 6.0.0

- ✅ Hover and click triggers
- ✅ Custom content support
- ✅ Placement options
- ✅ Controlled and uncontrolled modes
- ⚠️ Arrow customization not yet implemented
- ⚠️ Some advanced styling options may differ

