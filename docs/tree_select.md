# TreeSelect

## Overview

The TreeSelect component provides a tree-structured selection interface. It combines the functionality of a tree view with a select dropdown, allowing users to select items from a hierarchical structure.

## API Reference

### TreeSelectProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `tree_data` | `Option<Vec<TreeNode>>` | `None` | Tree data source |
| `value` | `Option<String>` | `None` | Controlled value for single-select |
| `values` | `Option<Vec<String>>` | `None` | Controlled values for multi-select |
| `multiple` | `bool` | `false` | Enable multiple selection |
| `tree_checkable` | `bool` | `false` | Show checkboxes in tree nodes |
| `show_search` | `bool` | `false` | Enable search functionality |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `disabled` | `bool` | `false` | Disable interactions |
| `status` | `Option<ControlStatus>` | `None` | Control status |
| `size` | `Option<ComponentSize>` | `None` | Component size |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `dropdown_class` | `Option<String>` | `None` | Extra class for dropdown |
| `dropdown_style` | `Option<String>` | `None` | Extra style for dropdown |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | Called when selection changes |

### TreeNode

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the node |
| `label` | `String` | Display label |
| `disabled` | `bool` | Whether node is disabled |
| `children` | `Vec<TreeNode>` | Child nodes |

## Usage Examples

### Basic TreeSelect

```rust
use adui_dioxus::{TreeSelect, TreeNode};
use dioxus::prelude::*;

let value = use_signal(|| None::<String>);

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "Node 1".to_string(),
                disabled: false,
                children: vec![
                    TreeNode {
                        key: "1-1".to_string(),
                        label: "Node 1-1".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                ],
            },
        ]),
        value: value.read().clone(),
        on_change: Some(move |values| {
            value.set(values.first().cloned());
        }),
        placeholder: Some("Select node".to_string()),
    }
}
```

### Multiple Selection

```rust
use adui_dioxus::{TreeSelect, TreeNode};
use dioxus::prelude::*;

let values = use_signal(|| vec![]);

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "Node 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        multiple: true,
        values: Some(values.read().clone()),
        on_change: Some(move |v| {
            values.set(v);
        }),
        placeholder: Some("Select nodes".to_string()),
    }
}
```

### With Checkboxes

```rust
use adui_dioxus::{TreeSelect, TreeNode};

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "Node 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        tree_checkable: true,
        placeholder: Some("Select nodes".to_string()),
    }
}
```

### With Search

```rust
use adui_dioxus::{TreeSelect, TreeNode};

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "Node 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        show_search: true,
        placeholder: Some("Search and select".to_string()),
    }
}
```

## Use Cases

- **Hierarchical Selection**: Select items from hierarchical structures
- **Category Selection**: Select categories in tree structures
- **Organization Selection**: Select organizational units
- **Multi-level Selection**: Select from multi-level data

## Differences from Ant Design 6.0.0

- ✅ Single and multiple selection
- ✅ Tree structure
- ✅ Search functionality
- ✅ Checkbox mode
- ⚠️ Some advanced features may differ

