# Rate

## Overview

The Rate component allows users to rate something using stars. It supports half-star ratings, custom characters, tooltips, and keyboard navigation.

## API Reference

### RateProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<f64>` | `None` | Controlled numeric value |
| `default_value` | `Option<f64>` | `None` | Uncontrolled initial value |
| `count` | `usize` | `5` | Total count of items (stars) |
| `allow_half` | `bool` | `false` | Allow selecting half steps (0.5 increments) |
| `allow_clear` | `bool` | `true` | Allow clearing when clicking the same value again |
| `disabled` | `bool` | `false` | Disable interactions |
| `character` | `Option<Element>` | `None` | Optional custom character for each item |
| `tooltips` | `Option<Vec<String>>` | `None` | Optional tooltips per item (aligned by index) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `on_change` | `Option<EventHandler<Option<f64>>>` | `None` | Change callback (None means cleared) |
| `on_hover_change` | `Option<EventHandler<Option<f64>>>` | `None` | Hover value callback |
| `on_focus` | `Option<EventHandler<()>>` | `None` | Focus event handler |
| `on_blur` | `Option<EventHandler<()>>` | `None` | Blur event handler |

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Rate;
use dioxus::prelude::*;

let rating = use_signal(|| Some(3.0));

rsx! {
    Rate {
        value: *rating.read(),
        on_change: Some(move |new_rating| {
            rating.set(new_rating);
        }),
    }
}
```

### Half Star Rating

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        allow_half: true,
        default_value: Some(3.5),
    }
}
```

### With Tooltips

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        tooltips: Some(vec![
            "Terrible".to_string(),
            "Bad".to_string(),
            "Normal".to_string(),
            "Good".to_string(),
            "Wonderful".to_string(),
        ]),
    }
}
```

### Custom Character

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        character: Some(rsx!(span { "❤" })),
        count: 5,
    }
}
```

### Disabled

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        value: Some(4.0),
        disabled: true,
    }
}
```

## Use Cases

- **Product Ratings**: Rate products or services
- **Reviews**: Collect user ratings
- **Feedback**: Gather user feedback
- **Surveys**: Rating questions in surveys

## Differences from Ant Design 6.0.0

- ✅ Star ratings with half-star support
- ✅ Custom characters
- ✅ Tooltips
- ✅ Keyboard navigation
- ✅ Hover feedback
- ⚠️ Some advanced styling options may differ

