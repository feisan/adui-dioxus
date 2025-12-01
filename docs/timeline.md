# Timeline

## Overview

The Timeline component displays a vertical or horizontal timeline of events. It supports different modes, orientations, and custom icons.

## API Reference

### TimelineProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<TimelineItem>` | - | Timeline items to display (required) |
| `mode` | `TimelineMode` | `TimelineMode::Left` | Timeline mode (position) |
| `orientation` | `TimelineOrientation` | `TimelineOrientation::Vertical` | Timeline orientation |
| `reverse` | `bool` | `false` | Whether to reverse the order of items |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### TimelineItem

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the item |
| `title` | `Option<Element>` | Optional title |
| `content` | `Option<Element>` | Optional content |
| `icon` | `Option<Element>` | Optional custom icon |
| `color` | `Option<TimelineColor>` | Optional color preset |
| `pending` | `bool` | Whether item is in pending/loading state |

### TimelineMode

- `Left` - All items on the left (default)
- `Right` - All items on the right
- `Alternate` - Items alternate between left and right

### TimelineOrientation

- `Vertical` - Vertical timeline (default)
- `Horizontal` - Horizontal timeline

### TimelineColor

- `Blue` - Blue color
- `Green` - Green color
- `Red` - Red color
- `Gray` - Gray color

## Usage Examples

### Basic Timeline

```rust
use adui_dioxus::{Timeline, TimelineItem};

rsx! {
    Timeline {
        items: vec![
            TimelineItem::new("1").title(rsx!("Event 1")).content(rsx!("Description 1")),
            TimelineItem::new("2").title(rsx!("Event 2")).content(rsx!("Description 2")),
            TimelineItem::new("3").title(rsx!("Event 3")).content(rsx!("Description 3")),
        ],
    }
}
```

### Alternate Mode

```rust
use adui_dioxus::{Timeline, TimelineMode, TimelineItem};

rsx! {
    Timeline {
        mode: TimelineMode::Alternate,
        items: vec![
            TimelineItem::new("1").title(rsx!("Event 1")),
            TimelineItem::new("2").title(rsx!("Event 2")),
            TimelineItem::new("3").title(rsx!("Event 3")),
        ],
    }
}
```

### With Colors

```rust
use adui_dioxus::{Timeline, TimelineItem, TimelineColor};

rsx! {
    Timeline {
        items: vec![
            TimelineItem::new("1")
                .title(rsx!("Success"))
                .color(TimelineColor::Green),
            TimelineItem::new("2")
                .title(rsx!("Error"))
                .color(TimelineColor::Red),
        ],
    }
}
```

### Pending Item

```rust
use adui_dioxus::{Timeline, TimelineItem};

rsx! {
    Timeline {
        items: vec![
            TimelineItem::new("1").title(rsx!("Completed")),
            TimelineItem::new("2").title(rsx!("In Progress")),
            TimelineItem::new("3").title(rsx!("Pending")).pending(true),
        ],
    }
}
```

## Use Cases

- **Activity Feeds**: Display activity timelines
- **Process Tracking**: Track process steps
- **History**: Display historical events
- **Status Updates**: Show status update timeline

## Differences from Ant Design 6.0.0

- ✅ Vertical and horizontal orientations
- ✅ Multiple modes (left, right, alternate)
- ✅ Custom icons
- ✅ Color presets
- ✅ Pending state
- ⚠️ Some advanced features may differ

