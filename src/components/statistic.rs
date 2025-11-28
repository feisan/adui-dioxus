use dioxus::prelude::*;

/// Props for the Statistic component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct StatisticProps {
    /// Optional title shown above the value.
    #[props(optional)]
    pub title: Option<Element>,
    /// Numeric value to display.
    #[props(optional)]
    pub value: Option<f64>,
    /// Optional preformatted value text. When provided, this takes
    /// precedence over `value`.
    #[props(optional)]
    pub value_text: Option<String>,
    /// Optional decimal precision applied to `value`.
    #[props(optional)]
    pub precision: Option<u8>,
    /// Optional prefix element rendered before the value.
    #[props(optional)]
    pub prefix: Option<Element>,
    /// Optional suffix element rendered after the value.
    #[props(optional)]
    pub suffix: Option<Element>,
    /// Optional inline style for the value span (e.g. color).
    #[props(optional)]
    pub value_style: Option<String>,
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style on the root element.
    #[props(optional)]
    pub style: Option<String>,
}

fn format_value(props: &StatisticProps) -> String {
    if let Some(text) = &props.value_text {
        return text.clone();
    }
    let value = props.value.unwrap_or(0.0);
    if let Some(precision) = props.precision {
        let p: usize = precision as usize;
        format!("{value:.p$}")
    } else {
        // Remove trailing .0 for integers
        let s = format!("{value}");
        if s.ends_with(".0") {
            s.trim_end_matches(".0").to_string()
        } else {
            s
        }
    }
}

/// Ant Design flavored Statistic (MVP: value + prefix/suffix + precision).
#[component]
pub fn Statistic(props: StatisticProps) -> Element {
    let display_text = format_value(&props);

    let mut class_list = vec!["adui-statistic".to_string()];
    if let Some(extra) = props.class.clone() {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = props.style.clone().unwrap_or_default();
    let value_style_attr = props.value_style.clone().unwrap_or_default();

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if let Some(title) = props.title.clone() {
                div { class: "adui-statistic-title", {title} }
            }
            div { class: "adui-statistic-content",
                if let Some(prefix) = props.prefix.clone() {
                    span { class: "adui-statistic-prefix", {prefix} }
                }
                span { class: "adui-statistic-value", style: "{value_style_attr}", "{display_text}" }
                if let Some(suffix) = props.suffix.clone() {
                    span { class: "adui-statistic-suffix", {suffix} }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_value_uses_value_text_first() {
        let props = StatisticProps {
            title: None,
            value: Some(123.456),
            value_text: Some("custom".into()),
            precision: None,
            prefix: None,
            suffix: None,
            value_style: None,
            class: None,
            style: None,
        };
        assert_eq!(format_value(&props), "custom");
    }

    #[test]
    fn format_value_applies_precision() {
        let props = StatisticProps {
            title: None,
            value: Some(std::f64::consts::PI),
            value_text: None,
            precision: Some(2),
            prefix: None,
            suffix: None,
            value_style: None,
            class: None,
            style: None,
        };
        assert_eq!(format_value(&props), "3.14");
    }

    #[test]
    fn format_value_trims_trailing_point_zero() {
        let props = StatisticProps {
            title: None,
            value: Some(10.0),
            value_text: None,
            precision: None,
            prefix: None,
            suffix: None,
            value_style: None,
            class: None,
            style: None,
        };
        assert_eq!(format_value(&props), "10");
    }
}
