use crate::components::config_provider::{Locale, use_config};
use crate::components::floating::use_floating_close_handle;
use crate::components::select_base::use_dropdown_layer;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use time::Date;

// Internal value used by RangePicker to represent a possibly-partial range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateRangeValue {
    pub start: Option<DateValue>,
    pub end: Option<DateValue>,
}

impl DateRangeValue {
    pub fn empty() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}

/// Internal value type for DatePicker.
///
/// MVP 选择一个尽量简单且可序列化的模型：
/// - 内部使用 `time::Date` 负责日期计算；
/// - 对外可以通过辅助函数转换为 `YYYY-MM-DD` 字符串，便于与 Form 的 `serde_json::Value` 协作。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateValue {
    pub inner: Date,
}

impl DateValue {
    /// Convenience constructor from year, month, day.
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Option<Self> {
        // We keep validation delegated to `time` to ensure correctness.
        let month_enum = time::Month::try_from(month).ok()?;
        Date::from_calendar_date(year, month_enum, day)
            .ok()
            .map(|d| DateValue { inner: d })
    }

    /// Format as `YYYY-MM-DD` string (MVP 默认格式)。
    pub fn to_ymd_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            self.inner.year(),
            self.inner.month() as u8,
            self.inner.day()
        )
    }
}

/// Compute the number of days in a given month of a given year.
fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // Gregorian leap year rules.
            let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if is_leap { 29 } else { 28 }
        }
        _ => 30,
    }
}

/// Compute weekday index for a given date, with Monday = 0 .. Sunday = 6.
fn weekday_index_monday(year: i32, month: u8, day: u8) -> u8 {
    // Tomohiko Sakamoto's algorithm, returning 0 = Sunday .. 6 = Saturday.
    let m = month as i32;
    let d = day as i32;
    let mut y = year;
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if m < 3 {
        y -= 1;
    }
    let w = (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d) % 7;
    let sunday_based = w as u8; // 0 = Sunday
    // Convert to Monday-based index: Monday = 0 .. Sunday = 6.
    (sunday_based + 6) % 7
}

/// Props for the DatePicker component (MVP subset for single date picker).
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// Controlled value. When set, the component behaves as a controlled picker.
    #[props(optional)]
    pub value: Option<DateValue>,
    /// Initial value in uncontrolled mode.
    #[props(optional)]
    pub default_value: Option<DateValue>,
    /// Placeholder text shown when no value is selected.
    #[props(optional)]
    pub placeholder: Option<String>,
    /// Display/parse format. MVP: primarily `YYYY-MM-DD`, reserved for future extension.
    #[props(optional)]
    pub format: Option<String>,
    /// Whether the picker is disabled.
    #[props(optional)]
    pub disabled: Option<bool>,
    /// Whether to show a clear icon which resets the current value.
    #[props(optional)]
    pub allow_clear: Option<bool>,
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Change callback fired when the selected date changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<Option<DateValue>>>,
}

