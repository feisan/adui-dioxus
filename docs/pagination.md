# Pagination

## Overview

The Pagination component allows users to navigate through pages of content. It supports page size selection, total count display, and controlled/uncontrolled modes.

## API Reference

### PaginationProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `current` | `Option<u32>` | `None` | Controlled current page index (1-based) |
| `default_current` | `Option<u32>` | `None` | Initial page index when uncontrolled |
| `page_size` | `Option<u32>` | `None` | Controlled page size |
| `default_page_size` | `Option<u32>` | `None` | Initial page size when uncontrolled (defaults to 10) |
| `total` | `u32` | - | Total number of items (required) |
| `page_size_options` | `Option<Vec<u32>>` | `None` | Optional page size options for size changer |
| `show_size_changer` | `bool` | `false` | Whether to show page size changer dropdown |
| `show_total` | `bool` | `false` | Whether to display total text |
| `on_change` | `Option<EventHandler<(u32, u32)>>` | `None` | Called when page or page size changes (page, page_size) |
| `on_page_size_change` | `Option<EventHandler<(u32, u32)>>` | `None` | Called when page size changes (page, page_size) |
| `class` | `Option<String>` | `None` | Extra class for root |
| `style` | `Option<String>` | `None` | Inline style for root |

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Pagination;

rsx! {
    Pagination {
        total: 100,
        default_current: Some(1),
        default_page_size: Some(10),
    }
}
```

### Controlled Pagination

```rust
use adui_dioxus::Pagination;
use dioxus::prelude::*;

let current_page = use_signal(|| 1u32);
let page_size = use_signal(|| 10u32);

rsx! {
    Pagination {
        current: Some(*current_page.read()),
        page_size: Some(*page_size.read()),
        total: 100,
        on_change: Some(move |(page, size)| {
            current_page.set(page);
            page_size.set(size);
        }),
    }
}
```

### With Size Changer

```rust
use adui_dioxus::Pagination;

rsx! {
    Pagination {
        total: 200,
        show_size_changer: true,
        page_size_options: Some(vec![10, 20, 50, 100]),
        default_page_size: Some(20),
    }
}
```

### With Total Display

```rust
use adui_dioxus::Pagination;

rsx! {
    Pagination {
        total: 150,
        show_total: true,
        default_current: Some(1),
    }
}
```

## Use Cases

- **Data Tables**: Paginate table data
- **Lists**: Navigate through list pages
- **Search Results**: Paginate search results
- **Content Pages**: Navigate through content pages

## Differences from Ant Design 6.0.0

- ✅ Basic pagination functionality
- ✅ Page size changer
- ✅ Total count display
- ✅ Controlled and uncontrolled modes
- ⚠️ Simple pagination mode not yet implemented
- ⚠️ Some advanced styling options may differ

