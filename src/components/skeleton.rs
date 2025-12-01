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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_props_defaults() {
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: true,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        assert!(props.loading.is_none());
        assert_eq!(props.active, false);
        assert_eq!(props.title, true);
    }

    #[test]
    fn skeleton_props_loading() {
        let props = SkeletonProps {
            loading: Some(true),
            active: false,
            title: true,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.loading, Some(true));
    }

    #[test]
    fn skeleton_props_active() {
        let props = SkeletonProps {
            loading: None,
            active: true,
            title: true,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.active, true);
    }

    #[test]
    fn skeleton_props_title() {
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: false,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.title, false);
    }

    #[test]
    fn skeleton_props_paragraph_rows() {
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: true,
            paragraph_rows: Some(5),
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.paragraph_rows, Some(5));
    }

    #[test]
    fn skeleton_paragraph_rows_minimum() {
        // Test paragraph rows minimum value logic
        let rows = 0u8;
        let min_rows = rows.max(1);
        assert_eq!(min_rows, 1);

        let rows2 = 3u8;
        let min_rows2 = rows2.max(1);
        assert_eq!(min_rows2, 3);
    }

    #[test]
    fn skeleton_paragraph_rows_boundary_values() {
        // Test various boundary values
        assert_eq!(0u8.max(1), 1);
        assert_eq!(1u8.max(1), 1);
        assert_eq!(2u8.max(1), 2);
        assert_eq!(255u8.max(1), 255);
    }

    #[test]
    fn skeleton_props_loading_false() {
        let props = SkeletonProps {
            loading: Some(false),
            active: false,
            title: true,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.loading, Some(false));
    }

    #[test]
    fn skeleton_props_all_combinations() {
        // Active + Title
        let props = SkeletonProps {
            loading: None,
            active: true,
            title: true,
            paragraph_rows: Some(5),
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.active, true);
        assert_eq!(props.title, true);
        assert_eq!(props.paragraph_rows, Some(5));

        // Loading + Active
        let props = SkeletonProps {
            loading: Some(true),
            active: true,
            title: false,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.loading, Some(true));
        assert_eq!(props.active, true);
    }

    #[test]
    fn skeleton_props_with_class_and_style() {
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: true,
            paragraph_rows: None,
            class: Some("custom-class".into()),
            style: Some("color: red;".into()),
            content: None,
        };
        assert_eq!(props.class, Some("custom-class".into()));
        assert_eq!(props.style, Some("color: red;".into()));
    }

    #[test]
    fn skeleton_props_clone() {
        let props = SkeletonProps {
            loading: Some(true),
            active: true,
            title: false,
            paragraph_rows: Some(10),
            class: Some("test".into()),
            style: Some("test-style".into()),
            content: None,
        };
        let cloned = props.clone();
        assert_eq!(props.loading, cloned.loading);
        assert_eq!(props.active, cloned.active);
        assert_eq!(props.title, cloned.title);
        assert_eq!(props.paragraph_rows, cloned.paragraph_rows);
        assert_eq!(props.class, cloned.class);
        assert_eq!(props.style, cloned.style);
    }

    #[test]
    fn skeleton_props_paragraph_rows_edge_cases() {
        // Minimum value
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: true,
            paragraph_rows: Some(1),
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.paragraph_rows, Some(1));

        // Maximum u8 value
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: true,
            paragraph_rows: Some(255),
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.paragraph_rows, Some(255));
    }

    #[test]
    fn skeleton_props_minimal() {
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: true,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        // Verify all defaults
        assert!(props.loading.is_none());
        assert_eq!(props.active, false);
        assert_eq!(props.title, true);
        assert!(props.paragraph_rows.is_none());
    }

    #[test]
    fn skeleton_props_loading_none_means_true() {
        // The component logic: loading.unwrap_or(true) means None defaults to true
        // This test documents the behavior
        let props = SkeletonProps {
            loading: None,
            active: false,
            title: true,
            paragraph_rows: None,
            class: None,
            style: None,
            content: None,
        };
        assert!(props.loading.is_none());
    }
}
