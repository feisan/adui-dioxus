use dioxus::prelude::*;

/// Built-in image presets for the Empty component.
#[derive(Clone, PartialEq)]
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
