use crate::components::config_provider::{ComponentSize, use_config};
use crate::theme::{ThemeTokens, use_theme};
use dioxus::prelude::*;

/// Supported button visual types（兼容旧 API）.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonType {
    #[default]
    Default,
    Primary,
    Dashed,
    Text,
    Link,
}

/// Button tone（新 API，向后兼容 danger 开关）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonColor {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

/// Button variant（新 API）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    Solid,
    #[default]
    Outlined,
    Dashed,
    Text,
    Link,
}

/// Button size variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl ButtonSize {
    fn from_global(size: ComponentSize) -> Self {
        match size {
            ComponentSize::Small => ButtonSize::Small,
            ComponentSize::Large => ButtonSize::Large,
            ComponentSize::Middle => ButtonSize::Middle,
        }
    }
}

/// Shape variants for the button outline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonShape {
    #[default]
    Default,
    Round,
    Circle,
}

/// Icon placement relative to content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonIconPlacement {
    #[default]
    Start,
    End,
}

/// Native button `type` attribute.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonHtmlType {
    #[default]
    Button,
    Submit,
    Reset,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ButtonGroupContext {
    size: Option<ButtonSize>,
    shape: Option<ButtonShape>,
    color: Option<ButtonColor>,
    variant: Option<ButtonVariant>,
}

