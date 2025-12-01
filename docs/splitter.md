# Splitter

## Overview

The Splitter component displays two resizable panes with a draggable gutter. It's commonly used for creating resizable layouts like code editors or file browsers.

## API Reference

### SplitterProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `orientation` | `SplitterOrientation` | `SplitterOrientation::Horizontal` | Splitter orientation |
| `split` | `Option<f32>` | `None` | Controlled split ratio (0.0-1.0) |
| `default_split` | `f32` | `0.5` | Default split ratio in uncontrolled mode |
| `on_change` | `Option<EventHandler<f32>>` | `None` | Called when split ratio changes |
| `on_moving` | `Option<EventHandler<f32>>` | `None` | Called while dragging |
| `on_release` | `Option<EventHandler<f32>>` | `None` | Called when drag ends |
| `min_primary` | `Option<f32>` | `None` | Minimum size for primary pane (defaults to 80px) |
| `min_secondary` | `Option<f32>` | `None` | Minimum size for secondary pane (defaults to 80px) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `gutter_aria_label` | `Option<String>` | `None` | ARIA label for gutter |
| `first` | `Element` | - | First pane content (required) |
| `second` | `Element` | - | Second pane content (required) |

### SplitterOrientation

- `Horizontal` - Horizontal splitter (default)
- `Vertical` - Vertical splitter

## Usage Examples

### Basic Splitter

```rust
use adui_dioxus::Splitter;

rsx! {
    Splitter {
        first: rsx! {
            div { "Left Pane" }
        }
        second: rsx! {
            div { "Right Pane" }
        }
    }
}
```

### Vertical Splitter

```rust
use adui_dioxus::{Splitter, SplitterOrientation};

rsx! {
    Splitter {
        orientation: SplitterOrientation::Vertical,
        first: rsx! {
            div { "Top Pane" }
        }
        second: rsx! {
            div { "Bottom Pane" }
        }
    }
}
```

### Controlled Splitter

```rust
use adui_dioxus::Splitter;
use dioxus::prelude::*;

let split_ratio = use_signal(|| 0.3);

rsx! {
    Splitter {
        split: Some(*split_ratio.read()),
        on_change: Some(move |ratio| {
            split_ratio.set(ratio);
        }),
        first: rsx! { div { "Left" } }
        second: rsx! { div { "Right" } }
    }
}
```

### With Minimum Sizes

```rust
use adui_dioxus::Splitter;

rsx! {
    Splitter {
        min_primary: Some(200.0),
        min_secondary: Some(300.0),
        first: rsx! { div { "Left" } }
        second: rsx! { div { "Right" } }
    }
}
```

## Use Cases

- **Code Editors**: Split panes for code and preview
- **File Browsers**: Resizable file and content panes
- **Dashboards**: Resizable dashboard panels
- **Layouts**: Create resizable layouts

## Differences from Ant Design 6.0.0

- ✅ Horizontal and vertical orientations
- ✅ Controlled and uncontrolled modes
- ✅ Minimum size constraints
- ✅ Drag handlers
- ⚠️ Some advanced features may differ

