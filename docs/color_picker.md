# ColorPicker

## Overview

The ColorPicker component provides a color selection interface with HSVA (Hue, Saturation, Value, Alpha) color model support. It includes a color palette, saturation/value picker, hue slider, and alpha slider.

## API Reference

### ColorPickerProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled color string (e.g., #RRGGBB or #RRGGBBAA) |
| `default_value` | `Option<String>` | `None` | Initial value in uncontrolled mode |
| `disabled` | `bool` | `false` | Disable interactions |
| `allow_clear` | `bool` | `false` | Show clear button |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called on every change with hex string |
| `on_change_complete` | `Option<EventHandler<String>>` | `None` | Called when interaction completes |

## Usage Examples

### Basic ColorPicker

```rust
use adui_dioxus::ColorPicker;
use dioxus::prelude::*;

let color = use_signal(|| None::<String>);

rsx! {
    ColorPicker {
        value: color.read().clone(),
        on_change: Some(move |c| {
            color.set(Some(c));
        }),
    }
}
```

### With Clear Button

```rust
use adui_dioxus::ColorPicker;

rsx! {
    ColorPicker {
        allow_clear: true,
        default_value: Some("#FF0000".to_string()),
    }
}
```

### With Change Complete Handler

```rust
use adui_dioxus::ColorPicker;

rsx! {
    ColorPicker {
        on_change_complete: Some(move |color| {
            println!("Color selection completed: {}", color);
        }),
    }
}
```

## Use Cases

- **Theme Customization**: Customize theme colors
- **Design Tools**: Color selection in design tools
- **Form Input**: Color input in forms
- **Settings**: Color settings in applications

## Differences from Ant Design 6.0.0

- ✅ HSVA color model
- ✅ Saturation/value picker
- ✅ Hue slider
- ✅ Alpha slider
- ✅ Hex color format
- ⚠️ Some advanced features may differ

