# List

## Overview

The List component displays a list of items with optional header, footer, loading state, and pagination. It's commonly used for displaying data lists.

## API Reference

### ListProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `header` | `Option<Element>` | `None` | Optional header content |
| `footer` | `Option<Element>` | `None` | Optional footer content |
| `bordered` | `bool` | `false` | Whether to render border |
| `size` | `Option<ComponentSize>` | `None` | Visual density |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `loading` | `bool` | `false` | Whether list is in loading state |
| `is_empty` | `Option<bool>` | `None` | Whether data set is empty |
| `empty` | `Option<Element>` | `None` | Custom empty content |
| `pagination_total` | `Option<u32>` | `None` | Pagination total items |
| `pagination_current` | `Option<u32>` | `None` | Current page index (1-based) |
| `pagination_page_size` | `Option<u32>` | `None` | Page size for pagination |
| `pagination_on_change` | `Option<EventHandler<(u32, u32)>>` | `None` | Callback when pagination changes |
| `children` | `Element` | - | List item content (required) |

## Usage Examples

### Basic List

```rust
use adui_dioxus::List;

rsx! {
    List {
        div { class: "adui-list-item", "Item 1" }
        div { class: "adui-list-item", "Item 2" }
        div { class: "adui-list-item", "Item 3" }
    }
}
```

### With Header and Footer

```rust
use adui_dioxus::List;

rsx! {
    List {
        header: Some(rsx!("List Header")),
        footer: Some(rsx!("List Footer")),
        div { class: "adui-list-item", "Item 1" }
        div { class: "adui-list-item", "Item 2" }
    }
}
```

### Bordered List

```rust
use adui_dioxus::List;

rsx! {
    List {
        bordered: true,
        div { class: "adui-list-item", "Item 1" }
        div { class: "adui-list-item", "Item 2" }
    }
}
```

### Loading State

```rust
use adui_dioxus::List;

rsx! {
    List {
        loading: true,
        div { class: "adui-list-item", "Item 1" }
        div { class: "adui-list-item", "Item 2" }
    }
}
```

### With Pagination

```rust
use adui_dioxus::List;
use dioxus::prelude::*;

let current_page = use_signal(|| 1u32);

rsx! {
    List {
        pagination_total: Some(100),
        pagination_current: Some(*current_page.read()),
        pagination_on_change: Some(move |(page, _size)| {
            current_page.set(page);
        }),
        div { class: "adui-list-item", "Item 1" }
        div { class: "adui-list-item", "Item 2" }
    }
}
```

## Use Cases

- **Data Lists**: Display lists of data items
- **User Lists**: Show user lists
- **Product Lists**: Display product catalogs
- **Activity Feeds**: Show activity feeds

## Differences from Ant Design 6.0.0

- ✅ Basic list functionality
- ✅ Header and footer
- ✅ Loading state
- ✅ Empty state
- ✅ Pagination support
- ⚠️ Grid layout not yet implemented
- ⚠️ Some advanced features may differ

