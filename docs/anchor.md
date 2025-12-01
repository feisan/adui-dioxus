# Anchor

## Overview

The Anchor component provides navigation links for scrolling within a page. It automatically highlights the active section based on scroll position.

## API Reference

### AnchorProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<AnchorLinkItem>` | - | List of anchor link items (required) |
| `affix` | `bool` | `true` | Whether to use Affix to fix anchor when scrolling |
| `offset_top` | `Option<f64>` | `None` | Offset from top when affixed (pixels) |
| `bounds` | `f64` | `5.0` | Bounding distance when calculating active state |
| `target_offset` | `Option<f64>` | `None` | Scroll offset when clicking anchor |
| `direction` | `AnchorDirection` | `AnchorDirection::Vertical` | Direction of anchor navigation |
| `replace` | `bool` | `false` | Whether to replace current history entry |
| `show_ink_in_fixed` | `bool` | `false` | Show ink indicator in fixed mode |
| `on_change` | `Option<EventHandler<String>>` | `None` | Called when active link changes |
| `on_click` | `Option<EventHandler<AnchorClickInfo>>` | `None` | Called when anchor link is clicked |
| `get_current_anchor` | `Option<fn(String) -> String>` | `None` | Custom function to determine active anchor |
| `class` | `Option<String>` | `None` | Additional CSS class |
| `style` | `Option<String>` | `None` | Additional inline styles |

### AnchorLinkItem

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the item |
| `href` | `String` | Target anchor href (e.g., "#section-1") |
| `title` | `String` | Display title for the link |
| `target` | `Option<String>` | Optional target attribute |
| `children` | `Option<Vec<AnchorLinkItem>>` | Nested child items (vertical only) |

### AnchorDirection

- `Vertical` - Vertical anchor navigation (default)
- `Horizontal` - Horizontal anchor navigation

## Usage Examples

### Basic Anchor

```rust
use adui_dioxus::{Anchor, AnchorLinkItem};

rsx! {
    Anchor {
        items: vec![
            AnchorLinkItem::new("1", "#section-1", "Section 1"),
            AnchorLinkItem::new("2", "#section-2", "Section 2"),
            AnchorLinkItem::new("3", "#section-3", "Section 3"),
        ],
    }
}
```

### With Nested Items

```rust
use adui_dioxus::{Anchor, AnchorLinkItem};

rsx! {
    Anchor {
        items: vec![
            AnchorLinkItem::with_children(
                "1",
                "#section-1",
                "Section 1",
                vec![
                    AnchorLinkItem::new("1-1", "#section-1-1", "Subsection 1.1"),
                    AnchorLinkItem::new("1-2", "#section-1-2", "Subsection 1.2"),
                ],
            ),
        ],
    }
}
```

### Custom Offset

```rust
use adui_dioxus::{Anchor, AnchorLinkItem};

rsx! {
    Anchor {
        offset_top: Some(100.0),
        items: vec![
            AnchorLinkItem::new("1", "#section-1", "Section 1"),
        ],
    }
}
```

### Horizontal Anchor

```rust
use adui_dioxus::{Anchor, AnchorDirection, AnchorLinkItem};

rsx! {
    Anchor {
        direction: AnchorDirection::Horizontal,
        items: vec![
            AnchorLinkItem::new("1", "#section-1", "Section 1"),
            AnchorLinkItem::new("2", "#section-2", "Section 2"),
        ],
    }
}
```

## Use Cases

- **Long Pages**: Navigate through long pages
- **Documentation**: Table of contents for documentation
- **Single Page Apps**: Navigate within single page applications
- **Sections**: Jump to different sections of a page

## Differences from Ant Design 6.0.0

- ✅ Vertical and horizontal directions
- ✅ Nested anchor links
- ✅ Auto-highlighting based on scroll
- ✅ Affix support
- ⚠️ Some advanced features may differ

