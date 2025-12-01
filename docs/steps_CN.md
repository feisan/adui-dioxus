# Steps 步骤条

## 概述

Steps 组件显示流程的步骤指示器。它显示当前步骤和通过多步骤工作流的进度。

## API 参考

### StepsProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<StepItem>` | - | 步骤项（必需） |
| `current` | `Option<usize>` | `None` | 受控的当前步骤索引 |
| `default_current` | `Option<usize>` | `None` | 非受控模式下的默认当前步骤 |
| `on_change` | `Option<EventHandler<usize>>` | `None` | 步骤改变时调用 |
| `direction` | `Option<StepsDirection>` | `None`（默认为 Horizontal） | 步骤方向 |
| `size` | `Option<ComponentSize>` | `None` | 步骤尺寸 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### StepItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 步骤的唯一键 |
| `title` | `Element` | 步骤标题 |
| `description` | `Option<Element>` | 可选的步骤描述 |
| `status` | `Option<StepStatus>` | 可选的显式状态 |
| `disabled` | `bool` | 步骤是否禁用 |

### StepStatus

- `Wait` - 等待状态
- `Process` - 进行中状态
- `Finish` - 完成状态
- `Error` - 错误状态

### StepsDirection

- `Horizontal` - 水平步骤（默认）
- `Vertical` - 垂直步骤

## 使用示例

### 基础步骤条

```rust
use adui_dioxus::{Steps, StepItem};

rsx! {
    Steps {
        items: vec![
            StepItem::new("1", rsx!("步骤 1")),
            StepItem::new("2", rsx!("步骤 2")),
            StepItem::new("3", rsx!("步骤 3")),
        ],
        default_current: Some(1),
    }
}
```

### 带描述

```rust
use adui_dioxus::{Steps, StepItem};

rsx! {
    Steps {
        items: vec![
            StepItem {
                key: "1".to_string(),
                title: rsx!("步骤 1"),
                description: Some(rsx!("描述 1")),
                status: None,
                disabled: false,
            },
            StepItem {
                key: "2".to_string(),
                title: rsx!("步骤 2"),
                description: Some(rsx!("描述 2")),
                status: None,
                disabled: false,
            },
        ],
    }
}
```

### 垂直步骤条

```rust
use adui_dioxus::{Steps, StepsDirection, StepItem};

rsx! {
    Steps {
        direction: Some(StepsDirection::Vertical),
        items: vec![
            StepItem::new("1", rsx!("步骤 1")),
            StepItem::new("2", rsx!("步骤 2")),
            StepItem::new("3", rsx!("步骤 3")),
        ],
    }
}
```

### 带状态

```rust
use adui_dioxus::{Steps, StepItem, StepStatus};

rsx! {
    Steps {
        items: vec![
            StepItem {
                key: "1".to_string(),
                title: rsx!("已完成"),
                description: None,
                status: Some(StepStatus::Finish),
                disabled: false,
            },
            StepItem {
                key: "2".to_string(),
                title: rsx!("进行中"),
                description: None,
                status: Some(StepStatus::Process),
                disabled: false,
            },
            StepItem {
                key: "3".to_string(),
                title: rsx!("等待中"),
                description: None,
                status: Some(StepStatus::Wait),
                disabled: false,
            },
        ],
    }
}
```

## 使用场景

- **表单向导**：多步骤表单流程
- **用户引导**：用户引导流程
- **结账流程**：电商结账步骤
- **工作流**：逐步工作流

## 与 Ant Design 6.0.0 的差异

- ✅ 水平和垂直方向
- ✅ 步骤状态
- ✅ 描述
- ✅ 可点击步骤
- ⚠️ 图标自定义尚未实现
- ⚠️ 某些高级功能可能有所不同

