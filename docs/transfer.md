# Transfer

## Overview

The Transfer component provides a double-column layout for moving items between two lists. It allows selecting items from a source list and moving them to a target list.

## API Reference

### TransferProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `data_source` | `Vec<TransferItem>` | - | Data source for the transfer lists (required) |
| `target_keys` | `Option<Vec<String>>` | `None` | Keys of items in the right (target) list |
| `selected_keys` | `Option<Vec<String>>` | `None` | Keys of currently selected items |
| `titles` | `Option<(String, String)>` | `None` | Titles for left and right lists |
| `operations` | `Option<(String, String)>` | `None` | Text for transfer operation buttons |
| `show_search` | `bool` | `false` | Whether to show search input |
| `search_placeholder` | `Option<String>` | `None` | Placeholder text for search input |
| `disabled` | `bool` | `false` | Whether the component is disabled |
| `show_select_all` | `bool` | `true` | Whether to show select all checkbox |
| `one_way` | `bool` | `false` | One-way mode (only left to right) |
| `on_change` | `Option<EventHandler<(Vec<String>, TransferDirection, Vec<String>)>>` | `None` | Called when target keys change |
| `on_select_change` | `Option<EventHandler<(Vec<String>, Vec<String>)>>` | `None` | Called when selection changes |
| `on_search` | `Option<EventHandler<(TransferDirection, String)>>` | `None` | Called when search text changes |
| `filter_option` | `Option<fn(&str, &TransferItem, TransferDirection) -> bool>` | `None` | Custom filter function |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### TransferItem

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique identifier for the item |
| `title` | `String` | Display title for the item |
| `description` | `Option<String>` | Optional description shown below title |
| `disabled` | `bool` | Whether this item is disabled |

### TransferDirection

- `Left` - Left list
- `Right` - Right list

## Usage Examples

### Basic Transfer

```rust
use adui_dioxus::{Transfer, TransferItem};
use dioxus::prelude::*;

let target_keys = use_signal(|| vec![]);

rsx! {
    Transfer {
        data_source: vec![
            TransferItem::new("1", "Item 1"),
            TransferItem::new("2", "Item 2"),
            TransferItem::new("3", "Item 3"),
        ],
        target_keys: Some(target_keys.read().clone()),
        on_change: Some(move |(keys, _dir, _selected)| {
            target_keys.set(keys);
        }),
    }
}
```

### With Search

```rust
use adui_dioxus::{Transfer, TransferItem};

rsx! {
    Transfer {
        data_source: vec![
            TransferItem::new("1", "Item 1"),
            TransferItem::new("2", "Item 2"),
        ],
        show_search: true,
        search_placeholder: Some("Search items".to_string()),
    }
}
```

### With Custom Titles

```rust
use adui_dioxus::{Transfer, TransferItem};

rsx! {
    Transfer {
        data_source: vec![
            TransferItem::new("1", "Item 1"),
        ],
        titles: Some(("Available".to_string(), "Selected".to_string())),
        operations: Some(("Add".to_string(), "Remove".to_string())),
    }
}
```

## Use Cases

- **Permission Management**: Manage user permissions
- **Role Assignment**: Assign roles to users
- **Data Selection**: Select data from source to target
- **List Management**: Manage items between lists

## Differences from Ant Design 6.0.0

- ✅ Double-column layout
- ✅ Item selection
- ✅ Search functionality
- ✅ One-way mode
- ⚠️ Some advanced features may differ

