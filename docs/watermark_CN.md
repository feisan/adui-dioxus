# Watermark 水印

## 概述

Watermark 组件在其子元素上添加水印层。它支持文本和图片水印，可自定义外观。

## API 参考

### WatermarkProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `z_index` | `i32` | `9` | 水印层的 z-index |
| `rotate` | `f32` | `-22.0` | 旋转角度（度） |
| `width` | `Option<f32>` | `None` | 水印宽度（如果未提供则自动计算） |
| `height` | `Option<f32>` | `None` | 水印高度（如果未提供则自动计算） |
| `image` | `Option<String>` | `None` | 图片水印的图片 URL（优先于 content） |
| `content` | `Option<Vec<String>>` | `None` | 水印的文本内容（可以是多行） |
| `font` | `Option<WatermarkFont>` | `None` | 文本水印的字体配置 |
| `gap` | `Option<[f32; 2]>` | `None` | 水印之间的间距，格式为 `[水平, 垂直]`（默认为 `[100, 100]`） |
| `offset` | `Option<[f32; 2]>` | `None` | 水印距离左上角的偏移量，格式为 `[左, 上]` |
| `class` | `Option<String>` | `None` | 包装器的额外类名 |
| `root_class` | `Option<String>` | `None` | 根元素的额外类名 |
| `style` | `Option<String>` | `None` | 包装器的内联样式 |
| `inherit` | `bool` | `true` | 嵌套水印上下文是否应继承此水印 |
| `children` | `Element` | - | 要添加水印的内容（必需） |

### WatermarkFont

| 字段 | 类型 | 说明 |
|------|------|------|
| `color` | `String` | 字体颜色（默认为 `rgba(0, 0, 0, 0.15)`） |
| `font_size` | `f32` | 字体大小（像素，默认为 16） |
| `font_weight` | `String` | 字体粗细（例如 "normal"、"bold" 或数字如 400、700） |
| `font_style` | `String` | 字体样式（例如 "normal"、"italic"） |
| `font_family` | `String` | 字体族（默认为 "sans-serif"） |
| `text_align` | `String` | 文本对齐（默认为 "center"） |

## 使用示例

### 基础文本水印

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        content: Some(vec!["机密".to_string()]),
        div {
            "受保护的内容"
        }
    }
}
```

### 多行文本水印

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        content: Some(vec![
            "机密".to_string(),
            "禁止分发".to_string(),
        ]),
        div {
            "受保护的内容"
        }
    }
}
```

### 图片水印

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        image: Some("https://example.com/logo.png".to_string()),
        div {
            "受保护的内容"
        }
    }
}
```

### 自定义字体

```rust
use adui_dioxus::{Watermark, WatermarkFont};

rsx! {
    Watermark {
        content: Some(vec!["机密".to_string()]),
        font: Some(WatermarkFont {
            color: "rgba(255, 0, 0, 0.3)".to_string(),
            font_size: 20.0,
            font_weight: "bold".to_string(),
            font_style: "normal".to_string(),
            font_family: "Arial".to_string(),
            text_align: "center".to_string(),
        }),
        div {
            "受保护的内容"
        }
    }
}
```

### 自定义间距和偏移量

```rust
use adui_dioxus::Watermark;

rsx! {
    Watermark {
        content: Some(vec!["机密".to_string()]),
        gap: Some([150.0, 150.0]),
        offset: Some([50.0, 50.0]),
        div {
            "受保护的内容"
        }
    }
}
```

## 使用场景

- **文档保护**：为文档添加水印以保护文档
- **版权**：显示版权信息
- **品牌**：为内容添加品牌水印
- **机密内容**：标记机密内容

## 与 Ant Design 6.0.0 的差异

- ✅ 文本和图片水印
- ✅ 自定义字体
- ✅ 自定义间距和偏移量
- ✅ 嵌套水印支持
- ⚠️ 某些高级功能可能有所不同

