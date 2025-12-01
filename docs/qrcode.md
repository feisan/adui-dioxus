# QRCode

## Overview

The QRCode component generates and displays QR codes. It supports SVG and Canvas rendering, custom icons, colors, and different status states.

## API Reference

### QRCodeProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | The value/content to encode (required) |
| `type` | `QRCodeType` | `QRCodeType::Svg` | Rendering type |
| `size` | `u32` | `160` | Size of QR code in pixels |
| `icon` | `Option<String>` | `None` | Icon URL to display in center |
| `icon_size` | `Option<u32>` | `None` | Size of the icon |
| `color` | `Option<String>` | `None` | Foreground color (defaults to current text color) |
| `bg_color` | `Option<String>` | `None` | Background color (defaults to transparent) |
| `error_level` | `QRCodeErrorLevel` | `QRCodeErrorLevel::M` | Error correction level |
| `status` | `QRCodeStatus` | `QRCodeStatus::Active` | Current status |
| `bordered` | `bool` | `true` | Whether to show border |
| `on_refresh` | `Option<EventHandler<()>>` | `None` | Callback when refresh is clicked |
| `class` | `Option<String>` | `None` | Extra class name for container |
| `root_class` | `Option<String>` | `None` | Extra class name for root |
| `style` | `Option<String>` | `None` | Inline style for container |

### QRCodeType

- `Svg` - Render as SVG (default, better for scaling)
- `Canvas` - Render as Canvas (better for large QR codes)

### QRCodeStatus

- `Active` - Active and scannable (default)
- `Expired` - Expired, needs refresh
- `Loading` - Loading state
- `Scanned` - Already scanned

### QRCodeErrorLevel

- `L` - ~7% error correction
- `M` - ~15% error correction (default)
- `Q` - ~25% error correction
- `H` - ~30% error correction

## Usage Examples

### Basic QRCode

```rust
use adui_dioxus::QRCode;

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
    }
}
```

### With Icon

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

### Custom Colors

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

### With Status

```rust
use adui_dioxus::{QRCode, QRCodeStatus};

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
        status: QRCodeStatus::Expired,
        on_refresh: Some(move |_| {
            println!("Refresh clicked");
        }),
    }
}
```

### Custom Size

```rust
use adui_dioxus::QRCode;

rsx! {
    QRCode {
        value: "https://example.com".to_string(),
        size: 200,
    }
}
```

## Use Cases

- **Payment QR Codes**: Display payment QR codes
- **Sharing**: Generate QR codes for sharing content
- **Authentication**: Display authentication QR codes
- **Links**: Generate QR codes for URLs

## Differences from Ant Design 6.0.0

- ✅ SVG and Canvas rendering
- ✅ Custom icons
- ✅ Custom colors
- ✅ Status states
- ✅ Error correction levels
- ⚠️ Canvas rendering may fallback to SVG
- ⚠️ Some advanced features may differ

