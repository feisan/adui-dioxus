# FloatButton

## Overview

The FloatButton component displays a floating action button that stays fixed on the page. It supports primary and default types, danger styling, and can be grouped.

## API Reference

### FloatButtonProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `type` | `FloatButtonType` | `FloatButtonType::Primary` | Visual type |
| `shape` | `FloatButtonShape` | `FloatButtonShape::Circle` | Button shape |
| `danger` | `bool` | `false` | Whether button uses danger styling |
| `href` | `Option<String>` | `None` | If provided, renders as link |
| `icon` | `Option<Element>` | `None` | Icon element |
| `description` | `Option<String>` | `None` | Description text shown on hover |
| `content` | `Option<String>` | `None` | Text content (alternative to icon) |
| `tooltip` | `Option<String>` | `None` | Tooltip text |
| `right` | `Option<f32>` | `None` | Right position (defaults to 24px) |
| `left` | `Option<f32>` | `None` | Left position |
| `top` | `Option<f32>` | `None` | Top position |
| `bottom` | `Option<f32>` | `None` | Bottom position (defaults to 72px) |
| `z_index` | `Option<i32>` | `None` | Z-index (defaults to 99) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | Click event handler |
| `children` | `Element` | - | Button content (required) |

### FloatButtonGroupProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `shape` | `FloatButtonShape` | `FloatButtonShape::Circle` | Shared shape for all buttons |
| `type` | `FloatButtonType` | `FloatButtonType::Primary` | Shared type for all buttons |
| `gap` | `f32` | `12.0` | Gap between buttons |
| `right` | `Option<f32>` | `None` | Right position |
| `left` | `Option<f32>` | `None` | Left position |
| `top` | `Option<f32>` | `None` | Top position |
| `bottom` | `Option<f32>` | `None` | Bottom position |
| `z_index` | `Option<i32>` | `None` | Z-index |
| `pure` | `bool` | `false` | Pure panel mode (no positioning) |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | FloatButton children (required) |

### BackTopProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `visibility_height` | `Option<f32>` | `None` | Scroll height to show button |
| `right` | `Option<f32>` | `None` | Right position |
| `bottom` | `Option<f32>` | `None` | Bottom position |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Button content (required) |

### FloatButtonType

- `Default` - Default type
- `Primary` - Primary type (default)

### FloatButtonShape

- `Circle` - Circular button (default)
- `Square` - Square button

## Usage Examples

### Basic FloatButton

```rust
use adui_dioxus::{FloatButton, Icon, IconKind};

rsx! {
    FloatButton {
        icon: Some(rsx!(Icon { kind: IconKind::Plus, size: 20.0 })),
    }
}
```

### With Description

```rust
use adui_dioxus::{FloatButton, Icon, IconKind};

rsx! {
    FloatButton {
        icon: Some(rsx!(Icon { kind: IconKind::Edit, size: 20.0 })),
        description: Some("Edit".to_string()),
    }
}
```

### FloatButton Group

```rust
use adui_dioxus::{FloatButton, FloatButtonGroup, Icon, IconKind};

rsx! {
    FloatButtonGroup {
        FloatButton {
            icon: Some(rsx!(Icon { kind: IconKind::Plus, size: 20.0 })),
        }
        FloatButton {
            icon: Some(rsx!(Icon { kind: IconKind::Edit, size: 20.0 })),
        }
    }
}
```

### BackTop

```rust
use adui_dioxus::BackTop;

rsx! {
    BackTop {
        visibility_height: Some(400.0),
    }
}
```

## Use Cases

- **Primary Actions**: Quick access to primary actions
- **Back to Top**: Scroll to top functionality
- **Action Groups**: Multiple floating actions
- **Quick Access**: Fast access to common actions

## Differences from Ant Design 6.0.0

- ✅ Basic float button functionality
- ✅ Button groups
- ✅ BackTop component
- ✅ Custom positioning
- ⚠️ Drag functionality not yet implemented
- ⚠️ Some advanced features may differ

