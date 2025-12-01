# Segmented

## Overview

The Segmented component provides a segmented control for single selection. It displays options as segments that can be clicked to select.

## API Reference

### SegmentedProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `options` | `Vec<SegmentedOption>` | - | Segmented options (required) |
| `value` | `Option<String>` | `None` | Controlled value |
| `default_value` | `Option<String>` | `None` | Initial value in uncontrolled mode |
| `block` | `bool` | `false` | Fill parent's width |
| `round` | `bool` | `false` | Rounded shape |
| `disabled` | `bool` | `false` | Disable interactions |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when value changes |

### SegmentedOption

| Field | Type | Description |
|-------|------|-------------|
| `label` | `String` | Display label |
| `value` | `String` | Option value |
| `icon` | `Option<Element>` | Optional icon |
| `tooltip` | `Option<String>` | Optional tooltip text |
| `disabled` | `bool` | Whether option is disabled |

## Usage Examples

### Basic Segmented

```rust
use adui_dioxus::{Segmented, SegmentedOption};
use dioxus::prelude::*;

let value = use_signal(|| Some("1".to_string()));

rsx! {
    Segmented {
        options: vec![
            SegmentedOption::new("Option 1", "1"),
            SegmentedOption::new("Option 2", "2"),
            SegmentedOption::new("Option 3", "3"),
        ],
        value: *value.read(),
        on_change: Some(move |v| {
            value.set(Some(v));
        }),
    }
}
```

### With Icons

```rust
use adui_dioxus::{Segmented, SegmentedOption, Icon, IconKind};

rsx! {
    Segmented {
        options: vec![
            SegmentedOption {
                label: "List".to_string(),
                value: "list".to_string(),
                icon: Some(rsx!(Icon { kind: IconKind::List })),
                tooltip: None,
                disabled: false,
            },
            SegmentedOption {
                label: "Grid".to_string(),
                value: "grid".to_string(),
                icon: Some(rsx!(Icon { kind: IconKind::Grid })),
                tooltip: None,
                disabled: false,
            },
        ],
    }
}
```

### Block Mode

```rust
use adui_dioxus::{Segmented, SegmentedOption};

rsx! {
    Segmented {
        block: true,
        options: vec![
            SegmentedOption::new("Option 1", "1"),
            SegmentedOption::new("Option 2", "2"),
        ],
    }
}
```

### Rounded

```rust
use adui_dioxus::{Segmented, SegmentedOption};

rsx! {
    Segmented {
        round: true,
        options: vec![
            SegmentedOption::new("Option 1", "1"),
            SegmentedOption::new("Option 2", "2"),
        ],
    }
}
```

## Use Cases

- **View Switching**: Switch between different views
- **Filter Selection**: Select filters
- **Mode Selection**: Select modes or states
- **Tab-like Navigation**: Tab-like navigation

## Differences from Ant Design 6.0.0

- ✅ Single selection
- ✅ Icons and tooltips
- ✅ Block and round modes
- ✅ Keyboard navigation
- ⚠️ Some advanced features may differ

