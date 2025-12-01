# Message

## Overview

The Message component displays global feedback messages at the top of the page. It's typically used via the `use_message` hook to show success, error, warning, or info messages.

## API Reference

### MessageApi

The Message API is accessed via the `use_message` hook:

```rust
let message = use_message();
```

#### Methods

- `open(config: MessageConfig) -> OverlayKey` - Open a message with full config
- `info(content: impl Into<String>) -> OverlayKey` - Show info message
- `success(content: impl Into<String>) -> OverlayKey` - Show success message
- `error(content: impl Into<String>) -> OverlayKey` - Show error message
- `warning(content: impl Into<String>) -> OverlayKey` - Show warning message
- `loading(content: impl Into<String>) -> OverlayKey` - Show loading message

### MessageConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | - | Message content (required) |
| `type` | `MessageType` | `MessageType::Info` | Message type |
| `duration` | `f32` | `3.0` | Auto close delay in seconds (0 for no auto-dismiss) |
| `icon` | `Option<Element>` | `None` | Custom icon element |
| `class` | `Option<String>` | `None` | Additional CSS class |
| `style` | `Option<String>` | `None` | Inline styles |
| `key` | `Option<String>` | `None` | Unique key for programmatic updates |
| `on_click` | `Option<EventHandler<()>>` | `None` | Callback when message is clicked |

### MessageType

- `Info` - Information message
- `Success` - Success message
- `Warning` - Warning message
- `Error` - Error message
- `Loading` - Loading message

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.info("This is an info message");
        },
        "Show Info"
    }
}
```

### Success Message

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.success("Operation completed successfully!");
        },
        "Success"
    }
}
```

### Error Message

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.error("An error occurred");
        },
        "Error"
    }
}
```

### Custom Duration

```rust
use adui_dioxus::{use_message, MessageConfig, MessageType};

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.open(MessageConfig {
                content: "This message stays for 5 seconds".to_string(),
                r#type: MessageType::Info,
                duration: 5.0,
                ..Default::default()
            });
        },
        "Custom Duration"
    }
}
```

### Loading Message

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.loading("Processing...");
        },
        "Loading"
    }
}
```

## Use Cases

- **Form Feedback**: Show validation errors or success messages
- **Operation Feedback**: Confirm successful operations
- **Error Notifications**: Display error messages
- **Status Updates**: Show status information

## Differences from Ant Design 6.0.0

- ✅ All message types supported
- ✅ Custom duration
- ✅ Custom icons
- ✅ Click callbacks
- ⚠️ Message stacking/positioning may differ
- ⚠️ Some advanced styling options may differ

