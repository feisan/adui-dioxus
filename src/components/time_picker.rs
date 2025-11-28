use crate::components::config_provider::{Locale, use_config};
use crate::components::floating::use_floating_close_handle;
use crate::components::select_base::use_dropdown_layer;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;

/// Internal value type for TimePicker (HH:mm:ss).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeValue {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl TimeValue {
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    /// Clamp components to a safe range and normalise into a `TimeValue`.
    pub fn normalised(hour: i32, minute: i32, second: i32) -> Self {
        let h = hour.clamp(0, 23) as u8;
        let m = minute.clamp(0, 59) as u8;
        let s = second.clamp(0, 59) as u8;
        TimeValue {
            hour: h,
            minute: m,
            second: s,
        }
    }

    /// Format as `HH:mm:ss` text.
    pub fn to_hms_string(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

/// Props for the TimePicker component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct TimePickerProps {
    /// Controlled time value.
    #[props(optional)]
    pub value: Option<TimeValue>,
    /// Initial value in uncontrolled mode.
    #[props(optional)]
    pub default_value: Option<TimeValue>,
    /// Placeholder shown when no time is selected.
    #[props(optional)]
    pub placeholder: Option<String>,
    /// Display/parse format. MVP: primarily `HH:mm:ss`.
    #[props(optional)]
    pub format: Option<String>,
    /// Step for hour column.
    #[props(optional)]
    pub hour_step: Option<u8>,
    /// Step for minute column.
    #[props(optional)]
    pub minute_step: Option<u8>,
    /// Step for second column.
    #[props(optional)]
    pub second_step: Option<u8>,
    /// Whether the picker is disabled.
    #[props(optional)]
    pub disabled: Option<bool>,
    /// Whether to show a clear icon.
    #[props(optional)]
    pub allow_clear: Option<bool>,
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Change callback.
    #[props(optional)]
    pub on_change: Option<EventHandler<Option<TimeValue>>>,
}

