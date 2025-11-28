# Calendar 说明

> 本文记录 Calendar 在 adui-dioxus 中的 MVP 实现与使用建议，对应 `plan/0013.md` 中 D 阶段任务。

## 1. 组件定位

- `Calendar`：日期视图组件，用于展示整月或全年视图，支持基础的日期选择与面板切换；
- 常与 Badge / Tag / Tooltip 等组合展示「日程标记」，以及与 Layout/Card 组合成简单日程视图。

当前实现聚焦月视图（`CalendarMode::Month`），年视图仅在 API 层预留。

## 2. 值模型与 Props

### 2.1 CalendarDate

Calendar 内部使用 `CalendarDate` 表达日期：

```rust
use adui_dioxus::CalendarDate;

pub struct CalendarDate {
    pub inner: time::Date,
}
```

辅助方法：

- `CalendarDate::from_ymd(year: i32, month: u8, day: u8) -> Option<CalendarDate>`；
- 访问器：`year()` / `month()` / `day()`。

### 2.2 CalendarMode

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarMode {
    Month,
    Year,
}
```

当前实装逻辑主要针对 `Month`，`Year` 作为占位模式，后续可扩展多年视图。

### 2.3 CalendarProps（MVP）

```rust
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    pub value: Option<CalendarDate>,
    pub default_value: Option<CalendarDate>,
    pub mode: Option<CalendarMode>,
    pub fullscreen: Option<bool>,
    pub on_select: Option<EventHandler<CalendarDate>>, 
    pub on_panel_change: Option<EventHandler<(CalendarDate, CalendarMode)>>,
    pub class: Option<String>,
    pub style: Option<String>,
}
```

说明：

- `value` / `default_value`：受控/非受控当前选中日期；
- `mode`：当前面板模式（默认 Month）；
- `fullscreen`：是否铺满容器，主要用于设置 `.adui-calendar-fullscreen` 类；
- `on_select`：用户点击日期单元格时触发；
- `on_panel_change`：
  - 在当前实现中，当通过前/后月按钮切换月份时，会以该月第一天和当前模式调用一次；
- `class` / `style`：附加类名/行内样式。

## 3. 结构与交互行为

### 3.1 月视图生成

- 使用内部 helpers 生成日期网格：
  - `days_in_month(year, month)`：返回当月天数；
  - `weekday_index_monday(year, month, day)`：返回给定日期对应的星期索引（周一=0 .. 周日=6）；
- 通过上述函数在当前年月下生成一个完整的 6 行 x 7 列网格：
  - 前导和尾部空白单元使用 `None` 占位，对应类名 `.adui-calendar-date-empty`；
  - 当前月份内的日期单元使用 `.adui-calendar-date-cell`。

### 3.2 选中逻辑

- 组件内部使用 `Signal<CalendarDate>` 保存当前选中日期（非受控时）；
- 点击日期单元格时：
  - 若 `cell_day` 存在，则构造对应的 `CalendarDate` 并更新内部 `selected`；
  - 无论受控/非受控，都会在有 `on_select` 时调用回调；
- 单元格高亮：
  - 当单元格对应的日期等于当前选中日期时，添加 `.adui-calendar-date-selected` 类。

### 3.3 月份切换

- Header 区域包含「前一月 / 当前年月 / 后一月」三部分：
  - 前一月按钮：
    - 若当前为 1 月，则切换至上一年 12 月；否则 `month -= 1`；
  - 后一月按钮：
    - 若当前为 12 月，则切换至下一年 1 月；否则 `month += 1`；
- 切换后：
  - 更新内部 `view_year` / `view_month`；
  - 若存在 `on_panel_change`，以新月份的第一天与当前 `mode` 调用一次回调。

> 当前实现未接入 click-outside 或 ESC 关闭逻辑，因为 Calendar 作为常驻内容组件，不使用浮层形式。

## 4. 样式与类名

样式定义在 `src/theme.rs` 的 `adui_calendar_style!` 宏中，并通过 `THEME_BASE_STYLE` 自动注入：

- 容器：
  - `.adui-calendar`：根容器，带边框与背景；
  - `.adui-calendar-fullscreen`：全宽模式；
- 头部与导航：
  - `.adui-calendar-header`：头部容器；
  - `.adui-calendar-nav-btn`：前/后月按钮；
  - `.adui-calendar-header-view`：当前年月文本；
- 星期与网格：
  - `.adui-calendar-week-row` / `.adui-calendar-week-cell`：星期行；
  - `.adui-calendar-body`：日期网格容器；
  - `.adui-calendar-date`：日期单元格根类；
  - `.adui-calendar-date-empty`：空白单元（非本月日期）；
  - `.adui-calendar-date-cell`：可点击日期单元；
  - `.adui-calendar-date-selected .adui-calendar-date-cell`：选中日期样式；
  - `.adui-calendar-date-value`：数字文本容器。

## 5. 示例

示例文件：`examples/calendar_demo.rs`

```rust
let mut selected = use_signal(|| None::<CalendarDate>);

Calendar {
    value: *selected.read(),
    on_select: move |date| {
        selected.set(Some(date));
    },
    fullscreen: Some(false),
    mode: Some(CalendarMode::Month),
}
```

## 6. 与 AntD 差异与后续规划

当前 Calendar 为 MVP 实现，相比 Ant Design 6.x 尚缺：

- 年视图（Year 模式）下的完整行为与网格生成；
- 日期单元格/头部的自定义渲染（如 `dateCellRender`、`monthCellRender` 等）；
- 更丰富的面板切换策略与动画；
- 与 RangePicker 等组合构成复杂日程视图的辅助 API。

后续可在新的 plan 中按需补充：

- Year 模式网格与交互；
- 单元格 Render 钩子以插入自定义内容；
- 与 Badge/Tag/Tooltip 的组合 demo（例如日程标记、事件列表）。
