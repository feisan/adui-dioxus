use dioxus::prelude::*;

/// Size variants for the Spin component.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinSize {
    Small,
    #[default]
    Default,
    Large,
}

/// Props for the Spin component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct SpinProps {
    /// Whether the spin indicator is active. Defaults to true.
    #[props(optional)]
    pub spinning: Option<bool>,
    /// Visual size of the indicator.
    #[props(optional)]
    pub size: Option<SpinSize>,
    /// Optional text shown under the indicator.
    #[props(optional)]
    pub tip: Option<String>,
    /// Extra class for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Whether to treat this as a fullscreen overlay. MVP only exposes
    /// a class hook, concrete layout can be refined later.
    #[props(default)]
    pub fullscreen: bool,
    /// Optional content wrapped by the spinner. When present, Spin will
    /// render children and, when spinning, show a semi-transparent mask
    /// with the indicator on top.
    pub children: Element,
}

/// Ant Design flavored loading spinner (MVP).
#[component]
pub fn Spin(props: SpinProps) -> Element {
    let SpinProps {
        spinning,
        size,
        tip,
        class,
        style,
        fullscreen,
        children,
    } = props;

    let is_spinning = spinning.unwrap_or(true);
    let size = size.unwrap_or_default();

    // Build root class list.
    let mut classes = vec!["adui-spin".to_string(), "adui-spin-nested".to_string()];
    match size {
        SpinSize::Small => classes.push("adui-spin-sm".into()),
        SpinSize::Large => classes.push("adui-spin-lg".into()),
        SpinSize::Default => {}
    }
    if fullscreen {
        classes.push("adui-spin-fullscreen".into());
    }
    if let Some(extra) = class {
        classes.push(extra);
    }
    let class_attr = classes.join(" ");
    let style_attr = style.unwrap_or_default();

    let tip_text = tip.unwrap_or_default();

    // When not spinning we just render child content.
    if !is_spinning {
        return rsx! {
            div { class: "{class_attr}", style: "{style_attr}",
                div { class: "adui-spin-nested-container", {children} }
            }
        };
    }

    // Spinning: render child content with an overlay mask and indicator.
    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            div { class: "adui-spin-nested-container", {children} }
            div { class: "adui-spin-nested-mask",
                div { class: "adui-spin-indicator",
                    span { class: "adui-spin-dot" }
                }
                if !tip_text.is_empty() {
                    div { class: "adui-spin-text", "{tip_text}" }
                }
            }
        }
    }
}
