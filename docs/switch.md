# Switch

## Overview

The Switch component is a toggle switch for binary choices. It supports controlled and uncontrolled modes, custom children for checked/unchecked states, and form integration.

## API Reference

### SwitchProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `Option<bool>` | `None` | Controlled checked state |
| `default_checked` | `bool` | `false` | Initial value in uncontrolled mode |
| `disabled` | `bool` | `false` | Whether the switch is disabled |
| `size` | `SwitchSize` | `SwitchSize::Default` | Visual size of the switch |
| `checked_children` | `Option<Element>` | `None` | Content shown when checked |
| `un_checked_children` | `Option<Element>` | `None` | Content shown when unchecked |
| `status` | `Option<ControlStatus>` | `None` | Optional status (success/warning/error) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<bool>>` | `None` | Change event with next checked state |

### SwitchSize

- `Default` - Default size
- `Small` - Small size

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Switch;
use dioxus::prelude::*;

let checked = use_signal(|| false);

rsx! {
    Switch {
        checked: Some(*checked.read()),
        on_change: Some(move |is_checked| {
            checked.set(is_checked);
        }),
    }
}
```

### With Children

```rust
use adui_dioxus::Switch;

rsx! {
    Switch {
        checked_children: Some(rsx!("ON")),
        un_checked_children: Some(rsx!("OFF")),
    }
}
```

### Small Size

```rust
use adui_dioxus::{Switch, SwitchSize};

rsx! {
    Switch {
        size: SwitchSize::Small,
    }
}
```

### Uncontrolled Mode

```rust
use adui_dioxus::Switch;

rsx! {
    Switch {
        default_checked: true,
    }
}
```

## Use Cases

- **Settings**: Toggle settings on/off
- **Feature Flags**: Enable/disable features
- **Form Controls**: Binary form inputs
- **UI Preferences**: Toggle UI preferences

## Differences from Ant Design 6.0.0

- ✅ Controlled and uncontrolled modes
- ✅ Custom children for checked/unchecked states
- ✅ Size variants
- ✅ Form integration
- ✅ Status variants
- ⚠️ Loading state not yet implemented
- ⚠️ Some advanced styling options may differ

