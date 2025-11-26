use super::layout_utils::{GapPreset, compose_gap_style, push_gap_preset_class};
use dioxus::core::DynamicNode;
use dioxus::prelude::*;

/// Layout direction for spaced items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpaceDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Cross-axis alignment strategy for space items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpaceAlign {
    #[default]
    Start,
    End,
    Center,
    Baseline,
}

/// Preset sizes that map to theme spacing tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpaceSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl From<SpaceSize> for GapPreset {
    fn from(value: SpaceSize) -> Self {
        match value {
            SpaceSize::Small => GapPreset::Small,
            SpaceSize::Middle => GapPreset::Middle,
            SpaceSize::Large => GapPreset::Large,
        }
    }
}

/// Props for the spacing wrapper.
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    #[props(default)]
    pub direction: SpaceDirection,
    #[props(default)]
    pub size: SpaceSize,
    #[props(optional)]
    pub gap: Option<f32>,
    #[props(optional)]
    pub gap_cross: Option<f32>,
    #[props(optional)]
    pub wrap: Option<bool>,
    #[props(default)]
    pub align: SpaceAlign,
    #[props(default)]
    pub compact: bool,
    #[props(optional)]
    pub split: Option<Element>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Flex-based spacing wrapper with optional split separators.
/// For custom split content, prefer passing children from an iterator or fragment so they can be interleaved.
#[component]
pub fn Space(props: SpaceProps) -> Element {
    let SpaceProps {
        direction,
        size,
        gap,
        gap_cross,
        wrap,
        align,
        compact,
        split,
        class,
        style,
        children,
    } = props;

    let nodes = collect_children(children)?;

    let should_wrap = wrap.unwrap_or(matches!(direction, SpaceDirection::Horizontal));

    let mut class_list = vec!["adui-space".to_string()];
    class_list.push(match direction {
        SpaceDirection::Horizontal => "adui-space-horizontal".into(),
        SpaceDirection::Vertical => "adui-space-vertical".into(),
    });
    if should_wrap && matches!(direction, SpaceDirection::Horizontal) {
        class_list.push("adui-space-wrap".into());
    }
    class_list.push(match align {
        SpaceAlign::Start => "adui-space-align-start".into(),
        SpaceAlign::End => "adui-space-align-end".into(),
        SpaceAlign::Center => "adui-space-align-center".into(),
        SpaceAlign::Baseline => "adui-space-align-baseline".into(),
    });
    if compact {
        class_list.push("adui-space-compact".into());
    } else if gap.is_none() && gap_cross.is_none() {
        push_gap_preset_class(&mut class_list, "adui-space-size", Some(size.into()));
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");
    let row_gap_override = if matches!(direction, SpaceDirection::Horizontal) {
        gap_cross
    } else {
        None
    };
    let column_gap_override = if matches!(direction, SpaceDirection::Vertical) {
        gap_cross
    } else {
        None
    };
    let style_attr = compose_gap_style(style, gap, row_gap_override, column_gap_override);

    if let Some(separator) = split {
        let sep_node: VNode = separator?;
        let total = nodes.len();
        return rsx! {
            div {
                class: "{class_attr}",
                style: "{style_attr}",
                {
                    nodes
                        .into_iter()
                        .enumerate()
                        .flat_map(move |(idx, child)| {
                            let mut group = vec![child];
                            if idx + 1 != total {
                                group.push(sep_node.clone());
                            }
                            group
                        })
                        .map(|node| rsx!({node}))
                }
            }
        };
    }

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {nodes.into_iter().map(|node| rsx!({node}))}
        }
    }
}

fn collect_children(children: Element) -> Result<Vec<VNode>, RenderError> {
    let vnode = children?;
    if let Some(fragment) = vnode.template.roots.iter().find_map(|root| {
        root.dynamic_id()
            .and_then(|id| match &vnode.dynamic_nodes[id] {
                DynamicNode::Fragment(list) => Some(list.clone()),
                _ => None,
            })
    }) {
        return Ok(fragment);
    }

    Ok(vec![vnode])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_children_handles_static_roots_with_fallback() {
        let nodes = collect_children(rsx! {
            div { "one" }
            div { "two" }
        })
        .unwrap();
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn collect_children_handles_single_node() {
        let nodes = collect_children(rsx!(div { "one" })).unwrap();
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn collect_children_extracts_dynamic_fragment() {
        let nodes = collect_children(rsx! {
            {(0..2).map(|idx| rsx!(div { "{idx}" }))}
        })
        .unwrap();
        assert_eq!(nodes.len(), 2);
    }
}
