# Checkbox

## Overview

The Checkbox component allows users to select one or more options from a set. It supports single checkboxes and checkbox groups, with indeterminate state support.

## API Reference

### CheckboxProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `Option<bool>` | `None` | Controlled checked state |
| `default_checked` | `bool` | `false` | Initial state in uncontrolled mode |
| `indeterminate` | `bool` | `false` | Visual indeterminate state (for "select all" scenarios) |
| `disabled` | `bool` | `false` | Whether the checkbox is disabled |
| `value` | `Option<String>` | `None` | Value used when inside a CheckboxGroup |
| `status` | `Option<ControlStatus>` | `None` | Optional status (success/warning/error) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<bool>>` | `None` | Change event handler |
| `children` | `Element` | - | Optional label content rendered to the right (required) |

### CheckboxGroupProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<Vec<String>>` | `None` | Controlled selected values |
| `default_value` | `Option<Vec<String>>` | `None` | Initial selected values in uncontrolled mode |
| `disabled` | `bool` | `false` | Whether all checkboxes in group are disabled |
| `options` | `Option<Vec<(String, String)>>` | `None` | Options as (value, label) pairs |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | Change event handler |
| `children` | `Element` | - | Checkbox children (required) |

## Usage Examples

### Basic Checkbox

```rust
use adui_dioxus::Checkbox;
use dioxus::prelude::*;

let checked = use_signal(|| false);

rsx! {
    Checkbox {
        checked: Some(*checked.read()),
        on_change: Some(move |is_checked| {
            checked.set(is_checked);
        }),
        "I agree to the terms"
    }
}
```

### Indeterminate Checkbox

```rust
use adui_dioxus::Checkbox;

rsx! {
    Checkbox {
        indeterminate: true,
        "Select All"
    }
}
```

### Checkbox Group

```rust
use adui_dioxus::{Checkbox, CheckboxGroup};
use dioxus::prelude::*;

let selected = use_signal(|| vec!["apple".to_string()]);

rsx! {
    CheckboxGroup {
        value: Some(selected.read().clone()),
        on_change: Some(move |values| {
            selected.set(values);
        }),
        Checkbox { value: Some("apple".to_string()), "Apple" }
        Checkbox { value: Some("banana".to_string()), "Banana" }
        Checkbox { value: Some("orange".to_string()), "Orange" }
    }
}
```

## Use Cases

- **Form Inputs**: Multiple choice form fields
- **Settings**: Enable/disable multiple options
- **Filters**: Multi-select filters
- **Agreements**: Terms and conditions checkboxes

## Differences from Ant Design 6.0.0

- ✅ Single and group checkboxes
- ✅ Indeterminate state
- ✅ Controlled and uncontrolled modes
- ✅ Form integration
- ⚠️ Some advanced styling options may differ

