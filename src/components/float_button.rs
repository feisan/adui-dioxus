use crate::theme::{ThemeTokens, use_theme};
use dioxus::prelude::*;
use web_sys::window;

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

/// Simplified badge config (subset of antd badge).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BadgeConfig {
    pub content: Option<String>,
    pub class: Option<String>,
    pub dot: bool,
}

impl BadgeConfig {
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }

    pub fn dot() -> Self {
        Self {
            dot: true,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FloatButtonGroupContext {
    shape: FloatButtonShape,
    kind: FloatButtonType,
}

/// Group container for multiple float buttons.
#[derive(Props, Clone, PartialEq)]
pub struct FloatButtonGroupProps {
    #[props(default)]
    pub shape: FloatButtonShape,
    #[props(default)]
    pub r#type: FloatButtonType,
    #[props(default = 12.0)]
    pub gap: f32,
    #[props(optional)]
    pub right: Option<f32>,
    #[props(optional)]
    pub left: Option<f32>,
    #[props(optional)]
    pub top: Option<f32>,
    #[props(optional)]
    pub bottom: Option<f32>,
    #[props(optional)]
    pub z_index: Option<i32>,
    #[props(default)]
    pub pure: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Wrap float buttons together (minimal vertical stack).
#[component]
pub fn FloatButtonGroup(props: FloatButtonGroupProps) -> Element {
    let FloatButtonGroupProps {
        shape,
        r#type,
        gap,
        right,
        left,
        top,
        bottom,
        z_index,
        pure,
        class,
        style,
        children,
    } = props;
    use_context_provider(|| FloatButtonGroupContext {
        shape,
        kind: r#type,
    });

    let mut class_list = vec!["adui-float-btn-group".to_string()];
    if pure {
        class_list.push("adui-float-btn-group-pure".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let placement = if pure {
        String::new()
    } else {
        format!(
            "{}{}{}{}{}",
            right
                .map(|v| format!("right:{v}px;"))
                .unwrap_or_else(|| "right:24px;".into()),
            left.map(|v| format!("left:{v}px;")).unwrap_or_default(),
            top.map(|v| format!("top:{v}px;")).unwrap_or_default(),
            bottom
                .map(|v| format!("bottom:{v}px;"))
                .unwrap_or_else(|| "bottom:72px;".into()),
            z_index
                .map(|z| format!("z-index:{z};"))
                .unwrap_or_else(|| "z-index:99;".into()),
        )
    };

    let style_attr = format!(
        "--adui-fb-group-gap:{}px;{}{}",
        gap,
        placement,
        style.unwrap_or_default()
    );
    rsx! {
        div { class: "{class_attr}", style: "{style_attr}", {children} }
    }
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
    pub content: Option<String>,
    #[props(optional)]
    pub badge: Option<BadgeConfig>,
    #[props(optional)]
    pub tooltip: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub class_names_root: Option<String>,
    #[props(optional)]
    pub class_names_icon: Option<String>,
    #[props(optional)]
    pub class_names_content: Option<String>,
    #[props(optional)]
    pub styles_root: Option<String>,
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
    pub z_index: Option<i32>,
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// BackTop helper: scrolls window to top and renders a FloatButton.
#[derive(Props, Clone, PartialEq)]
pub struct BackTopProps {
    #[props(default = FloatButtonType::Primary)]
    pub r#type: FloatButtonType,
    #[props(default)]
    pub shape: FloatButtonShape,
    #[props(default)]
    pub danger: bool,
    #[props(optional)]
    pub tooltip: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub icon: Option<Element>,
    #[props(optional)]
    pub description: Option<String>,
    #[props(optional)]
    pub content: Option<String>,
    #[props(optional)]
    pub badge: Option<BadgeConfig>,
    #[props(optional)]
    pub right: Option<f32>,
    #[props(optional)]
    pub left: Option<f32>,
    #[props(optional)]
    pub top: Option<f32>,
    #[props(optional)]
    pub bottom: Option<f32>,
    #[props(optional)]
    pub z_index: Option<i32>,
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn BackTop(props: BackTopProps) -> Element {
    let BackTopProps {
        r#type,
        shape,
        danger,
        tooltip,
        class,
        style,
        icon,
        description,
        content,
        badge,
        right,
        left,
        top,
        bottom,
        z_index,
        onclick,
    } = props;
    let default_icon = icon.unwrap_or_else(|| rsx!(span { "↑" }));
    let handler = onclick;
    rsx! {
        FloatButton {
            r#type,
            shape,
            danger,
            tooltip: tooltip.clone(),
            class: class.clone(),
            style: style.clone(),
            icon: Some(default_icon),
            description,
            content,
            badge,
            right,
            left,
            top,
            bottom,
            z_index,
            onclick: move |evt: Event<MouseData>| {
                if let Some(h) = handler.as_ref() {
                    h.call(evt.clone());
                }
                if let Some(win) = window() {
                    win.scroll_to_with_x_and_y(0.0, 0.0);
                }
            }
        }
    }
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
        content,
        badge,
        tooltip,
        class,
        class_names_root,
        class_names_icon,
        class_names_content,
        styles_root,
        style,
        right,
        left,
        top,
        bottom,
        z_index,
        onclick,
    } = props;

    let theme = use_theme();
    let tokens = theme.tokens();
    let group_ctx = try_use_context::<FloatButtonGroupContext>();
    let is_grouped = group_ctx.is_some();
    let (merged_shape, merged_type) = if let Some(ctx) = group_ctx {
        (ctx.shape, ctx.kind)
    } else {
        (shape, r#type)
    };
    let visuals = visuals(&tokens, merged_type, danger);
    let metrics = metrics(merged_shape);
    let text_slot = content.clone().or(description.clone());
    let has_content = text_slot.is_some();

    let mut class_list = vec!["adui-float-btn".to_string()];
    class_list.push(match merged_type {
        FloatButtonType::Primary => "adui-float-btn-primary".into(),
        FloatButtonType::Default => "adui-float-btn-default".into(),
    });
    class_list.push(match merged_shape {
        FloatButtonShape::Circle => "adui-float-btn-circle".into(),
        FloatButtonShape::Square => "adui-float-btn-square".into(),
    });
    if !is_grouped {
        class_list.push("adui-float-btn-individual".into());
    }
    if !has_content {
        class_list.push("adui-float-btn-icon-only".into());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    if let Some(extra) = class_names_root.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let placement = if is_grouped {
        String::new()
    } else {
        format!(
            "{}{}{}{}{}",
            right
                .map(|v| format!("right:{v}px;"))
                .unwrap_or_else(|| "right:24px;".into()),
            left.map(|v| format!("left:{v}px;")).unwrap_or_default(),
            top.map(|v| format!("top:{v}px;")).unwrap_or_default(),
            bottom
                .map(|v| format!("bottom:{v}px;"))
                .unwrap_or_else(|| "bottom:72px;".into()),
            z_index
                .map(|z| format!("z-index:{z};"))
                .unwrap_or_else(|| "z-index:99;".into()),
        )
    };

    let style_attr = format!(
        "--adui-fb-bg:{};--adui-fb-bg-hover:{};--adui-fb-bg-active:{};\
        --adui-fb-color:{};--adui-fb-color-hover:{};--adui-fb-color-active:{};\
        --adui-fb-border:{};--adui-fb-border-hover:{};--adui-fb-border-active:{};\
        --adui-fb-radius:{}px;--adui-fb-shadow:{};\
        --adui-fb-size:{}px;--adui-fb-padding-inline:{}px;\
        {}{}{}",
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
        metrics.size,
        metrics.padding_inline,
        placement,
        styles_root.unwrap_or_default(),
        style.unwrap_or_default()
    );

    let mut icon_class = "adui-float-btn-icon".to_string();
    if let Some(extra) = class_names_icon.as_ref() {
        icon_class.push(' ');
        icon_class.push_str(extra);
    }
    let mut content_class = "adui-float-btn-content".to_string();
    if let Some(extra) = class_names_content.as_ref() {
        content_class.push(' ');
        content_class.push_str(extra);
    }

    let badge_node = badge.map(|cfg| {
        let BadgeConfig {
            content,
            class,
            dot,
        } = cfg;
        let mut badge_class = "adui-float-btn-badge".to_string();
        if dot {
            badge_class.push_str(" adui-float-btn-badge-dot");
        }
        if let Some(extra) = class {
            badge_class.push(' ');
            badge_class.push_str(&extra);
        }
        rsx!(span { class: "{badge_class}",
            if !dot {
                if let Some(text) = content.clone() {
                    "{text}"
                }
            }
        })
    });

    let contents = rsx! {
        if let Some(icon_node) = icon {
            span { class: "{icon_class}", {icon_node} }
        }
        if let Some(desc) = text_slot.clone() {
            span { class: "{content_class}", "{desc}" }
        }
        if let Some(node) = badge_node {
            {node}
        }
    };

    let title_attr = tooltip.clone().unwrap_or_default();
    let aria_label = if title_attr.is_empty() {
        "float button".to_string()
    } else {
        title_attr.clone()
    };

    if let Some(href_val) = href {
        let handler = onclick;
        return rsx! {
            a {
                class: "{class_attr}",
                style: "{style_attr}",
                href: "{href_val}",
                role: "button",
                title: "{title_attr}",
                "aria-label": "{aria_label}",
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
            title: "{title_attr}",
            "aria-label": "{aria_label}",
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
    size: f32,
    padding_inline: f32,
}

fn metrics(shape: FloatButtonShape) -> FloatMetrics {
    match shape {
        FloatButtonShape::Circle => FloatMetrics {
            radius: 28.0,
            size: 56.0,
            padding_inline: 0.0,
        },
        FloatButtonShape::Square => FloatMetrics {
            radius: 16.0,
            size: 56.0,
            padding_inline: 12.0,
        },
    }
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
