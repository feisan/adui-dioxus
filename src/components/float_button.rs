use crate::theme::{ThemeTokens, use_theme};
use dioxus::prelude::*;

/// Visual style for the floating action button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatButtonType {
    Default,
    #[default]
    Primary,
}

/// Shape of the floating action button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FloatButtonShape {
    #[default]
    Circle,
    Square,
}

/// Floating action button props.
#[derive(Props, Clone, PartialEq)]
pub struct FloatButtonProps {
    #[props(default)]
    pub r#type: FloatButtonType,
    #[props(default)]
    pub shape: FloatButtonShape,
    #[props(default)]
    pub danger: bool,
    #[props(optional)]
    pub href: Option<String>,
    #[props(optional)]
    pub icon: Option<Element>,
    #[props(optional)]
    pub description: Option<String>,
    #[props(optional)]
    pub tooltip: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub right: Option<f32>,
    #[props(optional)]
    pub left: Option<f32>,
    #[props(optional)]
    pub top: Option<f32>,
    #[props(optional)]
    pub bottom: Option<f32>,
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// Floating action button with Ant Design flavored theming.
#[component]
pub fn FloatButton(props: FloatButtonProps) -> Element {
    let FloatButtonProps {
        r#type,
        shape,
        danger,
        href,
        icon,
        description,
        tooltip,
        class,
        style,
        right,
        left,
        top,
        bottom,
        onclick,
    } = props;

    let theme = use_theme();
    let tokens = theme.tokens();
    let visuals = visuals(&tokens, r#type, danger);
    let metrics = metrics(shape);

    let mut class_list = vec!["adui-float-btn".to_string()];
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let placement = format!(
        "{}{}{}{}",
        right
            .map(|v| format!("right:{}px;", v))
            .unwrap_or_else(|| "right:24px;".into()),
        left.map(|v| format!("left:{}px;", v)).unwrap_or_default(),
        top.map(|v| format!("top:{}px;", v)).unwrap_or_default(),
        bottom
            .map(|v| format!("bottom:{}px;", v))
            .unwrap_or_else(|| "bottom:72px;".into())
    );

    let style_attr = format!(
        "--adui-fb-bg:{};--adui-fb-bg-hover:{};--adui-fb-bg-active:{};\
        --adui-fb-color:{};--adui-fb-color-hover:{};--adui-fb-color-active:{};\
        --adui-fb-border:{};--adui-fb-border-hover:{};--adui-fb-border-active:{};\
        --adui-fb-radius:{}px;--adui-fb-shadow:{};{}{}",
        visuals.bg,
        visuals.bg_hover,
        visuals.bg_active,
        visuals.color,
        visuals.color_hover,
        visuals.color_active,
        visuals.border,
        visuals.border_hover,
        visuals.border_active,
        metrics.radius,
        visuals.shadow,
        placement,
        style.unwrap_or_default()
    );

    let contents = rsx! {
        if let Some(icon_node) = icon {
            span { class: "adui-float-btn-icon", {icon_node} }
        }
        if let Some(desc) = description {
            span { class: "adui-float-btn-desc", "{desc}" }
        }
    };

    if let Some(href_val) = href {
        let handler = onclick;
        return rsx! {
            a {
                class: "{class_attr}",
                style: "{style_attr}",
                href: "{href_val}",
                role: "button",
                "aria-label": tooltip.as_deref().unwrap_or("float button"),
                onclick: move |evt| {
                    if let Some(h) = handler.as_ref() {
                        h.call(evt);
                    }
                },
                {contents}
            }
        };
    }

    let handler = onclick;
    rsx! {
        button {
            class: "{class_attr}",
            style: "{style_attr}",
            r#type: "button",
            role: "button",
            "aria-label": tooltip.as_deref().unwrap_or("float button"),
            onclick: move |evt| {
                if let Some(h) = handler.as_ref() {
                    h.call(evt);
                }
            },
            {contents}
        }
    }
}

struct FloatVisuals {
    bg: String,
    bg_hover: String,
    bg_active: String,
    color: String,
    color_hover: String,
    color_active: String,
    border: String,
    border_hover: String,
    border_active: String,
    shadow: String,
}

struct FloatMetrics {
    radius: f32,
}

fn metrics(shape: FloatButtonShape) -> FloatMetrics {
    let radius = match shape {
        FloatButtonShape::Circle => 28.0,
        FloatButtonShape::Square => 12.0,
    };
    FloatMetrics { radius }
}

fn visuals(tokens: &ThemeTokens, kind: FloatButtonType, danger: bool) -> FloatVisuals {
    let (accent, accent_hover, accent_active) = if danger {
        (
            tokens.color_error.clone(),
            tokens.color_error_hover.clone(),
            tokens.color_error_active.clone(),
        )
    } else {
        (
            tokens.color_primary.clone(),
            tokens.color_primary_hover.clone(),
            tokens.color_primary_active.clone(),
        )
    };

    match kind {
        FloatButtonType::Primary => FloatVisuals {
            bg: accent.clone(),
            bg_hover: accent_hover.clone(),
            bg_active: accent_active.clone(),
            color: "#ffffff".into(),
            color_hover: "#ffffff".into(),
            color_active: "#ffffff".into(),
            border: accent.clone(),
            border_hover: accent_hover.clone(),
            border_active: accent_active.clone(),
            shadow: "0 6px 16px rgba(0,0,0,0.2)".into(),
        },
        FloatButtonType::Default => FloatVisuals {
            bg: tokens.color_bg_container.clone(),
            bg_hover: tokens.color_bg_container.clone(),
            bg_active: tokens.color_bg_container.clone(),
            color: tokens.color_text.clone(),
            color_hover: tokens.color_primary.clone(),
            color_active: tokens.color_primary_active.clone(),
            border: tokens.color_border.clone(),
            border_hover: tokens.color_border_hover.clone(),
            border_active: tokens.color_primary_active.clone(),
            shadow: "0 6px 16px rgba(0,0,0,0.12)".into(),
        },
    }
}
