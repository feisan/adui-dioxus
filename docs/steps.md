# Steps

## Overview

The Steps component displays a step indicator for processes. It shows the current step and progress through a multi-step workflow.

## API Reference

### StepsProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<StepItem>` | - | Step items (required) |
| `current` | `Option<usize>` | `None` | Controlled current step index |
| `default_current` | `Option<usize>` | `None` | Default current step in uncontrolled mode |
| `on_change` | `Option<EventHandler<usize>>` | `None` | Called when step changes |
| `direction` | `Option<StepsDirection>` | `None` (defaults to Horizontal) | Step direction |
| `size` | `Option<ComponentSize>` | `None` | Step size |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### StepItem

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the step |
| `title` | `Element` | Step title |
| `description` | `Option<Element>` | Optional step description |
| `status` | `Option<StepStatus>` | Optional explicit status |
| `disabled` | `bool` | Whether step is disabled |

### StepStatus

- `Wait` - Waiting status
- `Process` - In progress status
- `Finish` - Finished status
- `Error` - Error status

### StepsDirection

- `Horizontal` - Horizontal steps (default)
- `Vertical` - Vertical steps

## Usage Examples

### Basic Steps

```rust
use adui_dioxus::{Steps, StepItem};

rsx! {
    Steps {
        items: vec![
            StepItem::new("1", rsx!("Step 1")),
            StepItem::new("2", rsx!("Step 2")),
            StepItem::new("3", rsx!("Step 3")),
        ],
        default_current: Some(1),
    }
}
```

### With Descriptions

```rust
use adui_dioxus::{Steps, StepItem};

rsx! {
    Steps {
        items: vec![
            StepItem {
                key: "1".to_string(),
                title: rsx!("Step 1"),
                description: Some(rsx!("Description 1")),
                status: None,
                disabled: false,
            },
            StepItem {
                key: "2".to_string(),
                title: rsx!("Step 2"),
                description: Some(rsx!("Description 2")),
                status: None,
                disabled: false,
            },
        ],
    }
}
```

### Vertical Steps

```rust
use adui_dioxus::{Steps, StepsDirection, StepItem};

rsx! {
    Steps {
        direction: Some(StepsDirection::Vertical),
        items: vec![
            StepItem::new("1", rsx!("Step 1")),
            StepItem::new("2", rsx!("Step 2")),
            StepItem::new("3", rsx!("Step 3")),
        ],
    }
}
```

### With Status

```rust
use adui_dioxus::{Steps, StepItem, StepStatus};

rsx! {
    Steps {
        items: vec![
            StepItem {
                key: "1".to_string(),
                title: rsx!("Finished"),
                description: None,
                status: Some(StepStatus::Finish),
                disabled: false,
            },
            StepItem {
                key: "2".to_string(),
                title: rsx!("In Progress"),
                description: None,
                status: Some(StepStatus::Process),
                disabled: false,
            },
            StepItem {
                key: "3".to_string(),
                title: rsx!("Waiting"),
                description: None,
                status: Some(StepStatus::Wait),
                disabled: false,
            },
        ],
    }
}
```

## Use Cases

- **Form Wizards**: Multi-step form processes
- **Onboarding**: User onboarding flows
- **Checkout Processes**: E-commerce checkout steps
- **Workflows**: Step-by-step workflows

## Differences from Ant Design 6.0.0

- ✅ Horizontal and vertical directions
- ✅ Step statuses
- ✅ Descriptions
- ✅ Clickable steps
- ⚠️ Icon customization not yet implemented
- ⚠️ Some advanced features may differ

