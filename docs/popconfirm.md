# Popconfirm

## Overview

The Popconfirm component displays a confirmation dialog in a popover. It's used to confirm actions before they are executed, typically for destructive operations.

## API Reference

### PopconfirmProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `String` | - | Main title shown in confirmation panel (required) |
| `description` | `Option<String>` | `None` | Optional description text displayed under title |
| `ok_text` | `Option<String>` | `None` (defaults to "确定") | Text for confirm button |
| `cancel_text` | `Option<String>` | `None` (defaults to "取消") | Text for cancel button |
| `on_confirm` | `Option<EventHandler<()>>` | `None` | Called when user confirms action |
| `on_cancel` | `Option<EventHandler<()>>` | `None` | Called when user cancels action |
| `ok_type` | `Option<ButtonType>` | `None` (defaults to `Primary`) | Visual type for confirm button |
| `ok_danger` | `bool` | `false` | Whether confirm button uses danger styling |
| `placement` | `Option<TooltipPlacement>` | `None` (defaults to `Top`) | Placement relative to trigger |
| `trigger` | `TooltipTrigger` | `TooltipTrigger::Click` | Trigger mode |
| `open` | `Option<bool>` | `None` | Controlled open state |
| `default_open` | `Option<bool>` | `None` | Initial open state in uncontrolled mode |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | Called when open state changes |
| `disabled` | `bool` | `false` | Disable user interaction |
| `class` | `Option<String>` | `None` | Extra class for trigger wrapper |
| `overlay_class` | `Option<String>` | `None` | Extra class for popconfirm panel |
| `overlay_style` | `Option<String>` | `None` | Inline styles for popconfirm panel |
| `children` | `Element` | - | Trigger element (required) |

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Popconfirm, Button, ButtonType};

rsx! {
    Popconfirm {
        title: "Are you sure you want to delete this item?".to_string(),
        on_confirm: Some(move |_| {
            println!("Confirmed!");
        }),
        Button {
            r#type: ButtonType::Danger,
            "Delete"
        }
    }
}
```

### With Description

```rust
use adui_dioxus::{Popconfirm, Button, ButtonType};

rsx! {
    Popconfirm {
        title: "Delete Item".to_string(),
        description: Some("This action cannot be undone.".to_string()),
        ok_text: Some("Yes, Delete".to_string()),
        cancel_text: Some("Cancel".to_string()),
        ok_danger: true,
        on_confirm: Some(move |_| {
            println!("Item deleted");
        }),
        Button {
            r#type: ButtonType::Danger,
            "Delete"
        }
    }
}
```

### Custom Button Text

```rust
use adui_dioxus::{Popconfirm, Button, ButtonType};

rsx! {
    Popconfirm {
        title: "Confirm Action".to_string(),
        ok_text: Some("Proceed".to_string()),
        cancel_text: Some("Abort".to_string()),
        Button { "Action" }
    }
}
```

## Use Cases

- **Delete Confirmations**: Confirm before deleting items
- **Destructive Actions**: Confirm dangerous operations
- **Form Submissions**: Confirm before submitting forms
- **State Changes**: Confirm before changing important states

## Differences from Ant Design 6.0.0

- ✅ Basic confirmation functionality
- ✅ Custom button text
- ✅ Danger button styling
- ✅ Description support
- ⚠️ Icon customization not yet implemented
- ⚠️ Some advanced styling options may differ

