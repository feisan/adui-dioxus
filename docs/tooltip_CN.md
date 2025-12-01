# Tooltip 文字提示

## 概述

Tooltip 组件在用户悬停或点击元素时显示一个包含附加信息的小弹窗。通常用于提供上下文或帮助文本。

## API 参考

### TooltipProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `Option<String>` | `None` | 工具提示内显示的简单文本标题 |
| `content` | `Option<Element>` | `None` | 自定义工具提示内容节点 |
| `placement` | `Option<TooltipPlacement>` | `None`（默认为 `Top`） | 相对于触发器的位置 |
| `trigger` | `TooltipTrigger` | `TooltipTrigger::Hover` | 工具提示的触发方式 |
| `open` | `Option<bool>` | `None` | 受控的打开状态 |
| `default_open` | `Option<bool>` | `None` | 非受控模式下的初始打开状态 |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | 打开状态改变时调用 |
| `disabled` | `bool` | `false` | 禁用用户交互 |
| `class` | `Option<String>` | `None` | 触发器包装器的额外类名 |
| `overlay_class` | `Option<String>` | `None` | 工具提示气泡的额外类名 |
| `overlay_style` | `Option<String>` | `None` | 工具提示气泡的内联样式 |
| `children` | `Element` | - | 触发器元素（必需） |

### TooltipPlacement

- `Top` - 触发器上方（默认）
- `Bottom` - 触发器下方
- `Left` - 触发器左侧
- `Right` - 触发器右侧

### TooltipTrigger

- `Hover` - 悬停时显示（默认）
- `Click` - 点击时显示

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Tooltip, Button, ButtonType};

rsx! {
    Tooltip {
        title: Some("这是工具提示".to_string()),
        Button {
            r#type: ButtonType::Primary,
            "悬停我"
        }
    }
}
```

### 自定义内容

```rust
use adui_dioxus::Tooltip;

rsx! {
    Tooltip {
        content: Some(rsx! {
            div {
                "自定义工具提示内容"
                br {}
                "多行内容"
            }
        }),
        span { "悬停查看自定义工具提示" }
    }
}
```

### 点击触发

```rust
use adui_dioxus::{Tooltip, TooltipTrigger};

rsx! {
    Tooltip {
        title: Some("点击显示".to_string()),
        trigger: TooltipTrigger::Click,
        Button { "点击我" }
    }
}
```

### 受控工具提示

```rust
use adui_dioxus::Tooltip;
use dioxus::prelude::*;

let is_open = use_signal(|| false);

rsx! {
    Tooltip {
        open: Some(*is_open.read()),
        on_open_change: Some(move |open| {
            is_open.set(open);
        }),
        title: Some("受控工具提示".to_string()),
        Button { "切换工具提示" }
    }
}
```

## 使用场景

- **帮助文本**：提供附加信息
- **表单字段**：显示字段描述
- **图标**：解释图标含义
- **禁用元素**：解释为什么某些内容被禁用

## 与 Ant Design 6.0.0 的差异

- ✅ 悬停和点击触发
- ✅ 自定义内容支持
- ✅ 位置选项
- ✅ 受控和非受控模式
- ⚠️ 箭头自定义尚未实现
- ⚠️ 某些高级样式选项可能有所不同

