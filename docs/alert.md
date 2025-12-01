# Alert

## Overview

The Alert component displays important messages to users, such as warnings, errors, success messages, or informational notices. It supports multiple types with distinct visual styles and can be closable.

## API Reference

### AlertProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `type` | `AlertType` | `AlertType::Info` | Alert type: `Success`, `Info`, `Warning`, `Error` |
| `message` | `Element` | - | Main message content (required) |
| `description` | `Option<Element>` | `None` | Optional detailed description |
| `show_icon` | `bool` | `true` | Whether to show the semantic icon |
| `closable` | `bool` | `false` | Whether the alert can be closed |
| `on_close` | `Option<EventHandler<()>>` | `None` | Called when the close button is clicked |
| `icon` | `Option<Element>` | `None` | Optional custom icon element |
| `banner` | `bool` | `false` | Whether the alert should be rendered as a banner (full width, compact) |
| `class` | `Option<String>` | `None` | Extra class on the root element |
| `style` | `Option<String>` | `None` | Inline style on the root element |

### AlertType

- `Success` - Success message (green)
- `Info` - Informational message (blue)
- `Warning` - Warning message (orange)
- `Error` - Error message (red)

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Success,
        message: rsx!("Operation completed successfully!"),
    }
}
```

### With Description

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Warning,
        message: rsx!("Warning"),
        description: Some(rsx!("This action cannot be undone.")),
    }
}
```

### Closable Alert

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Info,
        message: rsx!("This alert can be closed"),
        closable: true,
        on_close: Some(move |_| {
            println!("Alert closed");
        }),
    }
}
```

### Banner Alert

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Error,
        message: rsx!("System Error"),
        banner: true,
    }
}
```

## Use Cases

- **Form Validation**: Display validation errors or warnings
- **System Notifications**: Show system-wide messages
- **Success Feedback**: Confirm successful operations
- **Error Messages**: Display error information
- **Banner Alerts**: Top-of-page notifications

## Differences from Ant Design 6.0.0

- ✅ All basic alert types supported
- ✅ Closable functionality
- ✅ Banner mode
- ✅ Custom icons
- ⚠️ Action buttons not yet implemented
- ⚠️ Some advanced styling options may differ