/// Ant Design flavored DatePicker (MVP: single-date picker with dropdown
/// calendar). RangePicker and richer behaviours will be built on top in later
/// steps.
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let DatePickerProps {
        value,
        default_value,
        placeholder,
        format: _format,
        disabled,
        allow_clear,
        class,
        style,
        on_change,
    } = props;

    let config = use_config();
    let locale = config.locale;

    let is_disabled = disabled.unwrap_or(false);
    let allow_clear_flag = allow_clear.unwrap_or(false);

    // Internal selection state used only when not controlled.
    let selected_state: Signal<Option<DateValue>> = use_signal(|| default_value);
    let current_value: Option<DateValue> = if let Some(v) = value {
        Some(v)
    } else {
        *selected_state.read()
    };

    let display_text = current_value.map(|v| v.to_ymd_string()).unwrap_or_default();

    let default_placeholder = match locale {
        Locale::ZhCN => "请选择日期".to_string(),
        Locale::EnUS => "Select date".to_string(),
    };
    let placeholder_str = placeholder.unwrap_or(default_placeholder);

    let controlled = value.is_some();

    // Dropdown open/close state.
    let open_state: Signal<bool> = use_signal(|| false);
    let open_flag = *open_state.read();

    // Register dropdown layer to get a stable z-index when open and install
    // click-outside handler.
    let close_handle = use_floating_close_handle(open_state);
    let dropdown_layer = use_dropdown_layer(open_flag);
    let current_z = *dropdown_layer.z_index.read();

    // Visible calendar month/year. Default to current value if present, else a
    // fixed reference (2024-01) to keep behaviour deterministic across
    // environments without relying on system time.
    let initial_year = current_value.map(|v| v.inner.year()).unwrap_or(2024);
    let initial_month = current_value.map(|v| v.inner.month() as u8).unwrap_or(1);
    let view_year: Signal<i32> = use_signal(|| initial_year);
    let view_month: Signal<u8> = use_signal(|| initial_month);

    let year_now = *view_year.read();
    let month_now = *view_month.read();
    let days_in_month_now = days_in_month(year_now, month_now);
    let first_weekday = weekday_index_monday(year_now, month_now, 1) as usize;
    let total_cells = first_weekday + days_in_month_now as usize;
    let padded_cells = total_cells.div_ceil(7) * 7; // round up to full weeks

    let locale_for_header = locale;
    let month_label = match locale_for_header {
        Locale::ZhCN => format!("{year_now}年{month_now}月"),
        Locale::EnUS => format!("{year_now}-{month_now:02}"),
    };
    let weekday_labels: [&str; 7] = match locale_for_header {
        Locale::ZhCN => ["一", "二", "三", "四", "五", "六", "日"],
        Locale::EnUS => ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
    };

    // Build root/control classes.
    let mut control_classes = vec!["adui-date-picker".to_string()];
    if is_disabled {
        control_classes.push("adui-date-picker-disabled".to_string());
    }
    if let Some(extra) = class.clone() {
        control_classes.push(extra);
    }
    let control_class_attr = control_classes.join(" ");
    let style_attr = style.unwrap_or_default();

    // Shared handles for handlers.
    let selected_for_day_click = selected_state;
    let on_change_cb = on_change;
    let controlled_flag = controlled;
    let open_for_toggle = open_state;
    let open_for_keydown = open_for_toggle;

    // Derived flags.
    let has_value = current_value.is_some();

    // Precompute calendar cells for the current view.
    let mut date_cells: Vec<Element> = Vec::new();
    for index in 0..padded_cells {
        let cell_day =
            if index < first_weekday || index >= first_weekday + days_in_month_now as usize {
                None
            } else {
                Some((index - first_weekday + 1) as u8)
            };

        let is_outside = cell_day.is_none();
        let is_selected = if let (Some(day), Some(current)) = (cell_day, current_value) {
            current.inner.year() == year_now
                && current.inner.month() as u8 == month_now
                && current.inner.day() == day
        } else {
            false
        };

        let mut cell_classes = vec!["adui-date-picker-cell".to_string()];
        if is_outside {
            cell_classes.push("adui-date-picker-cell-empty".to_string());
        } else {
            cell_classes.push("adui-date-picker-cell-date".to_string());
        }
        if is_selected {
            cell_classes.push("adui-date-picker-cell-selected".to_string());
        }
        let cell_class_attr = cell_classes.join(" ");

        let selected_state_for_cell = selected_for_day_click;
        let on_change_cb_for_cell = on_change_cb;
        let controlled_flag_for_cell = controlled_flag;
        let close_for_cell = close_handle;

        let cell_node = rsx! {
            span {
                class: "{cell_class_attr}",
                onclick: move |_| {
                    close_for_cell.mark_internal_click();
                    if let Some(day) = cell_day
                        && let Some(value) = DateValue::from_ymd(year_now, month_now, day)
                    {
                        if controlled_flag_for_cell {
                            if let Some(cb) = on_change_cb_for_cell {
                                cb.call(Some(value));
                            }
                        } else {
                            let mut state = selected_state_for_cell;
                            state.set(Some(value));
                            if let Some(cb) = on_change_cb_for_cell {
                                cb.call(Some(value));
                            }
                        }
                        // 选择后关闭面板。
                        close_for_cell.close();
                    }
                },
                match cell_day {
                    Some(day) => rsx!{ "{day}" },
                    None => rsx!{ "" },
                }
            }
        };

        date_cells.push(cell_node);
    }

    rsx! {
        div {
            class: "adui-date-picker-root",
            style: "position: relative; display: inline-block;",
            div {
                class: "{control_class_attr}",
                style: "{style_attr}",
                role: "combobox",
                tabindex: (!is_disabled).then_some(0),
                "aria-expanded": open_flag,
                "aria-disabled": is_disabled,
                onclick: move |_| {
                    if is_disabled {
                        return;
                    }
                    close_handle.mark_internal_click();
                    let mut open_signal = open_for_toggle;
                    let next = !*open_signal.read();
                    open_signal.set(next);
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if is_disabled {
                        return;
                    }
                    use dioxus::prelude::Key;
                    match evt.key() {
                        Key::Enter => {
                            evt.prevent_default();
                            let mut open_signal = open_for_keydown;
                            open_signal.set(true);
                        }
                        Key::Escape => {
                            close_handle.close();
                        }
                        _ => {}
                    }
                },
                input {
                    class: "adui-date-picker-input",
                    readonly: true,
                    disabled: is_disabled,
                    value: "{display_text}",
                    placeholder: "{placeholder_str}",
                }
                if allow_clear_flag && has_value && !is_disabled {
                    span {
                        class: "adui-date-picker-clear",
                        onclick: move |_| {
                            if controlled_flag {
                                if let Some(cb) = on_change_cb {
                                    cb.call(None);
                                }
                            } else {
                                let mut state = selected_for_day_click;
                                state.set(None);
                                if let Some(cb) = on_change_cb {
                                    cb.call(None);
                                }
                            }
                        },
                        "×"
                    }
                }
            }

            if open_flag {
                // Simple calendar dropdown.
                div {
                    class: "adui-date-picker-dropdown",
                    style: "position: absolute; top: 100%; left: 0; min-width: 100%; z-index: {current_z};",
                    // Header with month navigation.
                    div { class: "adui-date-picker-header",
                        button {
                            class: "adui-date-picker-nav-btn adui-date-picker-prev-month",
                            onclick: move |_| {
                                close_handle.mark_internal_click();
                                let mut year = *view_year.read();
                                let mut month = *view_month.read();
                                if month == 1 {
                                    month = 12;
                                    year -= 1;
                                } else {
                                    month -= 1;
                                }
                                let mut y = view_year;
                                let mut m = view_month;
                                y.set(year);
                                m.set(month);
                            },
                            "<"
                        }
                        span { class: "adui-date-picker-header-view", "{month_label}" }
                        button {
                            class: "adui-date-picker-nav-btn adui-date-picker-next-month",
                            onclick: move |_| {
                                close_handle.mark_internal_click();
                                let mut year = *view_year.read();
                                let mut month = *view_month.read();
                                if month == 12 {
                                    month = 1;
                                    year += 1;
                                } else {
                                    month += 1;
                                }
                                let mut y = view_year;
                                let mut m = view_month;
                                y.set(year);
                                m.set(month);
                            },
                            ">"
                        }
                    }

                    // Weekday header row.
                    div { class: "adui-date-picker-week-row",
                        for label in weekday_labels {
                            span { class: "adui-date-picker-week-cell", "{label}" }
                        }
                    }

                    // Date grid.
                    div { class: "adui-date-picker-body",
                        for cell in date_cells { {cell} }
                    }
                }
            }
        }
    }
}

