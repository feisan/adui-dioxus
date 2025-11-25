use dioxus::core::DynamicNode;
use dioxus::prelude::*;

/// Layout direction for spaced items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpaceDirection {
    Horizontal,
    Vertical,
}

impl Default for SpaceDirection {
    fn default() -> Self {
        SpaceDirection::Horizontal
    }
}

/// Cross-axis alignment strategy for space items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpaceAlign {
    Start,
    End,
    Center,
    Baseline,
}

impl Default for SpaceAlign {
    fn default() -> Self {
        SpaceAlign::Start
    }
}

/// Props for the spacing wrapper.
#[derive(Props, Clone, PartialEq)]
pub struct SpaceProps {
    #[props(default)]
    pub direction: SpaceDirection,
    #[props(optional)]
    pub gap: Option<f32>,
    #[props(optional)]
    pub wrap: Option<bool>,
    #[props(default)]
    pub align: SpaceAlign,
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
        gap,
        wrap,
        align,
        split,
        class,
        style,
        children,
    } = props;

    let nodes = collect_children(children)?;

    let mut class_list = vec!["adui-space".to_string()];
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");
    let gap_val = gap.unwrap_or(8.0);
    let style_attr = format!(
        "display:flex;flex-direction:{};flex-wrap:{};align-items:{};gap:{}px;{}",
        match direction {
            SpaceDirection::Horizontal => "row",
            SpaceDirection::Vertical => "column",
        },
        if wrap.unwrap_or(direction == SpaceDirection::Horizontal) {
            "wrap"
        } else {
            "nowrap"
        },
        match align {
            SpaceAlign::Start => "flex-start",
            SpaceAlign::End => "flex-end",
            SpaceAlign::Center => "center",
            SpaceAlign::Baseline => "baseline",
        },
        gap_val,
        style.unwrap_or_default()
    );

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
