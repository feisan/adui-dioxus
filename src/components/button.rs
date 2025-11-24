use crate::theme::{ThemeTokens, use_theme};
use dioxus::prelude::*;

/// Supported button visual types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Default,
    Primary,
    Dashed,
    Text,
    Link,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Default
    }
}

/// Button size variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Middle,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Middle
    }
}

/// Shape variants for the button outline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonShape {
    Default,
    Round,
    Circle,
}

impl Default for ButtonShape {
    fn default() -> Self {
        ButtonShape::Default
    }
}

/// Props for the Ant Design flavored button.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub r#type: ButtonType,
    #[props(default)]
    pub size: ButtonSize,
    #[props(default)]
    pub shape: ButtonShape,
    #[props(default)]
    pub danger: bool,
    #[props(default)]
    pub ghost: bool,
    #[props(default)]
    pub block: bool,
    #[props(default)]
    pub loading: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub icon: Option<Element>,
    #[props(optional)]
    pub href: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

/// Ant Design inspired button implementation for Dioxus.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let ButtonProps {
        r#type,
        size,
        shape,
        danger,
        ghost,
        block,
        loading,
        disabled,
        icon,
        href,
        class,
        onclick,
        children,
    } = props;

    let theme = use_theme();
    let tokens = theme.tokens();

    let visuals = visuals(&tokens, r#type, danger, ghost);
    let metrics = metrics(&tokens, size, shape);

    let disabled = disabled || loading;
    let mut class_list = vec!["adui-btn".to_string()];
    if block {
        class_list.push("adui-btn-block".into());
    }
    if disabled {
        class_list.push("adui-btn-disabled".into());
    }
    if loading {
        class_list.push("adui-btn-loading".into());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let style = format!(
        "--adui-btn-bg:{};--adui-btn-bg-hover:{};--adui-btn-bg-active:{};\
        --adui-btn-color:{};--adui-btn-color-hover:{};--adui-btn-color-active:{};\
        --adui-btn-border:{};--adui-btn-border-hover:{};--adui-btn-border-active:{};\
        --adui-btn-border-style:{};\
        --adui-btn-font-size:{}px;\
        --adui-btn-radius:{}px;\
        --adui-btn-height:{}px;\
        --adui-btn-padding-block:{}px;\
        --adui-btn-padding-inline:{}px;\
        --adui-btn-shadow:{};\
        --adui-btn-focus-shadow:{};",
        visuals.bg,
        visuals.bg_hover,
        visuals.bg_active,
        visuals.color,
        visuals.color_hover,
        visuals.color_active,
        visuals.border,
        visuals.border_hover,
        visuals.border_active,
        visuals.border_style,
        metrics.font_size,
        metrics.radius,
        metrics.height,
        metrics.padding_block,
        metrics.padding_inline,
        visuals.shadow,
        visuals.focus_shadow
    );

    let onclick = onclick.clone();

    let render_contents = rsx! {
        if loading {
            span { class: "adui-btn-spinner adui-btn-icon" }
        }
        if let Some(icon_node) = icon {
            span { class: "adui-btn-icon", {icon_node} }
        }
        span { class: "adui-btn-content", {children} }
    };

    if let Some(href) = href.clone() {
        let handler = onclick.clone();
        return rsx! {
            a {
                class: "{class_attr}",
                style: "{style}",
                href: "{href}",
                role: "button",
                "aria-disabled": disabled,
                "aria-busy": loading,
                tabindex: if disabled { "-1" } else { "0" },
                onclick: move |evt| {
                    if disabled || loading {
                        evt.stop_propagation();
                        return;
                    }
                    if let Some(h) = handler.as_ref() {
                        h.call(evt);
                    }
                },
                {render_contents}
            }
        };
    }

    let handler = onclick.clone();
    rsx! {
        button {
            class: "{class_attr}",
            style: "{style}",
            r#type: "button",
            role: "button",
            disabled: disabled,
            "aria-disabled": disabled,
            "aria-busy": loading,
            onclick: move |evt| {
                if disabled || loading {
                    evt.stop_propagation();
                    return;
                }
                if let Some(h) = handler.as_ref() {
                    h.call(evt);
                }
            },
            {render_contents}
        }
    }
}

struct ButtonVisuals {
    bg: String,
    bg_hover: String,
    bg_active: String,
    color: String,
    color_hover: String,
    color_active: String,
    border: String,
    border_hover: String,
    border_active: String,
    border_style: String,
    shadow: String,
    focus_shadow: String,
}

struct ButtonMetrics {
    height: f32,
    padding_block: f32,
    padding_inline: f32,
    radius: f32,
    font_size: f32,
}

fn metrics(tokens: &ThemeTokens, size: ButtonSize, shape: ButtonShape) -> ButtonMetrics {
    let (height, padding_block, padding_inline, font_size) = match size {
        ButtonSize::Small => (
            tokens.control_height_small,
            tokens.padding_block - 2.0,
            tokens.padding_inline - 4.0,
            tokens.font_size - 1.0,
        ),
        ButtonSize::Large => (
            tokens.control_height_large,
            tokens.padding_block + 2.0,
            tokens.padding_inline + 2.0,
            tokens.font_size + 1.0,
        ),
        ButtonSize::Middle => (
            tokens.control_height,
            tokens.padding_block,
            tokens.padding_inline,
            tokens.font_size,
        ),
    };

    let radius = match shape {
        ButtonShape::Circle => height / 2.0,
        ButtonShape::Round => (height / 2.0).max(tokens.border_radius),
        ButtonShape::Default => tokens.border_radius,
    };

    ButtonMetrics {
        height,
        padding_block,
        padding_inline,
        radius,
        font_size,
    }
}

fn visuals(tokens: &ThemeTokens, kind: ButtonType, danger: bool, ghost: bool) -> ButtonVisuals {
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
        ButtonType::Primary => ButtonVisuals {
            bg: if ghost {
                "transparent".into()
            } else {
                accent.clone()
            },
            bg_hover: if ghost {
                "transparent".into()
            } else {
                accent_hover.clone()
            },
            bg_active: if ghost {
                "transparent".into()
            } else {
                accent_active.clone()
            },
            color: if ghost {
                accent.clone()
            } else {
                "#ffffff".into()
            },
            color_hover: if ghost {
                accent_hover.clone()
            } else {
                "#ffffff".into()
            },
            color_active: if ghost {
                accent_active.clone()
            } else {
                "#ffffff".into()
            },
            border: accent.clone(),
            border_hover: accent_hover.clone(),
            border_active: accent_active.clone(),
            border_style: "solid".into(),
            shadow: if ghost {
                "none".into()
            } else {
                tokens.shadow.clone()
            },
            focus_shadow: if danger {
                "0 0 0 2px rgba(255, 77, 79, 0.26)".into()
            } else {
                "0 0 0 2px rgba(22, 119, 255, 0.28)".into()
            },
        },
        ButtonType::Link => ButtonVisuals {
            bg: "transparent".into(),
            bg_hover: "transparent".into(),
            bg_active: "transparent".into(),
            color: accent.clone(),
            color_hover: accent_hover.clone(),
            color_active: accent_active.clone(),
            border: "transparent".into(),
            border_hover: "transparent".into(),
            border_active: "transparent".into(),
            border_style: "solid".into(),
            shadow: "none".into(),
            focus_shadow: if danger {
                "0 0 0 2px rgba(255, 77, 79, 0.16)".into()
            } else {
                "0 0 0 2px rgba(22, 119, 255, 0.16)".into()
            },
        },
        ButtonType::Text => ButtonVisuals {
            bg: "transparent".into(),
            bg_hover: "rgba(0,0,0,0.03)".into(),
            bg_active: "rgba(0,0,0,0.06)".into(),
            color: accent.clone(),
            color_hover: accent_hover.clone(),
            color_active: accent_active.clone(),
            border: "transparent".into(),
            border_hover: "transparent".into(),
            border_active: "transparent".into(),
            border_style: "solid".into(),
            shadow: "none".into(),
            focus_shadow: if danger {
                "0 0 0 2px rgba(255, 77, 79, 0.12)".into()
            } else {
                "0 0 0 2px rgba(22, 119, 255, 0.12)".into()
            },
        },
        ButtonType::Dashed | ButtonType::Default => {
            let mut visuals = ButtonVisuals {
                bg: tokens.color_bg_container.clone(),
                bg_hover: tokens.color_bg_container.clone(),
                bg_active: tokens.color_bg_container.clone(),
                color: if danger {
                    accent.clone()
                } else {
                    tokens.color_text.clone()
                },
                color_hover: accent_hover.clone(),
                color_active: accent_active.clone(),
                border: if danger {
                    accent.clone()
                } else {
                    tokens.color_border.clone()
                },
                border_hover: if danger {
                    accent_hover.clone()
                } else {
                    tokens.color_border_hover.clone()
                },
                border_active: accent_active.clone(),
                border_style: if matches!(kind, ButtonType::Dashed) {
                    "dashed".into()
                } else {
                    "solid".into()
                },
                shadow: "none".into(),
                focus_shadow: "0 0 0 2px rgba(22, 119, 255, 0.15)".into(),
            };

            if ghost {
                visuals.bg = "transparent".into();
                visuals.bg_hover = "transparent".into();
                visuals.bg_active = "transparent".into();
                visuals.color = accent.clone();
                visuals.color_hover = accent_hover.clone();
                visuals.color_active = accent_active.clone();
                visuals.border = accent.clone();
                visuals.border_hover = accent_hover.clone();
                visuals.border_active = accent_active.clone();
            }

            visuals
        }
    }
}
