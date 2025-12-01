use crate::components::config_provider::ComponentSize;
use crate::components::skeleton::Skeleton;
use dioxus::prelude::*;

/// Props for the Card component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Optional card title rendered in the header.
    #[props(optional)]
    pub title: Option<Element>,
    /// Optional extra content rendered in the header's right area.
    #[props(optional)]
    pub extra: Option<Element>,
    /// Whether to show a border around the card.
    #[props(default = true)]
    pub bordered: bool,
    /// Visual density of the card paddings and typography.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Loading state. When true, the body renders a simple skeleton instead
    /// of the provided children.
    #[props(default)]
    pub loading: bool,
    /// Whether the card should have a hover effect.
    #[props(default)]
    pub hoverable: bool,
    /// Extra class name for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Card body content.
    pub children: Element,
}

fn build_card_classes(
    bordered: bool,
    size: Option<ComponentSize>,
    hoverable: bool,
    extra_class: Option<String>,
) -> String {
    let mut class_list = vec!["adui-card".to_string()];
    if bordered {
        class_list.push("adui-card-bordered".into());
    }
    if hoverable {
        class_list.push("adui-card-hoverable".into());
    }
    if let Some(sz) = size {
        match sz {
            ComponentSize::Small => class_list.push("adui-card-sm".into()),
            ComponentSize::Middle => {}
            ComponentSize::Large => class_list.push("adui-card-lg".into()),
        }
    }
    if let Some(extra) = extra_class {
        class_list.push(extra);
    }
    class_list.join(" ")
}

/// Ant Design flavored Card (MVP: basic card with optional title/extra/loading).
#[component]
pub fn Card(props: CardProps) -> Element {
    let CardProps {
        title,
        extra,
        bordered,
        size,
        loading,
        hoverable,
        class,
        style,
        children,
    } = props;

    let class_attr = build_card_classes(bordered, size, hoverable, class);
    let style_attr = style.unwrap_or_default();

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if title.is_some() || extra.is_some() {
                div { class: "adui-card-head",
                    if let Some(head_title) = title {
                        div { class: "adui-card-head-title", {head_title} }
                    }
                    if let Some(head_extra) = extra {
                        div { class: "adui-card-head-extra", {head_extra} }
                    }
                }
            }

            div { class: "adui-card-body",
                if loading {
                    Skeleton {
                        loading: Some(true),
                        active: true,
                        paragraph_rows: Some(3),
                    }
                } else {
                    {children}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_card_classes_includes_flags() {
        let classes =
            build_card_classes(true, Some(ComponentSize::Small), true, Some("extra".into()));

        assert!(classes.contains("adui-card"));
        assert!(classes.contains("adui-card-bordered"));
        assert!(classes.contains("adui-card-hoverable"));
        assert!(classes.contains("adui-card-sm"));
        assert!(classes.contains("extra"));
    }

    #[test]
    fn build_card_classes_handles_minimal_case() {
        let classes = build_card_classes(false, None, false, None);
        assert_eq!(classes, "adui-card");
    }

    #[test]
    fn build_card_classes_bordered_only() {
        let classes = build_card_classes(true, None, false, None);
        assert!(classes.contains("adui-card"));
        assert!(classes.contains("adui-card-bordered"));
        assert!(!classes.contains("adui-card-hoverable"));
    }

    #[test]
    fn build_card_classes_hoverable_only() {
        let classes = build_card_classes(false, None, true, None);
        assert!(classes.contains("adui-card"));
        assert!(!classes.contains("adui-card-bordered"));
        assert!(classes.contains("adui-card-hoverable"));
    }

    #[test]
    fn build_card_classes_size_small() {
        let classes = build_card_classes(false, Some(ComponentSize::Small), false, None);
        assert!(classes.contains("adui-card"));
        assert!(classes.contains("adui-card-sm"));
    }

    #[test]
    fn build_card_classes_size_middle() {
        let classes = build_card_classes(false, Some(ComponentSize::Middle), false, None);
        assert!(classes.contains("adui-card"));
        assert!(!classes.contains("adui-card-sm"));
        assert!(!classes.contains("adui-card-lg"));
    }

    #[test]
    fn build_card_classes_size_large() {
        let classes = build_card_classes(false, Some(ComponentSize::Large), false, None);
        assert!(classes.contains("adui-card"));
        assert!(classes.contains("adui-card-lg"));
    }

    #[test]
    fn build_card_classes_all_combinations() {
        // Bordered + Hoverable
        let classes = build_card_classes(true, None, true, None);
        assert!(classes.contains("adui-card-bordered"));
        assert!(classes.contains("adui-card-hoverable"));

        // Bordered + Small
        let classes = build_card_classes(true, Some(ComponentSize::Small), false, None);
        assert!(classes.contains("adui-card-bordered"));
        assert!(classes.contains("adui-card-sm"));

        // Hoverable + Large
        let classes = build_card_classes(false, Some(ComponentSize::Large), true, None);
        assert!(classes.contains("adui-card-hoverable"));
        assert!(classes.contains("adui-card-lg"));

        // All flags
        let classes = build_card_classes(
            true,
            Some(ComponentSize::Small),
            true,
            Some("custom-class".into()),
        );
        assert!(classes.contains("adui-card"));
        assert!(classes.contains("adui-card-bordered"));
        assert!(classes.contains("adui-card-hoverable"));
        assert!(classes.contains("adui-card-sm"));
        assert!(classes.contains("custom-class"));
    }

    #[test]
    fn build_card_classes_with_extra_class() {
        let classes = build_card_classes(false, None, false, Some("my-custom-class".into()));
        assert!(classes.contains("adui-card"));
        assert!(classes.contains("my-custom-class"));
    }

    #[test]
    fn build_card_classes_multiple_extra_classes() {
        // Note: The function only accepts one extra class string, but we can test it
        let classes = build_card_classes(false, None, false, Some("class1 class2".into()));
        assert!(classes.contains("adui-card"));
        assert!(classes.contains("class1 class2"));
    }

    #[test]
    fn build_card_classes_empty_extra_class() {
        let classes = build_card_classes(false, None, false, Some(String::new()));
        assert!(classes.contains("adui-card"));
        // Empty string should still be added
        let parts: Vec<&str> = classes.split(' ').collect();
        assert!(parts.len() >= 1);
    }

    #[test]
    fn card_props_defaults() {
        // CardProps requires children
        // bordered defaults to true
        // loading defaults to false
        // hoverable defaults to false
    }
}
