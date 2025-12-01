# Empty 空状态

## 概述

Empty 组件在没有数据可显示时显示空状态。它通常用于列表、表格和其他数据展示组件，在内容不可用时向用户提供反馈。

## API 参考

### EmptyProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `description` | `Option<String>` | `None` | 图片下方显示的可选描述文本（默认为"暂无数据"） |
| `image` | `Option<EmptyImage>` | `None` | 图片预设或自定义图片 |
| `class` | `Option<String>` | `None` | 应用于根元素的额外类名 |
| `style` | `Option<String>` | `None` | 应用于根元素的内联样式 |
| `footer` | `Option<Element>` | `None` | 描述下方渲染的可选页脚内容（例如操作按钮） |

### EmptyImage

- `Default` - 默认插图（SVG）
- `Simple` - 简洁插图
- `Small` - 较小尺寸变体
- `Custom(String)` - 自定义图片 URL

## 使用示例

### 基础用法

```rust
use adui_dioxus::Empty;

rsx! {
    Empty {}
}
```

### 自定义描述

```rust
use adui_dioxus::Empty;

rsx! {
    Empty {
        description: Some("暂无数据".to_string()),
    }
}
```

### 自定义图片

```rust
use adui_dioxus::{Empty, EmptyImage};

rsx! {
    Empty {
        image: Some(EmptyImage::Custom("https://example.com/empty.png".to_string())),
        description: Some("自定义空状态".to_string()),
    }
}
```

### 带页脚操作

```rust
use adui_dioxus::{Empty, Button, ButtonType};

rsx! {
    Empty {
        description: Some("未找到项目".to_string()),
        footer: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "创建项目"
            }
        }),
    }
}
```

## 使用场景

- **空列表**：列表没有项目时显示
- **空表格**：表格没有数据时显示
- **空搜索结果**：未找到搜索结果时显示
- **空状态**：任何数据容器的通用空状态

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有基础空状态图片预设
- ✅ 自定义图片支持
- ✅ 页脚内容支持
- ✅ 自定义描述支持
- ⚠️ 某些高级样式选项可能有所不同

