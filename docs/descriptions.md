# Descriptions

## Overview

The Descriptions component displays key-value pairs in a structured format. It's commonly used for displaying detailed information about an object or entity.

## API Reference

### DescriptionsProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<DescriptionsItem>` | - | Description items to display (required) |
| `title` | `Option<Element>` | `None` | Optional title for descriptions |
| `extra` | `Option<Element>` | `None` | Optional extra content in header |
| `bordered` | `bool` | `false` | Whether to show border |
| `layout` | `DescriptionsLayout` | `DescriptionsLayout::Horizontal` | Layout direction |
| `column` | `ColumnConfig` | `ColumnConfig::Simple(3)` | Column configuration |
| `size` | `Option<DescriptionsSize>` | `None` | Size variant |
| `colon` | `bool` | `true` | Whether to show colon after label |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |

### DescriptionsItem

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the item |
| `label` | `Element` | Label element |
| `content` | `Element` | Content element |
| `span` | `usize` | How many columns this item spans |

### DescriptionsLayout

- `Horizontal` - Horizontal layout (default)
- `Vertical` - Vertical layout

### DescriptionsSize

- `Small` - Small size
- `Middle` - Middle size (default)
- `Large` - Large size

### ColumnConfig

- `Simple(usize)` - Simple column count
- `Responsive(ResponsiveColumn)` - Responsive column configuration

## Usage Examples

### Basic Descriptions

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        items: vec![
            DescriptionsItem::new("name", rsx!("Name"), rsx!("John Doe")),
            DescriptionsItem::new("email", rsx!("Email"), rsx!("john@example.com")),
            DescriptionsItem::new("age", rsx!("Age"), rsx!("30")),
        ],
    }
}
```

### Bordered Descriptions

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        bordered: true,
        items: vec![
            DescriptionsItem::new("name", rsx!("Name"), rsx!("John Doe")),
            DescriptionsItem::new("email", rsx!("Email"), rsx!("john@example.com")),
        ],
    }
}
```

### With Title

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        title: Some(rsx!("User Information")),
        items: vec![
            DescriptionsItem::new("name", rsx!("Name"), rsx!("John Doe")),
            DescriptionsItem::new("email", rsx!("Email"), rsx!("john@example.com")),
        ],
    }
}
```

### Custom Column Span

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        items: vec![
            DescriptionsItem::new("name", rsx!("Name"), rsx!("John Doe")).span(2),
            DescriptionsItem::new("email", rsx!("Email"), rsx!("john@example.com")),
        ],
    }
}
```

## Use Cases

- **Detail Views**: Display detailed information about objects
- **User Profiles**: Show user profile information
- **Product Details**: Display product specifications
- **Form Reviews**: Review form data before submission

## Differences from Ant Design 6.0.0

- ✅ Basic descriptions functionality
- ✅ Bordered and borderless modes
- ✅ Horizontal and vertical layouts
- ✅ Responsive columns
- ✅ Custom column spans
- ⚠️ Some advanced features may differ

