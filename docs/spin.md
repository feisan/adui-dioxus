# Spin

## Overview

The Spin component displays a loading spinner indicator. It can be used standalone or to wrap content, showing a loading overlay when active.

## API Reference

### SpinProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `spinning` | `Option<bool>` | `None` (defaults to `true`) | Whether the spin indicator is active |
| `size` | `Option<SpinSize>` | `None` (defaults to `SpinSize::Default`) | Visual size of the indicator |
| `tip` | `Option<String>` | `None` | Optional text shown under the indicator |
| `class` | `Option<String>` | `None` | Extra class for the root element |
| `style` | `Option<String>` | `None` | Inline style for the root element |
| `fullscreen` | `bool` | `false` | Whether to treat this as a fullscreen overlay |
| `children` | `Element` | - | Optional content wrapped by the spinner (required) |

### SpinSize

- `Small` - Small size spinner
- `Default` - Default size spinner
- `Large` - Large size spinner

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Spin;

rsx! {
    Spin {
        div { "Content being loaded..." }
    }
}
```

### With Tip Text

```rust
use adui_dioxus::Spin;

rsx! {
    Spin {
        tip: Some("Loading...".to_string()),
        div { "Content" }
    }
}
```

### Controlled Spinning State

```rust
use adui_dioxus::Spin;
use dioxus::prelude::*;

let is_loading = use_signal(|| true);

rsx! {
    Spin {
        spinning: Some(*is_loading.read()),
        tip: Some("Please wait...".to_string()),
        div { "Content" }
    }
}
```

### Different Sizes

```rust
use adui_dioxus::{Spin, SpinSize};

rsx! {
    div {
        Spin { size: Some(SpinSize::Small), div { "Small" } }
        Spin { size: Some(SpinSize::Default), div { "Default" } }
        Spin { size: Some(SpinSize::Large), div { "Large" } }
    }
}
```

### Fullscreen Spinner

```rust
use adui_dioxus::Spin;

rsx! {
    Spin {
        fullscreen: true,
        tip: Some("Loading page...".to_string()),
        div { "Page content" }
    }
}
```

## Use Cases

- **Data Loading**: Show spinner while fetching data
- **Form Submission**: Display spinner during form submission
- **Content Loading**: Wrap content that's being loaded
- **Fullscreen Loading**: Show fullscreen loading state

## Differences from Ant Design 6.0.0

- ✅ All basic spinner sizes supported
- ✅ Tip text support
- ✅ Fullscreen mode
- ✅ Content wrapping with overlay
- ⚠️ Some advanced styling options may differ
- ⚠️ Custom indicator icons not yet implemented

