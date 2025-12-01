# Button

## Overview

The Button component is a fundamental UI element that provides various styles, sizes, and states for user interactions. It supports multiple variants (solid, outlined, dashed, text, link), colors (primary, success, warning, danger, default), and can be used in groups with shared styling.

## API Reference

### ButtonProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `type` | `ButtonType` | `ButtonType::Default` | Visual type (legacy API, maps to variant/color) |
| `size` | `ButtonSize` | `ButtonSize::Middle` | Button size: `Small`, `Middle`, `Large` |
| `shape` | `ButtonShape` | `ButtonShape::Default` | Button shape: `Default`, `Round`, `Circle` |
| `danger` | `bool` | `false` | Whether the button is in danger state |
| `ghost` | `bool` | `false` | Whether the button is ghost (transparent background) |
| `block` | `bool` | `false` | Whether the button should take full width |
| `loading` | `bool` | `false` | Whether the button is in loading state |
| `loading_delay` | `Option<u64>` | `None` | Loading delay in milliseconds before showing spinner |
| `loading_icon` | `Option<Element>` | `None` | Custom loading icon element |
| `auto_insert_space` | `bool` | `true` | Whether to insert space between CJK characters |
| `label` | `Option<String>` | `None` | Optional label text for auto spacing/icon-only detection |
| `icon_only` | `Option<bool>` | `None` | Mark as icon-only button (adds class) |
| `disabled` | `bool` | `false` | Whether the button is disabled |
| `color` | `Option<ButtonColor>` | `None` | Button color: `Default`, `Primary`, `Success`, `Warning`, `Danger` |
| `variant` | `Option<ButtonVariant>` | `None` | Button variant: `Solid`, `Outlined`, `Dashed`, `Text`, `Link` |
| `icon_placement` | `ButtonIconPlacement` | `ButtonIconPlacement::Start` | Icon placement: `Start`, `End` |
| `icon_position` | `Option<ButtonIconPlacement>` | `None` | **Deprecated**: Use `icon_placement` instead |
| `icon` | `Option<Element>` | `None` | Icon element to display |
| `href` | `Option<String>` | `None` | If provided, renders as `<a>` tag instead of `<button>` |
| `class` | `Option<String>` | `None` | Extra class name for root element |
| `class_names_root` | `Option<String>` | `None` | Extra class applied to button element |
| `class_names_icon` | `Option<String>` | `None` | Extra class applied to icon span |
| `class_names_content` | `Option<String>` | `None` | Extra class applied to content span |
| `styles_root` | `Option<String>` | `None` | Extra inline style applied to root |
| `html_type` | `ButtonHtmlType` | `ButtonHtmlType::Button` | Native button type: `Button`, `Submit`, `Reset` |
| `data_attributes` | `Option<Vec<(String, String)>>` | `None` | Data attributes as key-value pairs (without "data-" prefix) |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | Click event handler |
| `children` | `Element` | - | Button content (required) |

### ButtonGroupProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `size` | `Option<ButtonSize>` | `None` | Shared size for all buttons in group |
| `shape` | `Option<ButtonShape>` | `None` | Shared shape for all buttons in group |
| `color` | `Option<ButtonColor>` | `None` | Shared color for all buttons in group |
| `variant` | `Option<ButtonVariant>` | `None` | Shared variant for all buttons in group |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Button children (required) |

### Enums

#### ButtonType
- `Default` - Default button style
- `Primary` - Primary button style
- `Dashed` - Dashed border button
- `Text` - Text button (no border)
- `Link` - Link-style button

#### ButtonColor
- `Default` - Default color
- `Primary` - Primary color
- `Success` - Success color
- `Warning` - Warning color
- `Danger` - Danger color

#### ButtonVariant
- `Solid` - Solid filled button
- `Outlined` - Outlined button (default)
- `Dashed` - Dashed border button
- `Text` - Text button
- `Link` - Link button

#### ButtonSize
- `Small` - Small size
- `Middle` - Middle size (default)
- `Large` - Large size

#### ButtonShape
- `Default` - Default shape
- `Round` - Rounded corners
- `Circle` - Circular button

#### ButtonIconPlacement
- `Start` - Icon before text (default)
- `End` - Icon after text

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Button, ButtonType, ButtonSize};

rsx! {
    Button {
        r#type: ButtonType::Primary,
        size: ButtonSize::Large,
        onclick: move |_| {
            println!("Button clicked!");
        },
        "Click Me"
    }
}
```

### Button Variants

```rust
use adui_dioxus::{Button, ButtonVariant, ButtonColor};

rsx! {
    div {
        Button {
            variant: Some(ButtonVariant::Solid),
            color: Some(ButtonColor::Primary),
            "Solid Primary"
        }
        Button {
            variant: Some(ButtonVariant::Outlined),
            color: Some(ButtonColor::Primary),
            "Outlined Primary"
        }
        Button {
            variant: Some(ButtonVariant::Text),
            "Text Button"
        }
        Button {
            variant: Some(ButtonVariant::Link),
            "Link Button"
        }
    }
}
```

### Button with Icon

```rust
use adui_dioxus::{Button, Icon, IconKind, ButtonIconPlacement};

rsx! {
    Button {
        icon: rsx!(Icon { kind: IconKind::Search, size: 16.0 }),
        icon_placement: ButtonIconPlacement::Start,
        "Search"
    }
}
```

### Loading State

```rust
use adui_dioxus::Button;

rsx! {
    Button {
        loading: true,
        loading_delay: Some(300),
        "Loading..."
    }
}
```

### Button Group

```rust
use adui_dioxus::{Button, ButtonGroup, ButtonSize, ButtonVariant, ButtonColor};

rsx! {
    ButtonGroup {
        size: Some(ButtonSize::Small),
        variant: Some(ButtonVariant::Solid),
        color: Some(ButtonColor::Primary),
        Button { "Previous" }
        Button { "Refresh" }
        Button { "Next" }
    }
}
```

### Block Button

```rust
use adui_dioxus::Button;

rsx! {
    Button {
        block: true,
        "Full Width Button"
    }
}
```

## Use Cases

- **Form Actions**: Submit, reset, or cancel buttons in forms
- **Navigation**: Link-style buttons for navigation
- **Actions**: Primary and secondary actions in modals, drawers, or pages
- **Toolbars**: Button groups for toolbars and action bars
- **Icon Buttons**: Icon-only buttons for compact interfaces

## Differences from Ant Design 6.0.0

### Type System Differences

1. **Loading Property**: In Ant Design TypeScript, `loading` can be a boolean or an object `{ delay?: number, icon?: ReactNode }`. In this Rust implementation, it's split into:
   - `loading: bool` - Loading state
   - `loading_delay: Option<u64>` - Delay in milliseconds
   - `loading_icon: Option<Element>` - Custom loading icon

2. **Props Structure**: The Rust implementation uses explicit enum types (`ButtonColor`, `ButtonVariant`) instead of string literals, providing better type safety.

3. **Icon Placement**: The deprecated `icon_position` prop is maintained for backward compatibility but maps to `icon_placement`.

### Features

- ✅ All basic button types and variants
- ✅ Button groups with shared styling
- ✅ Loading states with delay support
- ✅ Icon support with placement control
- ✅ Block and ghost modes
- ✅ Link rendering via `href` prop
- ⚠️ Ripple effects not yet implemented (may be added via custom animations)
- ⚠️ Some advanced styling features may differ slightly due to Rust/Dioxus constraints

