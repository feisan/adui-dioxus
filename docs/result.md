# Result

## Overview

The Result component displays the result of an operation, such as success, error, or other status states. It's commonly used for empty states, error pages, and success confirmations.

## API Reference

### ResultProps

| Prop        | Type                   | Default                     | Description                                   |
| ----------- | ---------------------- | --------------------------- | --------------------------------------------- |
| `status`    | `Option<ResultStatus>` | `None` (defaults to `Info`) | Overall status of the result                  |
| `icon`      | `Option<Element>`      | `None`                      | Optional custom icon                          |
| `title`     | `Option<Element>`      | `None`                      | Title of the result page                      |
| `sub_title` | `Option<Element>`      | `None`                      | Optional subtitle/description text            |
| `extra`     | `Option<Element>`      | `None`                      | Extra action area, typically buttons          |
| `class`     | `Option<String>`       | `None`                      | Extra class on root element                   |
| `style`     | `Option<String>`       | `None`                      | Inline style on root element                  |
| `children`  | `Option<Element>`      | `None`                      | Optional content section rendered below extra |

### ResultStatus

- `Success` - Success status
- `Info` - Information status (default)
- `Warning` - Warning status
- `Error` - Error status
- `NotFound` - 404 not found
- `Forbidden` - 403 forbidden
- `ServerError` - 500 server error

## Usage Examples

### Success Result

```rust
use adui_dioxus::{Result, ResultStatus, Button, ButtonType};

rsx! {
    Result {
        status: Some(ResultStatus::Success),
        title: Some(rsx!("Operation Successful")),
        sub_title: Some(rsx!("Your operation has been completed successfully.")),
        extra: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "Go Home"
            }
        }),
    }
}
```

### Error Result

```rust
use adui_dioxus::{Result, ResultStatus};

rsx! {
    Result {
        status: Some(ResultStatus::Error),
        title: Some(rsx!("Submission Failed")),
        sub_title: Some(rsx!("Please check and modify the following information before resubmitting.")),
    }
}
```

### 404 Not Found

```rust
use adui_dioxus::{Result, ResultStatus, Button, ButtonType};

rsx! {
    Result {
        status: Some(ResultStatus::NotFound),
        title: Some(rsx!("404")),
        sub_title: Some(rsx!("Sorry, the page you visited does not exist.")),
        extra: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "Back Home"
            }
        }),
    }
}
```

## Use Cases

- **Success Pages**: Confirm successful operations
- **Error Pages**: Display error states (404, 403, 500)
- **Empty States**: Show when no data is available
- **Operation Results**: Display operation outcomes

## Differences from Ant Design 6.0.0

- ✅ All basic result statuses supported
- ✅ Custom icons
- ✅ Title and subtitle
- ✅ Extra action area
- ⚠️ Some advanced styling options may differ

