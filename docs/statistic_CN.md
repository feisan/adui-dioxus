# Statistic 统计数值

## 概述

Statistic 组件显示统计数据，包含标题、数值、前缀和后缀。通常用于仪表板和数据展示。

## API 参考

### StatisticProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `Option<Element>` | `None` | 数值上方显示的可选标题 |
| `value` | `Option<f64>` | `None` | 要显示的数值 |
| `value_text` | `Option<String>` | `None` | 可选的预格式化数值文本（优先于 value） |
| `precision` | `Option<u8>` | `None` | 应用于数值的可选小数精度 |
| `prefix` | `Option<Element>` | `None` | 在数值前渲染的可选前缀元素 |
| `suffix` | `Option<Element>` | `None` | 在数值后渲染的可选后缀元素 |
| `value_style` | `Option<String>` | `None` | 数值 span 的可选内联样式（例如颜色） |
| `class` | `Option<String>` | `None` | 根元素的额外类名 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |

## 使用示例

### 基础用法

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("总销售额")),
        value: Some(123456.78),
    }
}
```

### 带前缀和后缀

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("价格")),
        prefix: Some(rsx!("¥")),
        value: Some(99.99),
        suffix: Some(rsx!(".00")),
    }
}
```

### 带精度

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("百分比")),
        value: Some(85.6789),
        precision: Some(2),
    }
}
```

### 自定义数值文本

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("自定义格式")),
        value_text: Some("1,234,567".to_string()),
    }
}
```

### 带样式

```rust
use adui_dioxus::Statistic;

rsx! {
    Statistic {
        title: Some(rsx!("收入")),
        value: Some(50000.0),
        value_style: Some("color: #52c41a; font-size: 24px;".to_string()),
        suffix: Some(rsx!(" 元")),
    }
}
```

## 使用场景

- **仪表板**：显示关键指标和统计数据
- **数据卡片**：在卡片中显示统计数据
- **报告**：显示报告统计
- **分析**：显示分析数据

## 与 Ant Design 6.0.0 的差异

- ✅ 标题、数值、前缀和后缀
- ✅ 小数精度
- ✅ 自定义数值文本
- ✅ 数值样式
- ⚠️ 倒计时功能尚未实现
- ⚠️ 某些高级样式选项可能有所不同

