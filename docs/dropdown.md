# Dropdown

## Overview

The Dropdown component provides a dropdown menu that appears when clicking or hovering over a trigger element. It's commonly used for action menus and context menus.

## API Reference

### DropdownProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<DropdownItem>` | - | Menu items to display (required) |
| `trigger` | `DropdownTrigger` | `DropdownTrigger::Click` | Trigger mode |
| `placement` | `Option<DropdownPlacement>` | `None` | Placement of dropdown menu |
| `open` | `Option<bool>` | `None` | Controlled open state |
| `default_open` | `Option<bool>` | `None` | Initial open state |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | Called when open state changes |
| `on_click` | `Option<EventHandler<String>>` | `None` | Called when menu item is clicked |
| `disabled` | `bool` | `false` | Disable interactions |
| `class` | `Option<String>` | `None` | Extra class for trigger wrapper |
| `overlay_class` | `Option<String>` | `None` | Extra class for dropdown menu |
| `overlay_style` | `Option<String>` | `None` | Inline styles for dropdown menu |
| `overlay_width` | `Option<f32>` | `None` | Custom width for dropdown menu |
| `children` | `Element` | - | Trigger element (required) |

### DropdownItem

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the item |
| `label` | `String` | Display label |
| `disabled` | `bool` | Whether item is disabled |

### DropdownTrigger

- `Click` - Click to open (default)
- `Hover` - Hover to open

### DropdownPlacement

- `BottomLeft` - Bottom left (default)
- `BottomRight` - Bottom right

## Usage Examples

### Basic Dropdown

```rust
use adui_dioxus::{Dropdown, DropdownItem, Button, ButtonType};

rsx! {
    Dropdown {
        items: vec![
            DropdownItem::new("1", "Menu Item 1"),
            DropdownItem::new("2", "Menu Item 2"),
            DropdownItem::new("3", "Menu Item 3"),
        ],
        Button {
            r#type: ButtonType::Primary,
            "Open Menu"
        }
    }
}
```

### Hover Trigger

```rust
use adui_dioxus::{Dropdown, DropdownTrigger, DropdownItem, Button, ButtonType};

rsx! {
    Dropdown {
        trigger: DropdownTrigger::Hover,
        items: vec![
            DropdownItem::new("1", "Menu Item 1"),
            DropdownItem::new("2", "Menu Item 2"),
        ],
        Button {
            r#type: ButtonType::Primary,
            "Hover Me"
        }
    }
}
```

### With Click Handler

```rust
use adui_dioxus::{Dropdown, DropdownItem, Button, ButtonType};

rsx! {
    Dropdown {
        items: vec![
            DropdownItem::new("1", "Action 1"),
            DropdownItem::new("2", "Action 2"),
        ],
        on_click: Some(move |key| {
            println!("Clicked: {}", key);
        }),
        Button {
            r#type: ButtonType::Primary,
            "Actions"
        }
    }
}
```

### Controlled Dropdown

```rust
use adui_dioxus::{Dropdown, DropdownItem, Button, ButtonType};
use dioxus::prelude::*;

let open = use_signal(|| false);

rsx! {
    Dropdown {
        items: vec![
            DropdownItem::new("1", "Menu Item 1"),
        ],
        open: Some(*open.read()),
        on_open_change: Some(move |is_open| {
            open.set(is_open);
        }),
        Button {
            r#type: ButtonType::Primary,
            "Toggle Menu"
        }
    }
}
```

## Use Cases

- **Action Menus**: Display action menus
- **Context Menus**: Create context menus
- **Navigation**: Dropdown navigation
- **Settings**: Settings dropdowns

## Differences from Ant Design 6.0.0

- ✅ Click and hover triggers
- ✅ Menu items
- ✅ Controlled and uncontrolled modes
- ⚠️ Some advanced features may differ

