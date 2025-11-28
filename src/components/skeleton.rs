use dioxus::prelude::*;

/// Props for the Skeleton component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Whether to display the skeleton. When Some(false), render children instead.
    #[props(optional)]
    pub loading: Option<bool>,
    /// Whether to show active animation.
    #[props(default)]
    pub active: bool,
    /// Whether to render a title block.
    #[props(default = true)]
    pub title: bool,
    /// Number of paragraph lines.
    #[props(optional)]
    pub paragraph_rows: Option<u8>,
    /// Extra root class.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline styles for the root.
    #[props(optional)]
    pub style: Option<String>,
    /// Wrapped content. When `loading = Some(false)`, this content is rendered
    /// instead of the skeleton blocks.
    #[props(optional)]
    pub content: Option<Element>,
}

/// Simple Ant Design flavored Skeleton.
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let SkeletonProps {
        loading,
        active,
        title,
        paragraph_rows,
        class,
        style,
        content,
    } = props;

    let is_loading = loading.unwrap_or(true);

    if !is_loading {
        if let Some(node) = content {
            return node;
        }
        return rsx! {};
    }

    let mut classes = vec!["adui-skeleton".to_string()];
    if active {
        classes.push("adui-skeleton-active".into());
    }
    if let Some(extra) = class {
        classes.push(extra);
    }
    let class_attr = classes.join(" ");
    let style_attr = style.unwrap_or_default();

    let rows = paragraph_rows.unwrap_or(3).max(1);

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if title {
                div { class: "adui-skeleton-title" }
            }
            div { class: "adui-skeleton-paragraph",
                {(0..rows).map(|idx| {
                    let mut line_class = "adui-skeleton-paragraph-line".to_string();
                    if idx == rows - 1 {
                        line_class.push_str(" adui-skeleton-paragraph-line-last");
                    }
                    rsx! {
                        div { class: "{line_class}" }
                    }
                })}
            }
        }
    }
}
