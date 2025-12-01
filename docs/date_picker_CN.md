# DatePicker 日期选择器

## 概述

DatePicker 组件提供日期选择界面，带有下拉日历。它支持单日期选择、日期范围、时间选择和自定义日期格式。

## API 参考

### DatePickerProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<DateValue>` | `None` | 受控日期值 |
| `default_value` | `Option<DateValue>` | `None` | 非受控模式下的初始值 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `format` | `Option<String>` | `None` | 显示/解析格式（默认：YYYY-MM-DD） |
| `disabled` | `Option<bool>` | `None` | 禁用交互 |
| `allow_clear` | `Option<bool>` | `None` | 显示清除图标 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<Option<DateValue>>>` | `None` | 日期改变时调用 |
| `show_time` | `Option<ShowTimeConfig>` | `None` | 显示时间选择器 |
| `ranges` | `Option<HashMap<String, (DateValue, DateValue)>>` | `None` | 预设日期范围 |
| `disabled_date` | `Option<Rc<dyn Fn(DateValue) -> bool>>` | `None` | 禁用特定日期 |
| `disabled_time` | `Option<Rc<dyn Fn(DateValue) -> bool>>` | `None` | 禁用特定时间 |
| `render_extra_footer` | `Option<Rc<dyn Fn() -> Element>>` | `None` | 自定义页脚渲染 |
| `generate_config` | `Option<DateGenerateConfig>` | `None` | 自定义日期库配置 |

### DateValue

使用 `time::Date` 的内部日期值类型。可以使用 `DateValue::from_ymd(year, month, day)` 创建，并使用 `to_ymd_string()` 格式化。

### ShowTimeConfig

| 字段 | 类型 | 说明 |
|------|------|------|
| `format` | `Option<String>` | 时间格式 |
| `default_value` | `Option<String>` | 默认时间值 |
| `hour_step` | `Option<u8>` | 小时步长 |
| `minute_step` | `Option<u8>` | 分钟步长 |
| `second_step` | `Option<u8>` | 秒步长 |

## 使用示例

### 基础日期选择器

```rust
use adui_dioxus::{DatePicker, DateValue};
use dioxus::prelude::*;

let date = use_signal(|| None::<DateValue>);

rsx! {
    DatePicker {
        value: *date.read(),
        on_change: Some(move |d| {
            date.set(d);
        }),
        placeholder: Some("选择日期".to_string()),
    }
}
```

### 带时间

```rust
use adui_dioxus::{DatePicker, ShowTimeConfig};

rsx! {
    DatePicker {
        show_time: Some(ShowTimeConfig {
            format: Some("HH:mm:ss".to_string()),
            ..Default::default()
        }),
        placeholder: Some("选择日期和时间".to_string()),
    }
}
```

### 带预设范围

```rust
use adui_dioxus::{DatePicker, DateValue};
use std::collections::HashMap;

let ranges = {
    let mut map = HashMap::new();
    if let (Some(today), Some(yesterday)) = (
        DateValue::from_ymd(2024, 1, 15),
        DateValue::from_ymd(2024, 1, 14),
    ) {
        map.insert("昨天".to_string(), (yesterday, yesterday));
        map.insert("今天".to_string(), (today, today));
    }
    map
};

rsx! {
    DatePicker {
        ranges: Some(ranges),
        placeholder: Some("选择日期".to_string()),
    }
}
```

### 带禁用日期

```rust
use adui_dioxus::{DatePicker, DateValue};
use std::rc::Rc;

rsx! {
    DatePicker {
        disabled_date: Some(Rc::new(|date| {
            // 禁用周末（示例）
            let weekday = date.inner.weekday();
            weekday == time::Weekday::Saturday || weekday == time::Weekday::Sunday
        })),
        placeholder: Some("选择日期".to_string()),
    }
}
```

## 使用场景

- **表单**：表单中的日期输入
- **过滤**：日期范围过滤
- **调度**：预约调度
- **报表**：基于日期的报表

## 与 Ant Design 6.0.0 的差异

- ✅ 单日期选择
- ✅ 时间选择支持
- ✅ 预设范围
- ✅ 禁用日期
- ⚠️ 范围选择器可能有所不同
- ⚠️ 某些高级功能可能有所不同

