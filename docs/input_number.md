# InputNumber

## Overview

The InputNumber component is a numeric input with step controls and basic formatting. It supports min/max constraints, step increments, and precision control.

## API Reference

### InputNumberProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<f64>` | `None` | Controlled value |
| `default_value` | `Option<f64>` | `None` | Uncontrolled initial value |
| `min` | `Option<f64>` | `None` | Minimum value |
| `max` | `Option<f64>` | `None` | Maximum value |
| `step` | `Option<f64>` | `None` | Step increment |
| `precision` | `Option<u32>` | `None` | Decimal precision |
| `controls` | `bool` | `true` | Whether to show step controls |
| `disabled` | `bool` | `false` | Disable interactions |
| `status` | `Option<ControlStatus>` | `None` | Control status (error, warning) |
| `prefix` | `Option<Element>` | `None` | Prefix element |
| `suffix` | `Option<Element>` | `None` | Suffix element |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<Option<f64>>>` | `None` | Fired whenever numeric value changes |
| `on_change_complete` | `Option<EventHandler<Option<f64>>>` | `None` | Fired on blur or Enter |

## Usage Examples

### Basic InputNumber

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.0));

rsx! {
    InputNumber {
        value: *value.read(),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### With Min/Max

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(50.0));

rsx! {
    InputNumber {
        value: *value.read(),
        min: Some(0.0),
        max: Some(100.0),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### With Step

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.0));

rsx! {
    InputNumber {
        value: *value.read(),
        step: Some(5.0),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### With Precision

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.5));

rsx! {
    InputNumber {
        value: *value.read(),
        precision: Some(2),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### With Prefix/Suffix

```rust
use adui_dioxus::{InputNumber, Icon, IconKind};
use dioxus::prelude::*;

let value = use_signal(|| Some(100.0));

rsx! {
    InputNumber {
        value: *value.read(),
        prefix: Some(rsx!(Icon { kind: IconKind::Dollar })),
        suffix: Some(rsx!("USD")),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### Without Controls

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.0));

rsx! {
    InputNumber {
        value: *value.read(),
        controls: false,
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

## Use Cases

- **Quantity Input**: Input quantities in forms
- **Price Input**: Input prices with currency
- **Rating Input**: Input ratings or scores
- **Numeric Forms**: Any form requiring numeric input

## Differences from Ant Design 6.0.0

- ✅ Min/max constraints
- ✅ Step increments
- ✅ Precision control
- ✅ Prefix/suffix support
- ✅ Keyboard navigation
- ⚠️ Some advanced features may differ

