use crate::theme::use_theme;
use dioxus::prelude::*;

/// Shared layout props for container sections.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Root layout container with theme-aware background.
#[component]
pub fn Layout(props: LayoutProps) -> Element {
    let LayoutProps {
        class,
        style,
        children,
    } = props;
    let theme = use_theme();
    let tokens = theme.tokens();

    let class_attr = format!("adui-layout {}", class.unwrap_or_default());
    let style_attr = format!(
        "background:{};{}",
        tokens.color_bg_base,
        style.unwrap_or_default()
    );

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}

/// Top navigation/header area.
#[component]
pub fn Header(props: LayoutProps) -> Element {
    let LayoutProps {
        class,
        style,
        children,
    } = props;
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
            {children}
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
    } = props;
    let theme = use_theme();
    let tokens = theme.tokens();
    let class_attr = format!("adui-layout-footer {}", class.unwrap_or_default());
    let style_attr = format!(
        "color:{};{}",
        tokens.color_text_muted,
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

/// Properties for the side navigation container.
#[derive(Props, Clone, PartialEq)]
pub struct SiderProps {
    #[props(optional)]
    pub width: Option<f32>,
    #[props(default = true)]
    pub has_border: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Side navigation panel with optional border and fixed width.
#[component]
pub fn Sider(props: SiderProps) -> Element {
    let SiderProps {
        width,
        has_border,
        class,
        style,
        children,
    } = props;
    let theme = use_theme();
    let tokens = theme.tokens();
    let class_attr = format!("adui-layout-sider {}", class.unwrap_or_default());
    let style_attr = format!(
        "width:{w}px;min-width:{w}px;max-width:{w}px;background:{};color:{};border-right:{};{}",
        tokens.color_bg_container,
        tokens.color_text,
        if has_border {
            format!("1px solid {}", tokens.color_border)
        } else {
            "none".into()
        },
        style.unwrap_or_default(),
        w = width.unwrap_or(200.0),
    );
    rsx! {
        aside {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}
