# Slider

## Overview

The Slider component allows users to select a value or range of values from a continuous or discrete set. It supports single and range modes, vertical orientation, and custom marks.

## API Reference

### SliderProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<SliderValue>` | `None` | Controlled value (use `Range(_, _)` when `range = true`) |
| `default_value` | `Option<SliderValue>` | `None` | Default value in uncontrolled mode |
| `range` | `bool` | `false` | Whether to render two handles |
| `min` | `f64` | `0.0` | Minimum value |
| `max` | `f64` | `100.0` | Maximum value |
| `step` | `Option<f64>` | `None` | Step granularity (when None, slider is continuous) |
| `precision` | `Option<u32>` | `None` | Decimal precision used for snapping |
| `reverse` | `bool` | `false` | Reverse the visual direction |
| `vertical` | `bool` | `false` | Vertical orientation |
| `disabled` | `bool` | `false` | Disable interactions |
| `dots` | `bool` | `false` | Render tick dots for marks |
| `marks` | `Option<Vec<SliderMark>>` | `None` | Optional labeled marks along the track |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<SliderValue>>` | `None` | Fired on every value change |
| `on_change_complete` | `Option<EventHandler<SliderValue>>` | `None` | Fired when user finishes interaction |

### SliderValue

- `Single(f64)` - Single value
- `Range(f64, f64)` - Range value (start, end)

### SliderMark

| Field | Type | Description |
|-------|------|-------------|
| `value` | `f64` | Mark value |
| `label` | `String` | Mark label |

## Usage Examples

### Basic Slider

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### Range Slider

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Range(20.0, 80.0));

rsx! {
    Slider {
        range: true,
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### With Marks

```rust
use adui_dioxus::{Slider, SliderValue, SliderMark};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        marks: Some(vec![
            SliderMark { value: 0.0, label: "0".to_string() },
            SliderMark { value: 50.0, label: "50".to_string() },
            SliderMark { value: 100.0, label: "100".to_string() },
        ]),
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### Vertical Slider

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        vertical: true,
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### With Step

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        step: Some(10.0),
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

## Use Cases

- **Volume Control**: Control volume levels
- **Price Range**: Select price ranges
- **Time Selection**: Select time ranges
- **Rating**: Select ratings or scores

## Differences from Ant Design 6.0.0

- ✅ Single and range modes
- ✅ Vertical orientation
- ✅ Custom marks
- ✅ Step control
- ✅ Keyboard navigation
- ⚠️ Some advanced features may differ

