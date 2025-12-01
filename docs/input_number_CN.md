# InputNumber 数字输入框

## 概述

InputNumber 组件是一个带步进控制和基本格式化的数字输入框。它支持最小/最大约束、步进增量和精度控制。

## API 参考

### InputNumberProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<f64>` | `None` | 受控值 |
| `default_value` | `Option<f64>` | `None` | 非受控初始值 |
| `min` | `Option<f64>` | `None` | 最小值 |
| `max` | `Option<f64>` | `None` | 最大值 |
| `step` | `Option<f64>` | `None` | 步进增量 |
| `precision` | `Option<u32>` | `None` | 小数精度 |
| `controls` | `bool` | `true` | 是否显示步进控制 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `status` | `Option<ControlStatus>` | `None` | 控制状态（错误、警告） |
| `prefix` | `Option<Element>` | `None` | 前缀元素 |
| `suffix` | `Option<Element>` | `None` | 后缀元素 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<Option<f64>>>` | `None` | 数字值改变时触发 |
| `on_change_complete` | `Option<EventHandler<Option<f64>>>` | `None` | 失焦或按 Enter 时触发 |

## 使用示例

### 基础数字输入框

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.0));

rsx! {
    InputNumber {
        value: *value.read(),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 带最小/最大值

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(50.0));

rsx! {
    InputNumber {
        value: *value.read(),
        min: Some(0.0),
        max: Some(100.0),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 带步长

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.0));

rsx! {
    InputNumber {
        value: *value.read(),
        step: Some(5.0),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 带精度

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.5));

rsx! {
    InputNumber {
        value: *value.read(),
        precision: Some(2),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 带前缀/后缀

```rust
use adui_dioxus::{InputNumber, Icon, IconKind};
use dioxus::prelude::*;

let value = use_signal(|| Some(100.0));

rsx! {
    InputNumber {
        value: *value.read(),
        prefix: Some(rsx!(Icon { kind: IconKind::Dollar })),
        suffix: Some(rsx!("USD")),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 无控制按钮

```rust
use adui_dioxus::InputNumber;
use dioxus::prelude::*;

let value = use_signal(|| Some(10.0));

rsx! {
    InputNumber {
        value: *value.read(),
        controls: false,
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

## 使用场景

- **数量输入**：在表单中输入数量
- **价格输入**：输入带货币的价格
- **评分输入**：输入评分或分数
- **数字表单**：任何需要数字输入的表单

## 与 Ant Design 6.0.0 的差异

- ✅ 最小/最大约束
- ✅ 步进增量
- ✅ 精度控制
- ✅ 前缀/后缀支持
- ✅ 键盘导航
- ⚠️ 某些高级功能可能有所不同

