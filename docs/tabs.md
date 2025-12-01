# Tabs

## Overview

The Tabs component organizes content into multiple panels that can be switched between. It supports different visual types, placements, and editable tabs.

## API Reference

### TabsProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<TabItem>` | - | Tab items with key/label/content (required) |
| `active_key` | `Option<String>` | `None` | Controlled active key |
| `default_active_key` | `Option<String>` | `None` | Default active key in uncontrolled mode |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when active tab changes |
| `type` | `TabsType` | `TabsType::Line` | Visual type |
| `tab_placement` | `TabPlacement` | `TabPlacement::Top` | Tab placement position |
| `centered` | `bool` | `false` | Whether to center tabs |
| `hide_add` | `bool` | `false` | Hide add button (for editable-card) |
| `on_edit` | `Option<EventHandler<TabEditAction>>` | `None` | Called when tabs are added/removed |
| `add_icon` | `Option<Element>` | `None` | Custom add icon |
| `remove_icon` | `Option<Element>` | `None` | Custom close icon |
| `size` | `Option<ComponentSize>` | `None` | Visual density |
| `destroy_inactive_tab_pane` | `bool` | `false` | Destroy inactive tab panels |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `class_names` | `Option<TabsClassNames>` | `None` | Semantic class names |
| `styles` | `Option<TabsStyles>` | `None` | Semantic styles |

### TabItem

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the tab |
| `label` | `String` | Tab label text |
| `disabled` | `bool` | Whether tab is disabled |
| `closable` | `bool` | Whether tab can be closed (editable-card) |
| `icon` | `Option<Element>` | Custom icon |
| `content` | `Option<Element>` | Tab content |

### TabsType

- `Line` - Line style tabs (default)
- `Card` - Card style tabs
- `EditableCard` - Editable card tabs with add/remove

### TabPlacement

- `Top` - Tabs on top (default)
- `Right` - Tabs on right
- `Bottom` - Tabs on bottom
- `Left` - Tabs on left

## Usage Examples

### Basic Tabs

```rust
use adui_dioxus::{Tabs, TabItem};

rsx! {
    Tabs {
        items: vec![
            TabItem::new("1", "Tab 1", Some(rsx!("Content 1"))),
            TabItem::new("2", "Tab 2", Some(rsx!("Content 2"))),
            TabItem::new("3", "Tab 3", Some(rsx!("Content 3"))),
        ],
    }
}
```

### Card Tabs

```rust
use adui_dioxus::{Tabs, TabsType, TabItem};

rsx! {
    Tabs {
        r#type: TabsType::Card,
        items: vec![
            TabItem::new("1", "Tab 1", Some(rsx!("Content 1"))),
            TabItem::new("2", "Tab 2", Some(rsx!("Content 2"))),
        ],
    }
}
```

### Editable Tabs

```rust
use adui_dioxus::{Tabs, TabsType, TabItem, TabEditAction};
use dioxus::prelude::*;

let tabs = use_signal(|| vec![
    TabItem::new("1", "Tab 1", Some(rsx!("Content 1"))),
    TabItem::new("2", "Tab 2", Some(rsx!("Content 2"))),
]);

rsx! {
    Tabs {
        r#type: TabsType::EditableCard,
        items: tabs.read().clone(),
        on_edit: Some(move |action| {
            match action {
                TabEditAction::Add => {
                    // Add new tab
                }
                TabEditAction::Remove(key) => {
                    // Remove tab
                }
            }
        }),
    }
}
```

### Centered Tabs

```rust
use adui_dioxus::{Tabs, TabItem};

rsx! {
    Tabs {
        centered: true,
        items: vec![
            TabItem::new("1", "Tab 1", Some(rsx!("Content 1"))),
            TabItem::new("2", "Tab 2", Some(rsx!("Content 2"))),
        ],
    }
}
```

## Use Cases

- **Content Organization**: Organize content into multiple sections
- **Settings Panels**: Switch between different settings panels
- **Data Views**: Switch between different data views
- **Documentation**: Organize documentation sections

## Differences from Ant Design 6.0.0

- ✅ Line, card, and editable-card types
- ✅ Multiple placements
- ✅ Centered tabs
- ✅ Editable tabs with add/remove
- ⚠️ Some advanced features may differ

