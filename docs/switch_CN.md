# Switch 开关

## 概述

Switch 组件是用于二元选择的切换开关。它支持受控和非受控模式、选中/未选中状态的自定义子元素，以及表单集成。

## API 参考

### SwitchProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `checked` | `Option<bool>` | `None` | 受控的选中状态 |
| `default_checked` | `bool` | `false` | 非受控模式下的初始值 |
| `disabled` | `bool` | `false` | 开关是否禁用 |
| `size` | `SwitchSize` | `SwitchSize::Default` | 开关的视觉尺寸 |
| `checked_children` | `Option<Element>` | `None` | 选中时显示的内容 |
| `un_checked_children` | `Option<Element>` | `None` | 未选中时显示的内容 |
| `status` | `Option<ControlStatus>` | `None` | 可选状态（success/warning/error） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<bool>>` | `None` | 状态改变事件，返回下一个选中状态 |

### SwitchSize

- `Default` - 默认尺寸
- `Small` - 小尺寸

## 使用示例

### 基础用法

```rust
use adui_dioxus::Switch;
use dioxus::prelude::*;

let checked = use_signal(|| false);

rsx! {
    Switch {
        checked: Some(*checked.read()),
        on_change: Some(move |is_checked| {
            checked.set(is_checked);
        }),
    }
}
```

### 带子元素

```rust
use adui_dioxus::Switch;

rsx! {
    Switch {
        checked_children: Some(rsx!("开")),
        un_checked_children: Some(rsx!("关")),
    }
}
```

### 小尺寸

```rust
use adui_dioxus::{Switch, SwitchSize};

rsx! {
    Switch {
        size: SwitchSize::Small,
    }
}
```

### 非受控模式

```rust
use adui_dioxus::Switch;

rsx! {
    Switch {
        default_checked: true,
    }
}
```

## 使用场景

- **设置**：切换设置开关
- **功能标志**：启用/禁用功能
- **表单控件**：二元表单输入
- **UI 偏好**：切换 UI 偏好

## 与 Ant Design 6.0.0 的差异

- ✅ 受控和非受控模式
- ✅ 选中/未选中状态的自定义子元素
- ✅ 尺寸变体
- ✅ 表单集成
- ✅ 状态变体
- ⚠️ 加载状态尚未实现
- ⚠️ 某些高级样式选项可能有所不同

