# Popconfirm 气泡确认框

## 概述

Popconfirm 组件在气泡卡片中显示确认对话框。用于在执行操作之前确认操作，通常用于破坏性操作。

## API 参考

### PopconfirmProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `String` | - | 确认面板中显示的主要标题（必需） |
| `description` | `Option<String>` | `None` | 标题下方显示的可选描述文本 |
| `ok_text` | `Option<String>` | `None`（默认为"确定"） | 确认按钮的文本 |
| `cancel_text` | `Option<String>` | `None`（默认为"取消"） | 取消按钮的文本 |
| `on_confirm` | `Option<EventHandler<()>>` | `None` | 用户确认操作时调用 |
| `on_cancel` | `Option<EventHandler<()>>` | `None` | 用户取消操作时调用 |
| `ok_type` | `Option<ButtonType>` | `None`（默认为 `Primary`） | 确认按钮的视觉类型 |
| `ok_danger` | `bool` | `false` | 确认按钮是否使用危险样式 |
| `placement` | `Option<TooltipPlacement>` | `None`（默认为 `Top`） | 相对于触发器的位置 |
| `trigger` | `TooltipTrigger` | `TooltipTrigger::Click` | 触发模式 |
| `open` | `Option<bool>` | `None` | 受控的打开状态 |
| `default_open` | `Option<bool>` | `None` | 非受控模式下的初始打开状态 |
| `on_open_change` | `Option<EventHandler<bool>>` | `None` | 打开状态改变时调用 |
| `disabled` | `bool` | `false` | 禁用用户交互 |
| `class` | `Option<String>` | `None` | 触发器包装器的额外类名 |
| `overlay_class` | `Option<String>` | `None` | 气泡确认框面板的额外类名 |
| `overlay_style` | `Option<String>` | `None` | 气泡确认框面板的内联样式 |
| `children` | `Element` | - | 触发器元素（必需） |

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Popconfirm, Button, ButtonType};

rsx! {
    Popconfirm {
        title: "确定要删除此项吗？".to_string(),
        on_confirm: Some(move |_| {
            println!("已确认！");
        }),
        Button {
            r#type: ButtonType::Danger,
            "删除"
        }
    }
}
```

### 带描述

```rust
use adui_dioxus::{Popconfirm, Button, ButtonType};

rsx! {
    Popconfirm {
        title: "删除项目".to_string(),
        description: Some("此操作无法撤销。".to_string()),
        ok_text: Some("是的，删除".to_string()),
        cancel_text: Some("取消".to_string()),
        ok_danger: true,
        on_confirm: Some(move |_| {
            println!("项目已删除");
        }),
        Button {
            r#type: ButtonType::Danger,
            "删除"
        }
    }
}
```

### 自定义按钮文本

```rust
use adui_dioxus::{Popconfirm, Button, ButtonType};

rsx! {
    Popconfirm {
        title: "确认操作".to_string(),
        ok_text: Some("继续".to_string()),
        cancel_text: Some("中止".to_string()),
        Button { "操作" }
    }
}
```

## 使用场景

- **删除确认**：删除项目前确认
- **破坏性操作**：确认危险操作
- **表单提交**：提交表单前确认
- **状态更改**：更改重要状态前确认

## 与 Ant Design 6.0.0 的差异

- ✅ 基础确认功能
- ✅ 自定义按钮文本
- ✅ 危险按钮样式
- ✅ 描述支持
- ⚠️ 图标自定义尚未实现
- ⚠️ 某些高级样式选项可能有所不同

