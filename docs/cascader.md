# Cascader

## Overview

The Cascader component provides a cascading selection interface for hierarchical data. It displays options in multiple columns, allowing users to navigate through levels of data.

## API Reference

### CascaderProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `options` | `Vec<CascaderNode>` | - | Cascading option tree (required) |
| `value` | `Option<Vec<String>>` | `None` | Controlled path value for single-select |
| `multiple` | `bool` | `false` | Multiple selection (reserved, not yet implemented) |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `allow_clear` | `bool` | `false` | Show clear button |
| `disabled` | `bool` | `false` | Disable interactions |
| `status` | `Option<ControlStatus>` | `None` | Control status |
| `size` | `Option<ComponentSize>` | `None` | Component size |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `dropdown_class` | `Option<String>` | `None` | Extra class for dropdown |
| `dropdown_style` | `Option<String>` | `None` | Extra style for dropdown |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | Called when selection changes |

### CascaderNode

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the node |
| `label` | `String` | Display label |
| `disabled` | `bool` | Whether node is disabled |
| `children` | `Vec<CascaderNode>` | Child nodes |

## Usage Examples

### Basic Cascader

```rust
use adui_dioxus::{Cascader, CascaderNode};
use dioxus::prelude::*;

let value = use_signal(|| None::<Vec<String>>);

rsx! {
    Cascader {
        options: vec![
            CascaderNode {
                key: "zhejiang".to_string(),
                label: "Zhejiang".to_string(),
                disabled: false,
                children: vec![
                    CascaderNode {
                        key: "hangzhou".to_string(),
                        label: "Hangzhou".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                ],
            },
        ],
        value: value.read().clone(),
        on_change: Some(move |path| {
            value.set(Some(path));
        }),
        placeholder: Some("Select location".to_string()),
    }
}
```

### With Multiple Levels

```rust
use adui_dioxus::{Cascader, CascaderNode};

rsx! {
    Cascader {
        options: vec![
            CascaderNode {
                key: "china".to_string(),
                label: "China".to_string(),
                disabled: false,
                children: vec![
                    CascaderNode {
                        key: "zhejiang".to_string(),
                        label: "Zhejiang".to_string(),
                        disabled: false,
                        children: vec![
                            CascaderNode {
                                key: "hangzhou".to_string(),
                                label: "Hangzhou".to_string(),
                                disabled: false,
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
        ],
        placeholder: Some("Select location".to_string()),
    }
}
```

## Use Cases

- **Location Selection**: Select locations with hierarchical structure
- **Category Selection**: Select categories in nested structures
- **Organization Selection**: Select organizational units
- **Hierarchical Data**: Select from hierarchical data

## Differences from Ant Design 6.0.0

- ✅ Single selection
- ✅ Hierarchical navigation
- ✅ Multiple levels
- ⚠️ Multiple selection not yet implemented
- ⚠️ Some advanced features may differ

