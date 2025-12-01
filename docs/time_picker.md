# TimePicker

## Overview

The TimePicker component provides a time selection interface with hour, minute, and second columns. It supports custom step values and time formatting.

## API Reference

### TimePickerProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<TimeValue>` | `None` | Controlled time value |
| `default_value` | `Option<TimeValue>` | `None` | Initial value in uncontrolled mode |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `format` | `Option<String>` | `None` | Display/parse format (default: HH:mm:ss) |
| `hour_step` | `Option<u8>` | `None` | Step for hour column |
| `minute_step` | `Option<u8>` | `None` | Step for minute column |
| `second_step` | `Option<u8>` | `None` | Step for second column |
| `disabled` | `Option<bool>` | `None` | Disable interactions |
| `allow_clear` | `Option<bool>` | `None` | Show clear icon |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<Option<TimeValue>>>` | `None` | Called when time changes |

### TimeValue

Internal time value type with hour, minute, and second. Can be created with `TimeValue::new(hour, minute, second)` and formatted with `to_hms_string()`.

## Usage Examples

### Basic TimePicker

```rust
use adui_dioxus::{TimePicker, TimeValue};
use dioxus::prelude::*;

let time = use_signal(|| None::<TimeValue>);

rsx! {
    TimePicker {
        value: *time.read(),
        on_change: Some(move |t| {
            time.set(t);
        }),
        placeholder: Some("Select time".to_string()),
    }
}
```

### With Custom Steps

```rust
use adui_dioxus::TimePicker;

rsx! {
    TimePicker {
        hour_step: Some(2),
        minute_step: Some(15),
        second_step: Some(10),
        placeholder: Some("Select time".to_string()),
    }
}
```

### With Default Value

```rust
use adui_dioxus::{TimePicker, TimeValue};

rsx! {
    TimePicker {
        default_value: Some(TimeValue::new(9, 0, 0)),
        placeholder: Some("Select time".to_string()),
    }
}
```

## Use Cases

- **Forms**: Time input in forms
- **Scheduling**: Time slot selection
- **Filters**: Time-based filtering
- **Reports**: Time-based reporting

## Differences from Ant Design 6.0.0

- ✅ Hour, minute, second selection
- ✅ Custom step values
- ✅ Time formatting
- ⚠️ Some advanced features may differ

