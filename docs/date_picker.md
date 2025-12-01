# DatePicker

## Overview

The DatePicker component provides a date selection interface with a dropdown calendar. It supports single date selection, date ranges, time selection, and custom date formatting.

## API Reference

### DatePickerProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<DateValue>` | `None` | Controlled date value |
| `default_value` | `Option<DateValue>` | `None` | Initial value in uncontrolled mode |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `format` | `Option<String>` | `None` | Display/parse format (default: YYYY-MM-DD) |
| `disabled` | `Option<bool>` | `None` | Disable interactions |
| `allow_clear` | `Option<bool>` | `None` | Show clear icon |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<Option<DateValue>>>` | `None` | Called when date changes |
| `show_time` | `Option<ShowTimeConfig>` | `None` | Show time picker |
| `ranges` | `Option<HashMap<String, (DateValue, DateValue)>>` | `None` | Preset date ranges |
| `disabled_date` | `Option<Rc<dyn Fn(DateValue) -> bool>>` | `None` | Disable specific dates |
| `disabled_time` | `Option<Rc<dyn Fn(DateValue) -> bool>>` | `None` | Disable specific times |
| `render_extra_footer` | `Option<Rc<dyn Fn() -> Element>>` | `None` | Custom footer render |
| `generate_config` | `Option<DateGenerateConfig>` | `None` | Custom date library config |

### DateValue

Internal date value type using `time::Date`. Can be created with `DateValue::from_ymd(year, month, day)` and formatted with `to_ymd_string()`.

### ShowTimeConfig

| Field | Type | Description |
|-------|------|-------------|
| `format` | `Option<String>` | Time format |
| `default_value` | `Option<String>` | Default time value |
| `hour_step` | `Option<u8>` | Hour step |
| `minute_step` | `Option<u8>` | Minute step |
| `second_step` | `Option<u8>` | Second step |

## Usage Examples

### Basic DatePicker

```rust
use adui_dioxus::{DatePicker, DateValue};
use dioxus::prelude::*;

let date = use_signal(|| None::<DateValue>);

rsx! {
    DatePicker {
        value: *date.read(),
        on_change: Some(move |d| {
            date.set(d);
        }),
        placeholder: Some("Select date".to_string()),
    }
}
```

### With Time

```rust
use adui_dioxus::{DatePicker, ShowTimeConfig};

rsx! {
    DatePicker {
        show_time: Some(ShowTimeConfig {
            format: Some("HH:mm:ss".to_string()),
            ..Default::default()
        }),
        placeholder: Some("Select date and time".to_string()),
    }
}
```

### With Preset Ranges

```rust
use adui_dioxus::{DatePicker, DateValue};
use std::collections::HashMap;

let ranges = {
    let mut map = HashMap::new();
    if let (Some(today), Some(yesterday)) = (
        DateValue::from_ymd(2024, 1, 15),
        DateValue::from_ymd(2024, 1, 14),
    ) {
        map.insert("Yesterday".to_string(), (yesterday, yesterday));
        map.insert("Today".to_string(), (today, today));
    }
    map
};

rsx! {
    DatePicker {
        ranges: Some(ranges),
        placeholder: Some("Select date".to_string()),
    }
}
```

### With Disabled Dates

```rust
use adui_dioxus::{DatePicker, DateValue};
use std::rc::Rc;

rsx! {
    DatePicker {
        disabled_date: Some(Rc::new(|date| {
            // Disable weekends (example)
            let weekday = date.inner.weekday();
            weekday == time::Weekday::Saturday || weekday == time::Weekday::Sunday
        })),
        placeholder: Some("Select date".to_string()),
    }
}
```

## Use Cases

- **Forms**: Date input in forms
- **Filters**: Date range filtering
- **Scheduling**: Appointment scheduling
- **Reports**: Date-based reporting

## Differences from Ant Design 6.0.0

- ✅ Single date selection
- ✅ Time selection support
- ✅ Preset ranges
- ✅ Disabled dates
- ⚠️ Range picker may differ
- ⚠️ Some advanced features may differ

