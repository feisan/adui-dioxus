# Grid

## Overview

The Grid component provides a 24-column grid system for responsive layouts. It consists of `Row` and `Col` components that work together to create flexible, responsive layouts.

## API Reference

### RowProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `gutter` | `Option<f32>` | `None` | Horizontal gutter spacing |
| `gutter_vertical` | `Option<f32>` | `None` | Vertical gutter spacing |
| `responsive_gutter` | `Option<ResponsiveGutter>` | `None` | Responsive gutter configuration |
| `gutter_spec` | `Option<RowGutter>` | `None` | Gutter specification (uniform/pair/responsive) |
| `justify` | `RowJustify` | `RowJustify::Start` | Horizontal justification |
| `align` | `RowAlign` | `RowAlign::Top` | Cross-axis alignment |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Row children (Col components) (required) |

### ColProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `span` | `Option<f32>` | `None` | Number of columns to span (1-24) |
| `offset` | `Option<f32>` | `None` | Number of columns to offset |
| `order` | `Option<f32>` | `None` | Display order |
| `push` | `Option<f32>` | `None` | Number of columns to push |
| `pull` | `Option<f32>` | `None` | Number of columns to pull |
| `flex` | `Option<String>` | `None` | Flex value |
| `xs` | `Option<f32>` | `None` | Span for xs breakpoint |
| `sm` | `Option<f32>` | `None` | Span for sm breakpoint |
| `md` | `Option<f32>` | `None` | Span for md breakpoint |
| `lg` | `Option<f32>` | `None` | Span for lg breakpoint |
| `xl` | `Option<f32>` | `None` | Span for xl breakpoint |
| `xxl` | `Option<f32>` | `None` | Span for xxl breakpoint |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Column content (required) |

### RowJustify

- `Start` - Start alignment (default)
- `End` - End alignment
- `Center` - Center alignment
- `SpaceAround` - Space around items
- `SpaceBetween` - Space between items
- `SpaceEvenly` - Space evenly

### RowAlign

- `Top` - Top alignment (default)
- `Middle` - Middle alignment
- `Bottom` - Bottom alignment
- `Stretch` - Stretch alignment

## Usage Examples

### Basic Grid

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        Col { span: Some(8.0), "Col 1" }
        Col { span: Some(8.0), "Col 2" }
        Col { span: Some(8.0), "Col 3" }
    }
}
```

### With Gutter

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        gutter: Some(16.0),
        Col { span: Some(12.0), "Col 1" }
        Col { span: Some(12.0), "Col 2" }
    }
}
```

### Responsive Grid

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        Col {
            xs: Some(24.0),
            sm: Some(12.0),
            md: Some(8.0),
            lg: Some(6.0),
            "Responsive Col"
        }
    }
}
```

### With Offset

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        Col { span: Some(8.0), "Col 1" }
        Col { span: Some(8.0), offset: Some(8.0), "Col 2" }
    }
}
```

## Use Cases

- **Page Layouts**: Create responsive page layouts
- **Form Layouts**: Organize form fields
- **Content Grids**: Display content in grids
- **Dashboard**: Create dashboard layouts

## Differences from Ant Design 6.0.0

- ✅ 24-column grid system
- ✅ Responsive breakpoints
- ✅ Gutter spacing
- ✅ Flex support
- ⚠️ Some advanced features may differ

