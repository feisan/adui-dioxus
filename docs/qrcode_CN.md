# QRCode 二维码

## 概述

QRCode 组件生成并显示二维码。它支持 SVG 和 Canvas 渲染、自定义图标、颜色和不同的状态。

## API 参考

### QRCodeProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `String` | - | 要编码的值/内容（必需） |
| `type` | `QRCodeType` | `QRCodeType::Svg` | 渲染类型 |
| `size` | `u32` | `160` | 二维码尺寸（像素） |
| `icon` | `Option<String>` | `None` | 在中心显示的图标 URL |
| `icon_size` | `Option<u32>` | `None` | 图标尺寸 |
| `color` | `Option<String>` | `None` | 前景色（默认为当前文本颜色） |
| `bg_color` | `Option<String>` | `None` | 背景色（默认为透明） |
| `error_level` | `QRCodeErrorLevel` | `QRCodeErrorLevel::M` | 纠错级别 |
| `status` | `QRCodeStatus` | `QRCodeStatus::Active` | 当前状态 |
| `bordered` | `bool` | `true` | 是否显示边框 |
| `on_refresh` | `Option<EventHandler<()>>` | `None` | 点击刷新时的回调 |
| `class` | `Option<String>` | `None` | 容器的额外类名 |
| `root_class` | `Option<String>` | `None` | 根的额外类名 |
| `style` | `Option<String>` | `None` | 容器的内联样式 |

### QRCodeType

- `Svg` - 渲染为 SVG（默认，更适合缩放）
- `Canvas` - 渲染为 Canvas（更适合大型二维码）

### QRCodeStatus

- `Active` - 活动且可扫描（默认）
- `Expired` - 已过期，需要刷新
- `Loading` - 加载状态
- `Scanned` - 已扫描

### QRCodeErrorLevel

- `L` - ~7% 纠错
- `M` - ~15% 纠错（默认）
- `Q` - ~25% 纠错
- `H` - ~30% 纠错

## 使用示例

### 基础二维码

```rust
use adui_dioxus::QRCode;

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
    }
}
```

### 带图标

```rust
use adui_dioxus::QRCode;

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
        icon: Some("https://example.com/logo.png".to_string()),
        icon_size: Some(40),
    }
}
```

### 自定义颜色

```rust
use adui_dioxus::QRCode;

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
        color: Some("#000000".to_string()),
        bg_color: Some("#ffffff".to_string()),
    }
}
```

### 带状态

```rust
use adui_dioxus::{QRCode, QRCodeStatus};

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
        status: QRCodeStatus::Expired,
        on_refresh: Some(move |_| {
            println!("点击刷新");
        }),
    }
}
```

### 自定义尺寸

```rust
use adui_dioxus::QRCode;

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
        size: 200,
    }
}
```

## 使用场景

- **支付二维码**：显示支付二维码
- **分享**：生成用于分享内容的二维码
- **身份验证**：显示身份验证二维码
- **链接**：为 URL 生成二维码

## 与 Ant Design 6.0.0 的差异

- ✅ SVG 和 Canvas 渲染
- ✅ 自定义图标
- ✅ 自定义颜色
- ✅ 状态
- ✅ 纠错级别
- ⚠️ Canvas 渲染可能回退到 SVG
- ⚠️ 某些高级功能可能有所不同

