# Table

## Overview

The Table component displays data in a structured table format with support for sorting, filtering, pagination, row selection, and expandable rows.

## API Reference

### TableProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `columns` | `Vec<TableColumn>` | - | Column definitions (required) |
| `data` | `Vec<Value>` | - | Row data as JSON values (required) |
| `row_key_field` | `Option<String>` | `None` | Field used as row key |
| `row_class_name` | `Option<String>` | `None` | Extra class for each row |
| `row_class_name_fn` | `Option<RowClassNameFn>` | `None` | Dynamic row class name function |
| `row_props_fn` | `Option<RowPropsFn>` | `None` | Dynamic row props function |
| `bordered` | `bool` | `false` | Whether to show outer borders |
| `size` | `Option<ComponentSize>` | `None` | Visual density |
| `loading` | `bool` | `false` | Loading state |
| `is_empty` | `Option<bool>` | `None` | Whether table is empty |
| `empty` | `Option<Element>` | `None` | Custom empty node |
| `row_selection` | `Option<RowSelection>` | `None` | Row selection configuration |
| `scroll` | `Option<TableScroll>` | `None` | Scroll configuration |
| `sticky` | `Option<StickyConfig>` | `None` | Sticky header configuration |
| `expandable` | `Option<ExpandableConfig>` | `None` | Expandable row configuration |
| `summary` | `Option<SummaryConfig>` | `None` | Summary row configuration |
| `on_change` | `Option<EventHandler<TableChangeEvent>>` | `None` | Called when pagination/filters/sorter changes |
| `get_popup_container` | `Option<String>` | `None` | Container selector for popups |
| `virtual` | `bool` | `false` | Enable virtual scrolling |
| `locale` | `Option<TableLocale>` | `None` | Locale configuration |
| `show_header` | `bool` | `true` | Show table header |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `class_names` | `Option<TableClassNames>` | `None` | Semantic class names |
| `styles` | `Option<TableStyles>` | `None` | Semantic styles |
| `pagination_total` | `Option<u32>` | `None` | Pagination total items |
| `pagination_current` | `Option<u32>` | `None` | Current page index |
| `pagination_page_size` | `Option<u32>` | `None` | Page size |
| `pagination_on_change` | `Option<EventHandler<(u32, u32)>>` | `None` | Pagination change callback |

### TableColumn

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique column key |
| `title` | `String` | Column title |
| `data_index` | `Option<String>` | Data field index |
| `width` | `Option<f32>` | Column width |
| `align` | `Option<ColumnAlign>` | Cell alignment |
| `fixed` | `Option<ColumnFixed>` | Fixed position |
| `sortable` | `bool` | Whether column is sortable |
| `default_sort_order` | `Option<SortOrder>` | Default sort order |
| `sorter` | `Option<ColumnSorterFn>` | Custom sorter function |
| `filters` | `Option<Vec<ColumnFilter>>` | Filter options |
| `on_filter` | `Option<ColumnFilterFn>` | Custom filter function |
| `render` | `Option<ColumnRenderFn>` | Custom render function |
| `hidden` | `bool` | Whether column is hidden |
| `ellipsis` | `bool` | Ellipsis text overflow |

### ColumnAlign

- `Left` - Left alignment (default)
- `Center` - Center alignment
- `Right` - Right alignment

### SortOrder

- `Ascend` - Ascending order
- `Descend` - Descending order

### ColumnFixed

- `Left` - Fixed to left
- `Right` - Fixed to right

## Usage Examples

### Basic Table

```rust
use adui_dioxus::{Table, TableColumn};
use serde_json::json;

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "Name"),
            TableColumn::new("age", "Age"),
            TableColumn::new("email", "Email"),
        ],
        data: vec![
            json!({"name": "John", "age": 30, "email": "john@example.com"}),
            json!({"name": "Jane", "age": 25, "email": "jane@example.com"}),
        ],
    }
}
```

### With Sorting

```rust
use adui_dioxus::{Table, TableColumn, SortOrder};
use serde_json::Value;

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "Name")
                .sortable(),
            TableColumn::new("age", "Age")
                .sortable()
                .default_sort_order(Some(SortOrder::Ascend)),
        ],
        data: vec![],
    }
}
```

### With Row Selection

```rust
use adui_dioxus::{Table, TableColumn, RowSelection, SelectionType};
use dioxus::prelude::*;
use serde_json::json;

let selected = use_signal(|| vec![]);

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "Name"),
        ],
        data: vec![
            json!({"name": "John"}),
        ],
        row_selection: Some(RowSelection {
            selected_row_keys: selected.read().clone(),
            on_change: Some(move |keys| {
                selected.set(keys);
            }),
            selection_type: SelectionType::Checkbox,
            preserve_selected_row_keys: false,
        }),
    }
}
```

### With Pagination

```rust
use adui_dioxus::{Table, TableColumn};
use dioxus::prelude::*;
use serde_json::json;

let current_page = use_signal(|| 1u32);

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "Name"),
        ],
        data: vec![
            json!({"name": "John"}),
        ],
        pagination_total: Some(100),
        pagination_current: Some(*current_page.read()),
        pagination_on_change: Some(move |(page, _size)| {
            current_page.set(page);
        }),
    }
}
```

## Use Cases

- **Data Display**: Display structured data
- **Data Management**: Manage data with sorting and filtering
- **Reports**: Generate data reports
- **Dashboards**: Display dashboard data

## Differences from Ant Design 6.0.0

- ✅ Sorting and filtering
- ✅ Row selection
- ✅ Pagination
- ✅ Expandable rows
- ✅ Fixed columns
- ⚠️ Some advanced features may differ

