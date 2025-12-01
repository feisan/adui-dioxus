# Mentions

## Overview

The Mentions component provides a textarea-like input that shows a dropdown when the user types a trigger character (default `@`) to allow selecting from a list of options. It's commonly used for @-style mentions in text input.

## API Reference

### MentionsProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled value of the textarea |
| `default_value` | `Option<String>` | `None` | Default value for uncontrolled usage |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `options` | `Vec<MentionOption>` | `vec![]` | Available mention options |
| `prefix` | `Vec<char>` | `vec!['@']` | Trigger character(s) that open the dropdown |
| `split` | `char` | `' '` | Character that separates mentions from other text |
| `disabled` | `bool` | `false` | Whether the component is disabled |
| `read_only` | `bool` | `false` | Whether the component is read-only |
| `loading` | `bool` | `false` | Whether to show a loading indicator |
| `status` | `Option<ControlStatus>` | `None` | Status for validation styling |
| `size` | `Option<ComponentSize>` | `None` | Size variant |
| `placement` | `MentionPlacement` | `MentionPlacement::Bottom` | Dropdown placement |
| `auto_focus` | `bool` | `false` | Whether to auto-focus the input |
| `rows` | `u32` | `1` | Number of rows for the textarea |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when the value changes |
| `on_select` | `Option<EventHandler<MentionOption>>` | `None` | Called when an option is selected |
| `on_search` | `Option<EventHandler<(String, char)>>` | `None` | Called when search text changes |
| `on_focus` | `Option<EventHandler<()>>` | `None` | Called when focus changes |
| `on_blur` | `Option<EventHandler<()>>` | `None` | Called when blur occurs |
| `filter_option` | `Option<fn(&str, &MentionOption) -> bool>` | `None` | Custom filter function |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### MentionOption

| Field | Type | Description |
|-------|------|-------------|
| `value` | `String` | The value to insert when selected |
| `label` | `String` | The label to display in the dropdown |
| `disabled` | `bool` | Whether this option is disabled |

### MentionPlacement

- `Top` - Dropdown appears above the input
- `Bottom` - Dropdown appears below the input (default)

## Usage Examples

### Basic Mentions

```rust
use adui_dioxus::{Mentions, MentionOption};
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    Mentions {
        options: vec![
            MentionOption::new("user1", "User 1"),
            MentionOption::new("user2", "User 2"),
            MentionOption::new("user3", "User 3"),
        ],
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("Type @ to mention".to_string()),
    }
}
```

### With Custom Trigger

```rust
use adui_dioxus::{Mentions, MentionOption};

rsx! {
    Mentions {
        prefix: vec!['#'],
        options: vec![
            MentionOption::new("tag1", "Tag 1"),
            MentionOption::new("tag2", "Tag 2"),
        ],
        placeholder: Some("Type # to tag".to_string()),
    }
}
```

### With Search Handler

```rust
use adui_dioxus::{Mentions, MentionOption};

rsx! {
    Mentions {
        options: vec![
            MentionOption::new("user1", "User 1"),
        ],
        on_search: Some(move |(query, trigger)| {
            println!("Searching for: {} (trigger: {})", query, trigger);
        }),
        placeholder: Some("Type @ to mention".to_string()),
    }
}
```

## Use Cases

- **User Mentions**: @-mention users in comments or messages
- **Tagging**: Tag items with #-style mentions
- **Autocomplete**: Autocomplete in text input
- **Social Features**: Social media style mentions

## Differences from Ant Design 6.0.0

- ✅ Trigger character support
- ✅ Dropdown selection
- ✅ Search functionality
- ✅ Custom filter
- ⚠️ Some advanced features may differ

