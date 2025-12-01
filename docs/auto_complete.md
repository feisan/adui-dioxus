# AutoComplete

## Overview

The AutoComplete component provides an input field with autocomplete suggestions. It filters options based on user input and allows selection from a dropdown list.

## API Reference

### AutoCompleteProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `options` | `Option<Vec<SelectOption>>` | `None` | Candidate options list |
| `value` | `Option<String>` | `None` | Controlled input value |
| `default_value` | `Option<String>` | `None` | Initial value in uncontrolled mode |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `allow_clear` | `bool` | `false` | Show clear button |
| `disabled` | `bool` | `false` | Disable interactions |
| `status` | `Option<ControlStatus>` | `None` | Control status |
| `size` | `Option<ComponentSize>` | `None` | Component size |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `dropdown_class` | `Option<String>` | `None` | Extra class for dropdown |
| `dropdown_style` | `Option<String>` | `None` | Extra style for dropdown |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when input changes |
| `on_search` | `Option<EventHandler<String>>` | `None` | Called when search text changes |
| `on_select` | `Option<EventHandler<String>>` | `None` | Called when option is selected |

## Usage Examples

### Basic AutoComplete

```rust
use adui_dioxus::{AutoComplete, SelectOption};
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    AutoComplete {
        options: Some(vec![
            SelectOption::new("apple", "Apple"),
            SelectOption::new("banana", "Banana"),
            SelectOption::new("cherry", "Cherry"),
        ]),
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("Type to search...".to_string()),
    }
}
```

### With Search Handler

```rust
use adui_dioxus::{AutoComplete, SelectOption};
use dioxus::prelude::*;

let value = use_signal(|| String::new());
let options = use_signal(|| vec![
    SelectOption::new("1", "Option 1"),
]);

rsx! {
    AutoComplete {
        options: Some(options.read().clone()),
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        on_search: Some(move |query| {
            // Update options based on search query
            println!("Searching for: {}", query);
        }),
        placeholder: Some("Type to search...".to_string()),
    }
}
```

### With Selection Handler

```rust
use adui_dioxus::{AutoComplete, SelectOption};

rsx! {
    AutoComplete {
        options: Some(vec![
            SelectOption::new("1", "Option 1"),
            SelectOption::new("2", "Option 2"),
        ]),
        on_select: Some(move |key| {
            println!("Selected: {}", key);
        }),
        placeholder: Some("Type to search...".to_string()),
    }
}
```

## Use Cases

- **Search**: Autocomplete search input
- **Forms**: Form input with suggestions
- **Data Entry**: Data entry with autocomplete
- **Filtering**: Filter options as user types

## Differences from Ant Design 6.0.0

- ✅ Input with autocomplete
- ✅ Option filtering
- ✅ Selection handling
- ✅ Search callbacks
- ⚠️ Some advanced features may differ

