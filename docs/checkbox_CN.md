# Checkbox 多选框

## 概述

Checkbox 组件允许用户从一组选项中选择一个或多个。它支持单个复选框和复选框组，并支持不确定状态。

## API 参考

### CheckboxProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `Option<bool>` | `None` | 受控的选中状态 |
| `default_checked` | `bool` | `false` | 非受控模式下的初始状态 |
| `indeterminate` | `bool` | `false` | 视觉不确定状态（用于"全选"场景） |
| `disabled` | `bool` | `false` | 复选框是否禁用 |
| `value` | `Option<String>` | `None` | 在 CheckboxGroup 内使用的值 |
| `status` | `Option<ControlStatus>` | `None` | 可选状态（success/warning/error） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<bool>>` | `None` | 状态改变事件处理器 |
| `children` | `Element` | - | 右侧渲染的可选标签内容（必需） |

### CheckboxGroupProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<Vec<String>>` | `None` | 受控的选中值 |
| `default_value` | `Option<Vec<String>>` | `None` | 非受控模式下的初始选中值 |
| `disabled` | `bool` | `false` | 组内所有复选框是否禁用 |
| `options` | `Option<Vec<(String, String)>>` | `None` | 选项作为 (value, label) 对 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | 状态改变事件处理器 |
| `children` | `Element` | - | 复选框子元素（必需） |

## 使用示例

### 基础复选框

```rust
use adui_dioxus::Checkbox;
use dioxus::prelude::*;

let checked = use_signal(|| false);

rsx! {
    Checkbox {
        checked: Some(*checked.read()),
        on_change: Some(move |is_checked| {
            checked.set(is_checked);
        }),
        "我同意条款"
    }
}
```

### 不确定复选框

```rust
use adui_dioxus::Checkbox;

rsx! {
    Checkbox {
        indeterminate: true,
        "全选"
    }
}
```

### 复选框组

```rust
use adui_dioxus::{Checkbox, CheckboxGroup};
use dioxus::prelude::*;

let selected = use_signal(|| vec!["apple".to_string()]);

rsx! {
    CheckboxGroup {
        value: Some(selected.read().clone()),
        on_change: Some(move |values| {
            selected.set(values);
        }),
        Checkbox { value: Some("apple".to_string()), "苹果" }
        Checkbox { value: Some("banana".to_string()), "香蕉" }
        Checkbox { value: Some("orange".to_string()), "橙子" }
    }
}
```

## 使用场景

- **表单输入**：多选表单字段
- **设置**：启用/禁用多个选项
- **筛选器**：多选筛选器
- **协议**：条款和条件复选框

## 与 Ant Design 6.0.0 的差异

- ✅ 单个和组复选框
- ✅ 不确定状态
- ✅ 受控和非受控模式
- ✅ 表单集成
- ⚠️ 某些高级样式选项可能有所不同

