# Modal

## Overview

The Modal component displays a modal dialog for user interactions. It supports titles, custom footers, loading states, and various configuration options.

## API Reference

### ModalProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `bool` | - | Whether the modal is visible (required) |
| `title` | `Option<String>` | `None` | Optional title displayed in header |
| `footer` | `Option<Element>` | `None` | Custom footer content (None shows default OK/Cancel) |
| `footer_render` | `Option<Rc<dyn Fn(Element, FooterExtra) -> Element>>` | `None` | Custom footer render function |
| `show_footer` | `bool` | `true` | Whether to show footer |
| `on_ok` | `Option<EventHandler<()>>` | `None` | Called when user confirms |
| `on_cancel` | `Option<EventHandler<()>>` | `None` | Called when user cancels |
| `closable` | `bool` | `true` | Show close button in top-right |
| `closable_config` | `Option<ClosableConfig>` | `None` | Advanced closable configuration |
| `mask_closable` | `bool` | `true` | Whether clicking mask triggers on_cancel |
| `destroy_on_close` | `bool` | `false` | Remove content from tree when closed |
| `destroy_on_hidden` | `bool` | `false` | Remove content when hidden |
| `force_render` | `bool` | `false` | Force render even when not visible |
| `width` | `Option<f32>` | `None` | Fixed width in pixels |
| `width_responsive` | `Option<HashMap<String, f32>>` | `None` | Responsive width configuration |
| `centered` | `bool` | `false` | Whether to vertically center modal |
| `confirm_loading` | `bool` | `false` | Whether OK button is in loading state |
| `ok_text` | `Option<String>` | `None` | OK button text |
| `cancel_text` | `Option<String>` | `None` | Cancel button text |
| `ok_type` | `Option<ButtonType>` | `None` | OK button type |
| `keyboard` | `bool` | `true` | Enable keyboard (Escape to close) |
| `close_icon` | `Option<Element>` | `None` | Custom close icon |
| `after_close` | `Option<EventHandler<()>>` | `None` | Callback after modal closes |
| `after_open_change` | `Option<EventHandler<bool>>` | `None` | Callback when open state changes |
| `class` | `Option<String>` | `None` | Additional CSS class on root |
| `style` | `Option<String>` | `None` | Inline styles on root |
| `class_names` | `Option<ModalClassNames>` | `None` | Semantic class names |
| `styles` | `Option<ModalStyles>` | `None` | Semantic styles |
| `get_container` | `Option<String>` | `None` | Custom container selector |
| `z_index` | `Option<i32>` | `None` | Custom z-index |
| `mask` | `Option<MaskConfig>` | `None` | Mask configuration |
| `loading` | `bool` | `false` | Loading state for entire modal |
| `children` | `Element` | - | Modal content (required) |

### ModalType

- `Info` - Info modal
- `Success` - Success modal
- `Error` - Error modal
- `Warning` - Warning modal
- `Confirm` - Confirm modal

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::{Modal, Button, ButtonType};
use dioxus::prelude::*;

let is_open = use_signal(|| false);

rsx! {
    div {
        Button {
            onclick: move |_| {
                is_open.set(true);
            },
            "Open Modal"
        }
        Modal {
            open: *is_open.read(),
            title: Some("Modal Title".to_string()),
            on_cancel: Some(move |_| {
                is_open.set(false);
            }),
            on_ok: Some(move |_| {
                println!("OK clicked");
                is_open.set(false);
            }),
            "Modal content goes here"
        }
    }
}
```

### With Custom Footer

```rust
use adui_dioxus::{Modal, Button, ButtonType};

rsx! {
    Modal {
        open: true,
        title: Some("Custom Footer".to_string()),
        footer: Some(rsx! {
            Button { "Custom Button" }
        }),
        "Content"
    }
}
```

### Centered Modal

```rust
use adui_dioxus::Modal;

rsx! {
    Modal {
        open: true,
        title: Some("Centered Modal".to_string()),
        centered: true,
        "Centered content"
    }
}
```

### Loading State

```rust
use adui_dioxus::Modal;

rsx! {
    Modal {
        open: true,
        title: Some("Loading Modal".to_string()),
        confirm_loading: true,
        on_ok: Some(move |_| {
            // Async operation
        }),
        "Content"
    }
}
```

## Use Cases

- **Confirmations**: Confirm user actions
- **Forms**: Display forms in modals
- **Details**: Show detailed information
- **Alerts**: Display important alerts

## Differences from Ant Design 6.0.0

- ✅ Basic modal functionality
- ✅ Custom footers
- ✅ Loading states
- ✅ Centered positioning
- ✅ Mask configuration
- ⚠️ Some advanced features may differ
- ⚠️ Static method variants (Modal.info, etc.) accessed via use_modal hook

