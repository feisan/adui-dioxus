# Popover 气泡卡片

## 概述

Popover 组件显示一个包含丰富内容的浮动面板。它与 Tooltip 类似，但支持更复杂的内容，包括标题和交互元素。

## API 参考

### PopoverProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `Option<Element>` | `None` | 顶部显示的可选标题节点 |
| `content` | `Option<Element>` | `None` | 气泡卡片的主要内容 |
| `placement` | `Option<TooltipPlacement>` | `None`（默认为 `Top`） | 相对于触发器的位置 |
| `trigger` | `TooltipTrigger` | `TooltipTrigger::Click` | 触发模式 |
| `open` | `Option<bool>` | `None` | 受控的打开状态 |
| `default_open` | `Option<bool>` | `None` | 非受控模式下的初始打开状态 |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | 打开状态改变时调用 |
| `disabled` | `bool` | `false` | 禁用用户交互 |
| `class` | `Option<String>` | `None` | 触发器包装器的额外类名 |
| `overlay_class` | `Option<String>` | `None` | 气泡卡片面板的额外类名 |
| `overlay_style` | `Option<String>` | `None` | 气泡卡片面板的内联样式 |
| `children` | `Element` | - | 触发器元素（必需） |

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Popover, Button, ButtonType};

rsx! {
    Popover {
        title: Some(rsx!("气泡卡片标题")),
        content: Some(rsx!("这是气泡卡片内容")),
        Button {
            r#type: ButtonType::Primary,
            "点击我"
        }
    }
}
```

### 丰富内容

```rust
use adui_dioxus::{Popover, Button, ButtonType};

rsx! {
    Popover {
        title: Some(rsx!("设置")),
        content: Some(rsx! {
            div {
                p { "配置您的偏好" }
                Button {
                    r#type: ButtonType::Primary,
                    "保存"
                }
            }
        }),
        Button { "打开气泡卡片" }
    }
}
```

### 悬停触发

```rust
use adui_dioxus::{Popover, TooltipTrigger};

rsx! {
    Popover {
        trigger: TooltipTrigger::Hover,
        content: Some(rsx!("悬停内容")),
        span { "悬停我" }
    }
}
```

## 使用场景

- **上下文信息**：显示附加详细信息
- **操作菜单**：显示操作选项
- **表单**：在气泡卡片中显示表单字段
- **帮助内容**：显示帮助信息

## 与 Ant Design 6.0.0 的差异

- ✅ 标题和内容支持
- ✅ 悬停和点击触发
- ✅ 位置选项
- ✅ 受控和非受控模式
- ⚠️ 箭头自定义尚未实现
- ⚠️ 某些高级样式选项可能有所不同

