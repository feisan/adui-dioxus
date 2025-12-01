# Masonry

## Overview

The Masonry component displays items in a masonry (Pinterest-style) layout using CSS columns. It supports responsive column counts and custom gaps.

## API Reference

### MasonryProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `columns` | `u16` | `3` | Number of columns |
| `responsive` | `Option<MasonryResponsive>` | `None` | Responsive column configuration |
| `gap` | `Option<f32>` | `None` | Gap between items (defaults to 16px) |
| `row_gap` | `Option<f32>` | `None` | Row gap (defaults to gap value) |
| `min_column_width` | `Option<f32>` | `None` | Minimum column width |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Masonry items (required) |

### MasonryResponsive

| Field | Type | Description |
|-------|------|-------------|
| `xs` | `Option<u16>` | Columns for xs screens (< 576px) |
| `sm` | `Option<u16>` | Columns for sm screens (≥ 576px) |
| `md` | `Option<u16>` | Columns for md screens (≥ 768px) |
| `lg` | `Option<u16>` | Columns for lg screens (≥ 992px) |
| `xl` | `Option<u16>` | Columns for xl screens (≥ 1200px) |
| `xxl` | `Option<u16>` | Columns for xxl screens (≥ 1600px) |

## Usage Examples

### Basic Masonry

```rust
use adui_dioxus::Masonry;

rsx! {
    Masonry {
        columns: 3,
        div { "Item 1" }
        div { "Item 2" }
        div { "Item 3" }
        div { "Item 4" }
    }
}
```

### Responsive Masonry

```rust
use adui_dioxus::{Masonry, MasonryResponsive};

rsx! {
    Masonry {
        responsive: Some(MasonryResponsive {
            xs: Some(1),
            sm: Some(2),
            md: Some(3),
            lg: Some(4),
            xl: Some(5),
            xxl: Some(6),
        }),
        div { "Item 1" }
        div { "Item 2" }
        div { "Item 3" }
    }
}
```

### Custom Gap

```rust
use adui_dioxus::Masonry;

rsx! {
    Masonry {
        columns: 3,
        gap: Some(24.0),
        row_gap: Some(16.0),
        div { "Item 1" }
        div { "Item 2" }
    }
}
```

### With Minimum Column Width

```rust
use adui_dioxus::Masonry;

rsx! {
    Masonry {
        columns: 3,
        min_column_width: Some(200.0),
        div { "Item 1" }
        div { "Item 2" }
    }
}
```

## Use Cases

- **Image Galleries**: Display images in masonry layout
- **Card Grids**: Show cards in Pinterest-style layout
- **Content Feeds**: Display content in masonry format
- **Portfolios**: Show portfolio items

## Differences from Ant Design 6.0.0

- ✅ CSS columns-based masonry
- ✅ Responsive column counts
- ✅ Custom gaps
- ✅ Minimum column width
- ⚠️ Some advanced features may differ

