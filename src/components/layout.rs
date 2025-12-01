use crate::components::icon::{Icon, IconKind};
use crate::theme::{ThemeTokens, use_theme};
use dioxus::prelude::*;

/// Shared layout props for container sections.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// 标记该 Layout 是否包含 Sider，未设置则默认为 `false`。
    #[props(optional)]
    pub has_sider: Option<bool>,
    pub children: Element,
}

/// Root layout container with optional sider awareness.
#[component]
pub fn Layout(props: LayoutProps) -> Element {
    let LayoutProps {
        class,
        style,
        has_sider,
        children,
    } = props;

    let mut class_list = vec!["adui-layout".to_string()];
    if has_sider.unwrap_or(false) {
        class_list.push("adui-layout-has-sider".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    rsx! {
        div {
            class: "{class_attr}",
            style: style.unwrap_or_default(),
            {children}
        }
    }
}

/// Top navigation/header area.
#[component]
pub fn Header(props: LayoutProps) -> Element {
    let LayoutProps { class, style, .. } = props.clone();
    let theme = use_theme();
    let tokens = theme.tokens();
    let class_attr = format!("adui-layout-header {}", class.unwrap_or_default());
    let style_attr = format!(
        "background:{};color:{};{}",
        tokens.color_bg_container,
        tokens.color_text,
        style.unwrap_or_default()
    );
    rsx! {
        header {
            class: "{class_attr}",
            style: "{style_attr}",
            {props.children}
        }
    }
}

/// Main content area.
#[component]
pub fn Content(props: LayoutProps) -> Element {
    let LayoutProps {
        class,
        style,
        children,
        ..
    } = props;
    let class_attr = format!("adui-layout-content {}", class.unwrap_or_default());
    rsx! {
        main {
            class: "{class_attr}",
            style: style.unwrap_or_default(),
            {children}
        }
    }
}

/// Footer/extra information bar.
#[component]
pub fn Footer(props: LayoutProps) -> Element {
    let LayoutProps {
        class,
        style,
        children,
        ..
    } = props;
    let theme = use_theme();
    let tokens = theme.tokens();
    let class_attr = format!("adui-layout-footer {}", class.unwrap_or_default());
    let style_attr = format!(
        "color:{};{}",
        tokens.color_text_secondary,
        style.unwrap_or_default()
    );
    rsx! {
        footer {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}

/// Theme variants for the side navigation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SiderTheme {
    Light,
    #[default]
    Dark,
}

/// Properties for the side navigation container.
#[derive(Props, Clone, PartialEq)]
pub struct SiderProps {
    #[props(optional)]
    pub width: Option<f32>,
    #[props(optional)]
    pub collapsed_width: Option<f32>,
    #[props(optional)]
    pub collapsed: Option<bool>,
    #[props(default)]
    pub default_collapsed: bool,
    #[props(default)]
    pub collapsible: bool,
    #[props(default)]
    pub reverse_arrow: bool,
    #[props(optional)]
    pub trigger: Option<Element>,
    #[props(optional)]
    pub zero_width_trigger_style: Option<String>,
    #[props(default = SiderTheme::Dark)]
    pub theme: SiderTheme,
    #[props(default = true)]
    pub has_border: bool,
    #[props(optional)]
    pub on_collapse: Option<EventHandler<bool>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Side navigation panel with optional collapse control.
#[component]
pub fn Sider(props: SiderProps) -> Element {
    let SiderProps {
        width,
        collapsed_width,
        collapsed,
        default_collapsed,
        collapsible,
        reverse_arrow,
        trigger,
        zero_width_trigger_style,
        theme,
        has_border,
        on_collapse,
        class,
        style,
        children,
    } = props;

    let width_value = width.unwrap_or(200.0).max(0.0);
    let collapsed_value = collapsed_width.unwrap_or(80.0).max(0.0);

    let mut collapsed_state = use_signal(|| collapsed.unwrap_or(default_collapsed));
    if let Some(external) = collapsed {
        collapsed_state.set(external);
    }

    let theme_handle = use_theme();
    let tokens = theme_handle.tokens();
    let (bg_color, text_color) = sider_palette(&tokens, theme);
    let border_color = if has_border {
        format!("1px solid {}", tokens.color_border)
    } else {
        "none".into()
    };

    let is_collapsed = *collapsed_state.read();
    let current_width = if is_collapsed {
        collapsed_value
    } else {
        width_value
    };
    let width_str = format!("{}px", current_width);

    let mut class_list = vec!["adui-layout-sider".to_string()];
    class_list.push(match theme {
        SiderTheme::Light => "adui-layout-sider-light".into(),
        SiderTheme::Dark => "adui-layout-sider-dark".into(),
    });
    if is_collapsed {
        class_list.push("adui-layout-sider-collapsed".into());
    }
    if collapsible {
        class_list.push("adui-layout-sider-collapsible".into());
    }
    if collapsed_value == 0.0 {
        class_list.push("adui-layout-sider-zero-width".into());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let trigger_content = trigger
        .clone()
        .unwrap_or_else(|| default_trigger_icon(is_collapsed, reverse_arrow));

    let mut toggle = {
        let mut collapsed_signal = collapsed_state;
        let collapsible_flag = collapsible;
        let handler = on_collapse;
        move || {
            if !collapsible_flag {
                return;
            }
            let next = !*collapsed_signal.read();
            collapsed_signal.set(next);
            if let Some(cb) = handler.as_ref() {
                cb.call(next);
            }
        }
    };

    let zero_width_trigger = if collapsible && collapsed_value == 0.0 {
        let trigger_style = zero_width_trigger_style.unwrap_or_default();
        let trigger_icon = trigger_content.clone();
        Some(rsx! {
            span {
                class: format_args!(
                    "{} {}",
                    "adui-layout-sider-zero-trigger",
                    if reverse_arrow { "adui-layout-sider-zero-trigger-right" } else { "adui-layout-sider-zero-trigger-left" }
                ),
                style: trigger_style,
                onclick: move |_| toggle(),
                {trigger_icon}
            }
        })
    } else {
        None
    };

    let inline_trigger = if collapsible && collapsed_value > 0.0 {
        let trigger_icon = trigger_content;
        Some(rsx! {
            div {
                class: "adui-layout-sider-trigger",
                style: format!("width:{width_str};"),
                onclick: move |_| toggle(),
                {trigger_icon}
            }
        })
    } else {
        None
    };

    let mut style_buffer = format!(
        "flex:0 0 {w};max-width:{w};min-width:{w};width:{w};background:{};color:{};border-right:{};",
        bg_color,
        text_color,
        border_color,
        w = width_str
    );
    if let Some(extra) = style.as_ref() {
        style_buffer.push_str(extra);
    }

    rsx! {
        aside {
            class: "{class_attr}",
            style: "{style_buffer}",
            role: "complementary",
            "aria-expanded": (!is_collapsed).to_string(),
            div {
                class: "adui-layout-sider-children",
                {children}
            }
            if let Some(trigger) = zero_width_trigger {
                {trigger}
            } else if let Some(trigger) = inline_trigger {
                {trigger}
            }
        }
    }
}

fn default_trigger_icon(collapsed: bool, reverse_arrow: bool) -> Element {
    let should_point_right = if reverse_arrow { !collapsed } else { collapsed };
    let icon_kind = if should_point_right {
        IconKind::ArrowRight
    } else {
        IconKind::ArrowLeft
    };
    rsx!(Icon {
        kind: icon_kind,
        size: 16.0
    })
}

fn sider_palette(tokens: &ThemeTokens, theme: SiderTheme) -> (String, String) {
    match theme {
        SiderTheme::Light => (tokens.color_bg_container.clone(), tokens.color_text.clone()),
        SiderTheme::Dark => (tokens.color_bg_layout.clone(), "#fafafa".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sider_theme_default() {
        assert_eq!(SiderTheme::default(), SiderTheme::Dark);
    }

    #[test]
    fn sider_theme_variants() {
        assert_ne!(SiderTheme::Light, SiderTheme::Dark);
    }

    #[test]
    fn sider_theme_equality() {
        assert_eq!(SiderTheme::Light, SiderTheme::Light);
        assert_eq!(SiderTheme::Dark, SiderTheme::Dark);
    }

    #[test]
    fn layout_props_defaults() {
        let props = LayoutProps {
            class: None,
            style: None,
            has_sider: None,
            children: rsx!(div {}),
        };
        assert!(props.class.is_none());
        assert!(props.style.is_none());
        assert!(props.has_sider.is_none());
    }

    #[test]
    fn layout_props_has_sider() {
        let props_with_sider = LayoutProps {
            class: None,
            style: None,
            has_sider: Some(true),
            children: rsx!(div {}),
        };
        assert_eq!(props_with_sider.has_sider, Some(true));

        let props_without_sider = LayoutProps {
            class: None,
            style: None,
            has_sider: Some(false),
            children: rsx!(div {}),
        };
        assert_eq!(props_without_sider.has_sider, Some(false));
    }

    #[test]
    fn sider_props_defaults() {
        let props = SiderProps {
            width: None,
            collapsed_width: None,
            collapsed: None,
            default_collapsed: false,
            collapsible: false,
            reverse_arrow: false,
            trigger: None,
            zero_width_trigger_style: None,
            theme: SiderTheme::default(),
            has_border: true,
            on_collapse: None,
            class: None,
            style: None,
            children: rsx!(div {}),
        };
        assert_eq!(props.default_collapsed, false);
        assert_eq!(props.collapsible, false);
        assert_eq!(props.reverse_arrow, false);
        assert_eq!(props.theme, SiderTheme::Dark);
        assert_eq!(props.has_border, true);
    }

    #[test]
    fn sider_theme_clone() {
        let original = SiderTheme::Light;
        let cloned = original;
        assert_eq!(original, cloned);
    }
}
