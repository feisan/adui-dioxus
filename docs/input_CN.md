# Input 输入框

## 概述

Input 组件提供文本输入功能，支持前缀、后缀、清除按钮、字符计数和各种视觉变体。它包括专门的变体，如 Password、Search 和 OTP。

## API 参考

### InputProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<String>` | `None` | 受控值 |
| `default_value` | `Option<String>` | `None` | 非受控模式下的初始值 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `size` | `Option<InputSize>` | `None` | 组件尺寸 |
| `variant` | `Option<Variant>` | `None` | 视觉变体（outlined/filled/borderless） |
| `bordered` | `Option<bool>` | `None` | @deprecated 请使用 `variant` |
| `status` | `Option<ControlStatus>` | `None` | 控制状态（success/warning/error） |
| `prefix` | `Option<Element>` | `None` | 前导元素 |
| `suffix` | `Option<Element>` | `None` | 尾随元素 |
| `addon_before` | `Option<Element>` | `None` | @deprecated 请使用 `Space.Compact` |
| `addon_after` | `Option<Element>` | `None` | @deprecated 请使用 `Space.Compact` |
| `allow_clear` | `bool` | `false` | 有内容时显示清除图标 |
| `max_length` | `Option<usize>` | `None` | 最大长度 |
| `show_count` | `bool` | `false` | 显示字符计数 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `root_class_name` | `Option<String>` | `None` | 根元素的额外类 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `class_names` | `Option<InputClassNames>` | `None` | 语义类名 |
| `styles` | `Option<InputStyles>` | `None` | 语义样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 值改变时调用 |
| `on_press_enter` | `Option<EventHandler<()>>` | `None` | 按 Enter 时调用 |
| `data_attributes` | `Option<Vec<(String, String)>>` | `None` | 数据属性 |

### PasswordProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<String>` | `None` | 受控值 |
| `default_value` | `Option<String>` | `None` | 初始值 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `size` | `Option<InputSize>` | `None` | 组件尺寸 |
| `variant` | `Option<Variant>` | `None` | 视觉变体 |
| `status` | `Option<ControlStatus>` | `None` | 控制状态 |
| `prefix` | `Option<Element>` | `None` | 前导元素 |
| `visible` | `bool` | `false` | 密码是否默认可见 |
| `icon_render` | `Option<Element>` | `None` | 自定义可见性切换图标 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 值改变时调用 |
| `on_visible_change` | `Option<EventHandler<bool>>` | `None` | 可见性改变时调用 |

### SearchProps

与 InputProps 类似，但增加了 `on_search` 回调。

### OTPProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `length` | `usize` | `6` | OTP 数字位数 |
| `value` | `Option<String>` | `None` | 受控值 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 值改变时调用 |
| `on_finish` | `Option<EventHandler<String>>` | `None` | 所有数字填满时调用 |

### InputSize

- `Small` - 小尺寸
- `Middle` - 中等尺寸（默认）
- `Large` - 大尺寸

## 使用示例

### 基础输入框

```rust
use adui_dioxus::Input;
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    Input {
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("输入文本".to_string()),
    }
}
```

### 带前缀和后缀

```rust
use adui_dioxus::{Input, Icon, IconKind};

rsx! {
    Input {
        prefix: Some(rsx!(Icon { kind: IconKind::User })),
        suffix: Some(rsx!("@example.com")),
        placeholder: Some("用户名".to_string()),
    }
}
```

### 密码输入框

```rust
use adui_dioxus::Password;
use dioxus::prelude::*;

let password = use_signal(|| String::new());

rsx! {
    Password {
        value: Some(password.read().clone()),
        on_change: Some(move |v| {
            password.set(v);
        }),
        placeholder: Some("输入密码".to_string()),
    }
}
```

### 搜索输入框

```rust
use adui_dioxus::Search;

rsx! {
    Search {
        placeholder: Some("搜索...".to_string()),
        on_search: Some(move |value| {
            println!("搜索: {}", value);
        }),
    }
}
```

### 带字符计数

```rust
use adui_dioxus::Input;

rsx! {
    Input {
        max_length: Some(100),
        show_count: true,
        placeholder: Some("输入文本".to_string()),
    }
}
```

### OTP 输入框

```rust
use adui_dioxus::OTP;
use dioxus::prelude::*;

let otp = use_signal(|| String::new());

rsx! {
    OTP {
        length: 6,
        value: Some(otp.read().clone()),
        on_change: Some(move |v| {
            otp.set(v);
        }),
        on_finish: Some(move |v| {
            println!("OTP 完成: {}", v);
        }),
    }
}
```

## 使用场景

- **表单**：表单中的文本输入
- **搜索**：搜索输入字段
- **身份验证**：密码和 OTP 输入
- **数据录入**：通用数据录入字段

## 与 Ant Design 6.0.0 的差异

- ✅ 基础输入功能
- ✅ Password、Search 和 OTP 变体
- ✅ 前缀和后缀支持
- ✅ 字符计数
- ✅ 清除按钮
- ⚠️ 某些高级功能可能有所不同

