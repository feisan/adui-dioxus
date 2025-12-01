# Image 图片

## 概述

Image 组件显示图片，支持加载状态、回退图片和带有缩放和导航功能的交互式预览模态框。

## API 参考

### ImageProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `src` | `String` | - | 图片源 URL（必需） |
| `alt` | `Option<String>` | `None` | 图片的替代文本 |
| `width` | `Option<String>` | `None` | 图片宽度 |
| `height` | `Option<String>` | `None` | 图片高度 |
| `fallback` | `Option<String>` | `None` | 主图片失败时的回退图片源 |
| `placeholder` | `Option<Element>` | `None` | 加载时显示的占位符元素 |
| `preview` | `bool` | `true` | 是否启用点击预览 |
| `preview_config` | `Option<PreviewConfig>` | `None` | 预览配置 |
| `on_load` | `Option<EventHandler<()>>` | `None` | 图片成功加载时调用 |
| `on_error` | `Option<EventHandler<()>>` | `None` | 图片加载失败时调用 |
| `class` | `Option<String>` | `None` | 根元素的额外类 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |
| `image_class` | `Option<String>` | `None` | 图片元素的额外类 |
| `image_style` | `Option<String>` | `None` | 图片元素的内联样式 |

### PreviewConfig

| 字段 | 类型 | 说明 |
|------|------|------|
| `visible` | `bool` | 是否启用预览 |
| `mask` | `Option<String>` | 悬停时显示的遮罩元素或文本 |
| `close_icon` | `Option<Element>` | 自定义关闭图标 |
| `scale` | `f32` | 预览的初始缩放 |
| `min_scale` | `f32` | 最小缩放 |
| `max_scale` | `f32` | 最大缩放 |

### ImageStatus

- `Loading` - 图片正在加载（默认）
- `Loaded` - 图片成功加载
- `Error` - 图片加载失败

## 使用示例

### 基础图片

```rust
use adui_dioxus::Image;

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        alt: Some("示例图片".to_string()),
    }
}
```

### 带回退

```rust
use adui_dioxus::Image;

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        fallback: Some("https://example.com/fallback.jpg".to_string()),
    }
}
```

### 带占位符

```rust
use adui_dioxus::{Image, Spin};

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        placeholder: Some(rsx!(Spin {})),
    }
}
```

### 禁用预览

```rust
use adui_dioxus::Image;

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        preview: false,
    }
}
```

### 自定义预览配置

```rust
use adui_dioxus::{Image, PreviewConfig};

rsx! {
    Image {
        src: "https://example.com/image.jpg".to_string(),
        preview_config: Some(PreviewConfig::new()
            .with_mask("点击预览")
            .scale(1.5)
            .min_scale(0.5)
            .max_scale(4.0)),
    }
}
```

## 使用场景

- **图片画廊**：在画廊中显示图片
- **产品图片**：显示带预览的产品图片
- **用户头像**：显示带回退的用户头像
- **内容图片**：显示带加载状态的内容图片

## 与 Ant Design 6.0.0 的差异

- ✅ 加载状态
- ✅ 回退图片
- ✅ 带缩放的预览模态框
- ✅ 自定义预览配置
- ⚠️ 某些高级功能可能有所不同

