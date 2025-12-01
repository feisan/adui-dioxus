# Dropdown 下拉菜单

## 概述

Dropdown 组件提供下拉菜单，在点击或悬停触发器元素时显示。通常用于操作菜单和上下文菜单。

## API 参考

### DropdownProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<DropdownItem>` | - | 要显示的菜单项（必需） |
| `trigger` | `DropdownTrigger` | `DropdownTrigger::Click` | 触发模式 |
| `placement` | `Option<DropdownPlacement>` | `None` | 下拉菜单的位置 |
| `open` | `Option<bool>` | `None` | 受控的打开状态 |
| `default_open` | `Option<bool>` | `None` | 初始打开状态 |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | 打开状态改变时调用 |
| `on_click` | `Option<EventHandler<String>>` | `None` | 菜单项被点击时调用 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `class` | `Option<String>` | `None` | 触发器包装器的额外类 |
| `overlay_class` | `Option<String>` | `None` | 下拉菜单的额外类 |
| `overlay_style` | `Option<String>` | `None` | 下拉菜单的内联样式 |
| `overlay_width` | `Option<f32>` | `None` | 下拉菜单的自定义宽度 |
| `children` | `Element` | - | 触发器元素（必需） |

### DropdownItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 项的唯一键 |
| `label` | `String` | 显示标签 |
| `disabled` | `bool` | 项是否禁用 |

### DropdownTrigger

- `Click` - 点击打开（默认）
- `Hover` - 悬停打开

### DropdownPlacement

- `BottomLeft` - 底部左侧（默认）
- `BottomRight` - 底部右侧

## 使用示例

### 基础下拉菜单

```rust
use adui_dioxus::{Dropdown, DropdownItem, Button, ButtonType};

rsx! {
    Dropdown {
        items: vec![
            DropdownItem::new("1", "菜单项 1"),
            DropdownItem::new("2", "菜单项 2"),
            DropdownItem::new("3", "菜单项 3"),
        ],
        Button {
            r#type: ButtonType::Primary,
            "打开菜单"
        }
    }
}
```

### 悬停触发

```rust
use adui_dioxus::{Dropdown, DropdownTrigger, DropdownItem, Button, ButtonType};

rsx! {
    Dropdown {
        trigger: DropdownTrigger::Hover,
        items: vec![
            DropdownItem::new("1", "菜单项 1"),
            DropdownItem::new("2", "菜单项 2"),
        ],
        Button {
            r#type: ButtonType::Primary,
            "悬停我"
        }
    }
}
```

### 带点击处理器

```rust
use adui_dioxus::{Dropdown, DropdownItem, Button, ButtonType};

rsx! {
    Dropdown {
        items: vec![
            DropdownItem::new("1", "操作 1"),
            DropdownItem::new("2", "操作 2"),
        ],
        on_click: Some(move |key| {
            println!("点击: {}", key);
        }),
        Button {
            r#type: ButtonType::Primary,
            "操作"
        }
    }
}
```

### 受控下拉菜单

```rust
use adui_dioxus::{Dropdown, DropdownItem, Button, ButtonType};
use dioxus::prelude::*;

let open = use_signal(|| false);

rsx! {
    Dropdown {
        items: vec![
            DropdownItem::new("1", "菜单项 1"),
        ],
        open: Some(*open.read()),
        on_open_change: Some(move |is_open| {
            open.set(is_open);
        }),
        Button {
            r#type: ButtonType::Primary,
            "切换菜单"
        }
    }
}
```

## 使用场景

- **操作菜单**：显示操作菜单
- **上下文菜单**：创建上下文菜单
- **导航**：下拉导航
- **设置**：设置下拉菜单

## 与 Ant Design 6.0.0 的差异

- ✅ 点击和悬停触发
- ✅ 菜单项
- ✅ 受控和非受控模式
- ⚠️ 某些高级功能可能有所不同

