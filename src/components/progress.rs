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
    fn clamp_percent_handles_nan() {
        assert_eq!(clamp_percent(f32::NAN), 0.0);
    }

    #[test]
    fn clamp_percent_handles_infinity() {
        assert_eq!(clamp_percent(f32::INFINITY), 100.0);
        assert_eq!(clamp_percent(f32::NEG_INFINITY), 0.0);
    }

    #[test]
    fn clamp_percent_boundary_values() {
        assert_eq!(clamp_percent(0.0), 0.0);
        assert_eq!(clamp_percent(100.0), 100.0);
        assert_eq!(clamp_percent(0.1), 0.1);
        assert_eq!(clamp_percent(99.9), 99.9);
        assert_eq!(clamp_percent(-0.1), 0.0);
        assert_eq!(clamp_percent(100.1), 100.0);
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

    #[test]
    fn resolve_status_respects_explicit_status() {
        assert_eq!(
            resolve_status(0.0, Some(ProgressStatus::Success)),
            ProgressStatus::Success
        );
        assert_eq!(
            resolve_status(100.0, Some(ProgressStatus::Exception)),
            ProgressStatus::Exception
        );
        assert_eq!(
            resolve_status(50.0, Some(ProgressStatus::Active)),
            ProgressStatus::Active
        );
        assert_eq!(
            resolve_status(75.0, Some(ProgressStatus::Normal)),
            ProgressStatus::Normal
        );
    }

    #[test]
    fn resolve_status_auto_success_at_100() {
        assert_eq!(resolve_status(100.0, None), ProgressStatus::Success);
        assert_eq!(resolve_status(100.1, None), ProgressStatus::Success);
    }

    #[test]
    fn resolve_status_auto_normal_below_100() {
        assert_eq!(resolve_status(0.0, None), ProgressStatus::Normal);
        assert_eq!(resolve_status(50.0, None), ProgressStatus::Normal);
        assert_eq!(resolve_status(99.9, None), ProgressStatus::Normal);
    }

    #[test]
    fn progress_status_class_mapping() {
        assert_eq!(
            ProgressStatus::Normal.as_class(),
            "adui-progress-status-normal"
        );
        assert_eq!(
            ProgressStatus::Success.as_class(),
            "adui-progress-status-success"
        );
        assert_eq!(
            ProgressStatus::Exception.as_class(),
            "adui-progress-status-exception"
        );
        assert_eq!(
            ProgressStatus::Active.as_class(),
            "adui-progress-status-active"
        );
    }

    #[test]
    fn progress_status_all_variants() {
        let variants = [
            ProgressStatus::Normal,
            ProgressStatus::Success,
            ProgressStatus::Exception,
            ProgressStatus::Active,
        ];
        for variant in variants.iter() {
            let class = variant.as_class();
            assert!(!class.is_empty());
            assert!(class.starts_with("adui-progress-status-"));
        }
    }

    #[test]
    fn progress_status_equality() {
        assert_eq!(ProgressStatus::Normal, ProgressStatus::Normal);
        assert_eq!(ProgressStatus::Success, ProgressStatus::Success);
        assert_ne!(ProgressStatus::Normal, ProgressStatus::Success);
        assert_ne!(ProgressStatus::Exception, ProgressStatus::Active);
    }

    #[test]
    fn progress_status_clone() {
        let original = ProgressStatus::Active;
        let cloned = original;
        assert_eq!(original, cloned);
        assert_eq!(original.as_class(), cloned.as_class());
    }

    #[test]
    fn progress_type_equality() {
        assert_eq!(ProgressType::Line, ProgressType::Line);
        assert_eq!(ProgressType::Circle, ProgressType::Circle);
        assert_ne!(ProgressType::Line, ProgressType::Circle);
    }

    #[test]
    fn progress_type_clone() {
        let original = ProgressType::Circle;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn progress_type_debug() {
        let line = ProgressType::Line;
        let circle = ProgressType::Circle;
        let line_str = format!("{:?}", line);
        let circle_str = format!("{:?}", circle);
        assert!(line_str.contains("Line"));
        assert!(circle_str.contains("Circle"));
    }

    #[test]
    fn progress_props_defaults() {
        // ProgressProps requires no mandatory fields
        // percent defaults to 0.0
        // show_info defaults to true
        // type defaults to ProgressType::Line
    }

    #[test]
    fn clamp_percent_edge_cases() {
        // Test with very small negative values
        assert_eq!(clamp_percent(-0.1), 0.0);
        assert_eq!(clamp_percent(-100.0), 0.0);

        // Test with very large positive values
        assert_eq!(clamp_percent(100.1), 100.0);
        assert_eq!(clamp_percent(1000.0), 100.0);
    }

    #[test]
    fn clamp_percent_precision() {
        // Test with floating point precision
        assert_eq!(clamp_percent(0.0001), 0.0001);
        assert_eq!(clamp_percent(99.9999), 99.9999);
    }

    #[test]
    fn resolve_status_explicit_overrides_auto() {
        // Explicit status should always override auto status
        assert_eq!(
            resolve_status(100.0, Some(ProgressStatus::Exception)),
            ProgressStatus::Exception
        );
        assert_eq!(
            resolve_status(0.0, Some(ProgressStatus::Success)),
            ProgressStatus::Success
        );
    }

    #[test]
    fn progress_status_all_variants_equality() {
        let statuses = [
            ProgressStatus::Normal,
            ProgressStatus::Success,
            ProgressStatus::Exception,
            ProgressStatus::Active,
        ];
        for (i, status1) in statuses.iter().enumerate() {
            for (j, status2) in statuses.iter().enumerate() {
                if i == j {
                    assert_eq!(status1, status2);
                } else {
                    assert_ne!(status1, status2);
                }
            }
        }
    }

    #[test]
    fn progress_status_class_prefix() {
        // All progress status classes should start with "adui-progress-status-"
        assert!(
            ProgressStatus::Normal
                .as_class()
                .starts_with("adui-progress-status-")
        );
        assert!(
            ProgressStatus::Success
                .as_class()
                .starts_with("adui-progress-status-")
        );
        assert!(
            ProgressStatus::Exception
                .as_class()
                .starts_with("adui-progress-status-")
        );
        assert!(
            ProgressStatus::Active
                .as_class()
                .starts_with("adui-progress-status-")
        );
    }

    #[test]
    fn progress_status_unique_classes() {
        // All progress status classes should be unique
        let classes: Vec<&str> = vec![
            ProgressStatus::Normal.as_class(),
            ProgressStatus::Success.as_class(),
            ProgressStatus::Exception.as_class(),
            ProgressStatus::Active.as_class(),
        ];
        for (i, class1) in classes.iter().enumerate() {
            for (j, class2) in classes.iter().enumerate() {
                if i != j {
                    assert_ne!(class1, class2);
                }
            }
        }
    }

    #[test]
    fn progress_type_all_variants() {
        assert_eq!(ProgressType::Line, ProgressType::Line);
        assert_eq!(ProgressType::Circle, ProgressType::Circle);
        assert_ne!(ProgressType::Line, ProgressType::Circle);
    }

    #[test]
    fn progress_type_copy_semantics() {
        // ProgressType should be Copy
        let progress_type = ProgressType::Circle;
        let progress_type2 = progress_type;
        assert_eq!(progress_type, progress_type2);
    }

    #[test]
    fn clamp_percent_zero_and_hundred() {
        // Boundary values should remain unchanged
        assert_eq!(clamp_percent(0.0), 0.0);
        assert_eq!(clamp_percent(100.0), 100.0);
    }
}
