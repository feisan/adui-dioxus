use crate::components::icon::{Icon, IconKind};
use crate::theme::use_theme;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use dioxus::prelude::{Key, Modifiers};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, closure::Closure};

/// Text tone variants (aligned to Ant Design semantics subset).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextType {
    #[default]
    Default,
    Secondary,
    Success,
    Warning,
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

/// Copy interaction configuration.
#[derive(Clone, PartialEq)]
pub struct TypographyCopyable {
    pub text: String,
    pub icon: Option<Element>,
    pub copied_icon: Option<Element>,
    pub tooltips: Option<(String, String)>,
}

impl TypographyCopyable {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            copied_icon: None,
            tooltips: None,
        }
    }
}

/// Ellipsis behaviour configuration.
#[derive(Clone, PartialEq, Default)]
pub struct TypographyEllipsis {
    pub rows: Option<u16>,
    pub expandable: bool,
    pub expand_text: Option<String>,
    pub collapse_text: Option<String>,
    pub tooltip: Option<String>,
}

/// Inline edit configuration.
#[derive(Clone, PartialEq, Default)]
pub struct TypographyEditable {
    pub text: Option<String>,
    pub placeholder: Option<String>,
    pub auto_focus: bool,
    pub max_length: Option<usize>,
    pub enter_icon: Option<Element>,
    pub cancel_icon: Option<Element>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TypographyVariant {
    Text,
    Paragraph,
    Title(TitleLevel),
}

#[derive(Props, Clone, PartialEq)]
struct TypographyBaseProps {
    #[props(default = TypographyVariant::Text)]
    variant: TypographyVariant,
    #[props(default)]
    r#type: TextType,
    #[props(default)]
    strong: bool,
    #[props(default)]
    italic: bool,
    #[props(default)]
    underline: bool,
    #[props(default)]
    delete: bool,
    #[props(default)]
    code: bool,
    #[props(default)]
    mark: bool,
    #[props(default = true)]
    wrap: bool,
    #[props(default)]
    ellipsis: bool,
    #[props(optional)]
    ellipsis_config: Option<TypographyEllipsis>,
    #[props(optional)]
    copyable: Option<TypographyCopyable>,
    #[props(optional)]
    editable: Option<TypographyEditable>,
    #[props(default)]
    disabled: bool,
    #[props(optional)]
    on_copy: Option<EventHandler<String>>,
    #[props(optional)]
    on_edit: Option<EventHandler<String>>,
    #[props(optional)]
    on_edit_cancel: Option<EventHandler<String>>,
    #[props(optional)]
    on_edit_start: Option<EventHandler<()>>,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    style: Option<String>,
    pub children: Element,
}

#[component]
fn TypographyBase(props: TypographyBaseProps) -> Element {
    let TypographyBaseProps {
        variant,
        r#type,
        strong,
        italic,
        underline,
        delete,
        code,
        mark,
        wrap,
        ellipsis,
        ellipsis_config,
        copyable,
        editable,
        disabled,
        on_copy,
        on_edit,
        on_edit_cancel,
        on_edit_start,
        class,
        style,
        children,
    } = props;

    let theme = use_theme();
    let tokens = theme.tokens();
    let tone_color = resolve_color(&tokens, r#type, disabled);
    let decoration = text_decoration(underline, delete);
    let ellipsis_cfg = ellipsis_config.unwrap_or_default();
    let ellipsis_rows = ellipsis_cfg.rows.unwrap_or(1);
    let ellipsis_expandable = ellipsis_cfg.expandable;
    let ellipsis_expand_text = ellipsis_cfg
        .expand_text
        .unwrap_or_else(|| "展开".to_string());
    let ellipsis_collapse_text = ellipsis_cfg
        .collapse_text
        .unwrap_or_else(|| "收起".to_string());
    let ellipsis_tooltip = ellipsis_cfg.tooltip;
    let ellipsis_enabled = ellipsis || ellipsis_expandable || ellipsis_cfg.rows.is_some();

    let copy_status = use_signal(|| false);
    let editing = use_signal(|| false);
    let ellipsis_expanded = use_signal(|| false);
    let edit_value = use_signal(|| {
        editable
            .as_ref()
            .and_then(|cfg| cfg.text.clone())
            .unwrap_or_default()
    });
    {
        let mut state = edit_value;
        let source = editable.as_ref().and_then(|cfg| cfg.text.clone());
        use_effect(move || {
            if let Some(new_value) = source.clone() {
                state.set(new_value);
            }
        });
    }

    let mut class_list = match variant {
        TypographyVariant::Text => vec!["adui-text".to_string()],
        TypographyVariant::Paragraph => vec!["adui-paragraph".to_string(), "adui-text".to_string()],
        TypographyVariant::Title(level) => {
            vec![
                "adui-title".to_string(),
                format!("adui-title-{}", level_index(level)),
            ]
        }
    };
    if strong {
        class_list.push("adui-text-strong".into());
    }
    if italic {
        class_list.push("adui-text-italic".into());
    }
    if code {
        class_list.push("adui-text-code".into());
    }
    if mark {
        class_list.push("adui-text-mark".into());
    }
    if !wrap {
        class_list.push("adui-text-nowrap".into());
    }
    if disabled {
        class_list.push("adui-text-disabled".into());
    }
    let ellipsis_active = ellipsis_enabled && (!ellipsis_expandable || !*ellipsis_expanded.read());
    if ellipsis_active {
        class_list.push("adui-text-ellipsis".into());
        if ellipsis_rows > 1 {
            class_list.push("adui-text-ellipsis-multiline".into());
        }
    }
    if copyable.is_some() {
        class_list.push("adui-text-copyable".into());
    }
    if editable.is_some() {
        class_list.push("adui-text-editable".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let mut style_attr = format!("color:{};text-decoration:{};", tone_color, decoration);
    if ellipsis_active && ellipsis_rows > 1 {
        style_attr.push_str("-webkit-line-clamp:");
        style_attr.push_str(&ellipsis_rows.to_string());
        style_attr.push(';');
    }
    if let Some(extra) = style {
        style_attr.push_str(&extra);
    }

    let tooltip_attr = if ellipsis_active {
        ellipsis_tooltip.clone()
    } else {
        None
    };

    let content_node = if editable.is_some() && *editing.read() {
        render_editing(
            variant,
            editable.clone().unwrap(),
            edit_value,
            editing,
            on_edit,
            on_edit_cancel,
        )
    } else {
        rsx! { span {
            class: "adui-typography-content",
            {children}
        } }
    };

    let copy_cfg = copyable.clone();
    let edit_cfg = editable.clone();
    match (variant, content_node) {
        (TypographyVariant::Text, node) => rsx! {
            span {
                class: "{class_attr}",
                style: "{style_attr}",
                title: tooltip_attr.clone().unwrap_or_default(),
                {node}
                if let Some(cfg) = copy_cfg.clone() {
                    {render_copy_control(cfg, disabled, copy_status, on_copy)}
                }
                if let Some(cfg) = edit_cfg.clone() {
                    if let Some(control) = render_edit_trigger(
                        cfg,
                        disabled,
                        editing,
                        edit_value,
                        on_edit_start,
                    ) {
                        {control}
                    }
                }
                if let Some(btn) = render_expand_control(
                    ellipsis_enabled,
                    ellipsis_expandable,
                    ellipsis_expanded,
                    ellipsis_expand_text.as_str(),
                    ellipsis_collapse_text.as_str(),
                ) {
                    {btn}
                }
            }
        },
        (TypographyVariant::Paragraph, node) => rsx! {
            p {
                class: "{class_attr}",
                style: "{style_attr}",
                title: tooltip_attr.clone().unwrap_or_default(),
                {node}
                if let Some(cfg) = copy_cfg.clone() {
                    {render_copy_control(cfg, disabled, copy_status, on_copy)}
                }
                if let Some(cfg) = edit_cfg.clone() {
                    if let Some(control) = render_edit_trigger(
                        cfg,
                        disabled,
                        editing,
                        edit_value,
                        on_edit_start,
                    ) {
                        {control}
                    }
                }
                if let Some(btn) = render_expand_control(
                    ellipsis_enabled,
                    ellipsis_expandable,
                    ellipsis_expanded,
                    ellipsis_expand_text.as_str(),
                    ellipsis_collapse_text.as_str(),
                ) {
                    {btn}
                }
            }
        },
        (TypographyVariant::Title(level), node) => {
            let tooltip = tooltip_attr.unwrap_or_default();
            match level {
                TitleLevel::H1 => {
                    rsx!(h1 { class: "{class_attr}", style: "{style_attr}", title: tooltip.clone(), {node}
                        if let Some(cfg) = copy_cfg.clone() { {render_copy_control(cfg, disabled, copy_status, on_copy)} }
                        if let Some(cfg) = edit_cfg.clone() {
                            if let Some(control) = render_edit_trigger(cfg, disabled, editing, edit_value, on_edit_start) { {control} }
                        }
                        if let Some(btn) = render_expand_control(
                            ellipsis_enabled,
                            ellipsis_expandable,
                            ellipsis_expanded,
                            ellipsis_expand_text.as_str(),
                            ellipsis_collapse_text.as_str(),
                        ) { {btn} }
                    })
                }
                TitleLevel::H2 => {
                    rsx!(h2 { class: "{class_attr}", style: "{style_attr}", title: tooltip.clone(), {node}
                        if let Some(cfg) = copy_cfg.clone() { {render_copy_control(cfg, disabled, copy_status, on_copy)} }
                        if let Some(cfg) = edit_cfg.clone() {
                            if let Some(control) = render_edit_trigger(cfg, disabled, editing, edit_value, on_edit_start) { {control} }
                        }
                        if let Some(btn) = render_expand_control(
                            ellipsis_enabled,
                            ellipsis_expandable,
                            ellipsis_expanded,
                            ellipsis_expand_text.as_str(),
                            ellipsis_collapse_text.as_str(),
                        ) { {btn} }
                    })
                }
                TitleLevel::H3 => {
                    rsx!(h3 { class: "{class_attr}", style: "{style_attr}", title: tooltip.clone(), {node}
                        if let Some(cfg) = copy_cfg.clone() { {render_copy_control(cfg, disabled, copy_status, on_copy)} }
                        if let Some(cfg) = edit_cfg.clone() {
                            if let Some(control) = render_edit_trigger(cfg, disabled, editing, edit_value, on_edit_start) { {control} }
                        }
                        if let Some(btn) = render_expand_control(
                            ellipsis_enabled,
                            ellipsis_expandable,
                            ellipsis_expanded,
                            ellipsis_expand_text.as_str(),
                            ellipsis_collapse_text.as_str(),
                        ) { {btn} }
                    })
                }
                TitleLevel::H4 => {
                    rsx!(h4 { class: "{class_attr}", style: "{style_attr}", title: tooltip.clone(), {node}
                        if let Some(cfg) = copy_cfg.clone() { {render_copy_control(cfg, disabled, copy_status, on_copy)} }
                        if let Some(cfg) = edit_cfg.clone() {
                            if let Some(control) = render_edit_trigger(cfg, disabled, editing, edit_value, on_edit_start) { {control} }
                        }
                        if let Some(btn) = render_expand_control(
                            ellipsis_enabled,
                            ellipsis_expandable,
                            ellipsis_expanded,
                            ellipsis_expand_text.as_str(),
                            ellipsis_collapse_text.as_str(),
                        ) { {btn} }
                    })
                }
                TitleLevel::H5 => {
                    rsx!(h5 { class: "{class_attr}", style: "{style_attr}", title: tooltip, {node}
                        if let Some(cfg) = copy_cfg {
                            {render_copy_control(cfg, disabled, copy_status, on_copy)}
                        }
                        if let Some(cfg) = edit_cfg {
                            if let Some(control) = render_edit_trigger(cfg, disabled, editing, edit_value, on_edit_start) { {control} }
                        }
                        if let Some(btn) = render_expand_control(
                            ellipsis_enabled,
                            ellipsis_expandable,
                            ellipsis_expanded,
                            ellipsis_expand_text.as_str(),
                            ellipsis_collapse_text.as_str(),
                        ) { {btn} }
                    })
                }
            }
        }
    }
}

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
    #[props(default = true)]
    pub wrap: bool,
    #[props(default)]
    pub ellipsis: bool,
    #[props(optional)]
    pub ellipsis_config: Option<TypographyEllipsis>,
    #[props(optional)]
    pub copyable: Option<TypographyCopyable>,
    #[props(optional)]
    pub editable: Option<TypographyEditable>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub on_copy: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit_cancel: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit_start: Option<EventHandler<()>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

impl From<TextProps> for TypographyBaseProps {
    fn from(value: TextProps) -> Self {
        Self {
            variant: TypographyVariant::Text,
            r#type: value.r#type,
            strong: value.strong,
            italic: value.italic,
            underline: value.underline,
            delete: value.delete,
            code: value.code,
            mark: value.mark,
            wrap: value.wrap,
            ellipsis: value.ellipsis,
            ellipsis_config: value.ellipsis_config,
            copyable: value.copyable,
            editable: value.editable,
            disabled: value.disabled,
            on_copy: value.on_copy,
            on_edit: value.on_edit,
            on_edit_cancel: value.on_edit_cancel,
            on_edit_start: value.on_edit_start,
            class: value.class,
            style: value.style,
            children: value.children,
        }
    }
}

/// Inline text typography.
#[component]
pub fn Text(props: TextProps) -> Element {
    TypographyBase(props.into())
}

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
    #[props(default = true)]
    pub wrap: bool,
    #[props(default)]
    pub ellipsis: bool,
    #[props(optional)]
    pub ellipsis_config: Option<TypographyEllipsis>,
    #[props(optional)]
    pub copyable: Option<TypographyCopyable>,
    #[props(optional)]
    pub editable: Option<TypographyEditable>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub on_copy: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit_cancel: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit_start: Option<EventHandler<()>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

impl From<ParagraphProps> for TypographyBaseProps {
    fn from(value: ParagraphProps) -> Self {
        Self {
            variant: TypographyVariant::Paragraph,
            r#type: value.r#type,
            strong: value.strong,
            italic: value.italic,
            underline: value.underline,
            delete: value.delete,
            code: value.code,
            mark: value.mark,
            wrap: value.wrap,
            ellipsis: value.ellipsis,
            ellipsis_config: value.ellipsis_config,
            copyable: value.copyable,
            editable: value.editable,
            disabled: value.disabled,
            on_copy: value.on_copy,
            on_edit: value.on_edit,
            on_edit_cancel: value.on_edit_cancel,
            on_edit_start: value.on_edit_start,
            class: value.class,
            style: value.style,
            children: value.children,
        }
    }
}

/// Block paragraph typography.
#[component]
pub fn Paragraph(props: ParagraphProps) -> Element {
    TypographyBase(props.into())
}

#[derive(Props, Clone, PartialEq)]
pub struct TitleProps {
    #[props(default)]
    pub level: TitleLevel,
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
    #[props(default = true)]
    pub wrap: bool,
    #[props(default)]
    pub ellipsis: bool,
    #[props(optional)]
    pub ellipsis_config: Option<TypographyEllipsis>,
    #[props(optional)]
    pub copyable: Option<TypographyCopyable>,
    #[props(optional)]
    pub editable: Option<TypographyEditable>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub on_copy: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit_cancel: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_edit_start: Option<EventHandler<()>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

impl From<TitleProps> for TypographyBaseProps {
    fn from(value: TitleProps) -> Self {
        Self {
            variant: TypographyVariant::Title(value.level),
            r#type: value.r#type,
            strong: value.strong,
            italic: value.italic,
            underline: value.underline,
            delete: value.delete,
            code: value.code,
            mark: value.mark,
            wrap: value.wrap,
            ellipsis: value.ellipsis,
            ellipsis_config: value.ellipsis_config,
            copyable: value.copyable,
            editable: value.editable,
            disabled: value.disabled,
            on_copy: value.on_copy,
            on_edit: value.on_edit,
            on_edit_cancel: value.on_edit_cancel,
            on_edit_start: value.on_edit_start,
            class: value.class,
            style: value.style,
            children: value.children,
        }
    }
}

/// Heading typography rendered as h1-h5.
#[component]
pub fn Title(props: TitleProps) -> Element {
    TypographyBase(props.into())
}

fn resolve_color(tokens: &crate::theme::ThemeTokens, tone: TextType, disabled: bool) -> String {
    if disabled {
        return tokens.color_text_disabled.clone();
    }
    match tone {
        TextType::Default => tokens.color_text.clone(),
        TextType::Secondary => tokens.color_text_secondary.clone(),
        TextType::Success => tokens.color_success.clone(),
        TextType::Warning => tokens.color_warning.clone(),
        TextType::Danger => tokens.color_error.clone(),
        TextType::Disabled => tokens.color_text_disabled.clone(),
    }
}

fn text_decoration(underline: bool, delete: bool) -> String {
    let mut entries = Vec::new();
    if underline {
        entries.push("underline");
    }
    if delete {
        entries.push("line-through");
    }
    if entries.is_empty() {
        "none".into()
    } else {
        entries.join(" ")
    }
}

fn level_index(level: TitleLevel) -> u8 {
    match level {
        TitleLevel::H1 => 1,
        TitleLevel::H2 => 2,
        TitleLevel::H3 => 3,
        TitleLevel::H4 => 4,
        TitleLevel::H5 => 5,
    }
}

fn render_copy_control(
    cfg: TypographyCopyable,
    disabled: bool,
    copy_state: Signal<bool>,
    on_copy: Option<EventHandler<String>>,
) -> Element {
    let idle = cfg
        .tooltips
        .as_ref()
        .map(|pair| pair.0.clone())
        .unwrap_or_else(|| "复制".into());
    let success = cfg
        .tooltips
        .as_ref()
        .map(|pair| pair.1.clone())
        .unwrap_or_else(|| "已复制".into());
    let idle_icon = cfg.icon.clone().unwrap_or_else(|| {
        rsx!(Icon {
            kind: IconKind::Copy,
            size: 16.0
        })
    });
    let copied_icon = cfg.copied_icon.clone().unwrap_or_else(|| {
        rsx!(Icon {
            kind: IconKind::Check,
            size: 16.0
        })
    });
    let text_click = cfg.text.clone();
    let text_key = cfg.text.clone();
    let handler_click = on_copy;
    let handler_key = on_copy;
    let state_click = copy_state;
    let state_key = copy_state;
    rsx! {
        span {
            class: "adui-typography-control adui-typography-copy",
            tabindex: if disabled { "-1" } else { "0" },
            role: "button",
            title: if *copy_state.read() { success.clone() } else { idle.clone() },
            "aria-disabled": disabled,
            onclick: move |_| {
                if disabled {
                    return;
                }
                trigger_copy(text_click.clone(), handler_click, state_click);
            },
            onkeydown: move |evt: KeyboardEvent| {
                if disabled {
                    return;
                }
                if matches_key_activate(&evt) {
                    evt.stop_propagation();
                    evt.prevent_default();
                    trigger_copy(text_key.clone(), handler_key, state_key);
                }
            },
            if *copy_state.read() {
                {copied_icon}
            } else {
                {idle_icon}
            }
        }
    }
}

fn render_expand_control(
    enabled: bool,
    expandable: bool,
    state: Signal<bool>,
    expand_text: &str,
    collapse_text: &str,
) -> Option<Element> {
    if !enabled || !expandable {
        return None;
    }
    let label = if *state.read() {
        collapse_text.to_owned()
    } else {
        expand_text.to_owned()
    };
    let mut toggle = state;
    Some(rsx! {
        button {
            r#type: "button",
            class: "adui-typography-control adui-typography-expand",
            onclick: move |_| {
                let current = { *toggle.read() };
                toggle.set(!current);
            },
            {label}
        }
    })
}

fn render_edit_trigger(
    cfg: TypographyEditable,
    disabled: bool,
    editing: Signal<bool>,
    edit_value: Signal<String>,
    on_edit_start: Option<EventHandler<()>>,
) -> Option<Element> {
    if *editing.read() {
        return None;
    }
    let icon = cfg.enter_icon.clone().unwrap_or_else(|| {
        rsx!(Icon {
            kind: IconKind::Edit,
            size: 16.0
        })
    });
    let text_seed_click = cfg.text.clone();
    let text_seed_key = cfg.text.clone();
    let handler_click = on_edit_start;
    let handler_key = on_edit_start;
    let mut editing_click = editing;
    let mut editing_key = editing;
    let mut value_click = edit_value;
    let mut value_key = edit_value;
    Some(rsx! {
        span {
            class: "adui-typography-control adui-typography-edit",
            tabindex: if disabled { "-1" } else { "0" },
            role: "button",
            "aria-disabled": disabled,
            onclick: move |_| {
                if disabled {
                    return;
                }
                editing_click.set(true);
                if let Some(default_text) = text_seed_click.clone() {
                    value_click.set(default_text);
                }
                if let Some(handler) = handler_click.as_ref() {
                    handler.call(());
                }
            },
            onkeydown: move |evt: KeyboardEvent| {
                if disabled {
                    return;
                }
                if matches_key_activate(&evt) {
                    evt.prevent_default();
                    editing_key.set(true);
                    if let Some(default_text) = text_seed_key.clone() {
                        value_key.set(default_text);
                    }
                    if let Some(handler) = handler_key.as_ref() {
                        handler.call(());
                    }
                }
            },
            {icon}
        }
    })
}

fn render_editing(
    variant: TypographyVariant,
    cfg: TypographyEditable,
    edit_value: Signal<String>,
    editing: Signal<bool>,
    on_edit: Option<EventHandler<String>>,
    on_edit_cancel: Option<EventHandler<String>>,
) -> Element {
    let placeholder = cfg
        .placeholder
        .clone()
        .unwrap_or_else(|| "请输入".to_string());
    let enter_icon = cfg.enter_icon.clone().unwrap_or_else(|| {
        rsx!(Icon {
            kind: IconKind::Check,
            size: 16.0
        })
    });
    let cancel_icon = cfg.cancel_icon.clone().unwrap_or_else(|| {
        rsx!(Icon {
            kind: IconKind::Close,
            size: 16.0
        })
    });
    let auto_focus = cfg.auto_focus;
    let max_len = cfg.max_length.map(|len| len.to_string());
    let submit_handler = on_edit;
    let cancel_handler = on_edit_cancel;

    let text_control = match variant {
        TypographyVariant::Paragraph => {
            let mut value_signal = edit_value;
            let submit_value = edit_value;
            let submit_editing = editing;
            let submit_handler_clone = submit_handler;
            let cancel_value = edit_value;
            let cancel_editing = editing;
            let cancel_handler_clone = cancel_handler;
            rsx! {
                textarea {
                    class: "adui-typography-textarea",
                    value: "{submit_value.read().clone()}",
                    placeholder: "{placeholder}",
                    autofocus: auto_focus,
                    maxlength: max_len.clone().unwrap_or_default(),
                    oninput: move |evt| {
                        value_signal.set(evt.value());
                    },
                    onkeydown: move |evt: KeyboardEvent| {
                        if matches!(evt.key(), Key::Enter) && evt.modifiers().contains(Modifiers::CONTROL) {
                            evt.prevent_default();
                            submit_edit_action(submit_value, submit_editing, submit_handler_clone);
                        }
                        if matches!(evt.key(), Key::Escape) {
                            evt.prevent_default();
                            cancel_edit_action(cancel_value, cancel_editing, cancel_handler_clone);
                        }
                    }
                }
            }
        }
        _ => {
            let mut value_signal = edit_value;
            let submit_value = edit_value;
            let submit_editing = editing;
            let submit_handler_clone = submit_handler;
            let cancel_value = edit_value;
            let cancel_editing = editing;
            let cancel_handler_clone = cancel_handler;
            rsx! {
                input {
                    class: "adui-typography-input",
                    r#type: "text",
                    value: "{submit_value.read().clone()}",
                    placeholder: "{placeholder}",
                    autofocus: auto_focus,
                    maxlength: max_len.clone().unwrap_or_default(),
                    oninput: move |evt| {
                        value_signal.set(evt.value());
                    },
                    onkeydown: move |evt: KeyboardEvent| {
                        if matches!(evt.key(), Key::Enter) {
                            evt.prevent_default();
                            submit_edit_action(submit_value, submit_editing, submit_handler_clone);
                        }
                        if matches!(evt.key(), Key::Escape) {
                            evt.prevent_default();
                            cancel_edit_action(cancel_value, cancel_editing, cancel_handler_clone);
                        }
                    }
                }
            }
        }
    };
    let submit_button_value = edit_value;
    let submit_button_editing = editing;
    let submit_button_handler = on_edit;
    let cancel_button_value = edit_value;
    let cancel_button_editing = editing;
    let cancel_button_handler = on_edit_cancel;
    rsx! {
        span {
            class: "adui-text-editing",
            {text_control}
            button {
                class: "adui-typography-edit-btn",
                r#type: "button",
                onclick: move |_| {
                    submit_edit_action(
                        submit_button_value,
                        submit_button_editing,
                        submit_button_handler,
                    );
                },
                {enter_icon}
            }
            button {
                class: "adui-typography-edit-btn",
                r#type: "button",
                onclick: move |_| {
                    cancel_edit_action(
                        cancel_button_value,
                        cancel_button_editing,
                        cancel_button_handler,
                    );
                },
                {cancel_icon}
            }
        }
    }
}

fn submit_edit_action(
    value: Signal<String>,
    editing: Signal<bool>,
    handler: Option<EventHandler<String>>,
) {
    let current = value.read().clone();
    if let Some(cb) = handler {
        cb.call(current.clone());
    }
    let mut editing_signal = editing;
    editing_signal.set(false);
}

fn cancel_edit_action(
    value: Signal<String>,
    editing: Signal<bool>,
    handler: Option<EventHandler<String>>,
) {
    let mut editing_signal = editing;
    editing_signal.set(false);
    let current = value.read().clone();
    if let Some(cb) = handler {
        cb.call(current);
    }
}

fn trigger_copy(text: String, handler: Option<EventHandler<String>>, mut copy_state: Signal<bool>) {
    if let Some(cb) = handler {
        cb.call(text.clone());
    }
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            let _ = clipboard.write_text(&text);
        }
        copy_state.set(true);
        schedule_copy_reset(copy_state);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        copy_state.set(true);
    }
}

fn matches_key_activate(evt: &KeyboardEvent) -> bool {
    match evt.key() {
        Key::Enter => true,
        Key::Character(ref c) if c == " " => true,
        _ => false,
    }
}

#[cfg(target_arch = "wasm32")]
fn schedule_copy_reset(state: Signal<bool>) {
    if let Some(window) = web_sys::window() {
        let mut state_clone = state;
        let callback = Closure::once(move || {
            state_clone.set(false);
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            1500,
        );
        callback.forget();
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn schedule_copy_reset(_state: Signal<bool>) {}
