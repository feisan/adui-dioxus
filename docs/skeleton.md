# Skeleton

## Overview

The Skeleton component displays a placeholder loading state with animated skeleton blocks. It's used to show loading placeholders before content is ready.

## API Reference

### SkeletonProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `loading` | `Option<bool>` | `None` (defaults to `true`) | Whether to display skeleton (when false, renders content) |
| `active` | `bool` | `false` | Whether to show active animation |
| `title` | `bool` | `true` | Whether to render a title block |
| `paragraph_rows` | `Option<u8>` | `None` (defaults to 3) | Number of paragraph lines |
| `class` | `Option<String>` | `None` | Extra root class |
| `style` | `Option<String>` | `None` | Inline styles for root |
| `content` | `Option<Element>` | `None` | Wrapped content (rendered when loading is false) |

## Usage Examples

### Basic Skeleton

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {}
}
```

### With Custom Rows

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {
        paragraph_rows: Some(5),
    }
}
```

### Active Animation

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {
        active: true,
    }
}
```

### Without Title

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {
        title: false,
        paragraph_rows: Some(2),
    }
}
```

### With Content

```rust
use adui_dioxus::Skeleton;
use dioxus::prelude::*;

let is_loading = use_signal(|| true);

rsx! {
    Skeleton {
        loading: Some(*is_loading.read()),
        content: Some(rsx! {
            div { "Actual content here" }
        }),
    }
}
```

## Use Cases

- **Content Loading**: Show skeleton while content loads
- **Card Loading**: Display skeleton in cards
- **List Loading**: Show skeleton for list items
- **Page Loading**: Display skeleton for entire pages

## Differences from Ant Design 6.0.0

- ✅ Basic skeleton blocks
- ✅ Title and paragraph rows
- ✅ Active animation
- ✅ Content wrapping
- ⚠️ Avatar skeleton not yet implemented
- ⚠️ Button skeleton not yet implemented
- ⚠️ Some advanced styling options may differ

