# TimePicker 时间选择器

## 概述

TimePicker 组件提供时间选择界面，包含小时、分钟和秒列。它支持自定义步长值和时间格式。

## API 参考

### TimePickerProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<TimeValue>` | `None` | 受控时间值 |
| `default_value` | `Option<TimeValue>` | `None` | 非受控模式下的初始值 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `format` | `Option<String>` | `None` | 显示/解析格式（默认：HH:mm:ss） |
| `hour_step` | `Option<u8>` | `None` | 小时列步长 |
| `minute_step` | `Option<u8>` | `None` | 分钟列步长 |
| `second_step` | `Option<u8>` | `None` | 秒列步长 |
| `disabled` | `Option<bool>` | `None` | 禁用交互 |
| `allow_clear` | `Option<bool>` | `None` | 显示清除图标 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<Option<TimeValue>>>` | `None` | 时间改变时调用 |

### TimeValue

包含小时、分钟和秒的内部时间值类型。可以使用 `TimeValue::new(hour, minute, second)` 创建，并使用 `to_hms_string()` 格式化。

## 使用示例

### 基础时间选择器

```rust
use adui_dioxus::{TimePicker, TimeValue};
use dioxus::prelude::*;

let time = use_signal(|| None::<TimeValue>);

rsx! {
    TimePicker {
        value: *time.read(),
        on_change: Some(move |t| {
            time.set(t);
        }),
        placeholder: Some("选择时间".to_string()),
    }
}
```

### 带自定义步长

```rust
use adui_dioxus::TimePicker;

rsx! {
    TimePicker {
        hour_step: Some(2),
        minute_step: Some(15),
        second_step: Some(10),
        placeholder: Some("选择时间".to_string()),
    }
}
```

### 带默认值

```rust
use adui_dioxus::{TimePicker, TimeValue};

rsx! {
    TimePicker {
        default_value: Some(TimeValue::new(9, 0, 0)),
        placeholder: Some("选择时间".to_string()),
    }
}
```

## 使用场景

- **表单**：表单中的时间输入
- **调度**：时间段选择
- **过滤**：基于时间的过滤
- **报表**：基于时间的报表

## 与 Ant Design 6.0.0 的差异

- ✅ 小时、分钟、秒选择
- ✅ 自定义步长值
- ✅ 时间格式
- ⚠️ 某些高级功能可能有所不同

