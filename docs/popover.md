# Popover

## Overview

The Popover component displays a floating panel with rich content. It's similar to Tooltip but supports more complex content including titles and interactive elements.

## API Reference

### PopoverProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `Option<Element>` | `None` | Optional title node displayed at the top |
| `content` | `Option<Element>` | `None` | Main content of the popover |
| `placement` | `Option<TooltipPlacement>` | `None` (defaults to `Top`) | Placement relative to trigger |
| `trigger` | `TooltipTrigger` | `TooltipTrigger::Click` | Trigger mode |
| `open` | `Option<bool>` | `None` | Controlled open state |
| `default_open` | `Option<bool>` | `None` | Initial open state in uncontrolled mode |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | Called when open state changes |
| `disabled` | `bool` | `false` | Disable user interaction |
| `class` | `Option<String>` | `None` | Extra class for trigger wrapper |
| `overlay_class` | `Option<String>` | `None` | Extra class for popover panel |
| `overlay_style` | `Option<String>` | `None` | Inline styles for popover panel |
| `children` | `Element` | - | Trigger element (required) |

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Popover, Button, ButtonType};

rsx! {
    Popover {
        title: Some(rsx!("Popover Title")),
        content: Some(rsx!("This is the popover content")),
        Button {
            r#type: ButtonType::Primary,
            "Click me"
        }
    }
}
```

### With Rich Content

```rust
use adui_dioxus::{Popover, Button, ButtonType};

rsx! {
    Popover {
        title: Some(rsx!("Settings")),
        content: Some(rsx! {
            div {
                p { "Configure your preferences" }
                Button {
                    r#type: ButtonType::Primary,
                    "Save"
                }
            }
        }),
        Button { "Open Popover" }
    }
}
```

### Hover Trigger

```rust
use adui_dioxus::{Popover, TooltipTrigger};

rsx! {
    Popover {
        trigger: TooltipTrigger::Hover,
        content: Some(rsx!("Hover content")),
        span { "Hover me" }
    }
}
```

## Use Cases

- **Contextual Information**: Show additional details
- **Action Menus**: Display action options
- **Forms**: Show form fields in popover
- **Help Content**: Display help information

## Differences from Ant Design 6.0.0

- ✅ Title and content support
- ✅ Hover and click triggers
- ✅ Placement options
- ✅ Controlled and uncontrolled modes
- ⚠️ Arrow customization not yet implemented
- ⚠️ Some advanced styling options may differ

