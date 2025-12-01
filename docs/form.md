# Form

## Overview

The Form component provides form management with validation, layout control, and field coordination. It supports various layouts, validation rules, and integrates with form controls.

## API Reference

### FormProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `layout` | `FormLayout` | `FormLayout::Horizontal` | Form layout |
| `size` | `ControlSize` | `ControlSize::Middle` | Control size |
| `colon` | `bool` | `true` | Show colon after label |
| `required_mark` | `RequiredMark` | `RequiredMark::Default` | Required mark configuration |
| `label_align` | `LabelAlign` | `LabelAlign::Right` | Label alignment |
| `label_wrap` | `bool` | `false` | Whether to wrap label text |
| `label_col` | `Option<ColProps>` | `None` | Label column configuration |
| `wrapper_col` | `Option<ColProps>` | `None` | Wrapper column configuration |
| `disabled` | `bool` | `false` | Disable all form controls |
| `variant` | `Option<Variant>` | `None` | Visual variant |
| `scroll_to_first_error` | `bool` | `false` | Scroll to first error on submit |
| `scroll_to_first_error_config` | `Option<ScrollToFirstErrorConfig>` | `None` | Scroll configuration |
| `feedback_icons` | `Option<FeedbackIcons>` | `None` | Feedback icons configuration |
| `name` | `Option<String>` | `None` | Form name |
| `initial_values` | `Option<FormValues>` | `None` | Initial form values |
| `form` | `Option<FormHandle>` | `None` | Form handle instance |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `class_names` | `Option<FormClassNames>` | `None` | Semantic class names |
| `styles` | `Option<FormStyles>` | `None` | Semantic styles |
| `on_finish` | `Option<EventHandler<FormFinishEvent>>` | `None` | Called on successful submit |
| `on_finish_failed` | `Option<EventHandler<FormFinishFailedEvent>>` | `None` | Called on validation failure |
| `on_values_change` | `Option<EventHandler<ValuesChangeEvent>>` | `None` | Called when values change |
| `children` | `Element` | - | Form content (required) |

### FormItemProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `String` | - | Field name (required) |
| `label` | `Option<Element>` | `None` | Label element |
| `required` | `bool` | `false` | Whether field is required |
| `rules` | `Option<Vec<FormRule>>` | `None` | Validation rules |
| `value_prop_name` | `Option<String>` | `None` | Value prop name for custom controls |
| `get_value_from_event` | `Option<GetValueFromEventFn>` | `None` | Function to extract value from event |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Form control (required) |

### FormRule

| Field | Type | Description |
|-------|------|-------------|
| `required` | `bool` | Whether field is required |
| `min` | `Option<usize>` | Minimum length |
| `max` | `Option<usize>` | Maximum length |
| `len` | `Option<usize>` | Exact length |
| `pattern` | `Option<String>` | Regex pattern |
| `message` | `Option<String>` | Error message |
| `validator` | `Option<FormValidator>` | Custom validator function |

### FormLayout

- `Horizontal` - Horizontal layout (default)
- `Vertical` - Vertical layout
- `Inline` - Inline layout

### RequiredMark

- `None` - Hide required mark
- `Optional` - Show "optional" text
- `Default` - Show default required mark
- `Custom` - Custom render function

## Usage Examples

### Basic Form

```rust
use adui_dioxus::{Form, FormItem, Input, Button, ButtonType};
use dioxus::prelude::*;

rsx! {
    Form {
        on_finish: Some(move |event| {
            println!("Form values: {:?}", event.values);
        }),
        FormItem {
            name: "username".to_string(),
            label: Some(rsx!("Username")),
            required: true,
            Input {
                placeholder: Some("Enter username".to_string()),
            }
        }
        FormItem {
            name: "email".to_string(),
            label: Some(rsx!("Email")),
            required: true,
            Input {
                placeholder: Some("Enter email".to_string()),
            }
        }
        Button {
            r#type: ButtonType::Primary,
            html_type: ButtonHtmlType::Submit,
            "Submit"
        }
    }
}
```

### With Validation Rules

```rust
use adui_dioxus::{Form, FormItem, FormRule, Input};

rsx! {
    Form {
        FormItem {
            name: "email".to_string(),
            label: Some(rsx!("Email")),
            rules: Some(vec![
                FormRule {
                    required: true,
                    message: Some("Email is required".to_string()),
                    ..Default::default()
                },
                FormRule {
                    pattern: Some(r"^[^\s@]+@[^\s@]+\.[^\s@]+$".to_string()),
                    message: Some("Invalid email format".to_string()),
                    ..Default::default()
                },
            ]),
            Input {
                placeholder: Some("Enter email".to_string()),
            }
        }
    }
}
```

### Vertical Layout

```rust
use adui_dioxus::{Form, FormLayout, FormItem, Input};

rsx! {
    Form {
        layout: FormLayout::Vertical,
        FormItem {
            name: "username".to_string(),
            label: Some(rsx!("Username")),
            Input {}
        }
    }
}
```

### Using FormHandle

```rust
use adui_dioxus::{Form, FormItem, Input, Button, ButtonType, use_form};
use dioxus::prelude::*;

fn MyForm() -> Element {
    let form = use_form();
    
    rsx! {
        Form {
            form: Some(form.clone()),
            FormItem {
                name: "username".to_string(),
                label: Some(rsx!("Username")),
                Input {}
            }
            Button {
                onclick: move |_| {
                    let values = form.values();
                    println!("Form values: {:?}", values);
                },
                "Get Values"
            }
        }
    }
}
```

## Use Cases

- **User Registration**: Registration forms
- **Settings**: Settings forms
- **Data Entry**: Data entry forms
- **Search**: Search forms with validation

## Differences from Ant Design 6.0.0

- ✅ Form validation
- ✅ Multiple layouts
- ✅ Form handle API
- ✅ Custom validators
- ✅ Form list support
- ⚠️ Some advanced features may differ

