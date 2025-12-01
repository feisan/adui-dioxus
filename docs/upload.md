# Upload

## Overview

The Upload component provides file upload functionality with support for drag-and-drop, multiple files, progress tracking, and various list display types.

## API Reference

### UploadProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `action` | `Option<String>` | `None` | Upload action URL |
| `action_fn` | `Option<Rc<dyn Fn(&UploadFileMeta) -> String>>` | `None` | Upload action function |
| `directory` | `bool` | `false` | Upload directory instead of files |
| `multiple` | `bool` | `false` | Allow multiple file selection |
| `disabled` | `bool` | `false` | Disable interactions |
| `list_type` | `UploadListType` | `UploadListType::Text` | List display type |
| `field_name` | `Option<String>` | `None` | Field name for form data |
| `method` | `UploadHttpMethod` | `UploadHttpMethod::Post` | HTTP method |
| `with_credentials` | `bool` | `false` | Include credentials in request |
| `headers` | `Option<Vec<(String, String)>>` | `None` | Request headers |
| `data` | `Option<HashMap<String, String>>` | `None` | Additional form data |
| `data_fn` | `Option<Rc<dyn Fn(&UploadFile) -> HashMap<String, String>>>` | `None` | Data function |
| `accept` | `Option<AcceptConfig>` | `None` | File type filter |
| `before_upload` | `Option<BeforeUploadFn>` | `None` | Function called before upload |
| `file_list` | `Option<Vec<UploadFile>>` | `None` | Controlled file list |
| `default_file_list` | `Option<Vec<UploadFile>>` | `None` | Default file list |
| `max_count` | `Option<usize>` | `None` | Maximum number of files |
| `progress` | `Option<UploadProgressConfig>` | `None` | Progress configuration |
| `on_change` | `Option<EventHandler<UploadChangeInfo>>` | `None` | Called when file list changes |
| `on_preview` | `Option<EventHandler<UploadFile>>` | `None` | Called when file is previewed |
| `on_remove` | `Option<EventHandler<UploadFile>>` | `None` | Called when file is removed |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Upload trigger (required) |

### UploadListType

- `Text` - Text list (default)
- `Picture` - Picture list
- `PictureCard` - Picture card list

### UploadStatus

- `Ready` - Ready to upload
- `Uploading` - Currently uploading
- `Done` - Upload completed
- `Error` - Upload failed

### UploadHttpMethod

- `Post` - POST method (default)
- `Put` - PUT method
- `Patch` - PATCH method

## Usage Examples

### Basic Upload

```rust
use adui_dioxus::{Upload, Button, ButtonType};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        Button {
            r#type: ButtonType::Primary,
            "Upload File"
        }
    }
}
```

### Multiple Files

```rust
use adui_dioxus::{Upload, Button, ButtonType};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        multiple: true,
        Button {
            r#type: ButtonType::Primary,
            "Upload Files"
        }
    }
}
```

### Drag and Drop

```rust
use adui_dioxus::{Upload, UploadDragger};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        UploadDragger {
            "Drag and drop files here"
        }
    }
}
```

### Picture List

```rust
use adui_dioxus::{Upload, UploadListType, Button, ButtonType};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        list_type: UploadListType::Picture,
        Button {
            r#type: ButtonType::Primary,
            "Upload Image"
        }
    }
}
```

### With Before Upload

```rust
use adui_dioxus::{Upload, Button, ButtonType};
use std::rc::Rc;

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        before_upload: Some(Rc::new(|meta| {
            // Validate file before upload
            meta.size.unwrap_or(0) < 5 * 1024 * 1024 // 5MB limit
        })),
        Button {
            r#type: ButtonType::Primary,
            "Upload File"
        }
    }
}
```

## Use Cases

- **File Upload**: Upload files to server
- **Image Upload**: Upload images with preview
- **Document Upload**: Upload documents
- **Bulk Upload**: Upload multiple files at once

## Differences from Ant Design 6.0.0

- ✅ Basic upload functionality
- ✅ Multiple file support
- ✅ Drag and drop
- ✅ Progress tracking
- ✅ List display types
- ⚠️ Some advanced features may differ

