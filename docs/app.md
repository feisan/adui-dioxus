# App

## Overview

The App component is a top-level container that wires OverlayManager and exposes global feedback APIs (Message, Notification, Modal) through context. It should wrap your application root.

## API Reference

### AppProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Additional CSS class |
| `style` | `Option<String>` | `None` | Additional inline styles |
| `children` | `Element` | - | Application content (required) |

## Usage Examples

### Basic App

```rust
use adui_dioxus::App;

rsx! {
    App {
        div {
            "Your application content"
        }
    }
}
```

### Using Message API

```rust
use adui_dioxus::{App, use_message};

fn MyComponent() -> Element {
    let message = use_message();
    
    rsx! {
        Button {
            onclick: move |_| {
                if let Some(msg) = message {
                    msg.success("Operation successful");
                }
            },
            "Show Message"
        }
    }
}

rsx! {
    App {
        MyComponent {}
    }
}
```

### Using Notification API

```rust
use adui_dioxus::{App, use_notification};

fn MyComponent() -> Element {
    let notification = use_notification();
    
    rsx! {
        Button {
            onclick: move |_| {
                if let Some(notif) = notification {
                    notif.info("Notification", "This is a notification");
                }
            },
            "Show Notification"
        }
    }
}

rsx! {
    App {
        MyComponent {}
    }
}
```

### Using Modal API

```rust
use adui_dioxus::{App, use_modal};

fn MyComponent() -> Element {
    let modal = use_modal();
    
    rsx! {
        Button {
            onclick: move |_| {
                if let Some(m) = modal {
                    m.open();
                }
            },
            "Open Modal"
        }
    }
}

rsx! {
    App {
        MyComponent {}
    }
}
```

## Hooks

### use_app()

Returns the full App context value. Returns a default value when used outside of an App subtree.

### use_message()

Convenience hook to access the Message API. Returns `None` if used outside of App.

### use_notification()

Convenience hook to access the Notification API. Returns `None` if used outside of App.

### use_modal()

Convenience hook to access the Modal API. Returns `None` if used outside of App.

## Use Cases

- **Application Root**: Wrap your application root
- **Global Feedback**: Enable Message, Notification, and Modal APIs
- **Overlay Management**: Manage overlays globally
- **Context Provision**: Provide context for global APIs

## Differences from Ant Design 6.0.0

- ✅ Message API support
- ✅ Notification API support
- ✅ Modal API support
- ✅ Overlay management
- ⚠️ Simplified compared to Ant Design's full App component
- ⚠️ Some advanced features may differ

