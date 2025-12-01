use crate::theme::use_theme;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    Left,
    #[default]
    Center,
    Right,
}

/// Props for rendering a horizontal or vertical divider.
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    #[props(default)]
    pub dashed: bool,
    #[props(default)]
    pub plain: bool,
    #[props(default)]
    pub vertical: bool,
    #[props(default)]
    pub orientation: DividerOrientation,
    #[props(optional)]
    pub orientation_margin: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// 可选内容。
    #[props(optional)]
    pub content: Option<Element>,
}

/// Divider with optional title content and orientation.
#[component]
pub fn Divider(props: DividerProps) -> Element {
    let DividerProps {
        dashed,
        plain,
        vertical,
        orientation,
        orientation_margin,
        class,
        style,
        content,
    } = props;
    let theme = use_theme();
    let tokens = theme.tokens();

    let has_content = content.is_some();

    let mut class_list = vec!["adui-divider".to_string()];
    if vertical {
        class_list.push("adui-divider-vertical".into());
    } else {
        class_list.push("adui-divider-horizontal".into());
        if has_content {
            class_list.push(match orientation {
                DividerOrientation::Left => "adui-divider-left".into(),
                DividerOrientation::Center => "adui-divider-center".into(),
                DividerOrientation::Right => "adui-divider-right".into(),
            });
        }
    }
    if dashed {
        class_list.push("adui-divider-dashed".into());
    }
    if plain {
        class_list.push("adui-divider-plain".into());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let style_attr = format!(
        "border-color:{};{}",
        tokens.color_border,
        style.unwrap_or_default()
    );

    if vertical {
        return rsx! {
            div {
                class: "{class_attr}",
                style: "{style_attr}",
                role: "separator",
                "aria-orientation": "vertical",
            }
        };
    }

    let margin = orientation_margin.unwrap_or_else(|| "16px".into());

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "separator",
            "aria-orientation": "horizontal",
            if let Some(node) = content {
                span {
                    class: "adui-divider-inner-text",
                    style: format!("margin: 0 {margin};"),
                    {node}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divider_orientation_default() {
        assert_eq!(DividerOrientation::default(), DividerOrientation::Center);
    }

    #[test]
    fn divider_orientation_variants() {
        assert_ne!(DividerOrientation::Left, DividerOrientation::Center);
        assert_ne!(DividerOrientation::Center, DividerOrientation::Right);
        assert_ne!(DividerOrientation::Left, DividerOrientation::Right);
    }

    #[test]
    fn divider_orientation_equality() {
        assert_eq!(DividerOrientation::Left, DividerOrientation::Left);
        assert_eq!(DividerOrientation::Center, DividerOrientation::Center);
        assert_eq!(DividerOrientation::Right, DividerOrientation::Right);
    }

    #[test]
    fn divider_props_defaults() {
        let props = DividerProps {
            dashed: false,
            plain: false,
            vertical: false,
            orientation: DividerOrientation::default(),
            orientation_margin: None,
            class: None,
            style: None,
            content: None,
        };
        assert_eq!(props.dashed, false);
        assert_eq!(props.plain, false);
        assert_eq!(props.vertical, false);
        assert_eq!(props.orientation, DividerOrientation::Center);
    }

    #[test]
    fn divider_orientation_clone() {
        let original = DividerOrientation::Left;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn divider_orientation_all_variants() {
        // Test all variants exist
        let left = DividerOrientation::Left;
        let center = DividerOrientation::Center;
        let right = DividerOrientation::Right;

        assert_ne!(left, center);
        assert_ne!(center, right);
        assert_ne!(left, right);
    }
}
