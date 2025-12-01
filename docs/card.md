# Card

## Overview

The Card component is a container for displaying content in a card format. It supports titles, extra content, loading states, and hover effects.

## API Reference

### CardProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `Option<Element>` | `None` | Optional card title rendered in the header |
| `extra` | `Option<Element>` | `None` | Optional extra content rendered in the header's right area |
| `bordered` | `bool` | `true` | Whether to show a border around the card |
| `size` | `Option<ComponentSize>` | `None` | Visual density of the card paddings and typography |
| `loading` | `bool` | `false` | Loading state. When true, the body renders a skeleton instead of children |
| `hoverable` | `bool` | `false` | Whether the card should have a hover effect |
| `class` | `Option<String>` | `None` | Extra class name for the root element |
| `style` | `Option<String>` | `None` | Inline style for the root element |
| `children` | `Element` | - | Card body content (required) |

### ComponentSize

- `Small` - Small size
- `Middle` - Middle size (default)
- `Large` - Large size

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        "Card content goes here"
    }
}
```

### With Title

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        title: Some(rsx!("Card Title")),
        "Card content"
    }
}
```

### With Title and Extra

```rust
use adui_dioxus::{Card, Button, ButtonType};

rsx! {
    Card {
        title: Some(rsx!("Card Title")),
        extra: Some(rsx! {
            Button {
                r#type: ButtonType::Link,
                "More"
            }
        }),
        "Card content"
    }
}
```

### Loading State

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        title: Some(rsx!("Loading Card")),
        loading: true,
        "This content will be replaced by skeleton"
    }
}
```

### Hoverable Card

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        hoverable: true,
        title: Some(rsx!("Hoverable Card")),
        "Hover over this card to see the effect"
    }
}
```

### Without Border

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        bordered: false,
        "Card without border"
    }
}
```

## Use Cases

- **Content Containers**: Display content in card format
- **Dashboard Cards**: Show statistics or information cards
- **Product Cards**: Display products or items
- **Settings Panels**: Organize settings in card layout
- **Loading States**: Show skeleton loading in cards

## Differences from Ant Design 6.0.0

- ✅ All basic card features supported
- ✅ Title and extra content
- ✅ Loading state with skeleton
- ✅ Hover effects
- ✅ Size variants
- ⚠️ Card actions (footer buttons) not yet implemented
- ⚠️ Some advanced styling options may differ

