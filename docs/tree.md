# Tree

## Overview

The Tree component displays hierarchical data in a tree structure. It supports expand/collapse, selection, checkable mode, and keyboard navigation.

## API Reference

### TreeProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `tree_data` | `Option<Vec<TreeNode>>` | `None` | Tree data source |
| `expanded_keys` | `Option<Vec<String>>` | `None` | Controlled expanded keys |
| `default_expanded_keys` | `Option<Vec<String>>` | `None` | Default expanded keys |
| `default_expand_all` | `bool` | `false` | Expand all nodes by default |
| `auto_expand_parent` | `bool` | `true` | Auto expand parent nodes |
| `on_expand` | `Option<EventHandler<Vec<String>>>` | `None` | Called when expand keys change |
| `selected_keys` | `Option<Vec<String>>` | `None` | Controlled selected keys |
| `default_selected_keys` | `Option<Vec<String>>` | `None` | Default selected keys |
| `selectable` | `bool` | `true` | Whether nodes are selectable |
| `multiple` | `bool` | `false` | Allow multiple selection |
| `on_select` | `Option<EventHandler<Vec<String>>>` | `None` | Called when selected keys change |
| `checkable` | `bool` | `false` | Enable checkbox mode |
| `checked_keys` | `Option<Vec<String>>` | `None` | Controlled checked keys |
| `default_checked_keys` | `Option<Vec<String>>` | `None` | Default checked keys |
| `on_check` | `Option<EventHandler<Vec<String>>>` | `None` | Called when checked keys change |
| `show_line` | `bool` | `false` | Show line connecting nodes |
| `block_node` | `bool` | `false` | Block node style |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### TreeNode

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the node |
| `label` | `String` | Node label text |
| `disabled` | `bool` | Whether node is disabled |
| `children` | `Vec<TreeNode>` | Child nodes |

## Usage Examples

### Basic Tree

```rust
use adui_dioxus::{Tree, TreeNode};

rsx! {
    Tree {
        tree_data: Some(vec![
            TreeNode {
                key: "parent".to_string(),
                label: "Parent".to_string(),
                disabled: false,
                children: vec![
                    TreeNode {
                        key: "child1".to_string(),
                        label: "Child 1".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                    TreeNode {
                        key: "child2".to_string(),
                        label: "Child 2".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                ],
            },
        ]),
    }
}
```

### Checkable Tree

```rust
use adui_dioxus::{Tree, TreeNode};
use dioxus::prelude::*;

let checked = use_signal(|| vec![]);

rsx! {
    Tree {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "Node 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        checkable: true,
        checked_keys: Some(checked.read().clone()),
        on_check: Some(move |keys| {
            checked.set(keys);
        }),
    }
}
```

### With Selection

```rust
use adui_dioxus::{Tree, TreeNode};
use dioxus::prelude::*;

let selected = use_signal(|| vec![]);

rsx! {
    Tree {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "Node 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        selected_keys: Some(selected.read().clone()),
        on_select: Some(move |keys| {
            selected.set(keys);
        }),
    }
}
```

### Show Line

```rust
use adui_dioxus::{Tree, TreeNode};

rsx! {
    Tree {
        show_line: true,
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "Node 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
    }
}
```

## Use Cases

- **File Browsers**: Display file system structures
- **Category Navigation**: Show category hierarchies
- **Organization Charts**: Display organizational structures
- **Menu Structures**: Show menu hierarchies

## Differences from Ant Design 6.0.0

- ✅ Expand/collapse functionality
- ✅ Single and multiple selection
- ✅ Checkable mode
- ✅ Keyboard navigation
- ✅ Show line mode
- ⚠️ Drag and drop not yet implemented
- ⚠️ Some advanced features may differ

