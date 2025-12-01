# ConfigProvider

## Overview

The ConfigProvider component provides global configuration for all components in its subtree. It wraps ThemeProvider and allows setting size, disabled state, prefix class, and locale.

## API Reference

### ConfigProviderProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `size` | `Option<ComponentSize>` | `None` | Global default size for components |
| `disabled` | `Option<bool>` | `None` | Global disabled flag (when true, interactive components are disabled unless explicitly overridden) |
| `prefix_cls` | `Option<String>` | `None` | Global CSS class name prefix (defaults to "adui") |
| `locale` | `Option<Locale>` | `None` | Global locale flag for basic UI language |
| `theme` | `Option<Theme>` | `None` | Optional initial theme |
| `children` | `Element` | - | Child components (required) |

### ComponentSize

- `Small` - Small size
- `Middle` - Middle size (default)
- `Large` - Large size

### Locale

- `ZhCN` - Simplified Chinese (default)
- `EnUS` - English (US)

## Usage Examples

### Basic ConfigProvider

```rust
use adui_dioxus::{ConfigProvider, ComponentSize};

rsx! {
    ConfigProvider {
        size: Some(ComponentSize::Large),
        Button {
            "Large Button"
        }
    }
}
```

### Global Disabled

```rust
use adui_dioxus::ConfigProvider;

rsx! {
    ConfigProvider {
        disabled: Some(true),
        Button {
            "Disabled Button"
        }
    }
}
```

### Custom Prefix

```rust
use adui_dioxus::ConfigProvider;

rsx! {
    ConfigProvider {
        prefix_cls: Some("custom".to_string()),
        Button {
            "Custom Prefix"
        }
    }
}
```

### With Theme

```rust
use adui_dioxus::{ConfigProvider, Theme};

rsx! {
    ConfigProvider {
        theme: Some(Theme::Dark),
        div {
            "Dark theme content"
        }
    }
}
```

### Nested ConfigProviders

```rust
use adui_dioxus::{ConfigProvider, ComponentSize};

rsx! {
    ConfigProvider {
        size: Some(ComponentSize::Middle),
        div {
            "Middle size"
            ConfigProvider {
                size: Some(ComponentSize::Small),
                Button {
                    "Small Button"
                }
            }
        }
    }
}
```

## Use Cases

- **Global Configuration**: Set global defaults for all components
- **Theme Management**: Provide theme context
- **Size Control**: Control component sizes globally
- **Localization**: Set locale for components

## Differences from Ant Design 6.0.0

- ✅ Global size configuration
- ✅ Global disabled state
- ✅ Custom prefix class
- ✅ Locale support
- ✅ Theme integration
- ⚠️ Simplified compared to Ant Design's full ConfigProvider
- ⚠️ Some advanced features may differ

