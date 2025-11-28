use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use time::{Date, Month};

use crate::components::config_provider::{Locale, use_config};

/// Internal value type for Calendar (date without time).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarDate {
    pub inner: Date,
}

impl CalendarDate {
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Option<Self> {
        let m = Month::try_from(month).ok()?;
        Date::from_calendar_date(year, m, day)
            .ok()
            .map(|d| CalendarDate { inner: d })
    }

    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    pub fn month(&self) -> u8 {
        self.inner.month() as u8
    }

    pub fn day(&self) -> u8 {
        self.inner.day()
    }
}

/// Calendar mode controls the type of panel displayed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    #[default]
    Month,
    Year,
}

/// Props for the Calendar component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// Controlled value: the currently selected date.
    #[props(optional)]
    pub value: Option<CalendarDate>,
    /// Initial value when uncontrolled.
    #[props(optional)]
    pub default_value: Option<CalendarDate>,
    /// Current panel mode (month/year).
    #[props(optional)]
    pub mode: Option<CalendarMode>,
    /// Whether the calendar should take full width of the container.
    #[props(optional)]
    pub fullscreen: Option<bool>,
    /// Selection callback.
    #[props(optional)]
    pub on_select: Option<EventHandler<CalendarDate>>,
    /// Panel change callback (mode or month/year changed).
    #[props(optional)]
    pub on_panel_change: Option<EventHandler<(CalendarDate, CalendarMode)>>,
    /// Extra class for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if is_leap { 29 } else { 28 }
        }
        _ => 30,
    }
}

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
    let sunday_based = w as u8;
    (sunday_based + 6) % 7
}

/// Ant Design flavored Calendar (MVP: month/year mode with single-date select).
#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let CalendarProps {
        value,
        default_value,
        mode,
        fullscreen,
        on_select,
        on_panel_change,
        class,
        style,
    } = props;

    let config = use_config();
    let locale = config.locale;

    let initial_date = value.or(default_value).unwrap_or_else(|| {
        // Fixed reference date to keep behaviour deterministic across
        // environments without relying on system time.
        CalendarDate::from_ymd(2024, 1, 1).expect("valid date")
    });

    let selected: Signal<CalendarDate> = use_signal(|| initial_date);
    let current_mode: Signal<CalendarMode> = use_signal(|| mode.unwrap_or_default());

    let view_year: Signal<i32> = use_signal(|| initial_date.year());
    let view_month: Signal<u8> = use_signal(|| initial_date.month());

    // For month view.
    let year_now = *view_year.read();
    let month_now = *view_month.read();
    let days_in_month_now = days_in_month(year_now, month_now);
    let first_weekday = weekday_index_monday(year_now, month_now, 1) as usize;
    let total_cells = first_weekday + days_in_month_now as usize;
    let padded_cells = total_cells.div_ceil(7) * 7;

    let locale_for_header = locale;
    let header_label = match locale_for_header {
        Locale::ZhCN => format!("{year_now}年{month_now}月"),
        Locale::EnUS => format!("{year_now}-{month_now:02}"),
    };
    let weekday_labels: [&str; 7] = match locale_for_header {
        Locale::ZhCN => ["一", "二", "三", "四", "五", "六", "日"],
        Locale::EnUS => ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
    };

    let fullscreen_flag = fullscreen.unwrap_or(false);

    let mut class_list = vec!["adui-calendar".to_string()];
    if fullscreen_flag {
        class_list.push("adui-calendar-fullscreen".to_string());
    }
    if let Some(extra) = class.clone() {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let mode_signal = current_mode;
    let on_panel_change_cb = on_panel_change;

    let selected_for_click = selected;
    let on_select_cb = on_select;

    // Build date cells for month mode.
    let mut date_cells: Vec<Element> = Vec::new();
    let selected_now = *selected_for_click.read();

    for index in 0..padded_cells {
        let cell_day =
            if index < first_weekday || index >= first_weekday + days_in_month_now as usize {
                None
            } else {
                Some((index - first_weekday + 1) as u8)
            };

        let is_outside = cell_day.is_none();
        let mut cell_classes = vec!["adui-calendar-date".to_string()];
        if is_outside {
            cell_classes.push("adui-calendar-date-empty".to_string());
        } else {
            cell_classes.push("adui-calendar-date-cell".to_string());
        }
        if let (Some(day), CalendarDate { inner }) = (cell_day, selected_now)
            && inner.year() == year_now
            && inner.month() as u8 == month_now
            && inner.day() == day
        {
            cell_classes.push("adui-calendar-date-selected".to_string());
        }
        let cell_class_attr = cell_classes.join(" ");

        let on_click_day = {
            let mut selected_state = selected_for_click;
            move |_: MouseEvent| {
                if let Some(day) = cell_day
                    && let Some(date) = CalendarDate::from_ymd(year_now, month_now, day)
                {
                    selected_state.set(date);
                    if let Some(cb) = on_select_cb {
                        cb.call(date);
                    }
                }
            }
        };

        let cell_node = rsx! {
            div {
                class: "{cell_class_attr}",
                onclick: on_click_day,
                match cell_day {
                    Some(day) => rsx!{ span { class: "adui-calendar-date-value", "{day}" } },
                    None => rsx!{ span { class: "adui-calendar-date-value", "" } },
                }
            }
        };

        date_cells.push(cell_node);
    }

    let on_prev_month = {
        let mut vy = view_year;
        let mut vm = view_month;
        move |_| {
            if *mode_signal.read() != CalendarMode::Month {
                return;
            }
            let mut year = *vy.read();
            let mut month = *vm.read();
            if month == 1 {
                month = 12;
                year -= 1;
            } else {
                month -= 1;
            }
            vy.set(year);
            vm.set(month);
            if let Some(cb) = on_panel_change_cb
                && let Some(date) = CalendarDate::from_ymd(year, month, 1)
            {
                cb.call((date, CalendarMode::Month));
            }
        }
    };

    let on_next_month = {
        let mut vy = view_year;
        let mut vm = view_month;
        move |_| {
            if *mode_signal.read() != CalendarMode::Month {
                return;
            }
            let mut year = *vy.read();
            let mut month = *vm.read();
            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }
            vy.set(year);
            vm.set(month);
            if let Some(cb) = on_panel_change_cb
                && let Some(date) = CalendarDate::from_ymd(year, month, 1)
            {
                cb.call((date, CalendarMode::Month));
            }
        }
    };

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            // Header with month navigation for month mode.
            div { class: "adui-calendar-header",
                button { class: "adui-calendar-nav-btn", onclick: on_prev_month, "<" }
                span { class: "adui-calendar-header-view", "{header_label}" }
                button { class: "adui-calendar-nav-btn", onclick: on_next_month, ">" }
            }

            // Weekday header.
            div { class: "adui-calendar-week-row",
                for label in weekday_labels {
                    span { class: "adui-calendar-week-cell", "{label}" }
                }
            }

            // Date grid.
            div { class: "adui-calendar-body",
                for cell in date_cells { {cell} }
            }
        }
    }
}
