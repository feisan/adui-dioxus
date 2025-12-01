# Input

## Overview

The Input component provides text input functionality with support for prefixes, suffixes, clear buttons, character counts, and various visual variants. It includes specialized variants like Password, Search, and OTP.

## API Reference

### InputProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled value |
| `default_value` | `Option<String>` | `None` | Initial value in uncontrolled mode |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `disabled` | `bool` | `false` | Disable interactions |
| `size` | `Option<InputSize>` | `None` | Component size |
| `variant` | `Option<Variant>` | `None` | Visual variant (outlined/filled/borderless) |
| `bordered` | `Option<bool>` | `None` | @deprecated Use `variant` instead |
| `status` | `Option<ControlStatus>` | `None` | Control status (success/warning/error) |
| `prefix` | `Option<Element>` | `None` | Leading element |
| `suffix` | `Option<Element>` | `None` | Trailing element |
| `addon_before` | `Option<Element>` | `None` | @deprecated Use `Space.Compact` instead |
| `addon_after` | `Option<Element>` | `None` | @deprecated Use `Space.Compact` instead |
| `allow_clear` | `bool` | `false` | Show clear icon when there is content |
| `max_length` | `Option<usize>` | `None` | Maximum length |
| `show_count` | `bool` | `false` | Show character count |
| `class` | `Option<String>` | `None` | Extra class name |
| `root_class_name` | `Option<String>` | `None` | Extra class for root element |
| `style` | `Option<String>` | `None` | Inline style |
| `class_names` | `Option<InputClassNames>` | `None` | Semantic class names |
| `styles` | `Option<InputStyles>` | `None` | Semantic styles |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when value changes |
| `on_press_enter` | `Option<EventHandler<()>>` | `None` | Called when Enter is pressed |
| `data_attributes` | `Option<Vec<(String, String)>>` | `None` | Data attributes |

### PasswordProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled value |
| `default_value` | `Option<String>` | `None` | Initial value |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `disabled` | `bool` | `false` | Disable interactions |
| `size` | `Option<InputSize>` | `None` | Component size |
| `variant` | `Option<Variant>` | `None` | Visual variant |
| `status` | `Option<ControlStatus>` | `None` | Control status |
| `prefix` | `Option<Element>` | `None` | Leading element |
| `visible` | `bool` | `false` | Whether password is visible by default |
| `icon_render` | `Option<Element>` | `None` | Custom visibility toggle icon |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when value changes |
| `on_visible_change` | `Option<EventHandler<bool>>` | `None` | Called when visibility changes |

### SearchProps

Similar to InputProps with additional `on_search` callback.

### OTPProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `length` | `usize` | `6` | Number of OTP digits |
| `value` | `Option<String>` | `None` | Controlled value |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when value changes |
| `on_finish` | `Option<EventHandler<String>>` | `None` | Called when all digits are filled |

### InputSize

- `Small` - Small size
- `Middle` - Middle size (default)
- `Large` - Large size

## Usage Examples

### Basic Input

```rust
use adui_dioxus::Input;
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    Input {
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("Enter text".to_string()),
    }
}
```

### With Prefix and Suffix

```rust
use adui_dioxus::{Input, Icon, IconKind};

rsx! {
    Input {
        prefix: Some(rsx!(Icon { kind: IconKind::User })),
        suffix: Some(rsx!("@example.com")),
        placeholder: Some("Username".to_string()),
    }
}
```

### Password Input

```rust
use adui_dioxus::Password;
use dioxus::prelude::*;

let password = use_signal(|| String::new());

rsx! {
    Password {
        value: Some(password.read().clone()),
        on_change: Some(move |v| {
            password.set(v);
        }),
        placeholder: Some("Enter password".to_string()),
    }
}
```

### Search Input

```rust
use adui_dioxus::Search;

rsx! {
    Search {
        placeholder: Some("Search...".to_string()),
        on_search: Some(move |value| {
            println!("Searching for: {}", value);
        }),
    }
}
```

### With Character Count

```rust
use adui_dioxus::Input;

rsx! {
    Input {
        max_length: Some(100),
        show_count: true,
        placeholder: Some("Enter text".to_string()),
    }
}
```

### OTP Input

```rust
use adui_dioxus::OTP;
use dioxus::prelude::*;

let otp = use_signal(|| String::new());

rsx! {
    OTP {
        length: 6,
        value: Some(otp.read().clone()),
        on_change: Some(move |v| {
            otp.set(v);
        }),
        on_finish: Some(move |v| {
            println!("OTP completed: {}", v);
        }),
    }
}
```

## Use Cases

- **Forms**: Text input in forms
- **Search**: Search input fields
- **Authentication**: Password and OTP inputs
- **Data Entry**: General data entry fields

## Differences from Ant Design 6.0.0

- ✅ Basic input functionality
- ✅ Password, Search, and OTP variants
- ✅ Prefix and suffix support
- ✅ Character count
- ✅ Clear button
- ⚠️ Some advanced features may differ

