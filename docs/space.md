# Space

## Overview

The Space component provides spacing between child elements. It supports horizontal and vertical layouts, custom gaps, alignment, and optional separators.

## API Reference

### SpaceProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `direction` | `SpaceDirection` | `SpaceDirection::Horizontal` | Layout direction |
| `size` | `SpaceSize` | `SpaceSize::Middle` | Preset size for spacing |
| `gap` | `Option<f32>` | `None` | Custom gap value (overrides size) |
| `gap_cross` | `Option<f32>` | `None` | Cross-axis gap value |
| `wrap` | `Option<bool>` | `None` | Whether to wrap items (defaults based on direction) |
| `align` | `SpaceAlign` | `SpaceAlign::Start` | Cross-axis alignment |
| `compact` | `bool` | `false` | Compact mode (reduces spacing) |
| `split` | `Option<Element>` | `None` | Optional separator element between items |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Child elements (required) |

### SpaceDirection

- `Horizontal` - Horizontal layout (default)
- `Vertical` - Vertical layout

### SpaceAlign

- `Start` - Align to start (default)
- `End` - Align to end
- `Center` - Center alignment
- `Baseline` - Baseline alignment

### SpaceSize

- `Small` - Small spacing
- `Middle` - Middle spacing (default)
- `Large` - Large spacing

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Space, Button, ButtonType};

rsx! {
    Space {
        Button { r#type: ButtonType::Primary, "Button 1" }
        Button { r#type: ButtonType::Default, "Button 2" }
        Button { r#type: ButtonType::Dashed, "Button 3" }
    }
}
```

### Vertical Layout

```rust
use adui_dioxus::{Space, SpaceDirection, Button, ButtonType};

rsx! {
    Space {
        direction: SpaceDirection::Vertical,
        Button { r#type: ButtonType::Primary, "Top" }
        Button { r#type: ButtonType::Default, "Middle" }
        Button { r#type: ButtonType::Dashed, "Bottom" }
    }
}
```

### Custom Gap

```rust
use adui_dioxus::Space;

rsx! {
    Space {
        gap: Some(32.0),
        div { "Item 1" }
        div { "Item 2" }
        div { "Item 3" }
    }
}
```

### With Separator

```rust
use adui_dioxus::{Space, Divider};

rsx! {
    Space {
        split: Some(rsx!(Divider { vertical: true })),
        span { "Item 1" }
        span { "Item 2" }
        span { "Item 3" }
    }
}
```

### Compact Mode

```rust
use adui_dioxus::Space;

rsx! {
    Space {
        compact: true,
        Button { "Compact 1" }
        Button { "Compact 2" }
        Button { "Compact 3" }
    }
}
```

## Use Cases

- **Button Groups**: Space buttons evenly
- **Form Fields**: Add spacing between form elements
- **Lists**: Space list items
- **Flexible Layouts**: Create flexible spacing in layouts

## Differences from Ant Design 6.0.0

- ✅ Horizontal and vertical layouts
- ✅ Preset and custom gap sizes
- ✅ Alignment options
- ✅ Separator support
- ✅ Compact mode
- ⚠️ Some advanced styling options may differ

