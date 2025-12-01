# Upload 上传

## 概述

Upload 组件提供文件上传功能，支持拖放、多文件、进度跟踪和多种列表显示类型。

## API 参考

### UploadProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `action` | `Option<String>` | `None` | 上传动作 URL |
| `action_fn` | `Option<Rc<dyn Fn(&UploadFileMeta) -> String>>` | `None` | 上传动作函数 |
| `directory` | `bool` | `false` | 上传目录而不是文件 |
| `multiple` | `bool` | `false` | 允许多文件选择 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `list_type` | `UploadListType` | `UploadListType::Text` | 列表显示类型 |
| `field_name` | `Option<String>` | `None` | 表单数据的字段名 |
| `method` | `UploadHttpMethod` | `UploadHttpMethod::Post` | HTTP 方法 |
| `with_credentials` | `bool` | `false` | 请求中包含凭证 |
| `headers` | `Option<Vec<(String, String)>>` | `None` | 请求头 |
| `data` | `Option<HashMap<String, String>>` | `None` | 额外的表单数据 |
| `data_fn` | `Option<Rc<dyn Fn(&UploadFile) -> HashMap<String, String>>>` | `None` | 数据函数 |
| `accept` | `Option<AcceptConfig>` | `None` | 文件类型过滤器 |
| `before_upload` | `Option<BeforeUploadFn>` | `None` | 上传前调用的函数 |
| `file_list` | `Option<Vec<UploadFile>>` | `None` | 受控的文件列表 |
| `default_file_list` | `Option<Vec<UploadFile>>` | `None` | 默认文件列表 |
| `max_count` | `Option<usize>` | `None` | 最大文件数 |
| `progress` | `Option<UploadProgressConfig>` | `None` | 进度配置 |
| `on_change` | `Option<EventHandler<UploadChangeInfo>>` | `None` | 文件列表改变时调用 |
| `on_preview` | `Option<EventHandler<UploadFile>>` | `None` | 文件预览时调用 |
| `on_remove` | `Option<EventHandler<UploadFile>>` | `None` | 文件移除时调用 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 上传触发器（必需） |

### UploadListType

- `Text` - 文本列表（默认）
- `Picture` - 图片列表
- `PictureCard` - 图片卡片列表

### UploadStatus

- `Ready` - 准备上传
- `Uploading` - 正在上传
- `Done` - 上传完成
- `Error` - 上传失败

### UploadHttpMethod

- `Post` - POST 方法（默认）
- `Put` - PUT 方法
- `Patch` - PATCH 方法

## 使用示例

### 基础上传

```rust
use adui_dioxus::{Upload, Button, ButtonType};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        Button {
            r#type: ButtonType::Primary,
            "上传文件"
        }
    }
}
```

### 多文件

```rust
use adui_dioxus::{Upload, Button, ButtonType};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        multiple: true,
        Button {
            r#type: ButtonType::Primary,
            "上传多个文件"
        }
    }
}
```

### 拖放上传

```rust
use adui_dioxus::{Upload, UploadDragger};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        UploadDragger {
            "拖放文件到这里"
        }
    }
}
```

### 图片列表

```rust
use adui_dioxus::{Upload, UploadListType, Button, ButtonType};

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        list_type: UploadListType::Picture,
        Button {
            r#type: ButtonType::Primary,
            "上传图片"
        }
    }
}
```

### 带上传前验证

```rust
use adui_dioxus::{Upload, Button, ButtonType};
use std::rc::Rc;

rsx! {
    Upload {
        action: Some("https://example.com/upload".to_string()),
        before_upload: Some(Rc::new(|meta| {
            // 上传前验证文件
            meta.size.unwrap_or(0) < 5 * 1024 * 1024 // 5MB 限制
        })),
        Button {
            r#type: ButtonType::Primary,
            "上传文件"
        }
    }
}
```

## 使用场景

- **文件上传**：上传文件到服务器
- **图片上传**：上传带预览的图片
- **文档上传**：上传文档
- **批量上传**：一次上传多个文件

## 与 Ant Design 6.0.0 的差异

- ✅ 基础上传功能
- ✅ 多文件支持
- ✅ 拖放
- ✅ 进度跟踪
- ✅ 列表显示类型
- ⚠️ 某些高级功能可能有所不同

