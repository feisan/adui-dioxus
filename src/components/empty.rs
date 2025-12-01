use dioxus::prelude::*;

/// Built-in image presets for the Empty component.
#[derive(Clone, Debug, PartialEq)]
pub enum EmptyImage {
    /// Default illustration.
    Default,
    /// Minimal/simple illustration.
    Simple,
    /// Smaller footprint variant.
    Small,
    /// Custom image URL or label.
    Custom(String),
}

/// Props for the Empty component.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyProps {
    /// Optional description text shown under the image.
    #[props(optional)]
    pub description: Option<String>,
    /// Image preset or custom image.
    #[props(optional)]
    pub image: Option<EmptyImage>,
    /// Extra class name applied to the root.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style applied to the root.
    #[props(optional)]
    pub style: Option<String>,
    /// Optional footer content rendered below the description (e.g. action buttons).
    #[props(optional)]
    pub footer: Option<Element>,
}

/// Ant Design flavored Empty component (MVP).
#[component]
pub fn Empty(props: EmptyProps) -> Element {
    let EmptyProps {
        description,
        image,
        class,
        style,
        footer,
    } = props;

    let mut classes = vec!["adui-empty".to_string()];
    if let Some(extra) = class {
        classes.push(extra);
    }

    // Mark small variant for styling when using `EmptyImage::Small`.
    if matches!(image, Some(EmptyImage::Small)) {
        classes.push("adui-empty-sm".to_string());
    }

    let class_attr = classes.join(" ");
    let style_attr = style.unwrap_or_default();

    let description_text = description.unwrap_or_else(|| "暂无数据".to_string());

    // Render image content based on preset.
    let image_node = match image.unwrap_or(EmptyImage::Default) {
        EmptyImage::Default => rsx! {
            svg {
                class: "adui-empty-image-svg",
                view_box: "0 0 64 41",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M8 33h48v2H8z", fill: "#f5f5f5" }
                rect { x: "16", y: "13", width: "32", height: "16", rx: "2", fill: "#fafafa", stroke: "#e5e5e5" }
                circle { cx: "24", cy: "21", r: "3", fill: "#e5e5e5" }
                rect { x: "30", y: "19", width: "12", height: "2", fill: "#e5e5e5" }
                rect { x: "30", y: "23", width: "10", height: "2", fill: "#f0f0f0" }
            }
        },
        EmptyImage::Simple => rsx! {
            div { class: "adui-empty-image-simple" }
        },
        EmptyImage::Small => rsx! {
            div { class: "adui-empty-image-simple" }
        },
        EmptyImage::Custom(url) => rsx! {
            img { class: "adui-empty-image-img", src: "{url}", alt: "empty" }
        },
    };

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            div { class: "adui-empty-image",
                {image_node}
            }
            p { class: "adui-empty-description", "{description_text}" }
            if let Some(footer_node) = footer {
                div { class: "adui-empty-footer", {footer_node} }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_image_variants() {
        assert!(EmptyImage::Default != EmptyImage::Simple);
        assert!(EmptyImage::Simple != EmptyImage::Small);
        assert!(EmptyImage::Default != EmptyImage::Small);
    }

    #[test]
    fn empty_image_custom() {
        let custom1 = EmptyImage::Custom("url1".to_string());
        let custom2 = EmptyImage::Custom("url2".to_string());
        let custom3 = EmptyImage::Custom("url1".to_string());

        assert!(custom1 != custom2);
        assert!(custom1 == custom3);
    }

    #[test]
    fn empty_image_equality() {
        assert!(EmptyImage::Default == EmptyImage::Default);
        assert!(EmptyImage::Simple == EmptyImage::Simple);
        assert!(EmptyImage::Small == EmptyImage::Small);
    }

    #[test]
    fn empty_image_clone() {
        let original = EmptyImage::Default;
        let cloned = original.clone();
        assert!(original == cloned);

        let custom_original = EmptyImage::Custom("test".to_string());
        let custom_cloned = custom_original.clone();
        assert!(custom_original == custom_cloned);
    }

    #[test]
    fn empty_props_defaults() {
        let props = EmptyProps {
            description: None,
            image: None,
            class: None,
            style: None,
            footer: None,
        };
        assert!(props.description.is_none());
        assert!(props.image.is_none());
    }

    #[test]
    fn empty_image_custom_string() {
        let url = "https://example.com/image.png";
        let custom = EmptyImage::Custom(url.to_string());
        match custom {
            EmptyImage::Custom(s) => assert_eq!(s, url),
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn empty_image_all_variants() {
        let default = EmptyImage::Default;
        let simple = EmptyImage::Simple;
        let small = EmptyImage::Small;
        let custom = EmptyImage::Custom("test".to_string());

        assert_ne!(default, simple);
        assert_ne!(default, small);
        assert_ne!(default, custom);
        assert_ne!(simple, small);
        assert_ne!(simple, custom);
        assert_ne!(small, custom);
    }

    #[test]
    fn empty_image_debug() {
        let default = EmptyImage::Default;
        let debug_str = format!("{:?}", default);
        // Just verify it doesn't panic
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn empty_props_with_all_fields() {
        let props = EmptyProps {
            description: Some("Custom description".to_string()),
            image: Some(EmptyImage::Simple),
            class: Some("custom-class".to_string()),
            style: Some("color: red;".to_string()),
            footer: None,
        };
        assert_eq!(props.description, Some("Custom description".to_string()));
        assert_eq!(props.image, Some(EmptyImage::Simple));
        assert_eq!(props.class, Some("custom-class".to_string()));
    }

    #[test]
    fn empty_props_minimal() {
        let props = EmptyProps {
            description: None,
            image: None,
            class: None,
            style: None,
            footer: None,
        };
        assert!(props.description.is_none());
        assert!(props.image.is_none());
        assert!(props.class.is_none());
        assert!(props.style.is_none());
        assert!(props.footer.is_none());
    }

    #[test]
    fn empty_image_custom_empty_string() {
        let custom = EmptyImage::Custom(String::new());
        match custom {
            EmptyImage::Custom(s) => assert_eq!(s, ""),
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn empty_image_custom_long_string() {
        let long_url = "https://example.com/very/long/path/to/image.png?query=param&other=value";
        let custom = EmptyImage::Custom(long_url.to_string());
        match custom {
            EmptyImage::Custom(s) => assert_eq!(s, long_url),
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn empty_props_clone() {
        let props = EmptyProps {
            description: Some("Test".to_string()),
            image: Some(EmptyImage::Default),
            class: None,
            style: None,
            footer: None,
        };
        let cloned = props.clone();
        assert_eq!(props.description, cloned.description);
        assert_eq!(props.image, cloned.image);
    }

    #[test]
    fn empty_image_custom_equality_with_same_string() {
        let custom1 = EmptyImage::Custom("test".to_string());
        let custom2 = EmptyImage::Custom("test".to_string());
        assert_eq!(custom1, custom2);
    }

    #[test]
    fn empty_image_custom_equality_with_different_string() {
        let custom1 = EmptyImage::Custom("test1".to_string());
        let custom2 = EmptyImage::Custom("test2".to_string());
        assert_ne!(custom1, custom2);
    }
}
