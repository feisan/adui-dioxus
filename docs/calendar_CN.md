# Calendar 日历

## 概述

Calendar 组件显示日历视图用于日期选择。它支持月份和年份模式、全屏显示和日期选择。

## API 参考

### CalendarProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<CalendarDate>` | `None` | 受控的选中日期 |
| `default_value` | `Option<CalendarDate>` | `None` | 非受控模式下的初始值 |
| `mode` | `Option<CalendarMode>` | `None` | 当前面板模式 |
| `fullscreen` | `Option<bool>` | `None` | 是否占满宽度 |
| `on_select` | `Option<EventHandler<CalendarDate>>` | `None` | 日期被选中时调用 |
| `on_panel_change` | `Option<EventHandler<(CalendarDate, CalendarMode)>>` | `None` | 面板改变时调用 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### CalendarMode

- `Month` - 月份视图（默认）
- `Year` - 年份视图

### CalendarDate

内部日期值类型。可以使用 `CalendarDate::from_ymd(year, month, day)` 创建。

## 使用示例

### 基础日历

```rust
use adui_dioxus::{Calendar, CalendarDate};
use dioxus::prelude::*;

let date = use_signal(|| None::<CalendarDate>);

rsx! {
    Calendar {
        value: *date.read(),
        on_select: Some(move |d| {
            date.set(Some(d));
        }),
    }
}
```

### 全屏日历

```rust
use adui_dioxus::Calendar;

rsx! {
    Calendar {
        fullscreen: Some(true),
    }
}
```

### 带面板改变处理器

```rust
use adui_dioxus::{Calendar, CalendarMode};

rsx! {
    Calendar {
        on_panel_change: Some(move |(date, mode)| {
            println!("面板改变: {:?}, 模式: {:?}", date, mode);
        }),
    }
}
```

## 使用场景

- **日期选择**：从日历中选择日期
- **调度**：查看和选择调度日期
- **日期显示**：显示日历视图
- **日期导航**：浏览日期

## 与 Ant Design 6.0.0 的差异

- ✅ 月份和年份模式
- ✅ 日期选择
- ✅ 全屏模式
- ⚠️ 某些高级功能可能有所不同

