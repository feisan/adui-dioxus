# Segmented 分段控制器

## 概述

Segmented 组件提供用于单选的分段控制器。它将选项显示为可点击选择的段。

## API 参考

### SegmentedProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `options` | `Vec<SegmentedOption>` | - | 分段选项（必需） |
| `value` | `Option<String>` | `None` | 受控值 |
| `default_value` | `Option<String>` | `None` | 非受控模式下的初始值 |
| `block` | `bool` | `false` | 填充父级宽度 |
| `round` | `bool` | `false` | 圆角形状 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 值改变时调用 |

### SegmentedOption

| 字段 | 类型 | 说明 |
|------|------|------|
| `label` | `String` | 显示标签 |
| `value` | `String` | 选项值 |
| `icon` | `Option<Element>` | 可选图标 |
| `tooltip` | `Option<String>` | 可选提示文本 |
| `disabled` | `bool` | 选项是否禁用 |

## 使用示例

### 基础分段控制器

```rust
use adui_dioxus::{Segmented, SegmentedOption};
use dioxus::prelude::*;

let value = use_signal(|| Some("1".to_string()));

rsx! {
    Segmented {
        options: vec![
            SegmentedOption::new("选项 1", "1"),
            SegmentedOption::new("选项 2", "2"),
            SegmentedOption::new("选项 3", "3"),
        ],
        value: *value.read(),
        on_change: Some(move |v| {
            value.set(Some(v));
        }),
    }
}
```

### 带图标

```rust
use adui_dioxus::{Segmented, SegmentedOption, Icon, IconKind};

rsx! {
    Segmented {
        options: vec![
            SegmentedOption {
                label: "列表".to_string(),
                value: "list".to_string(),
                icon: Some(rsx!(Icon { kind: IconKind::List })),
                tooltip: None,
                disabled: false,
            },
            SegmentedOption {
                label: "网格".to_string(),
                value: "grid".to_string(),
                icon: Some(rsx!(Icon { kind: IconKind::Grid })),
                tooltip: None,
                disabled: false,
            },
        ],
    }
}
```

### 块模式

```rust
use adui_dioxus::{Segmented, SegmentedOption};

rsx! {
    Segmented {
        block: true,
        options: vec![
            SegmentedOption::new("选项 1", "1"),
            SegmentedOption::new("选项 2", "2"),
        ],
    }
}
```

### 圆角

```rust
use adui_dioxus::{Segmented, SegmentedOption};

rsx! {
    Segmented {
        round: true,
        options: vec![
            SegmentedOption::new("选项 1", "1"),
            SegmentedOption::new("选项 2", "2"),
        ],
    }
}
```

## 使用场景

- **视图切换**：在不同视图之间切换
- **过滤选择**：选择过滤器
- **模式选择**：选择模式或状态
- **标签式导航**：标签式导航

## 与 Ant Design 6.0.0 的差异

- ✅ 单选
- ✅ 图标和提示
- ✅ 块和圆角模式
- ✅ 键盘导航
- ⚠️ 某些高级功能可能有所不同

