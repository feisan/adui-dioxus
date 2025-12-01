# Icon

## Overview

The Icon component provides a built-in set of SVG icons commonly used in UI interfaces. It supports rotation, sizing, custom colors, and spinning animations.

## API Reference

### IconProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `kind` | `IconKind` | `IconKind::Info` | Icon type from built-in set |
| `size` | `f32` | `20.0` | Icon size in pixels |
| `color` | `Option<String>` | `None` | Custom color (defaults to "currentColor") |
| `rotate` | `Option<f32>` | `None` | Rotation angle in degrees |
| `spin` | `bool` | `false` | Whether to animate spinning |
| `class` | `Option<String>` | `None` | Extra class name |
| `aria_label` | `Option<String>` | `None` | ARIA label for accessibility |
| `view_box` | `Option<String>` | `None` | Custom SVG viewBox |
| `custom` | `Option<Element>` | `None` | Custom SVG content (overrides `kind`) |

### IconKind

Built-in icon types:
- `Plus` - Plus icon
- `Minus` - Minus icon
- `Check` - Check mark
- `Close` - Close/X icon
- `Info` - Information icon (default)
- `Question` - Question mark
- `ArrowRight` - Right arrow
- `ArrowLeft` - Left arrow
- `ArrowUp` - Up arrow
- `ArrowDown` - Down arrow
- `Search` - Search icon
- `Copy` - Copy icon
- `Edit` - Edit icon
- `Loading` - Loading spinner (auto-spins)
- `Eye` - Eye icon
- `EyeInvisible` - Eye with slash icon

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::Search,
    }
}
```

### With Custom Size and Color

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::Check,
        size: 24.0,
        color: Some("#52c41a".to_string()),
    }
}
```

### Spinning Icon

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::Loading,
        spin: true,
    }
}
```

### Rotated Icon

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::ArrowRight,
        rotate: Some(45.0),
    }
}
```

### Custom SVG Icon

```rust
use adui_dioxus::Icon;

rsx! {
    Icon {
        custom: Some(rsx! {
            path { d: "M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z" }
        }),
        size: 24.0,
    }
}
```

## Use Cases

- **UI Indicators**: Show status, actions, or information
- **Buttons**: Add icons to buttons for better UX
- **Navigation**: Use arrow icons for navigation
- **Loading States**: Use loading spinner icon
- **Form Controls**: Icons for input fields, checkboxes, etc.

## Differences from Ant Design 6.0.0

- ✅ Built-in icon set with common icons
- ✅ Custom SVG support
- ✅ Rotation and spinning animations
- ✅ Size and color customization
- ⚠️ Limited built-in icon set compared to Ant Design's extensive icon library
- ⚠️ Icon font support not implemented (SVG only)

