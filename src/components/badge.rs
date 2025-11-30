use dioxus::prelude::*;

/// Status style for Badge (MVP subset).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeStatus {
    Default,
    Success,
    Warning,
    Error,
}

impl BadgeStatus {
    fn as_class(&self) -> &'static str {
        match self {
            BadgeStatus::Default => "adui-badge-status-default",
            BadgeStatus::Success => "adui-badge-status-success",
            BadgeStatus::Warning => "adui-badge-status-warning",
            BadgeStatus::Error => "adui-badge-status-error",
        }
    }
}

fn compute_badge_indicator(
    count: Option<u32>,
    overflow_count: u32,
    dot: bool,
    show_zero: bool,
) -> (bool, bool, String) {
    if dot {
        (true, true, String::new())
    } else if let Some(c) = count {
        if c == 0 && !show_zero {
            (false, false, String::new())
        } else {
            let text = if c > overflow_count {
                format!("{}+", overflow_count)
            } else {
                c.to_string()
            };
            (true, false, text)
        }
    } else {
        (false, false, String::new())
    }
}

/// Badge color configuration.
#[derive(Clone, Debug, PartialEq)]
pub enum BadgeColor {
    /// Preset color (primary, success, warning, error, etc.).
    Preset(String),
    /// Custom color (hex, rgb, etc.).
    Custom(String),
}

/// Badge size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    #[default]
    Default,
    Small,
}

/// Props for the Badge component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Number or custom element to show in badge.
    /// Can be a number (u32) or a custom Element.
    #[props(optional)]
    pub count: Option<Element>,
    /// Numeric count (for backward compatibility and simple cases).
    #[props(optional)]
    pub count_number: Option<u32>,
    /// Max count to show before displaying "overflow+".
    #[props(default = 99)]
    pub overflow_count: u32,
    /// Whether to show red dot without number.
    #[props(default)]
    pub dot: bool,
    /// Whether to show badge when count is zero.
    #[props(default)]
    pub show_zero: bool,
    /// Optional semantic status (default/success/warning/error).
    #[props(optional)]
    pub status: Option<BadgeStatus>,
    /// Badge color (preset or custom).
    #[props(optional)]
    pub color: Option<BadgeColor>,
    /// Text shown next to status indicator (for status mode).
    #[props(optional)]
    pub text: Option<String>,
    /// Badge size.
    #[props(default)]
    pub size: BadgeSize,
    /// Offset position [x, y] for badge placement.
    #[props(optional)]
    pub offset: Option<(f32, f32)>,
    /// Title attribute for badge (tooltip).
    #[props(optional)]
    pub title: Option<String>,
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Wrapped element to display the badge on.
    pub children: Option<Element>,
}

/// Ant Design flavored Badge.
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let BadgeProps {
        count,
        count_number,
        overflow_count,
        dot,
        show_zero,
        status,
        color,
        text,
        size,
        offset,
        title,
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-badge".to_string()];
    if let Some(st) = status {
        class_list.push(st.as_class().into());
    }
    if matches!(size, BadgeSize::Small) {
        class_list.push("adui-badge-sm".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let mut style_attr = style.unwrap_or_default();
    if let Some((x, y)) = offset {
        style_attr.push_str(&format!(
            "--adui-badge-offset-x: {}px; --adui-badge-offset-y: {}px;",
            x, y
        ));
    }
    if let Some(BadgeColor::Custom(color_str)) = color {
        style_attr.push_str(&format!("--adui-badge-color: {};", color_str));
    } else if let Some(BadgeColor::Preset(preset)) = color {
        class_list.push(format!("adui-badge-{}", preset));
    }

    // Determine what to render as indicator.
    let count_value = count_number;
    let (show_indicator, is_dot, display_text) =
        compute_badge_indicator(count_value, overflow_count, dot, show_zero);

    let title_attr = title.unwrap_or_default();

    rsx! {
        span {
            class: "{class_attr}",
            style: "{style_attr}",
            title: "{title_attr}",
            if let Some(node) = children { {node} }
            if show_indicator {
                if is_dot {
                    span { class: "adui-badge-dot" }
                } else {
                    span {
                        class: "adui-badge-count",
                        if let Some(custom_count) = count {
                            {custom_count}
                        } else {
                            "{display_text}"
                        }
                    }
                }
            }
            if let Some(status_text) = text {
                if status.is_some() {
                    span { class: "adui-badge-status-text", "{status_text}" }
                }
            }
        }
    }
}

/// Ribbon badge component (sub-component of Badge).
#[derive(Props, Clone, PartialEq)]
pub struct RibbonProps {
    /// Ribbon text content.
    pub text: String,
    /// Ribbon color.
    #[props(optional)]
    pub color: Option<BadgeColor>,
    /// Placement of the ribbon.
    #[props(default)]
    pub placement: RibbonPlacement,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Ribbon placement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RibbonPlacement {
    #[default]
    End,
    Start,
}

/// Ribbon badge component.
#[component]
pub fn Ribbon(props: RibbonProps) -> Element {
    let RibbonProps {
        text,
        color,
        placement,
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-badge-ribbon".to_string()];
    if matches!(placement, RibbonPlacement::Start) {
        class_list.push("adui-badge-ribbon-start".into());
    } else {
        class_list.push("adui-badge-ribbon-end".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let mut style_attr = style.unwrap_or_default();
    if let Some(BadgeColor::Custom(color_str)) = color {
        style_attr.push_str(&format!("--adui-badge-ribbon-color: {};", color_str));
    } else if let Some(BadgeColor::Preset(preset)) = color {
        class_list.push(format!("adui-badge-ribbon-{}", preset));
    }

    rsx! {
        div { class: "adui-badge-ribbon-wrapper",
            {children}
            div { class: "{class_attr}", style: "{style_attr}",
                span { class: "adui-badge-ribbon-text", "{text}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_mode_ignores_count_and_shows_dot() {
        let (show, is_dot, text) = compute_badge_indicator(Some(5), 99, true, false);
        assert!(show);
        assert!(is_dot);
        assert!(text.is_empty());
    }

    #[test]
    fn zero_count_respects_show_zero_flag() {
        let (show1, _, _) = compute_badge_indicator(Some(0), 99, false, false);
        assert!(!show1);

        let (show2, is_dot2, text2) = compute_badge_indicator(Some(0), 99, false, true);
        assert!(show2);
        assert!(!is_dot2);
        assert_eq!(text2, "0");
    }

    #[test]
    fn count_overflow_is_capped() {
        let (show, is_dot, text) = compute_badge_indicator(Some(120), 99, false, true);
        assert!(show);
        assert!(!is_dot);
        assert_eq!(text, "99+");
    }
}
