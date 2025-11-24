use crate::theme::use_theme;
use dioxus::prelude::*;

/// Text tone variants (aligned to Ant Design semantics subset).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextType {
    #[default]
    Default,
    Secondary,
    Danger,
    Disabled,
}

/// Heading levels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TitleLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
}

/// Text component props.
#[derive(Props, Clone, PartialEq)]
pub struct TextProps {
    #[props(default)]
    pub r#type: TextType,
    #[props(default)]
    pub strong: bool,
    #[props(default)]
    pub italic: bool,
    #[props(default)]
    pub underline: bool,
    #[props(default)]
    pub delete: bool,
    #[props(default)]
    pub code: bool,
    #[props(default)]
    pub mark: bool,
    #[props(default)]
    pub ellipsis: bool,
    #[props(default = true)]
    pub wrap: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Paragraph props.
#[derive(Props, Clone, PartialEq)]
pub struct ParagraphProps {
    #[props(default)]
    pub r#type: TextType,
    #[props(default)]
    pub strong: bool,
    #[props(default)]
    pub italic: bool,
    #[props(default)]
    pub underline: bool,
    #[props(default)]
    pub delete: bool,
    #[props(default)]
    pub code: bool,
    #[props(default)]
    pub mark: bool,
    #[props(default)]
    pub ellipsis: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Title props.
#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    #[props(default)]
    pub level: TitleLevel,
    #[props(default)]
    pub r#type: TextType,
    #[props(default)]
    pub strong: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Inline text typography.
#[component]
pub fn Text(props: TextProps) -> Element {
    let TextProps {
        r#type,
        strong,
        italic,
        underline,
        delete,
        code,
        mark,
        ellipsis,
        wrap,
        class,
        style,
        children,
    } = props;

    let theme = use_theme();
    let tokens = theme.tokens();
    let color = match r#type {
        TextType::Default => tokens.color_text.clone(),
        TextType::Secondary => tokens.color_text_muted.clone(),
        TextType::Danger => tokens.color_error.clone(),
        TextType::Disabled => tokens.color_text_disabled.clone(),
    };

    let mut class_list = vec!["adui-text".to_string()];
    if code {
        class_list.push("adui-text-code".into());
    }
    if mark {
        class_list.push("adui-text-mark".into());
    }
    if strong {
        class_list.push("adui-text-strong".into());
    }
    if italic {
        class_list.push("adui-text-italic".into());
    }
    if !wrap {
        class_list.push("adui-text-nowrap".into());
    }
    if ellipsis {
        class_list.push("adui-text-ellipsis".into());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let mut decorations: Vec<&str> = Vec::new();
    if underline {
        decorations.push("underline");
    }
    if delete {
        decorations.push("line-through");
    }
    let decoration = if decorations.is_empty() {
        "none".to_string()
    } else {
        decorations.join(" ")
    };

    let mut style_attr = format!(
        "color:{};text-decoration:{};{}",
        color,
        decoration,
        style.unwrap_or_default()
    );
    if ellipsis {
        style_attr.push_str("max-width: 100%;");
    }

    rsx! {
        span {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}

/// Block paragraph typography.
#[component]
pub fn Paragraph(props: ParagraphProps) -> Element {
    let ParagraphProps {
        r#type,
        strong,
        italic,
        underline,
        delete,
        code,
        mark,
        ellipsis,
        class,
        style,
        children,
    } = props;

    let class_combined = class.unwrap_or_default();
    let style_val = style.unwrap_or_default();
    rsx! {
        p {
            class: "adui-paragraph {class_combined}",
            style: style_val,
            Text {
                r#type,
                strong,
                italic,
                underline,
                delete,
                code,
                mark,
                ellipsis,
                children: children
            }
        }
    }
}

/// Heading typography rendered as h1-h5.
#[component]
pub fn Title(props: TitleProps) -> Element {
    let TitleProps {
        level,
        r#type,
        strong,
        class,
        style,
        children,
    } = props;

    let heading_class = format!(
        "adui-title adui-title-{} {}",
        match level {
            TitleLevel::H1 => 1,
            TitleLevel::H2 => 2,
            TitleLevel::H3 => 3,
            TitleLevel::H4 => 4,
            TitleLevel::H5 => 5,
        },
        class.unwrap_or_default()
    );
    let style_val = style.unwrap_or_default();

    match level {
        TitleLevel::H1 => {
            rsx!(h1 { class: "{heading_class}", style: "{style_val}", Text { r#type, strong, children: children.clone() } })
        }
        TitleLevel::H2 => {
            rsx!(h2 { class: "{heading_class}", style: "{style_val}", Text { r#type, strong, children: children.clone() } })
        }
        TitleLevel::H3 => {
            rsx!(h3 { class: "{heading_class}", style: "{style_val}", Text { r#type, strong, children: children.clone() } })
        }
        TitleLevel::H4 => {
            rsx!(h4 { class: "{heading_class}", style: "{style_val}", Text { r#type, strong, children: children.clone() } })
        }
        TitleLevel::H5 => {
            rsx!(h5 { class: "{heading_class}", style: "{style_val}", Text { r#type, strong, children: children.clone() } })
        }
    }
}
