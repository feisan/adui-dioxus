# Notification

## Overview

The Notification component displays notification messages in a corner of the page. It's similar to Message but supports titles, descriptions, and different placements.

## API Reference

### NotificationApi

The Notification API is accessed via the `use_notification` hook:

```rust
let notification = use_notification();
```

#### Methods

- `open(config: NotificationConfig) -> OverlayKey` - Open a notification with full config
- `info(title: impl Into<String>, description: Option<String>) -> OverlayKey` - Show info notification
- `success(title: impl Into<String>, description: Option<String>) -> OverlayKey` - Show success notification
- `error(title: impl Into<String>, description: Option<String>) -> OverlayKey` - Show error notification
- `warning(title: impl Into<String>, description: Option<String>) -> OverlayKey` - Show warning notification

### NotificationConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String` | - | Notification title (required) |
| `description` | `Option<String>` | `None` | Optional description text |
| `type` | `NotificationType` | `NotificationType::Info` | Notification type |
| `placement` | `NotificationPlacement` | `NotificationPlacement::TopRight` | Placement on screen |
| `duration` | `f32` | `4.5` | Auto close delay in seconds (0 for no auto-dismiss) |
| `icon` | `Option<Element>` | `None` | Custom icon element |
| `class` | `Option<String>` | `None` | Additional CSS class |
| `style` | `Option<String>` | `None` | Inline styles |
| `on_click` | `Option<EventHandler<()>>` | `None` | Callback when notification is clicked |
| `key` | `Option<String>` | `None` | Unique key for programmatic updates |

### NotificationType

- `Info` - Information notification
- `Success` - Success notification
- `Warning` - Warning notification
- `Error` - Error notification

### NotificationPlacement

- `TopRight` - Top right corner (default)
- `TopLeft` - Top left corner
- `BottomRight` - Bottom right corner
- `BottomLeft` - Bottom left corner

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::use_notification;

let notification = use_notification();

rsx! {
    Button {
        onclick: move |_| {
            notification.info(
                "Notification Title".to_string(),
                Some("This is the description".to_string())
            );
        },
        "Show Notification"
    }
}
```

### Success Notification

```rust
use adui_dioxus::use_notification;

let notification = use_notification();

rsx! {
    Button {
        onclick: move |_| {
            notification.success(
                "Success".to_string(),
                Some("Operation completed successfully".to_string())
            );
        },
        "Success"
    }
}
```

### Custom Placement

```rust
use adui_dioxus::{use_notification, NotificationConfig, NotificationType, NotificationPlacement};

let notification = use_notification();

rsx! {
    Button {
        onclick: move |_| {
            notification.open(NotificationConfig {
                title: "Custom Placement".to_string(),
                description: Some("Bottom left".to_string()),
                r#type: NotificationType::Info,
                placement: NotificationPlacement::BottomLeft,
                ..Default::default()
            });
        },
        "Custom Placement"
    }
}
```

## Use Cases

- **System Notifications**: Show system-wide notifications
- **User Feedback**: Provide feedback for user actions
- **Alerts**: Display important alerts
- **Updates**: Notify users of updates or changes

## Differences from Ant Design 6.0.0

- ✅ All notification types supported
- ✅ Title and description
- ✅ Custom placement
- ✅ Custom duration
- ⚠️ Close button not yet implemented
- ⚠️ Some advanced styling options may differ

