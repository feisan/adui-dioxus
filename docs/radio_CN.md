# Radio 单选框

## 概述

Radio 组件允许用户从一组选项中选择一个。它支持单个单选按钮和单选按钮组，并支持按钮样式变体。

## API 参考

### RadioProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `String` | - | 在组内此单选按钮的值（必需） |
| `checked` | `Option<bool>` | `None` | 不在组内时的受控选中状态 |
| `default_checked` | `bool` | `false` | 初始选中状态 |
| `disabled` | `bool` | `false` | 单选按钮是否禁用 |
| `status` | `Option<ControlStatus>` | `None` | 可选状态（success/warning/error） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 单选按钮被选中时调用 |
| `children` | `Element` | - | 渲染的标签内容（必需） |
| `button` | `bool` | `false` | 渲染为按钮样式的单选按钮 |

### RadioGroupProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<String>` | `None` | 受控的选中值 |
| `default_value` | `Option<String>` | `None` | 非受控模式下的初始选中值 |
| `disabled` | `bool` | `false` | 组内所有单选按钮是否禁用 |
| `name` | `Option<String>` | `None` | 表单提交的 name 属性 |
| `options` | `Option<Vec<(String, String)>>` | `None` | 选项作为 (value, label) 对 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 状态改变事件处理器 |
| `children` | `Element` | - | 单选按钮子元素（必需） |

## 使用示例

### 基础单选按钮

```rust
use adui_dioxus::Radio;
use dioxus::prelude::*;

let selected = use_signal(|| "option1".to_string());

rsx! {
    Radio {
        value: "option1".to_string(),
        checked: Some(*selected.read() == "option1"),
        on_change: Some(move |_| {
            selected.set("option1".to_string());
        }),
        "选项 1"
    }
}
```

### 单选按钮组

```rust
use adui_dioxus::{Radio, RadioGroup};
use dioxus::prelude::*;

let selected = use_signal(|| Some("apple".to_string()));

rsx! {
    RadioGroup {
        value: *selected.read(),
        on_change: Some(move |value| {
            selected.set(Some(value));
        }),
        Radio { value: "apple".to_string(), "苹果" }
        Radio { value: "banana".to_string(), "香蕉" }
        Radio { value: "orange".to_string(), "橙子" }
    }
}
```

### 按钮样式单选按钮

```rust
use adui_dioxus::{Radio, RadioGroup};

rsx! {
    RadioGroup {
        Radio { value: "left".to_string(), button: true, "左" }
        Radio { value: "center".to_string(), button: true, "中" }
        Radio { value: "right".to_string(), button: true, "右" }
    }
}
```

## 使用场景

- **表单输入**：单选表单字段
- **设置**：从多个选项中选择一个
- **筛选器**：单选筛选器
- **选项**：从一组选项中选择一个

## 与 Ant Design 6.0.0 的差异

- ✅ 单个和组单选按钮
- ✅ 按钮样式单选按钮
- ✅ 受控和非受控模式
- ✅ 表单集成
- ⚠️ 某些高级样式选项可能有所不同