/// Props for the RangePicker component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct RangePickerProps {
    /// Controlled range value. Each side may be `None` to represent a
    /// partially selected range.
    #[props(optional)]
    pub value: Option<DateRangeValue>,
    /// Initial value in uncontrolled mode.
    #[props(optional)]
    pub default_value: Option<DateRangeValue>,
    /// Placeholders for the start and end inputs.
    #[props(optional)]
    pub placeholder: Option<(String, String)>,
    /// Display/parse format. MVP: primarily `YYYY-MM-DD`.
    #[props(optional)]
    pub format: Option<String>,
    /// Whether the picker is disabled.
    #[props(optional)]
    pub disabled: Option<bool>,
    /// Whether to show a clear icon which resets the current range.
    #[props(optional)]
    pub allow_clear: Option<bool>,
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Change callback fired when the range changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<DateRangeValue>>,
}

/// MVP RangePicker: single-month range selection with basic highlighting.
#[component]
pub fn RangePicker(props: RangePickerProps) -> Element {
    let RangePickerProps {
        value,
        default_value,
        placeholder,
        format: _format,
        disabled,
        allow_clear,
        class,
        style,
        on_change,
    } = props;

    let config = use_config();
    let locale = config.locale;

    let is_disabled = disabled.unwrap_or(false);
    let allow_clear_flag = allow_clear.unwrap_or(false);

    let initial_range = default_value.unwrap_or_else(DateRangeValue::empty);
    let range_state: Signal<DateRangeValue> = use_signal(|| initial_range);
    let range = value.unwrap_or_else(|| *range_state.read());

    let start_text = range.start.map(|d| d.to_ymd_string()).unwrap_or_default();
    let end_text = range.end.map(|d| d.to_ymd_string()).unwrap_or_default();

    let default_placeholder = match locale {
        Locale::ZhCN => ("开始日期".to_string(), "结束日期".to_string()),
        Locale::EnUS => ("Start date".to_string(), "End date".to_string()),
    };
    let (start_ph, end_ph) = placeholder.unwrap_or(default_placeholder);

    let controlled = value.is_some();

    let open_state: Signal<bool> = use_signal(|| false);
    let open_flag = *open_state.read();
    let dropdown_layer = use_dropdown_layer(open_flag);
    let current_z = *dropdown_layer.z_index.read();

    // Visible calendar month/year. Use start date if present, otherwise end,
    // otherwise a fixed reference.
    let initial_year = range
        .start
        .or(range.end)
        .map(|v| v.inner.year())
        .unwrap_or(2024);
    let initial_month = range
        .start
        .or(range.end)
        .map(|v| v.inner.month() as u8)
        .unwrap_or(1);

    let view_year: Signal<i32> = use_signal(|| initial_year);
    let view_month: Signal<u8> = use_signal(|| initial_month);

    let year_now = *view_year.read();
    let month_now = *view_month.read();
    let days_in_month_now = days_in_month(year_now, month_now);
    let first_weekday = weekday_index_monday(year_now, month_now, 1) as usize;
    let total_cells = first_weekday + days_in_month_now as usize;
    let padded_cells = total_cells.div_ceil(7) * 7;

    let locale_for_header = locale;
    let month_label = match locale_for_header {
        Locale::ZhCN => format!("{year_now}年{month_now}月"),
        Locale::EnUS => format!("{year_now}-{month_now:02}"),
    };
    let weekday_labels: [&str; 7] = match locale_for_header {
        Locale::ZhCN => ["一", "二", "三", "四", "五", "六", "日"],
        Locale::EnUS => ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
    };

    let mut control_classes = vec!["adui-date-picker adui-date-picker-range".to_string()];
    if is_disabled {
        control_classes.push("adui-date-picker-disabled".to_string());
    }
    if let Some(extra) = class.clone() {
        control_classes.push(extra);
    }
    let control_class_attr = control_classes.join(" ");
    let style_attr = style.unwrap_or_default();

    let open_for_toggle = open_state;

    let on_change_cb = on_change;
    let range_for_click = range_state;

    // Shared floating close handle for range picker dropdown.
    let close_handle = use_floating_close_handle(open_for_toggle);

    // Precompute date cells with simple range highlighting.
    let mut date_cells: Vec<Element> = Vec::new();
    for index in 0..padded_cells {
        let cell_day =
            if index < first_weekday || index >= first_weekday + days_in_month_now as usize {
                None
            } else {
                Some((index - first_weekday + 1) as u8)
            };

        let is_outside = cell_day.is_none();
        let mut is_selected_start = false;
        let mut is_selected_end = false;
        let mut in_range = false;

        if let Some(day) = cell_day {
            if let Some(start) = range.start
                && start.inner.year() == year_now
                && start.inner.month() as u8 == month_now
                && start.inner.day() == day
            {
                is_selected_start = true;
            }
            if let Some(end) = range.end
                && end.inner.year() == year_now
                && end.inner.month() as u8 == month_now
                && end.inner.day() == day
            {
                is_selected_end = true;
            }
            if let (Some(start), Some(end)) = (range.start, range.end) {
                let date = DateValue::from_ymd(year_now, month_now, day).unwrap();
                if date.inner >= start.inner && date.inner <= end.inner {
                    in_range = true;
                }
            }
        }

        let mut cell_classes = vec!["adui-date-picker-cell".to_string()];
        if is_outside {
            cell_classes.push("adui-date-picker-cell-empty".to_string());
        } else {
            cell_classes.push("adui-date-picker-cell-date".to_string());
        }
        if in_range {
            cell_classes.push("adui-date-picker-cell-in-range".to_string());
        }
        if is_selected_start {
            cell_classes.push("adui-date-picker-cell-range-start".to_string());
        }
        if is_selected_end {
            cell_classes.push("adui-date-picker-cell-range-end".to_string());
        }
        let cell_class_attr = cell_classes.join(" ");

        let on_change_for_cell = on_change_cb;
        let controlled_for_cell = controlled;
        let close_for_cell = close_handle;

        let cell_node = rsx! {
            span {
                class: "{cell_class_attr}",
                onclick: move |_| {
                    if is_disabled {
                        return;
                    }
                    close_for_cell.mark_internal_click();
                    if let Some(day) = cell_day
                        && let Some(clicked) = DateValue::from_ymd(year_now, month_now, day)
                    {
                        let mut next = range;
                        match (next.start, next.end) {
                            (None, _) => {
                                next.start = Some(clicked);
                                next.end = None;
                            }
                            (Some(start), None) => {
                                if clicked.inner < start.inner {
                                    next.start = Some(clicked);
                                    next.end = Some(start);
                                } else {
                                    next.end = Some(clicked);
                                }

                                close_for_cell.close();
                            }
                            (Some(_), Some(_)) => {
                                next.start = Some(clicked);
                                next.end = None;
                            }
                        }

                        if controlled_for_cell {
                            if let Some(cb) = on_change_for_cell {
                                cb.call(next);
                            }
                        } else {
                            let mut state = range_for_click;
                            state.set(next);
                            if let Some(cb) = on_change_for_cell {
                                cb.call(next);
                            }
                        }
                    }
                },
                match cell_day {
                    Some(day) => rsx!{ "{day}" },
                    None => rsx!{ "" },
                }
            }
        };

        date_cells.push(cell_node);
    }

    let has_any_value = range.start.is_some() || range.end.is_some();

    rsx! {
        div {
            class: "adui-date-picker-root",
            style: "position: relative; display: inline-block;",
            div {
                class: "{control_class_attr}",
                style: "{style_attr}",
                role: "group",
                tabindex: (!is_disabled).then_some(0),
                onclick: move |_| {
                    if is_disabled {
                        return;
                    }
                    close_handle.mark_internal_click();
                    let mut open_signal = open_for_toggle;
                    let next = !*open_signal.read();
                    open_signal.set(next);
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if is_disabled {
                        return;
                    }
                    use dioxus::prelude::Key;
                    match evt.key() {
                        Key::Enter => {
                            evt.prevent_default();
                            let mut open_signal = open_for_toggle;
                            open_signal.set(true);
                        }
                        Key::Escape => {
                            close_handle.close();
                        }
                        _ => {}
                    }
                },
                input {
                    class: "adui-date-picker-input adui-date-picker-input-start",
                    readonly: true,
                    disabled: is_disabled,
                    value: "{start_text}",
                    placeholder: "{start_ph}",
                }
                span { class: "adui-date-picker-range-separator", " ~ " }
                input {
                    class: "adui-date-picker-input adui-date-picker-input-end",
                    readonly: true,
                    disabled: is_disabled,
                    value: "{end_text}",
                    placeholder: "{end_ph}",
                }
                if allow_clear_flag && has_any_value && !is_disabled {
                    span {
                        class: "adui-date-picker-clear",
                        onclick: move |_| {
                            if controlled {
                                if let Some(cb) = on_change {
                                    cb.call(DateRangeValue::empty());
                                }
                            } else {
                                let mut state = range_state;
                                state.set(DateRangeValue::empty());
                                if let Some(cb) = on_change {
                                    cb.call(DateRangeValue::empty());
                                }
                            }
                        },
                        "×"
                    }
                }
            }

            if open_flag {
                div {
                    class: "adui-date-picker-dropdown",
                    style: "position: absolute; top: 100%; left: 0; min-width: 100%; z-index: {current_z};",
                    div { class: "adui-date-picker-header",
                        button {
                            class: "adui-date-picker-nav-btn adui-date-picker-prev-month",
                            onclick: move |_| {
                                close_handle.mark_internal_click();
                                let mut year = *view_year.read();
                                let mut month = *view_month.read();
                                if month == 1 {
                                    month = 12;
                                    year -= 1;
                                } else {
                                    month -= 1;
                                }
                                let mut y = view_year;
                                let mut m = view_month;
                                y.set(year);
                                m.set(month);
                            },
                            "<"
                        }
                        span { class: "adui-date-picker-header-view", "{month_label}" }
                        button {
                            class: "adui-date-picker-nav-btn adui-date-picker-next-month",
                            onclick: move |_| {
                                close_handle.mark_internal_click();
                                let mut year = *view_year.read();
                                let mut month = *view_month.read();
                                if month == 12 {
                                    month = 1;
                                    year += 1;
                                } else {
                                    month += 1;
                                }
                                let mut y = view_year;
                                let mut m = view_month;
                                y.set(year);
                                m.set(month);
                            },
                            ">"
                        }
                    }

                    div { class: "adui-date-picker-week-row",
                        for label in weekday_labels {
                            span { class: "adui-date-picker-week-cell", "{label}" }
                        }
                    }

                    div { class: "adui-date-picker-body",
                        for cell in date_cells { {cell} }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_value_from_ymd_and_to_string_are_consistent() {
        let value = DateValue::from_ymd(2024, 5, 17).expect("valid date");
        assert_eq!(value.to_ymd_string(), "2024-05-17");
    }
}
