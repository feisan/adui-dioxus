# Statistic

## Overview

The Statistic component displays statistical data with a title, value, prefix, and suffix. It's commonly used in dashboards and data displays.

## API Reference

### StatisticProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `Option<Element>` | `None` | Optional title shown above the value |
| `value` | `Option<f64>` | `None` | Numeric value to display |
| `value_text` | `Option<String>` | `None` | Optional preformatted value text (takes precedence over value) |
| `precision` | `Option<u8>` | `None` | Optional decimal precision applied to value |
| `prefix` | `Option<Element>` | `None` | Optional prefix element rendered before value |
| `suffix` | `Option<Element>` | `None` | Optional suffix element rendered after value |
| `value_style` | `Option<String>` | `None` | Optional inline style for value span (e.g. color) |
| `class` | `Option<String>` | `None` | Extra class on root element |
| `style` | `Option<String>` | `None` | Inline style on root element |

## Usage Examples

### Basic Usage

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("Total Sales")),
        value: Some(123456.78),
    }
}
```

### With Prefix and Suffix

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("Price")),
        prefix: Some(rsx!("$")),
        value: Some(99.99),
        suffix: Some(rsx!(".00")),
    }
}
```

### With Precision

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("Percentage")),
        value: Some(85.6789),
        precision: Some(2),
    }
}
```

### Custom Value Text

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("Custom Format")),
        value_text: Some("1,234,567".to_string()),
    }
}
```

### With Styling

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("Revenue")),
        value: Some(50000.0),
        value_style: Some("color: #52c41a; font-size: 24px;".to_string()),
        suffix: Some(rsx!(" USD")),
    }
}
```

## Use Cases

- **Dashboards**: Display key metrics and statistics
- **Data Cards**: Show statistical data in cards
- **Reports**: Display report statistics
- **Analytics**: Show analytics data

## Differences from Ant Design 6.0.0

- ✅ Title, value, prefix, and suffix
- ✅ Decimal precision
- ✅ Custom value text
- ✅ Value styling
- ⚠️ Countdown functionality not yet implemented
- ⚠️ Some advanced styling options may differ

