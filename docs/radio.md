# Radio

## Overview

The Radio component allows users to select a single option from a set. It supports single radio buttons and radio groups, with button-style variants.

## API Reference

### RadioProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | Value for this radio within a group (required) |
| `checked` | `Option<bool>` | `None` | Controlled checked state when not in a group |
| `default_checked` | `bool` | `false` | Initial checked state |
| `disabled` | `bool` | `false` | Whether the radio is disabled |
| `status` | `Option<ControlStatus>` | `None` | Optional status (success/warning/error) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when radio becomes checked |
| `children` | `Element` | - | Rendered label content (required) |
| `button` | `bool` | `false` | Render as button-style radio |

### RadioGroupProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled selected value |
| `default_value` | `Option<String>` | `None` | Initial selected value in uncontrolled mode |
| `disabled` | `bool` | `false` | Whether all radios in group are disabled |
| `name` | `Option<String>` | `None` | Name attribute for form submission |
| `options` | `Option<Vec<(String, String)>>` | `None` | Options as (value, label) pairs |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<String>>` | `None` | Change event handler |
| `children` | `Element` | - | Radio children (required) |

## Usage Examples

### Basic Radio

```rust
use adui_dioxus::Radio;
use dioxus::prelude::*;

let selected = use_signal(|| "option1".to_string());

rsx! {
    Radio {
        value: "option1".to_string(),
        checked: Some(*selected.read() == "option1"),
        on_change: Some(move |_| {
            selected.set("option1".to_string());
        }),
        "Option 1"
    }
}
```

### Radio Group

```rust
use adui_dioxus::{Radio, RadioGroup};
use dioxus::prelude::*;

let selected = use_signal(|| Some("apple".to_string()));

rsx! {
    RadioGroup {
        value: *selected.read(),
        on_change: Some(move |value| {
            selected.set(Some(value));
        }),
        Radio { value: "apple".to_string(), "Apple" }
        Radio { value: "banana".to_string(), "Banana" }
        Radio { value: "orange".to_string(), "Orange" }
    }
}
```

### Button Style Radio

```rust
use adui_dioxus::{Radio, RadioGroup};

rsx! {
    RadioGroup {
        Radio { value: "left".to_string(), button: true, "Left" }
        Radio { value: "center".to_string(), button: true, "Center" }
        Radio { value: "right".to_string(), button: true, "Right" }
    }
}
```

## Use Cases

- **Form Inputs**: Single choice form fields
- **Settings**: Select one option from multiple
- **Filters**: Single-select filters
- **Options**: Choose one option from a set

## Differences from Ant Design 6.0.0

- ✅ Single and group radios
- ✅ Button-style radios
- ✅ Controlled and uncontrolled modes
- ✅ Form integration
- ⚠️ Some advanced styling options may differ

