use crate::components::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Preset tag colors aligned with Ant Design semantics (MVP subset).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagColor {
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

impl TagColor {
    fn as_class(&self) -> &'static str {
        match self {
            TagColor::Default => "adui-tag-default",
            TagColor::Primary => "adui-tag-primary",
            TagColor::Success => "adui-tag-success",
            TagColor::Warning => "adui-tag-warning",
            TagColor::Error => "adui-tag-error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_color_class_mapping_is_stable() {
        assert_eq!(TagColor::Default.as_class(), "adui-tag-default");
        assert_eq!(TagColor::Primary.as_class(), "adui-tag-primary");
        assert_eq!(TagColor::Success.as_class(), "adui-tag-success");
        assert_eq!(TagColor::Warning.as_class(), "adui-tag-warning");
        assert_eq!(TagColor::Error.as_class(), "adui-tag-error");
    }
}

/// Props for the Tag component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    /// Preset color for the tag.
    #[props(optional)]
    pub color: Option<TagColor>,
    /// Whether the tag can be closed.
    #[props(default)]
    pub closable: bool,
    /// Called when the close icon is clicked.
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
    /// Whether the tag is checkable (togglable selection).
    #[props(default)]
    pub checkable: bool,
    /// Controlled checked state for checkable tags.
    #[props(optional)]
    pub checked: Option<bool>,
    /// Default checked state for uncontrolled checkable tags.
    #[props(optional)]
    pub default_checked: Option<bool>,
    /// Called when the checked state changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<bool>>,
    /// Extra class for the tag.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the tag.
    #[props(optional)]
    pub style: Option<String>,
    /// Tag content.
    pub children: Element,
}

/// Ant Design flavored Tag (MVP: preset colors, closable, simple checkable).
#[component]
pub fn Tag(props: TagProps) -> Element {
    let TagProps {
        color,
        closable,
        on_close,
        checkable,
        checked,
        default_checked,
        on_change,
        class,
        style,
        children,
    } = props;

    let is_checked_controlled = checked.is_some();
    let initial_checked = default_checked.unwrap_or(false);
    let checked_signal: Signal<bool> = use_signal(|| initial_checked);
    let current_checked = checked.unwrap_or_else(|| *checked_signal.read());

    let mut class_list = vec!["adui-tag".to_string()];
    if let Some(color_kind) = color {
        class_list.push(color_kind.as_class().into());
    }
    if checkable {
        class_list.push("adui-tag-checkable".into());
        if current_checked {
            class_list.push("adui-tag-checkable-checked".into());
        }
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let on_close_cb = on_close;
    let on_change_cb = on_change;
    let checked_signal_for_click = checked_signal;

    rsx! {
        span {
            class: "{class_attr}",
            style: "{style_attr}",
            onclick: move |_| {
                if checkable {
                    let next = !current_checked;
                    if !is_checked_controlled {
                        let mut sig = checked_signal_for_click;
                        sig.set(next);
                    }
                    if let Some(cb) = on_change_cb {
                        cb.call(next);
                    }
                }
            },
            {children}
            if closable {
                button {
                    r#type: "button",
                    class: "adui-tag-close",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        if let Some(cb) = on_close_cb {
                            cb.call(());
                        }
                    },
                    Icon { kind: IconKind::Close, size: 12.0 }
                }
            }
        }
    }
}