/// Container that forwards size/variant hints to child buttons.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    #[props(optional)]
    pub size: Option<ButtonSize>,
    #[props(optional)]
    pub shape: Option<ButtonShape>,
    #[props(optional)]
    pub color: Option<ButtonColor>,
    #[props(optional)]
    pub variant: Option<ButtonVariant>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Wrap multiple buttons with shared styling hints.
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let ButtonGroupProps {
        size,
        shape,
        color,
        variant,
        class,
        style,
        children,
    } = props;
    use_context_provider(|| ButtonGroupContext {
        size,
        shape,
        color,
        variant,
    });
    let mut class_list = vec!["adui-btn-group".to_string()];
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();
    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
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
    /// Optional loading delay in milliseconds before showing spinner.
    #[props(optional)]
    pub loading_delay: Option<u64>,
    /// Custom loading icon.
    #[props(optional)]
    pub loading_icon: Option<Element>,
    /// If true, inserts a space between two CJK chars (antd behavior).
    #[props(default = true)]
    pub auto_insert_space: bool,
    /// Optional label text used for auto spacing/icon-only detection; if set, overrides children text for button content.
    #[props(optional)]
    pub label: Option<String>,
    /// Mark as icon-only (adds class); if unset, derives from `label` being empty with an icon.
    #[props(optional)]
    pub icon_only: Option<bool>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub color: Option<ButtonColor>,
    #[props(optional)]
    pub variant: Option<ButtonVariant>,
    #[props(default)]
    pub icon_placement: ButtonIconPlacement,
    #[props(optional)]
    pub icon: Option<Element>,
    #[props(optional)]
    pub href: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    /// Extra class applied to button element (aligned to antd classNames.root).
    #[props(optional)]
    pub class_names_root: Option<String>,
    /// Extra class applied to icon span.
    #[props(optional)]
    pub class_names_icon: Option<String>,
    /// Extra class applied to content span.
    #[props(optional)]
    pub class_names_content: Option<String>,
    /// Extra inline style applied to root.
    #[props(optional)]
    pub styles_root: Option<String>,
    /// Native button type, used when rendering as `<button>`.
    #[props(default)]
    pub html_type: ButtonHtmlType,
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
        loading_delay,
        loading_icon,
        auto_insert_space,
        label,
        icon_only,
        disabled,
        color,
        variant,
        icon_placement,
        icon,
        href,
        class,
        class_names_root,
        class_names_icon,
        class_names_content,
        styles_root,
        html_type,
        onclick,
        children,
    } = props;

    // Merge size/shape/variant/color from ButtonGroup and global ConfigProvider.
    let mut size = size;
    let mut shape = shape;
    let mut variant = variant;
    let mut color = color;

    if let Some(ctx) = try_use_context::<ButtonGroupContext>() {
        if let Some(shared_size) = ctx.size {
            size = shared_size;
        }
        if let Some(shared_shape) = ctx.shape {
            shape = shared_shape;
        }
        if variant.is_none() {
            variant = ctx.variant;
        }
        if color.is_none() {
            color = ctx.color;
        }
    } else if matches!(size, ButtonSize::Middle) {
        // Fall back to global ConfigProvider size when not inside a ButtonGroup
        // and the user did not explicitly specify size.
        let cfg = use_config();
        size = ButtonSize::from_global(cfg.size);
    }

    let theme = use_theme();

    // Derive variant/color from legacy type/danger if not provided.
    let derived_variant = variant.unwrap_or(match r#type {
        ButtonType::Primary => ButtonVariant::Solid,
        ButtonType::Dashed => ButtonVariant::Dashed,
        ButtonType::Text => ButtonVariant::Text,
        ButtonType::Link => ButtonVariant::Link,
        ButtonType::Default => ButtonVariant::Outlined,
    });
    let derived_color = color.unwrap_or({
        if danger {
            ButtonColor::Danger
        } else {
            match r#type {
                ButtonType::Primary => ButtonColor::Primary,
                _ => ButtonColor::Default,
            }
        }
    });

    let html_type_attr = match html_type {
        ButtonHtmlType::Button => "button",
        ButtonHtmlType::Submit => "submit",
        ButtonHtmlType::Reset => "reset",
    };

    // Loading delay handling: debounce before showing spinner.
    let inner_loading = use_signal(|| loading);
    {
        let mut state = inner_loading;
        let delay_ms = loading_delay.unwrap_or(0);
        use_effect(move || {
            if loading {
                if delay_ms == 0 {
                    state.set(true);
                } else {
                    let mut delayed_state = state;
                    let delay = delay_ms;
                    // Fallback: block current task; avoids Send requirement on signals.
                    std::thread::sleep(std::time::Duration::from_millis(delay));
                    delayed_state.set(true);
                }
            } else {
                state.set(false);
            }
        });
    }

    let tokens = theme.tokens();
    let visuals = visuals(&tokens, derived_variant, derived_color, ghost);
    let metrics = metrics(&tokens, size, shape);

    let disabled = disabled || *inner_loading.read();
    let mut class_list = vec!["adui-btn".to_string()];
    class_list.push(match derived_variant {
        ButtonVariant::Solid => "adui-btn-solid".into(),
        ButtonVariant::Outlined => "adui-btn-outlined".into(),
        ButtonVariant::Dashed => "adui-btn-dashed".into(),
        ButtonVariant::Text => "adui-btn-text".into(),
        ButtonVariant::Link => "adui-btn-link".into(),
    });
    class_list.push(match derived_color {
        ButtonColor::Primary => "adui-btn-primary".into(),
        ButtonColor::Success => "adui-btn-success".into(),
        ButtonColor::Warning => "adui-btn-warning".into(),
        ButtonColor::Danger => "adui-btn-danger".into(),
        ButtonColor::Default => "adui-btn-default".into(),
    });
    if block {
        class_list.push("adui-btn-block".into());
    }
    if ghost {
        class_list.push("adui-btn-ghost".into());
    }
    if disabled {
        class_list.push("adui-btn-disabled".into());
    }
    if *inner_loading.read() {
        class_list.push("adui-btn-loading".into());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    if let Some(extra) = class_names_root.as_ref() {
        class_list.push(extra.clone());
    }
    let icon_only_flag = icon_only.unwrap_or_else(|| {
        label.as_ref().map(|s| s.trim().is_empty()).unwrap_or(false) && icon.is_some()
    });
    if icon_only_flag {
        class_list.push("adui-btn-icon-only".into());
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

    let spinner = loading_icon.unwrap_or_else(|| {
        rsx!(span {
            class: "adui-btn-spinner adui-btn-icon"
        })
    });
    let mut icon_class = "adui-btn-icon".to_string();
    if let Some(extra) = class_names_icon.as_ref() {
        icon_class.push(' ');
        icon_class.push_str(extra);
    }
    let mut content_class = "adui-btn-content".to_string();
    if let Some(extra) = class_names_content.as_ref() {
        content_class.push(' ');
        content_class.push_str(extra);
    }
    let mut content_text = label.clone();
    if let Some(text) = content_text.as_mut()
        && auto_insert_space
        && is_two_cjk(text)
    {
        let mut chars = text.chars();
        let first = chars.next().unwrap_or_default();
        let second = chars.next().unwrap_or_default();
        *text = format!("{} {}", first, second);
    }

    let icon_node = icon.map(|node| {
        let cls = icon_class.clone();
        rsx!(span { class: "{cls}", {node} })
    });

    let contents = match icon_placement {
        ButtonIconPlacement::Start => rsx! {
            if *inner_loading.read() {
                {spinner.clone()}
            } else if let Some(icon_el) = icon_node.clone() {
                {icon_el}
            }
            span { class: "{content_class}",
                if let Some(text) = content_text.clone() {
                    "{text}"
                } else {
                    {children.clone()}
                }
            }
        },
        ButtonIconPlacement::End => rsx! {
            span { class: "{content_class}",
                if let Some(text) = content_text.clone() {
                    "{text}"
                } else {
                    {children.clone()}
                }
            }
            if *inner_loading.read() {
                {spinner.clone()}
            } else if let Some(icon_el) = icon_node.clone() {
                {icon_el}
            }
        },
    };

    if let Some(href) = href {
        let handler = onclick;
        return rsx! {
            a {
                class: "{class_attr}",
                style: format!("{style}{}", styles_root.clone().unwrap_or_default()),
                href: "{href}",
                role: "button",
                "aria-disabled": disabled,
                "aria-busy": *inner_loading.read(),
                tabindex: if disabled { "-1" } else { "0" },
                onclick: move |evt| {
                    if disabled || *inner_loading.read() {
                        evt.stop_propagation();
                        evt.prevent_default();
                        return;
                    }
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
            style: format!("{style}{}", styles_root.unwrap_or_default()),
            r#type: "{html_type_attr}",
            role: "button",
            disabled: disabled,
            "aria-disabled": disabled,
            "aria-busy": *inner_loading.read(),
            onclick: move |evt| {
                if disabled || *inner_loading.read() {
                    evt.stop_propagation();
                    return;
                }
                if let Some(h) = handler.as_ref() {
                    h.call(evt);
                }
            },
            {contents}
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

fn is_two_cjk(text: &str) -> bool {
    let mut chars = text.chars();
    let first = chars.next();
    let second = chars.next();
    second.is_some()
        && chars.next().is_none()
        && first.map(is_cjk).unwrap_or(false)
        && second.map(is_cjk).unwrap_or(false)
}

fn is_cjk(ch: char) -> bool {
    matches!(ch as u32,
        0x4E00..=0x9FFF // CJK Unified Ideographs
        | 0x3400..=0x4DBF // Extension A
        | 0x20000..=0x2A6DF // Extension B
        | 0x2A700..=0x2B73F
        | 0x2B740..=0x2B81F
        | 0x2B820..=0x2CEAF
        | 0xF900..=0xFAFF // Compatibility Ideographs
    )
}

fn visuals(
    tokens: &ThemeTokens,
    variant: ButtonVariant,
    color: ButtonColor,
    ghost: bool,
) -> ButtonVisuals {
    match variant {
        ButtonVariant::Solid => solid_visuals(tokens, color, ghost),
        ButtonVariant::Link => link_visuals(tokens, color),
        ButtonVariant::Text => text_visuals(tokens, color),
        ButtonVariant::Dashed | ButtonVariant::Outlined => outline_visuals(
            tokens,
            color,
            ghost,
            matches!(variant, ButtonVariant::Dashed),
        ),
    }
}

fn solid_visuals(tokens: &ThemeTokens, color: ButtonColor, ghost: bool) -> ButtonVisuals {
    let mut visuals = match color {
        ButtonColor::Primary
        | ButtonColor::Success
        | ButtonColor::Warning
        | ButtonColor::Danger => {
            let (accent, hover, active) = tone_palette(tokens, color);
            ButtonVisuals {
                bg: accent.clone(),
                bg_hover: hover.clone(),
                bg_active: active.clone(),
                color: "#ffffff".into(),
                color_hover: "#ffffff".into(),
                color_active: "#ffffff".into(),
                border: accent.clone(),
                border_hover: hover.clone(),
                border_active: active.clone(),
                border_style: "solid".into(),
                shadow: tokens.shadow.clone(),
                focus_shadow: focus_ring(color, "0 0 0 2px rgba(22, 119, 255, 0.28)"),
            }
        }
        ButtonColor::Default => ButtonVisuals {
            bg: tokens.color_bg_container.clone(),
            bg_hover: tokens.color_bg_base.clone(),
            bg_active: tokens.color_bg_base.clone(),
            color: tokens.color_text.clone(),
            color_hover: tokens.color_text.clone(),
            color_active: tokens.color_text.clone(),
            border: tokens.color_border.clone(),
            border_hover: tokens.color_border_hover.clone(),
            border_active: tokens.color_border_hover.clone(),
            border_style: "solid".into(),
            shadow: tokens.shadow.clone(),
            focus_shadow: "0 0 0 2px rgba(0, 0, 0, 0.08)".into(),
        },
    };

    if ghost {
        visuals.bg = "transparent".into();
        visuals.bg_hover = "transparent".into();
        visuals.bg_active = "transparent".into();
        visuals.color = visuals.border.clone();
        visuals.color_hover = visuals.border_hover.clone();
        visuals.color_active = visuals.border_active.clone();
        visuals.shadow = "none".into();
    }

    visuals
}

fn link_visuals(tokens: &ThemeTokens, color: ButtonColor) -> ButtonVisuals {
    let (accent, hover, active) = if color == ButtonColor::Default {
        (
            tokens.color_link.clone(),
            tokens.color_link_hover.clone(),
            tokens.color_link_active.clone(),
        )
    } else {
        tone_palette(tokens, color)
    };

    ButtonVisuals {
        bg: "transparent".into(),
        bg_hover: "transparent".into(),
        bg_active: "transparent".into(),
        color: accent.clone(),
        color_hover: hover.clone(),
        color_active: active.clone(),
        border: "transparent".into(),
        border_hover: "transparent".into(),
        border_active: "transparent".into(),
        border_style: "solid".into(),
        shadow: "none".into(),
        focus_shadow: focus_ring(color, "0 0 0 2px rgba(22, 119, 255, 0.16)"),
    }
}

fn text_visuals(tokens: &ThemeTokens, color: ButtonColor) -> ButtonVisuals {
    let (accent, hover, active) = match color {
        ButtonColor::Default => (
            tokens.color_text.clone(),
            tokens.color_primary.clone(),
            tokens.color_primary_active.clone(),
        ),
        _ => tone_palette(tokens, color),
    };

    ButtonVisuals {
        bg: "transparent".into(),
        bg_hover: "rgba(0,0,0,0.03)".into(),
        bg_active: "rgba(0,0,0,0.06)".into(),
        color: accent.clone(),
        color_hover: hover.clone(),
        color_active: active.clone(),
        border: "transparent".into(),
        border_hover: "transparent".into(),
        border_active: "transparent".into(),
        border_style: "solid".into(),
        shadow: "none".into(),
        focus_shadow: focus_ring(color, "0 0 0 2px rgba(22, 119, 255, 0.12)"),
    }
}

fn outline_visuals(
    tokens: &ThemeTokens,
    color: ButtonColor,
    ghost: bool,
    dashed: bool,
) -> ButtonVisuals {
    let mut visuals = match color {
        ButtonColor::Default => ButtonVisuals {
            bg: tokens.color_bg_container.clone(),
            bg_hover: tokens.color_bg_container.clone(),
            bg_active: tokens.color_bg_container.clone(),
            color: tokens.color_text.clone(),
            color_hover: tokens.color_primary.clone(),
            color_active: tokens.color_primary_active.clone(),
            border: tokens.color_border.clone(),
            border_hover: tokens.color_primary.clone(),
            border_active: tokens.color_primary_active.clone(),
            border_style: if dashed {
                "dashed".into()
            } else {
                "solid".into()
            },
            shadow: "none".into(),
            focus_shadow: "0 0 0 2px rgba(22, 119, 255, 0.12)".into(),
        },
        _ => {
            let (accent, hover, active) = tone_palette(tokens, color);
            ButtonVisuals {
                bg: tokens.color_bg_container.clone(),
                bg_hover: tokens.color_bg_container.clone(),
                bg_active: tokens.color_bg_container.clone(),
                color: accent.clone(),
                color_hover: hover.clone(),
                color_active: active.clone(),
                border: accent.clone(),
                border_hover: hover.clone(),
                border_active: active.clone(),
                border_style: if dashed {
                    "dashed".into()
                } else {
                    "solid".into()
                },
                shadow: "none".into(),
                focus_shadow: focus_ring(color, "0 0 0 2px rgba(22, 119, 255, 0.15)"),
            }
        }
    };

    if ghost {
        visuals.bg = "transparent".into();
        visuals.bg_hover = "transparent".into();
        visuals.bg_active = "transparent".into();
    }

    visuals
}

fn tone_palette(tokens: &ThemeTokens, color: ButtonColor) -> (String, String, String) {
    match color {
        ButtonColor::Primary => (
            tokens.color_primary.clone(),
            tokens.color_primary_hover.clone(),
            tokens.color_primary_active.clone(),
        ),
        ButtonColor::Success => (
            tokens.color_success.clone(),
            tokens.color_success_hover.clone(),
            tokens.color_success_active.clone(),
        ),
        ButtonColor::Warning => (
            tokens.color_warning.clone(),
            tokens.color_warning_hover.clone(),
            tokens.color_warning_active.clone(),
        ),
        ButtonColor::Danger => (
            tokens.color_error.clone(),
            tokens.color_error_hover.clone(),
            tokens.color_error_active.clone(),
        ),
        ButtonColor::Default => (
            tokens.color_text.clone(),
            tokens.color_text_muted.clone(),
            tokens.color_text_secondary.clone(),
        ),
    }
}

fn focus_ring(color: ButtonColor, fallback: &str) -> String {
    match color {
        ButtonColor::Primary => "0 0 0 2px rgba(22, 119, 255, 0.28)".into(),
        ButtonColor::Success => "0 0 0 2px rgba(82, 196, 26, 0.26)".into(),
        ButtonColor::Warning => "0 0 0 2px rgba(250, 173, 20, 0.26)".into(),
        ButtonColor::Danger => "0 0 0 2px rgba(255, 77, 79, 0.26)".into(),
        ButtonColor::Default => fallback.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::ThemeTokens;

    #[test]
    fn metrics_respect_size_and_shape() {
        let tokens = ThemeTokens::light();
        let circle = metrics(&tokens, ButtonSize::Small, ButtonShape::Circle);
        assert_eq!(circle.height, tokens.control_height_small);
        assert!((circle.radius - circle.height / 2.0).abs() < f32::EPSILON);

        let round = metrics(&tokens, ButtonSize::Large, ButtonShape::Round);
        assert!(round.radius >= tokens.border_radius);
        assert!(round.padding_inline > circle.padding_inline);
    }

    #[test]
    fn detects_two_cjk_characters() {
        assert!(is_two_cjk("按钮"));
        assert!(!is_two_cjk("按钮A"));
        assert!(!is_two_cjk("btn"));
    }

    #[test]
    fn visuals_follow_variant_and_tone_rules() {
        let tokens = ThemeTokens::light();
        let solid = visuals(&tokens, ButtonVariant::Solid, ButtonColor::Primary, false);
        assert_eq!(solid.bg, tokens.color_primary);
        assert_eq!(solid.color, "#ffffff");

        let ghost = visuals(&tokens, ButtonVariant::Solid, ButtonColor::Primary, true);
        assert_eq!(ghost.bg, "transparent");
        assert_eq!(ghost.color, ghost.border);

        let link_style = visuals(&tokens, ButtonVariant::Link, ButtonColor::Default, false);
        assert_eq!(link_style.bg, "transparent");
        assert_eq!(link_style.border, "transparent");
        assert_eq!(link_style.color, tokens.color_link);
    }
}
