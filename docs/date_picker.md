# DatePicker / RangePicker 说明

> 本文记录 DatePicker / RangePicker 在 adui-dioxus 中的 MVP 实现与使用建议，对应 `plan/0013.md` 中 B 阶段任务。

## 1. 组件定位

- `DatePicker`：单个日期选择控件，常用于表单中的日期字段，例如生效日期、截止日期等；
- `RangePicker`：日期区间选择控件，常用于筛选条件、统计区间配置等；
- 两者都优先与 `Form` 集成，对应 Ant Design 中 `DatePicker`/`RangePicker` 的基础用法。

当前实现聚焦 `picker="date"` 模式，暂不支持 week/month/quarter/year 等变体。

## 2. 值模型与 Props

### 2.1 DatePicker

导出类型：

```rust
use adui_dioxus::{DatePicker, DatePickerProps, DateValue};
```

内部值模型：

- `DateValue`：基于 `time::Date` 的轻量包装：
  - `DateValue::from_ymd(year: i32, month: u8, day: u8) -> Option<DateValue>`
  - `DateValue::to_ymd_string(&self) -> String`（`YYYY-MM-DD`）

核心 Props（MVP）：

- `value: Option<DateValue>`：受控值；
- `default_value: Option<DateValue>`：非受控初始值；
- `placeholder: Option<String>`：占位文案，默认「请选择日期」；
- `format: Option<String>`：预留，当前内部固定使用 `to_ymd_string`；
- `disabled: Option<bool>`：禁用；
- `allow_clear: Option<bool>`：是否展示清除按钮；
- `class: Option<String>` / `style: Option<String>`：额外类名与行内样式；
- `on_change: Option<EventHandler<Option<DateValue>>>`：值变更回调。

受控/非受控行为：

- 若 `value` 为 `Some`，组件视为受控，内部不修改值，仅在交互时调用 `on_change(Some/None)`；
- 若 `value` 为 `None`，使用 `default_value` 初始化内部 `Signal<Option<DateValue>>`，交互时更新内部状态并调用 `on_change`（若存在）。

### 2.2 RangePicker

导出类型：

```rust
use adui_dioxus::{RangePicker, RangePickerProps, DateRangeValue};
```

内部区间模型：

- `DateRangeValue { start: Option<DateValue>, end: Option<DateValue> }`：
  - 支持部分选择（仅 start）与完整选择（start + end）；
  - 提供辅助 `DateRangeValue::empty()` 用于表示空区间。

核心 Props：

- `value: Option<DateRangeValue>` / `default_value: Option<DateRangeValue>`：受控/非受控区间；
- `placeholder: Option<(String, String)>`：起始/结束输入占位；
- 其他字段与 DatePicker 类似：`format`、`disabled`、`allow_clear`、`class`、`style`；
- `on_change: Option<EventHandler<DateRangeValue>>`：区间变更回调。

受控/非受控行为与 DatePicker 相同：

- 受控：仅通过 `on_change` 通知外层；
- 非受控：内部 `Signal<DateRangeValue>` 保存当前区间，并同步回调。

## 3. 下拉面板与交互行为

### 3.1 单日期选择

- 触发区：
  - 使用 Input 风格的只读输入框展示当前日期；
  - `allow_clear` 时在右侧显示 `×` 按钮，点击后清空值并触发 `on_change(None)`；
- 下拉面板：
  - 通过 `use_dropdown_layer(open)` 从 OverlayManager 获取稳定的 `z-index`，以绝对定位渲染；
  - 使用内部 `days_in_month(year, month)` 与 `weekday_index_monday(year, month, 1)` 生成当前月的日期网格（按星期从周一到周日排列）；
  - 支持前/后一月切换（年切换通过跨越 1/12 月份自动完成）。
- 键盘：
  - Enter 打开日历；
  - Esc 关闭日历；
- 当前版本点击日期后更新值，不强制自动关闭下拉，后续可按需调整为「选择后自动关闭」。

### 3.2 区间选择逻辑

- 选择步骤：
  - 第一次点击：设置 `start = Some(clicked)`, `end = None`；
  - 第二次点击：
    - 若 `clicked < start`，自动调换顺序，`start = clicked`, `end = 原 start`；
    - 否则 `end = clicked`；
    - 完成后自动关闭下拉；
  - 再次点击：视为重新开始，`start = Some(clicked)`, `end = None`；
- 高亮规则（单月视图内）：
  - 在 `start..=end` 范围内使用 `.adui-date-picker-cell-in-range`；
  - 起止日期分别添加 `.adui-date-picker-cell-range-start` / `.adui-date-picker-cell-range-end`；
  - 跨月时当前实现只高亮当前面板内的日期，后续可扩展多月联动。

## 4. 样式与类名

样式定义位于 `src/theme.rs` 的 `adui_date_picker_style!` 宏中，通过 `THEME_BASE_STYLE` 注入：

- 触发区：
  - `.adui-date-picker`：单日期输入容器；
  - `.adui-date-picker-range`：区间输入容器；
  - `.adui-date-picker-disabled`：禁用态；
  - `.adui-date-picker-input` / `-input-start` / `-input-end`：内部输入框；
  - `.adui-date-picker-range-separator`：区间分隔符（` ~ `）；
  - `.adui-date-picker-clear`：清除按钮。
- 下拉面板：
  - `.adui-date-picker-dropdown`：整体容器；
  - `.adui-date-picker-header`：头部（月切换按钮 + 当前年月）；
  - `.adui-date-picker-nav-btn`：左右切换按钮；
  - `.adui-date-picker-header-view`：年月标题；
  - `.adui-date-picker-week-row` / `.adui-date-picker-week-cell`：星期行；
  - `.adui-date-picker-body`：日期网格容器；
  - `.adui-date-picker-cell` / `-cell-date` / `-cell-empty`：日期单元格；
  - `.adui-date-picker-cell-selected`：选中日期；
  - `.adui-date-picker-cell-in-range`：区间内部日期；
  - `.adui-date-picker-cell-range-start` / `-range-end`：区间起止日期。

## 5. 示例与使用建议

### 5.1 基础示例

示例文件：`examples/date_picker_demo.rs`

```rust
DatePicker {
    value: *single_value.read(),
    on_change: move |next| {
        single_value.set(next);
    },
}

RangePicker {
    value: Some(*range_value.read()),
    on_change: move |next| {
        range_value.set(next);
    },
}
```

### 5.2 与 Form 结合（MVP 示意）

当前 demo 中包含简单的 Form 集成示例（仅占位性质）：

- DatePicker / RangePicker 可以直接作为 `FormItem` 的 children；
- 后续计划在 `FormItemControlContext` 中补充日期时间与 `serde_json::Value` 之间的转换工具，统一表单值模型（例如使用字符串存储日期）。

## 6. 与 AntD 差异与后续规划

暂未覆盖的能力包括但不限于：

- 不支持 `picker="week"/"month"/"quarter"/"year"` 等模式；
- 不支持 `showTime`、`timePicker` 联动；
- 不支持复杂的 `disabledDate` 策略（当前尚未暴露禁用日期逻辑）；
- 不支持多面板联动、多月并列选择；
- 不支持完整的 locale 文案与格式化定制（当前仅使用英文星期缩写 + 简单年月展示）。

这些能力将视后续需求在新的计划中逐步补充。当前建议将 DatePicker/RangePicker 用于：

- 基础表单日期/区间选择；
- 简单筛选与统计区间配置；
- 与 TimePicker/Calendar 的组合在后续阶段规划中。
