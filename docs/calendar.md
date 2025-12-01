# Calendar

## Overview

The Calendar component displays a calendar view for date selection. It supports month and year modes, fullscreen display, and date selection.

## API Reference

### CalendarProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<CalendarDate>` | `None` | Controlled selected date |
| `default_value` | `Option<CalendarDate>` | `None` | Initial value in uncontrolled mode |
| `mode` | `Option<CalendarMode>` | `None` | Current panel mode |
| `fullscreen` | `Option<bool>` | `None` | Whether to take full width |
| `on_select` | `Option<EventHandler<CalendarDate>>` | `None` | Called when date is selected |
| `on_panel_change` | `Option<EventHandler<(CalendarDate, CalendarMode)>>` | `None` | Called when panel changes |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### CalendarMode

- `Month` - Month view (default)
- `Year` - Year view

### CalendarDate

Internal date value type. Can be created with `CalendarDate::from_ymd(year, month, day)`.

## Usage Examples

### Basic Calendar

```rust
use adui_dioxus::{Calendar, CalendarDate};
use dioxus::prelude::*;

let date = use_signal(|| None::<CalendarDate>);

rsx! {
    Calendar {
        value: *date.read(),
        on_select: Some(move |d| {
            date.set(Some(d));
        }),
    }
}
```

### Fullscreen Calendar

```rust
use adui_dioxus::Calendar;

rsx! {
    Calendar {
        fullscreen: Some(true),
    }
}
```

### With Panel Change Handler

```rust
use adui_dioxus::{Calendar, CalendarMode};

rsx! {
    Calendar {
        on_panel_change: Some(move |(date, mode)| {
            println!("Panel changed: {:?}, mode: {:?}", date, mode);
        }),
    }
}
```

## Use Cases

- **Date Selection**: Select dates from calendar
- **Scheduling**: View and select dates for scheduling
- **Date Display**: Display calendar view
- **Date Navigation**: Navigate through dates

## Differences from Ant Design 6.0.0

- ✅ Month and year modes
- ✅ Date selection
- ✅ Fullscreen mode
- ⚠️ Some advanced features may differ

