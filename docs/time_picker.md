# TimePicker 说明

> 本文记录 TimePicker 在 adui-dioxus 中的 MVP 实现与使用建议，对应 `plan/0013.md` 中 C 阶段任务。

## 1. 组件定位

- `TimePicker`：时间点选择控件（HH:mm:ss），常与 DatePicker/RangePicker 组合形成“日期 + 时间”表单项；
- 优先满足中后台中常见的“精确到分/秒”的配置场景，例如定时任务、预约时间、通知发送时间等。

当前实现聚焦单个时间点选择，不包含时间区间 TimeRangePicker 变体。

## 2. 值模型与 Props

### 2.1 TimeValue

内部值模型：

```rust
use adui_dioxus::TimeValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeValue {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
```

辅助方法：

- `TimeValue::new(hour, minute, second)`：直接构造；
- `TimeValue::normalised(hour: i32, minute: i32, second: i32)`：将输入 clamp 到安全范围（0–23 / 0–59）；
- `TimeValue::to_hms_string()`：格式化为 `HH:mm:ss`。

### 2.2 TimePickerProps（MVP）

```rust
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    pub value: Option<TimeValue>,
    pub default_value: Option<TimeValue>,
    pub placeholder: Option<String>,
    pub format: Option<String>,
    pub hour_step: Option<u8>,
    pub minute_step: Option<u8>,
    pub second_step: Option<u8>,
    pub disabled: Option<bool>,
    pub allow_clear: Option<bool>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub on_change: Option<EventHandler<Option<TimeValue>>>,
}
```

说明：

- `value` / `default_value`：受控 / 非受控时间值；
- `placeholder`：无值时的占位文案，默认「请选择时间」；
- `format`：预留字段，当前内部统一使用 `HH:mm:ss`；
- `hour_step` / `minute_step` / `second_step`：列步进（默认 1），如 2 小时步长、15 分钟步长等；
- `disabled` / `allow_clear`：禁用与清除；
- `on_change`：当时间值变化时回调，`Some(TimeValue)` 表示选中，`None` 表示清空。

受控/非受控行为：

- 当 `value` 为 `Some` 时组件视为受控，仅通过 `on_change` 通知外层，不更新内部 `Signal`；
- 当 `value` 为 `None` 时使用 `default_value` 初始化内部 `Signal<TimeValue>`，交互时更新内部状态并同步回调。

## 3. 下拉面板与交互行为

### 3.1 触发区

- 使用 Input 风格的只读输入框显示当前时间：
  - 无值且有 `placeholder` 时显示占位；
  - 有值时使用 `TimeValue::to_hms_string()` 显示；
- `allow_clear` 时，在右侧显示 `×` 按钮，点击后触发 `on_change(None)`：
  - 非受控模式下内部时间重置为 00:00:00；
  - 受控模式下仅通过回调通知外层。

### 3.2 下拉面板结构

- 通过 `use_dropdown_layer(open)` 从 OverlayManager 获取 z-index，并使用绝对定位渲染：
  - 下拉根类名：`.adui-time-picker-dropdown`；
  - 内部面板：`.adui-time-picker-panel`；
- 三列布局：
  - 小时列：0–23，按 `hour_step` 生成；
  - 分钟列：0–59，按 `minute_step` 生成；
  - 秒列：0–59，按 `second_step` 生成；
- 单元格类名：
  - `.adui-time-picker-cell`：基础样式；
  - `.adui-time-picker-cell-active`：当前选中时刻对应的单元格。

### 3.3 打开/关闭与键盘行为

- 打开/关闭状态由内部 `Signal<bool>` 管理，配合：
  - `use_dropdown_layer(open)` 确保多层浮层 z-index 正确；
  - `use_floating_close_handle(open_signal)` 提供 click-outside + ESC 关闭逻辑；
- 行为细节：
  - 点击触发区：
    - 调用 `mark_internal_click()` 将事件标记为内部交互；
    - 切换 open 状态；
  - 点击下拉面板内部（任意列的单元格）：同样标记为内部交互，避免被「点击空白关闭」误伤；
  - 键盘：
    - Enter：打开下拉；
    - Esc：通过 `close_handle.close()` 关闭下拉；
  - 点击空白：document 级 click 监听在当前事件非内部时关闭下拉。

### 3.4 选择行为

- 点击任意列的单元格会构建一个新的 `TimeValue`：
  - 小时列：更新 hour，保留当前 minute/second；
  - 分钟列：更新 minute，保留当前 hour/second；
  - 秒列：更新 second，保留当前 hour/minute，并在选择后关闭下拉；
- 对外通过 `on_change(Some(TimeValue))` 通知；
- 非受控模式下内部 `Signal<TimeValue>` 同步更新。

## 4. 样式与类名

样式定义在 `src/theme.rs` 的 `adui_time_picker_style!` 宏中，并通过 `THEME_BASE_STYLE` 自动注入：

- 触发区：
  - `.adui-time-picker-root`：根容器（用于绝对定位下拉面板）；
  - `.adui-time-picker`：触发输入容器；
  - `.adui-time-picker-disabled`：禁用态；
  - `.adui-time-picker-input`：内部 input；
  - `.adui-time-picker-clear`：清除按钮；
- 下拉面板：
  - `.adui-time-picker-dropdown`：整体浮层；
  - `.adui-time-picker-panel`：三列布局容器；
  - `.adui-time-picker-column`：单列滚动区域；
  - `.adui-time-picker-cell`：单元格；
  - `.adui-time-picker-cell-active`：当前选中项。

## 5. 示例

示例文件：`examples/time_picker_demo.rs`

```rust
let mut basic_value = use_signal(|| None);
let mut stepped_value = use_signal(|| Some(TimeValue::new(9, 30, 0)));

TimePicker {
    value: *basic_value.read(),
    on_change: move |next| basic_value.set(next),
}

TimePicker {
    value: *stepped_value.read(),
    hour_step: Some(2),
    minute_step: Some(15),
    second_step: Some(30),
    on_change: move |next| stepped_value.set(next),
}
```

## 6. 与 AntD 差异与后续规划

当前 TimePicker 为 MVP 实现，与 Ant Design 6.x 相比尚缺少：

- 时间区间选择 TimeRangePicker；
- 12 小时制（AM/PM）及本地化格式；
- 复杂禁用规则（`disabledHours`/`disabledMinutes`/`disabledSeconds` 回调）；
- 自定义 footer/addon 区域；
- 与 DatePicker 的 `showTime` 组合模式。

这些能力计划在后续阶段按需补充。当前推荐用法：

- 单独使用 TimePicker 作为表单字段；
- 与 DatePicker/RangePicker 组合为「日期 + 时间」表单项；
- 在布局上遵循与 Input/Select 一致的样式（由 ThemeProvider/ConfigProvider 统一控制）。
