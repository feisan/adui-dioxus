# Layout

## Overview

The Layout component provides a page layout container with Header, Footer, Sider, and Content sections. It's commonly used for structuring page layouts in applications.

## API Reference

### LayoutProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `has_sider` | `Option<bool>` | `None` | Whether layout contains Sider (auto-detected if None) |
| `children` | `Element` | - | Layout children (required) |

### SiderProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `width` | `Option<f32>` | `None` | Sider width in pixels |
| `collapsed_width` | `Option<f32>` | `None` | Width when collapsed |
| `collapsed` | `Option<bool>` | `None` | Controlled collapsed state |
| `default_collapsed` | `bool` | `false` | Initial collapsed state |
| `collapsible` | `bool` | `false` | Whether sider can be collapsed |
| `reverse_arrow` | `bool` | `false` | Reverse arrow direction |
| `trigger` | `Option<Element>` | `None` | Custom trigger element |
| `zero_width_trigger_style` | `Option<String>` | `None` | Style for zero-width trigger |
| `theme` | `SiderTheme` | `SiderTheme::Dark` | Sider theme |
| `has_border` | `bool` | `true` | Whether to show border |
| `on_collapse` | `Option<EventHandler<bool>>` | `None` | Called when collapse state changes |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Sider content (required) |

### SiderTheme

- `Light` - Light theme
- `Dark` - Dark theme (default)

## Usage Examples

### Basic Layout

```rust
use adui_dioxus::{Layout, Header, Content, Footer};

rsx! {
    Layout {
        Header {
            "Header"
        }
        Content {
            "Content"
        }
        Footer {
            "Footer"
        }
    }
}
```

### With Sider

```rust
use adui_dioxus::{Layout, Header, Sider, Content, Footer};

rsx! {
    Layout {
        has_sider: Some(true),
        Sider {
            width: Some(200.0),
            "Sider"
        }
        Layout {
            Header { "Header" }
            Content { "Content" }
            Footer { "Footer" }
        }
    }
}
```

### Collapsible Sider

```rust
use adui_dioxus::{Layout, Sider, Content};
use dioxus::prelude::*;

let collapsed = use_signal(|| false);

rsx! {
    Layout {
        Sider {
            collapsed: Some(*collapsed.read()),
            collapsible: true,
            on_collapse: Some(move |is_collapsed| {
                collapsed.set(is_collapsed);
            }),
            "Sider Content"
        }
        Content {
            "Main Content"
        }
    }
}
```

## Use Cases

- **Page Structure**: Structure page layouts
- **Admin Dashboards**: Create admin dashboard layouts
- **Side Navigation**: Add side navigation to pages
- **Responsive Layouts**: Create responsive page layouts

## Differences from Ant Design 6.0.0

- ✅ Basic layout components (Header, Footer, Content, Sider)
- ✅ Collapsible sider
- ✅ Sider themes
- ⚠️ Some advanced features may differ
- ⚠️ Responsive breakpoints may differ

