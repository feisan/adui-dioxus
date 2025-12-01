# Tour

## Overview

The Tour component provides a guided user onboarding experience. It displays a series of steps, highlighting UI elements and providing descriptions to help users learn how to use the application.

## API Reference

### TourProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `bool` | - | Whether the tour is visible (required) |
| `steps` | `Vec<TourStep>` | - | Tour steps to display (required) |
| `current` | `Option<usize>` | `None` | Controlled current step index |
| `on_close` | `Option<EventHandler<()>>` | `None` | Called when tour is closed |
| `on_change` | `Option<EventHandler<usize>>` | `None` | Called when current step changes |
| `on_finish` | `Option<EventHandler<()>>` | `None` | Called when user completes the tour |
| `r#type` | `TourType` | `TourType::Default` | Visual type of the tour |
| `mask_closable` | `bool` | `true` | Whether clicking mask closes the tour |
| `closable` | `bool` | `true` | Whether to show close button |
| `show_indicators` | `bool` | `true` | Whether to show step indicators |
| `next_button_text` | `Option<String>` | `None` | Text for "Next" button |
| `prev_button_text` | `Option<String>` | `None` | Text for "Previous" button |
| `finish_button_text` | `Option<String>` | `None` | Text for "Finish" button |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### TourStep

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the step |
| `title` | `Option<String>` | Title displayed in the tour panel |
| `description` | `Option<Element>` | Description text or element |
| `cover` | `Option<Element>` | Cover image or element |
| `placement` | `Option<TooltipPlacement>` | Placement of the tour panel |
| `target` | `Option<String>` | CSS selector for target element |
| `next_button_text` | `Option<String>` | Custom "Next" button text |
| `prev_button_text` | `Option<String>` | Custom "Previous" button text |

### TourType

- `Default` - Default style with light background
- `Primary` - Primary style with colored background

## Usage Examples

### Basic Tour

```rust
use adui_dioxus::{Tour, TourStep};
use dioxus::prelude::*;

let open = use_signal(|| true);

rsx! {
    Tour {
        open: *open.read(),
        steps: vec![
            TourStep::new("step1", "Welcome", "This is the first step"),
            TourStep::new("step2", "Feature", "Explore this feature"),
        ],
        on_close: Some(move |_| {
            open.set(false);
        }),
        on_finish: Some(move |_| {
            open.set(false);
        }),
    }
}
```

### With Target Elements

```rust
use adui_dioxus::{Tour, TourStep, TooltipPlacement};

rsx! {
    Tour {
        open: true,
        steps: vec![
            TourStep::new("step1", "Button", "Click this button")
                .target("#my-button")
                .placement(TooltipPlacement::Bottom),
        ],
        on_close: Some(move |_| {}),
    }
}
```

### Primary Style

```rust
use adui_dioxus::{Tour, TourStep, TourType};

rsx! {
    Tour {
        open: true,
        r#type: TourType::Primary,
        steps: vec![
            TourStep::new("step1", "Welcome", "Welcome to the app"),
        ],
        on_close: Some(move |_| {}),
    }
}
```

## Use Cases

- **User Onboarding**: Guide new users through the application
- **Feature Introduction**: Introduce new features
- **Tutorials**: Create step-by-step tutorials
- **Help System**: Provide contextual help

## Differences from Ant Design 6.0.0

- ✅ Step-by-step guidance
- ✅ Element highlighting
- ✅ Custom placement
- ✅ Keyboard navigation
- ⚠️ Some advanced features may differ

