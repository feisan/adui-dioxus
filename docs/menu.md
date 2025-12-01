# Menu

## Overview

The Menu component provides navigation menus with horizontal and vertical modes. It supports nested menu items, selection, and expansion control.

## API Reference

### MenuProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<MenuItemNode>` | - | Menu items in tree structure (required) |
| `mode` | `MenuMode` | `MenuMode::Inline` | Display mode |
| `selected_keys` | `Option<Vec<String>>` | `None` | Controlled selected keys |
| `default_selected_keys` | `Option<Vec<String>>` | `None` | Default selected keys |
| `open_keys` | `Option<Vec<String>>` | `None` | Controlled open keys (inline mode) |
| `default_open_keys` | `Option<Vec<String>>` | `None` | Default open keys |
| `on_select` | `Option<EventHandler<String>>` | `None` | Called when menu item is selected |
| `on_open_change` | `Option<EventHandler<Vec<String>>>` | `None` | Called when open keys change |
| `inline_collapsed` | `bool` | `false` | Whether inline menu is collapsed |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### MenuItemNode

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String` | Unique identifier for the item |
| `label` | `String` | Display label |
| `icon` | `Option<Element>` | Optional icon |
| `disabled` | `bool` | Whether item is disabled |
| `children` | `Option<Vec<MenuItemNode>>` | Nested children (max two levels) |

### MenuMode

- `Inline` - Inline mode (default, for sider)
- `Horizontal` - Horizontal mode (for header)

## Usage Examples

### Basic Menu

```rust
use adui_dioxus::{Menu, MenuItemNode};

rsx! {
    Menu {
        items: vec![
            MenuItemNode::leaf("1", "Item 1"),
            MenuItemNode::leaf("2", "Item 2"),
            MenuItemNode::leaf("3", "Item 3"),
        ],
    }
}
```

### Horizontal Menu

```rust
use adui_dioxus::{Menu, MenuMode, MenuItemNode};

rsx! {
    Menu {
        mode: MenuMode::Horizontal,
        items: vec![
            MenuItemNode::leaf("1", "Home"),
            MenuItemNode::leaf("2", "About"),
            MenuItemNode::leaf("3", "Contact"),
        ],
    }
}
```

### With Nested Items

```rust
use adui_dioxus::{Menu, MenuItemNode};

rsx! {
    Menu {
        items: vec![
            MenuItemNode {
                id: "1".to_string(),
                label: "Parent".to_string(),
                icon: None,
                disabled: false,
                children: Some(vec![
                    MenuItemNode::leaf("1-1", "Child 1"),
                    MenuItemNode::leaf("1-2", "Child 2"),
                ]),
            },
        ],
    }
}
```

### With Selection

```rust
use adui_dioxus::{Menu, MenuItemNode};
use dioxus::prelude::*;

let selected = use_signal(|| vec!["1".to_string()]);

rsx! {
    Menu {
        items: vec![
            MenuItemNode::leaf("1", "Item 1"),
            MenuItemNode::leaf("2", "Item 2"),
        ],
        selected_keys: Some(selected.read().clone()),
        on_select: Some(move |key| {
            selected.set(vec![key]);
        }),
    }
}
```

## Use Cases

- **Navigation**: Create navigation menus
- **Sidebars**: Build sidebar navigation
- **Headers**: Create header navigation
- **Settings**: Organize settings menus

## Differences from Ant Design 6.0.0

- ✅ Inline and horizontal modes
- ✅ Nested menu items (max two levels)
- ✅ Selection control
- ✅ Expansion control
- ⚠️ Some advanced features may differ

