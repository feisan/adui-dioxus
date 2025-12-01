# Tag

## Overview

The Tag component is used for labeling and categorizing content. It supports preset colors, closable functionality, and checkable states.

## API Reference

### TagProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `color` | `Option<TagColor>` | `None` | Preset color for the tag |
| `closable` | `bool` | `false` | Whether the tag can be closed |
| `on_close` | `Option<EventHandler<()>>` | `None` | Called when the close icon is clicked |
| `checkable` | `bool` | `false` | Whether the tag is checkable (togglable selection) |
| `checked` | `Option<bool>` | `None` | Controlled checked state for checkable tags |
| `default_checked` | `Option<bool>` | `None` | Default checked state for uncontrolled checkable tags |
| `on_change` | `Option<EventHandler<bool>>` | `None` | Called when the checked state changes |
| `class` | `Option<String>` | `None` | Extra class for the tag |
| `style` | `Option<String>` | `None` | Inline style for the tag |
| `children` | `Element` | - | Tag content (required) |

### TagColor

- `Default` - Default color
- `Primary` - Primary color
- `Success` - Success color (green)
- `Warning` - Warning color (orange)
- `Error` - Error color (red)

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Tag;

rsx! {
    Tag {
        "Tag Label"
    }
}
```

### With Colors

```rust
use adui_dioxus::{Tag, TagColor};

rsx! {
    div {
        Tag { color: Some(TagColor::Default), "Default" }
        Tag { color: Some(TagColor::Primary), "Primary" }
        Tag { color: Some(TagColor::Success), "Success" }
        Tag { color: Some(TagColor::Warning), "Warning" }
        Tag { color: Some(TagColor::Error), "Error" }
    }
}
```

### Closable Tag

```rust
use adui_dioxus::Tag;

rsx! {
    Tag {
        closable: true,
        on_close: Some(move |_| {
            println!("Tag closed");
        }),
        "Closable Tag"
    }
}
```

### Checkable Tag

```rust
use adui_dioxus::Tag;
use dioxus::prelude::*;

let checked = use_signal(|| false);

rsx! {
    Tag {
        checkable: true,
        checked: Some(*checked.read()),
        on_change: Some(move |is_checked| {
            checked.set(is_checked);
        }),
        "Checkable Tag"
    }
}
```

## Use Cases

- **Labels**: Categorize and label content
- **Filters**: Use checkable tags for filtering
- **Status Indicators**: Show status with colored tags
- **Removable Items**: Use closable tags for removable items
- **Tags Input**: Build tag input components

## Differences from Ant Design 6.0.0

- ✅ All preset colors supported
- ✅ Closable functionality
- ✅ Checkable states
- ✅ Controlled and uncontrolled modes
- ⚠️ Custom colors (hex/rgb) not yet implemented
- ⚠️ Some advanced styling options may differ

