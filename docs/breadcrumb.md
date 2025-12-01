# Breadcrumb

## Overview

The Breadcrumb component displays a navigation path, showing the user's current location within a hierarchy. It's commonly used in page headers and navigation areas.

## API Reference

### BreadcrumbProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<BreadcrumbItem>` | - | Ordered list of breadcrumb items (required) |
| `separator` | `Option<String>` | `None` (defaults to "/") | Separator string between items |
| `class` | `Option<String>` | `None` | Additional CSS class applied to root |
| `style` | `Option<String>` | `None` | Inline styles applied to root |
| `on_item_click` | `Option<EventHandler<String>>` | `None` | Click handler for non-final items (receives item id) |

### BreadcrumbItem

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String` | Unique identifier for the item |
| `label` | `String` | Text label displayed |
| `href` | `Option<String>` | Optional link target |

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        items: vec![
            BreadcrumbItem::new("home", "Home"),
            BreadcrumbItem::new("category", "Category"),
            BreadcrumbItem::new("item", "Current Item"),
        ],
    }
}
```

### With Links

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        items: vec![
            BreadcrumbItem::with_href("home", "Home", "/"),
            BreadcrumbItem::with_href("category", "Category", "/category"),
            BreadcrumbItem::new("item", "Current Item"),
        ],
    }
}
```

### Custom Separator

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        separator: Some(">".to_string()),
        items: vec![
            BreadcrumbItem::new("1", "First"),
            BreadcrumbItem::new("2", "Second"),
            BreadcrumbItem::new("3", "Third"),
        ],
    }
}
```

### With Click Handler

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        items: vec![
            BreadcrumbItem::new("home", "Home"),
            BreadcrumbItem::new("category", "Category"),
            BreadcrumbItem::new("item", "Current"),
        ],
        on_item_click: Some(move |id| {
            println!("Clicked: {}", id);
        }),
    }
}
```

## Use Cases

- **Page Navigation**: Show current page location
- **Hierarchical Navigation**: Display navigation hierarchy
- **Breadcrumb Trails**: Help users understand their location
- **Deep Navigation**: Navigate through nested structures

## Differences from Ant Design 6.0.0

- ✅ Basic breadcrumb functionality
- ✅ Custom separators
- ✅ Link support
- ✅ Click handlers
- ⚠️ Icon support not yet implemented
- ⚠️ Some advanced styling options may differ

