# Collapse

## Overview

The Collapse component displays collapsible content panels. It supports accordion mode, custom icons, and various styling options.

## API Reference

### CollapseProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<CollapsePanel>` | - | Panel items to display (required) |
| `active_key` | `Option<Vec<String>>` | `None` | Controlled active keys (expanded panels) |
| `default_active_key` | `Option<Vec<String>>` | `None` | Default active keys in uncontrolled mode |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | Called when active keys change |
| `accordion` | `bool` | `false` | Accordion mode (only one panel expanded) |
| `bordered` | `bool` | `true` | Whether to show border |
| `ghost` | `bool` | `false` | Ghost mode (transparent background) |
| `size` | `Option<CollapseSize>` | `None` | Size variant |
| `expand_icon_placement` | `ExpandIconPlacement` | `ExpandIconPlacement::Start` | Expand icon placement |
| `collapsible` | `Option<CollapsibleType>` | `None` | Default collapsible type for all panels |
| `destroy_on_hidden` | `bool` | `true` | Destroy inactive panel content |
| `expand_icon` | `Option<ExpandIconRenderFn>` | `None` | Custom expand icon render function |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `class_names` | `Option<CollapseClassNames>` | `None` | Semantic class names |
| `styles` | `Option<CollapseStyles>` | `None` | Semantic styles |

### CollapsePanel

| Field | Type | Description |
|-------|------|-------------|
| `key` | `String` | Unique key for the panel |
| `header` | `Element` | Panel header content |
| `content` | `Element` | Panel content |
| `disabled` | `bool` | Whether panel is disabled |
| `show_arrow` | `bool` | Whether to show expand arrow |
| `collapsible` | `Option<CollapsibleType>` | Collapsible type for this panel |
| `extra` | `Option<Element>` | Extra content in header |

### CollapseSize

- `Small` - Small size
- `Middle` - Middle size (default)
- `Large` - Large size

### CollapsibleType

- `Header` - Trigger by clicking header
- `Icon` - Trigger by clicking icon only
- `Disabled` - Disabled, cannot be triggered

### ExpandIconPlacement

- `Start` - Icon at start (default)
- `End` - Icon at end

## Usage Examples

### Basic Collapse

```rust
use adui_dioxus::{Collapse, CollapsePanel};

rsx! {
    Collapse {
        items: vec![
            CollapsePanel::new("1", rsx!("Panel 1"), rsx!("Content 1")),
            CollapsePanel::new("2", rsx!("Panel 2"), rsx!("Content 2")),
            CollapsePanel::new("3", rsx!("Panel 3"), rsx!("Content 3")),
        ],
    }
}
```

### Accordion Mode

```rust
use adui_dioxus::{Collapse, CollapsePanel};

rsx! {
    Collapse {
        accordion: true,
        items: vec![
            CollapsePanel::new("1", rsx!("Panel 1"), rsx!("Content 1")),
            CollapsePanel::new("2", rsx!("Panel 2"), rsx!("Content 2")),
        ],
    }
}
```

### Ghost Mode

```rust
use adui_dioxus::{Collapse, CollapsePanel};

rsx! {
    Collapse {
        ghost: true,
        items: vec![
            CollapsePanel::new("1", rsx!("Panel 1"), rsx!("Content 1")),
        ],
    }
}
```

### With Extra Content

```rust
use adui_dioxus::{Collapse, CollapsePanel, Button};

rsx! {
    Collapse {
        items: vec![
            CollapsePanel::new("1", rsx!("Panel 1"), rsx!("Content 1"))
                .extra(rsx!(Button { "Action" })),
        ],
    }
}
```

## Use Cases

- **FAQ Sections**: Display frequently asked questions
- **Settings Panels**: Organize settings into collapsible sections
- **Content Organization**: Organize content into collapsible sections
- **Accordions**: Create accordion-style interfaces

## Differences from Ant Design 6.0.0

- ✅ Basic collapse functionality
- ✅ Accordion mode
- ✅ Ghost mode
- ✅ Custom icons
- ✅ Size variants
- ⚠️ Some advanced features may differ

