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

/// Props for the Badge component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Number to show in badge.
    #[props(optional)]
    pub count: Option<u32>,
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
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Wrapped element to display the badge on.
    pub children: Option<Element>,
}

/// Ant Design flavored Badge (MVP: count/dot/overflow/status).
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let BadgeProps {
        count,
        overflow_count,
        dot,
        show_zero,
        status,
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-badge".to_string()];
    if let Some(st) = status {
        class_list.push(st.as_class().into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // Determine what to render as indicator.
    let (show_indicator, is_dot, display_text) =
        compute_badge_indicator(count, overflow_count, dot, show_zero);

    rsx! {
        span { class: "{class_attr}", style: "{style_attr}",
            if let Some(node) = children { {node} }
            if show_indicator {
                if is_dot {
                    span { class: "adui-badge-dot" }
                } else {
                    span { class: "adui-badge-count", "{display_text}" }
                }
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
