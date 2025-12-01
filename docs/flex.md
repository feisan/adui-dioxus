# Flex

## Overview

The Flex component provides a flexible box layout container with configurable alignment, wrapping, and gap spacing. It supports both horizontal and vertical orientations.

## API Reference

### FlexProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `direction` | `FlexDirection` | `FlexDirection::Row` | Flex direction |
| `justify` | `FlexJustify` | `FlexJustify::Start` | Justify content |
| `align` | `FlexAlign` | `FlexAlign::Stretch` | Align items |
| `wrap` | `FlexWrap` | `FlexWrap::NoWrap` | Wrap behavior |
| `orientation` | `Option<FlexOrientation>` | `None` | Orientation helper |
| `vertical` | `bool` | `false` | Vertical layout |
| `component` | `FlexComponent` | `FlexComponent::Div` | Root element type |
| `gap` | `Option<f32>` | `None` | Gap between items |
| `row_gap` | `Option<f32>` | `None` | Row gap |
| `column_gap` | `Option<f32>` | `None` | Column gap |
| `gap_size` | `Option<FlexGap>` | `None` | Preset gap size |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Flex children (required) |

### FlexDirection

- `Row` - Row direction (default)
- `RowReverse` - Row reverse
- `Column` - Column direction
- `ColumnReverse` - Column reverse

### FlexJustify

- `Start` - Start alignment (default)
- `End` - End alignment
- `Center` - Center alignment
- `Between` - Space between
- `Around` - Space around
- `Evenly` - Space evenly

### FlexAlign

- `Start` - Start alignment
- `End` - End alignment
- `Center` - Center alignment
- `Stretch` - Stretch alignment (default)
- `Baseline` - Baseline alignment

### FlexWrap

- `NoWrap` - No wrap (default)
- `Wrap` - Wrap
- `WrapReverse` - Wrap reverse

### FlexGap

- `Small` - Small gap
- `Middle` - Middle gap
- `Large` - Large gap

## Usage Examples

### Basic Flex

```rust
use adui_dioxus::Flex;

rsx! {
    Flex {
        div { "Item 1" }
        div { "Item 2" }
        div { "Item 3" }
    }
}
```

### Vertical Flex

```rust
use adui_dioxus::Flex;

rsx! {
    Flex {
        vertical: true,
        div { "Item 1" }
        div { "Item 2" }
    }
}
```

### With Gap

```rust
use adui_dioxus::{Flex, FlexGap};

rsx! {
    Flex {
        gap_size: Some(FlexGap::Middle),
        div { "Item 1" }
        div { "Item 2" }
    }
}
```

### With Justify and Align

```rust
use adui_dioxus::{Flex, FlexJustify, FlexAlign};

rsx! {
    Flex {
        justify: FlexJustify::Center,
        align: FlexAlign::Center,
        div { "Centered Item" }
    }
}
```

### With Wrap

```rust
use adui_dioxus::{Flex, FlexWrap};

rsx! {
    Flex {
        wrap: FlexWrap::Wrap,
        div { "Item 1" }
        div { "Item 2" }
        div { "Item 3" }
    }
}
```

## Use Cases

- **Layouts**: Create flexible layouts
- **Component Arrangement**: Arrange components with spacing
- **Responsive Design**: Create responsive layouts
- **Alignment**: Align items in containers

## Differences from Ant Design 6.0.0

- ✅ Flex direction and wrap
- ✅ Justify and align options
- ✅ Gap spacing
- ✅ Preset gap sizes
- ⚠️ Some advanced features may differ