/// Ant Design flavored TimePicker (MVP: HH:mm:ss with simple steps and dropdown).
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let TimePickerProps {
        value,
        default_value,
        placeholder,
        format: _format,
        hour_step,
        minute_step,
        second_step,
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

    let step_h = hour_step.unwrap_or(1).max(1);
    let step_m = minute_step.unwrap_or(1).max(1);
    let step_s = second_step.unwrap_or(1).max(1);

    let initial_inner = default_value.unwrap_or(TimeValue::new(0, 0, 0));
    let inner_state: Signal<TimeValue> = use_signal(|| initial_inner);

    let current_value = if let Some(v) = value {
        v
    } else {
        *inner_state.read()
    };
    let display_text = if value.is_some() || default_value.is_some() {
        current_value.to_hms_string()
    } else {
        String::new()
    };

    let default_placeholder = match locale {
        Locale::ZhCN => "请选择时间".to_string(),
        Locale::EnUS => "Select time".to_string(),
    };
    let placeholder_str = placeholder.unwrap_or(default_placeholder);

    let controlled = value.is_some();

    let TimeValue {
        hour: current_hour,
        minute: current_minute,
        second: current_second,
    } = current_value;

    // Dropdown open/close state.
    let open_state: Signal<bool> = use_signal(|| false);
    let open_flag = *open_state.read();
    let close_handle = use_floating_close_handle(open_state);
    let dropdown_layer = use_dropdown_layer(open_flag);
    let current_z = *dropdown_layer.z_index.read();

    let mut control_classes = vec!["adui-time-picker".to_string()];
    if is_disabled {
        control_classes.push("adui-time-picker-disabled".to_string());
    }
    if let Some(extra) = class.clone() {
        control_classes.push(extra);
    }
    let control_class_attr = control_classes.join(" ");
    let style_attr = style.unwrap_or_default();

    // Prepare option lists.
    let hours: Vec<u8> = (0..24).step_by(step_h as usize).collect();
    let minutes: Vec<u8> = (0..60).step_by(step_m as usize).collect();
    let seconds: Vec<u8> = (0..60).step_by(step_s as usize).collect();

    // Decorated cells with active class and zero-padded label.
    let hour_cells: Vec<(u8, String, String)> = hours
        .iter()
        .map(|&h| {
            let mut classes = vec!["adui-time-picker-cell".to_string()];
            if current_hour == h {
                classes.push("adui-time-picker-cell-active".to_string());
            }
            let class_attr = classes.join(" ");
            let label = format!("{:02}", h);
            (h, class_attr, label)
        })
        .collect();
    let minute_cells: Vec<(u8, String, String)> = minutes
        .iter()
        .map(|&m| {
            let mut classes = vec!["adui-time-picker-cell".to_string()];
            if current_minute == m {
                classes.push("adui-time-picker-cell-active".to_string());
            }
            let class_attr = classes.join(" ");
            let label = format!("{:02}", m);
            (m, class_attr, label)
        })
        .collect();
    let second_cells: Vec<(u8, String, String)> = seconds
        .iter()
        .map(|&s| {
            let mut classes = vec!["adui-time-picker-cell".to_string()];
            if current_second == s {
                classes.push("adui-time-picker-cell-active".to_string());
            }
            let class_attr = classes.join(" ");
            let label = format!("{:02}", s);
            (s, class_attr, label)
        })
        .collect();

    // Shared helper to apply a new time value.
    let on_change_cb = on_change;
    let mut inner_for_change = inner_state;
    let apply_time = move |next: TimeValue| {
        if controlled {
            if let Some(cb) = on_change_cb {
                cb.call(Some(next));
            }
        } else {
            let mut state = inner_for_change;
            state.set(next);
            if let Some(cb) = on_change_cb {
                cb.call(Some(next));
            }
        }
    };

    rsx! {
        div {
            class: "adui-time-picker-root",
            style: "position: relative; display: inline-block;",
            div {
                class: "{control_class_attr}",
                style: "{style_attr}",
                role: "combobox",
                tabindex: (!is_disabled).then_some(0),
                "aria-expanded": open_flag,
                "aria-disabled": is_disabled,
                onclick: move |_| {
                    if is_disabled { return; }
                    close_handle.mark_internal_click();
                    let mut open_signal = open_state;
                    let next = !*open_signal.read();
                    open_signal.set(next);
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if is_disabled { return; }
                    use dioxus::prelude::Key;
                    match evt.key() {
                        Key::Enter => {
                            evt.prevent_default();
                            let mut open_signal = open_state;
                            open_signal.set(true);
                        }
                        Key::Escape => {
                            close_handle.close();
                        }
                        _ => {}
                    }
                },
                input {
                    class: "adui-time-picker-input",
                    readonly: true,
                    disabled: is_disabled,
                    value: "{display_text}",
                    placeholder: "{placeholder_str}",
                }
                if allow_clear_flag && !display_text.is_empty() && !is_disabled {
                    span {
                        class: "adui-time-picker-clear",
                        onclick: move |_| {
                            close_handle.mark_internal_click();
                            if controlled {
                                if let Some(cb) = on_change {
                                    cb.call(None);
                                }
                            } else {
                                let mut state = inner_state;
                                state.set(TimeValue::new(0, 0, 0));
                                if let Some(cb) = on_change {
                                    cb.call(None);
                                }
                            }
                        },
                        "×"
                    }
                }
            }

            if open_flag {
                div {
                    class: "adui-time-picker-dropdown",
                    style: "position: absolute; top: 100%; left: 0; min-width: 100%; z-index: {current_z};",
                    div { class: "adui-time-picker-panel",
                        // Hours column
                        div { class: "adui-time-picker-column",
                            for (h, class_attr, label) in hour_cells {
                                span {
                                    class: "{class_attr}",
                                    onclick: move |_| {
                                        close_handle.mark_internal_click();
                                        let next = TimeValue::new(h, current_minute, current_second);
                                        apply_time(next);
                                    },
                                    "{label}"
                                }
                            }
                        }
                        // Minutes column
                        div { class: "adui-time-picker-column",
                            for (m, class_attr, label) in minute_cells {
                                span {
                                    class: "{class_attr}",
                                    onclick: move |_| {
                                        close_handle.mark_internal_click();
                                        let next = TimeValue::new(current_hour, m, current_second);
                                        apply_time(next);
                                    },
                                    "{label}"
                                }
                            }
                        }
                        // Seconds column
                        div { class: "adui-time-picker-column",
                            for (s, class_attr, label) in second_cells {
                                span {
                                    class: "{class_attr}",
                                    onclick: move |_| {
                                        close_handle.mark_internal_click();
                                        let next = TimeValue::new(current_hour, current_minute, s);
                                        apply_time(next);
                                        // 默认在选择秒后关闭面板。
                                        close_handle.close();
                                    },
                                    "{label}"
                                }
                            }
                        }
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
    fn time_value_to_string_roundtrip() {
        let v = TimeValue::new(9, 5, 7);
        assert_eq!(v.to_hms_string(), "09:05:07");
    }
}
