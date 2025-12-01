# Select

## Overview

The Select component provides a dropdown selector with support for single selection, multiple selection, tags mode, and combobox mode. It includes search functionality and custom filtering.

## API Reference

### SelectProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled value for single-select |
| `values` | `Option<Vec<String>>` | `None` | Controlled values for multi-select |
| `options` | `Vec<SelectOption>` | - | Option list (required) |
| `mode` | `SelectMode` | `SelectMode::Single` | Selection mode |
| `multiple` | `bool` | `false` | @deprecated Use `mode` instead |
| `allow_clear` | `bool` | `false` | Show clear icon when there is selection |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `disabled` | `bool` | `false` | Disable interactions |
| `show_search` | `bool` | `false` | Enable search functionality |
| `filter_option` | `Option<Rc<dyn Fn(&str, &SelectOption) -> bool>>` | `None` | Custom filter function |
| `token_separators` | `Option<Vec<String>>` | `None` | Token separators for tags mode |
| `status` | `Option<ControlStatus>` | `None` | Control status |
| `size` | `Option<ComponentSize>` | `None` | Component size |
| `variant` | `Option<Variant>` | `None` | Visual variant |
| `bordered` | `Option<bool>` | `None` | @deprecated Use `variant` instead |
| `prefix` | `Option<Element>` | `None` | Prefix element |
| `suffix_icon` | `Option<Element>` | `None` | Custom suffix icon |
| `placement` | `SelectPlacement` | `SelectPlacement::BottomLeft` | Dropdown placement |
| `popup_match_select_width` | `bool` | `true` | Whether dropdown width matches select width |
| `class` | `Option<String>` | `None` | Extra class name |
| `root_class_name` | `Option<String>` | `None` | Extra class for root element |
| `style` | `Option<String>` | `None` | Inline style |
| `class_names` | `Option<SelectClassNames>` | `None` | Semantic class names |
| `styles` | `Option<SelectStyles>` | `None` | Semantic styles |
| `dropdown_class` | `Option<String>` | `None` | Extra class for dropdown |
| `dropdown_style` | `Option<String>` | `None` | Extra style for dropdown |
| `popup_render` | `Option<Rc<dyn Fn(Element) -> Element>>` | `None` | Custom dropdown render function |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | Called when selection changes |
| `on_dropdown_visible_change` | `Option<EventHandler<bool>>` | `None` | Called when dropdown visibility changes |

### SelectMode

- `Single` - Single selection (default)
- `Multiple` - Multiple selection
- `Tags` - Tags mode (allows creating new options)
- `Combobox` - Combobox mode (allows free text input)

### SelectPlacement

- `BottomLeft` - Bottom left (default)
- `BottomRight` - Bottom right
- `TopLeft` - Top left
- `TopRight` - Top right

## Usage Examples

### Basic Select

```rust
use adui_dioxus::{Select, SelectOption};
use dioxus::prelude::*;

let value = use_signal(|| None::<String>);

rsx! {
    Select {
        options: vec![
            SelectOption::new("1", "Option 1"),
            SelectOption::new("2", "Option 2"),
            SelectOption::new("3", "Option 3"),
        ],
        value: *value.read(),
        on_change: Some(move |values| {
            value.set(values.first().cloned());
        }),
        placeholder: Some("Select an option".to_string()),
    }
}
```

### Multiple Select

```rust
use adui_dioxus::{Select, SelectMode, SelectOption};
use dioxus::prelude::*;

let values = use_signal(|| vec![]);

rsx! {
    Select {
        mode: SelectMode::Multiple,
        options: vec![
            SelectOption::new("1", "Option 1"),
            SelectOption::new("2", "Option 2"),
        ],
        values: Some(values.read().clone()),
        on_change: Some(move |v| {
            values.set(v);
        }),
    }
}
```

### With Search

```rust
use adui_dioxus::{Select, SelectOption};

rsx! {
    Select {
        show_search: true,
        options: vec![
            SelectOption::new("1", "Apple"),
            SelectOption::new("2", "Banana"),
            SelectOption::new("3", "Cherry"),
        ],
        placeholder: Some("Search...".to_string()),
    }
}
```

### Tags Mode

```rust
use adui_dioxus::{Select, SelectMode, SelectOption};
use dioxus::prelude::*;

let values = use_signal(|| vec![]);

rsx! {
    Select {
        mode: SelectMode::Tags,
        options: vec![
            SelectOption::new("1", "Tag 1"),
            SelectOption::new("2", "Tag 2"),
        ],
        values: Some(values.read().clone()),
        on_change: Some(move |v| {
            values.set(v);
        }),
        token_separators: Some(vec![",".to_string(), " ".to_string()]),
    }
}
```

## Use Cases

- **Form Selection**: Select options in forms
- **Data Filtering**: Filter data by selection
- **Tag Management**: Manage tags with tags mode
- **Search**: Search and select options

## Differences from Ant Design 6.0.0

- ✅ Single and multiple selection
- ✅ Tags and combobox modes
- ✅ Search functionality
- ✅ Custom filtering
- ⚠️ Some advanced features may differ

