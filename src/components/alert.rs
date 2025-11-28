use crate::components::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Semantic type of an Alert.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertType {
    Success,
    Info,
    Warning,
    Error,
}

impl AlertType {
    fn as_class(&self) -> &'static str {
        match self {
            AlertType::Success => "adui-alert-success",
            AlertType::Info => "adui-alert-info",
            AlertType::Warning => "adui-alert-warning",
            AlertType::Error => "adui-alert-error",
        }
    }

    fn icon_kind(&self) -> IconKind {
        match self {
            AlertType::Success => IconKind::Check,
            AlertType::Info => IconKind::Info,
            AlertType::Warning => IconKind::Info,
            AlertType::Error => IconKind::Close,
        }
    }
}

/// Props for the Alert component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// Semantic type of the alert, controlling colors and default icon.
    #[props(default = AlertType::Info)]
    pub r#type: AlertType,
    /// Main message content.
    pub message: Element,
    /// Optional detailed description.
    #[props(optional)]
    pub description: Option<Element>,
    /// Whether to show the semantic icon.
    #[props(default = true)]
    pub show_icon: bool,
    /// Whether the alert can be closed.
    #[props(default)]
    pub closable: bool,
    /// Called when the close button is clicked.
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
    /// Optional custom icon element.
    #[props(optional)]
    pub icon: Option<Element>,
    /// Whether the alert should be rendered as a banner (full width, compact).
    #[props(default)]
    pub banner: bool,
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style on the root element.
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Alert (MVP: type + icon + closable).
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let AlertProps {
        r#type,
        message,
        description,
        show_icon,
        closable,
        on_close,
        icon,
        banner,
        class,
        style,
    } = props;

    let mut class_list = vec!["adui-alert".to_string(), r#type.as_class().to_string()];
    if banner {
        class_list.push("adui-alert-banner".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let on_close_cb = on_close;

    // The visible flag allows the alert to hide itself after close when
    // used in uncontrolled mode.
    let visible = use_signal(|| true);

    if !*visible.read() {
        return VNode::empty();
    }

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if show_icon {
                div { class: "adui-alert-icon",
                    if let Some(custom) = icon.clone() {
                        {custom}
                    } else {
                        Icon { kind: r#type.icon_kind(), size: 16.0 }
                    }
                }
            }
            div { class: "adui-alert-content",
                div { class: "adui-alert-message", {message} }
                if let Some(desc) = description {
                    div { class: "adui-alert-description", {desc} }
                }
            }
            if closable {
                button {
                    r#type: "button",
                    class: "adui-alert-close-icon",
                    onclick: move |_| {
                        if let Some(cb) = on_close_cb {
                            cb.call(());
                        }
                        let mut v = visible;
                        v.set(false);
                    },
                    Icon { kind: IconKind::Close, size: 12.0 }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_type_class_mapping_is_stable() {
        assert_eq!(AlertType::Success.as_class(), "adui-alert-success");
        assert_eq!(AlertType::Info.as_class(), "adui-alert-info");
        assert_eq!(AlertType::Warning.as_class(), "adui-alert-warning");
        assert_eq!(AlertType::Error.as_class(), "adui-alert-error");
    }
}
