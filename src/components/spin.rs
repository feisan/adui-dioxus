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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin_size_default() {
        assert_eq!(SpinSize::default(), SpinSize::Default);
    }

    #[test]
    fn spin_size_variants() {
        assert_ne!(SpinSize::Small, SpinSize::Default);
        assert_ne!(SpinSize::Default, SpinSize::Large);
        assert_ne!(SpinSize::Small, SpinSize::Large);
    }

    #[test]
    fn spin_size_equality() {
        assert_eq!(SpinSize::Small, SpinSize::Small);
        assert_eq!(SpinSize::Default, SpinSize::Default);
        assert_eq!(SpinSize::Large, SpinSize::Large);
    }

    #[test]
    fn spin_size_clone() {
        let original = SpinSize::Small;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn spin_props_defaults() {
        let props = SpinProps {
            spinning: None,
            size: None,
            tip: None,
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        assert!(props.spinning.is_none());
        assert!(props.size.is_none());
        assert!(props.tip.is_none());
        assert_eq!(props.fullscreen, false);
    }

    #[test]
    fn spin_size_all_variants() {
        // Test all variants exist
        let small = SpinSize::Small;
        let default = SpinSize::Default;
        let large = SpinSize::Large;

        assert_ne!(small, default);
        assert_ne!(default, large);
        assert_ne!(small, large);
    }

    #[test]
    fn spin_size_debug() {
        let small = SpinSize::Small;
        let default = SpinSize::Default;
        let large = SpinSize::Large;

        let small_str = format!("{:?}", small);
        let default_str = format!("{:?}", default);
        let large_str = format!("{:?}", large);

        assert!(small_str.contains("Small"));
        assert!(default_str.contains("Default"));
        assert!(large_str.contains("Large"));
    }

    #[test]
    fn spin_props_with_all_fields() {
        let props = SpinProps {
            spinning: Some(true),
            size: Some(SpinSize::Large),
            tip: Some("Loading...".into()),
            class: Some("custom-class".into()),
            style: Some("color: red;".into()),
            fullscreen: true,
            children: rsx!(div {}),
        };
        assert_eq!(props.spinning, Some(true));
        assert_eq!(props.size, Some(SpinSize::Large));
        assert_eq!(props.tip, Some("Loading...".into()));
        assert_eq!(props.fullscreen, true);
    }

    #[test]
    fn spin_props_spinning_false() {
        let props = SpinProps {
            spinning: Some(false),
            size: None,
            tip: None,
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        assert_eq!(props.spinning, Some(false));
    }

    #[test]
    fn spin_props_size_small() {
        let props = SpinProps {
            spinning: None,
            size: Some(SpinSize::Small),
            tip: None,
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        assert_eq!(props.size, Some(SpinSize::Small));
    }

    #[test]
    fn spin_props_size_default() {
        let props = SpinProps {
            spinning: None,
            size: Some(SpinSize::Default),
            tip: None,
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        assert_eq!(props.size, Some(SpinSize::Default));
    }

    #[test]
    fn spin_props_size_large() {
        let props = SpinProps {
            spinning: None,
            size: Some(SpinSize::Large),
            tip: None,
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        assert_eq!(props.size, Some(SpinSize::Large));
    }

    #[test]
    fn spin_props_with_tip() {
        let props = SpinProps {
            spinning: None,
            size: None,
            tip: Some("Please wait...".into()),
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        assert_eq!(props.tip, Some("Please wait...".into()));
    }

    #[test]
    fn spin_props_fullscreen() {
        let props = SpinProps {
            spinning: None,
            size: None,
            tip: None,
            class: None,
            style: None,
            fullscreen: true,
            children: rsx!(div {}),
        };
        assert_eq!(props.fullscreen, true);
    }

    #[test]
    fn spin_props_all_combinations() {
        // Spinning + Size + Tip
        let props = SpinProps {
            spinning: Some(true),
            size: Some(SpinSize::Small),
            tip: Some("Loading".into()),
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        assert_eq!(props.spinning, Some(true));
        assert_eq!(props.size, Some(SpinSize::Small));
        assert_eq!(props.tip, Some("Loading".into()));

        // Fullscreen + Large
        let props = SpinProps {
            spinning: Some(true),
            size: Some(SpinSize::Large),
            tip: None,
            class: None,
            style: None,
            fullscreen: true,
            children: rsx!(div {}),
        };
        assert_eq!(props.fullscreen, true);
        assert_eq!(props.size, Some(SpinSize::Large));
    }

    #[test]
    fn spin_props_clone() {
        let props = SpinProps {
            spinning: Some(true),
            size: Some(SpinSize::Large),
            tip: Some("Test".into()),
            class: Some("test-class".into()),
            style: Some("test-style".into()),
            fullscreen: true,
            children: rsx!(div {}),
        };
        let cloned = props.clone();
        assert_eq!(props.spinning, cloned.spinning);
        assert_eq!(props.size, cloned.size);
        assert_eq!(props.tip, cloned.tip);
        assert_eq!(props.fullscreen, cloned.fullscreen);
    }

    #[test]
    fn spin_props_minimal() {
        let props = SpinProps {
            spinning: None,
            size: None,
            tip: None,
            class: None,
            style: None,
            fullscreen: false,
            children: rsx!(div {}),
        };
        // Verify all defaults
        assert!(props.spinning.is_none());
        assert!(props.size.is_none());
        assert!(props.tip.is_none());
        assert_eq!(props.fullscreen, false);
    }
}
