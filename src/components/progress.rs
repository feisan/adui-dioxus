use dioxus::prelude::*;

/// Visual status of a Progress bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressStatus {
    Normal,
    Success,
    Exception,
    Active,
}

impl ProgressStatus {
    fn as_class(&self) -> &'static str {
        match self {
            ProgressStatus::Normal => "adui-progress-status-normal",
            ProgressStatus::Success => "adui-progress-status-success",
            ProgressStatus::Exception => "adui-progress-status-exception",
            ProgressStatus::Active => "adui-progress-status-active",
        }
    }
}

/// Progress visual type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressType {
    Line,
    Circle,
}

/// Props for the Progress component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Percentage in the range [0.0, 100.0]. Values outside this range
    /// will be clamped.
    #[props(default = 0.0)]
    pub percent: f32,
    /// Optional status. When omitted and `percent >= 100`, the component
    /// will treat the status as `Success` for styling.
    #[props(optional)]
    pub status: Option<ProgressStatus>,
    /// Whether to render the textual percentage.
    #[props(default = true)]
    pub show_info: bool,
    /// Visual type of the progress indicator.
    #[props(default = ProgressType::Line)]
    pub r#type: ProgressType,
    /// Optional stroke width (height for line, border width for circle).
    #[props(optional)]
    pub stroke_width: Option<f32>,
    /// Extra CSS class name on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style on the root element.
    #[props(optional)]
    pub style: Option<String>,
}

fn clamp_percent(value: f32) -> f32 {
    if value.is_nan() {
        0.0
    } else {
        value.clamp(0.0, 100.0)
    }
}

fn resolve_status(percent: f32, status: Option<ProgressStatus>) -> ProgressStatus {
    if let Some(s) = status {
        s
    } else if percent >= 100.0 {
        ProgressStatus::Success
    } else {
        ProgressStatus::Normal
    }
}

/// Ant Design flavored Progress (MVP: line + simple circle).
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let ProgressProps {
        percent,
        status,
        show_info,
        r#type,
        stroke_width,
        class,
        style,
    } = props;

    let percent = clamp_percent(percent);
    let status_value = resolve_status(percent, status);

    let mut class_list = vec![
        "adui-progress".to_string(),
        status_value.as_class().to_string(),
    ];
    match r#type {
        ProgressType::Line => class_list.push("adui-progress-line".into()),
        ProgressType::Circle => class_list.push("adui-progress-circle".into()),
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let display_text = format!("{}%", percent.round() as i32);

    match r#type {
        ProgressType::Line => {
            let height = stroke_width.unwrap_or(6.0);
            rsx! {
                div { class: "{class_attr}", style: "{style_attr}",
                    div { class: "adui-progress-outer",
                        div { class: "adui-progress-inner",
                            div {
                                class: "adui-progress-bg",
                                style: "width:{percent}%;height:{height}px;",
                            }
                        }
                    }
                    if show_info {
                        span { class: "adui-progress-text", "{display_text}" }
                    }
                }
            }
        }
        ProgressType::Circle => {
            let size = 80.0f32;
            let border = stroke_width.unwrap_or(6.0);
            let circle_style = format!(
                "width:{size}px;height:{size}px;border-width:{border}px;background:conic-gradient(currentColor {percent}%, rgba(0,0,0,0.06) 0);",
            );

            rsx! {
                div { class: "{class_attr}", style: "{style_attr}",
                    div { class: "adui-progress-circle-inner", style: "{circle_style}", }
                    if show_info {
                        div { class: "adui-progress-text", "{display_text}" }
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
    fn clamp_percent_bounds_values() {
        assert_eq!(clamp_percent(-10.0), 0.0);
        assert_eq!(clamp_percent(0.0), 0.0);
        assert_eq!(clamp_percent(50.0), 50.0);
        assert_eq!(clamp_percent(120.0), 100.0);
    }

    #[test]
    fn resolve_status_defaults_to_success_when_full() {
        assert_eq!(resolve_status(100.0, None), ProgressStatus::Success);
        assert_eq!(resolve_status(50.0, None), ProgressStatus::Normal);
        assert_eq!(
            resolve_status(80.0, Some(ProgressStatus::Exception)),
            ProgressStatus::Exception
        );
    }
}
