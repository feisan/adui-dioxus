use super::layout_utils::{GapPreset, compose_gap_style, push_gap_preset_class};
use dioxus::prelude::*;

/// Orientation helper used by design tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Root element type for the flex container.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexComponent {
    #[default]
    Div,
    Section,
    Article,
    Nav,
    Span,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexJustify {
    #[default]
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexAlign {
    Start,
    End,
    Center,
    #[default]
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Preset gap sizes aligned with design tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexGap {
    Small,
    Middle,
    Large,
}

impl From<FlexGap> for GapPreset {
    fn from(value: FlexGap) -> Self {
        match value {
            FlexGap::Small => GapPreset::Small,
            FlexGap::Middle => GapPreset::Middle,
            FlexGap::Large => GapPreset::Large,
        }
    }
}

/// Shared configuration provided via context.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlexSharedConfig {
    pub class: Option<String>,
    pub style: Option<String>,
    pub vertical: Option<bool>,
}

/// Provide flex configuration to descendants (mirrors Ant Design ConfigProvider.flex).
#[derive(Props, Clone, PartialEq)]
pub struct FlexConfigProviderProps {
    pub value: FlexSharedConfig,
    pub children: Element,
}

#[component]
pub fn FlexConfigProvider(props: FlexConfigProviderProps) -> Element {
    let FlexConfigProviderProps { value, children } = props;
    use_context_provider(|| value);
    children
}

#[derive(Props, Clone, PartialEq)]
pub struct FlexProps {
    #[props(default)]
    pub direction: FlexDirection,
    #[props(default)]
    pub justify: FlexJustify,
    #[props(default)]
    pub align: FlexAlign,
    #[props(default)]
    pub wrap: FlexWrap,
    #[props(optional)]
    pub orientation: Option<FlexOrientation>,
    #[props(default)]
    pub vertical: bool,
    #[props(default)]
    pub component: FlexComponent,
    #[props(optional)]
    pub gap: Option<f32>,
    #[props(optional)]
    pub row_gap: Option<f32>,
    #[props(optional)]
    pub column_gap: Option<f32>,
    #[props(optional)]
    pub gap_size: Option<FlexGap>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Flexible box container with configurable alignment and wrapping.
#[component]
pub fn Flex(props: FlexProps) -> Element {
    let FlexProps {
        direction,
        justify,
        align,
        wrap,
        orientation,
        vertical,
        component,
        gap,
        row_gap,
        column_gap,
        gap_size,
        class,
        style,
        children,
    } = props;

    let inherited = try_use_context::<FlexSharedConfig>();
    let inherited_vertical = inherited.as_ref().and_then(|ctx| ctx.vertical);
    let resolved_direction =
        compute_direction(direction, orientation, vertical, inherited_vertical);

    let mut class_list = base_classes(resolved_direction, wrap, justify, align);
    if let Some(ctx) = inherited.as_ref()
        && let Some(extra) = ctx.class.as_ref()
    {
        class_list.push(extra.clone());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    if gap.is_none() && row_gap.is_none() && column_gap.is_none() {
        let preset = gap_size.map(Into::into);
        push_gap_preset_class(&mut class_list, "adui-flex-gap", preset);
    }
    let class_attr = class_list.join(" ");

    let mut base_style = String::new();
    if let Some(ctx) = inherited.as_ref()
        && let Some(st) = ctx.style.as_ref()
    {
        base_style.push_str(st);
    }
    if let Some(extra) = style {
        base_style.push_str(&extra);
    }
    let base_style_opt = if base_style.is_empty() {
        None
    } else {
        Some(base_style)
    };
    let style_attr = compose_gap_style(base_style_opt, gap, row_gap, column_gap);

    render_component(component, &class_attr, &style_attr, &children)
}

fn render_component(
    component: FlexComponent,
    class_attr: &str,
    style_attr: &str,
    children: &Element,
) -> Element {
    match component {
        FlexComponent::Div => {
            rsx!(div { class: "{class_attr}", style: "{style_attr}", {children.clone()} })
        }
        FlexComponent::Section => {
            rsx!(section { class: "{class_attr}", style: "{style_attr}", {children.clone()} })
        }
        FlexComponent::Article => {
            rsx!(article { class: "{class_attr}", style: "{style_attr}", {children.clone()} })
        }
        FlexComponent::Nav => {
            rsx!(nav { class: "{class_attr}", style: "{style_attr}", {children.clone()} })
        }
        FlexComponent::Span => {
            rsx!(span { class: "{class_attr}", style: "{style_attr}", {children.clone()} })
        }
    }
}

fn compute_direction(
    explicit: FlexDirection,
    orientation: Option<FlexOrientation>,
    vertical_flag: bool,
    inherited_vertical: Option<bool>,
) -> FlexDirection {
    if let Some(orientation) = orientation {
        return match orientation {
            FlexOrientation::Horizontal => FlexDirection::Row,
            FlexOrientation::Vertical => FlexDirection::Column,
        };
    }

    if let Some(flag) = inherited_vertical
        && flag
        && matches!(explicit, FlexDirection::Row | FlexDirection::RowReverse)
    {
        return FlexDirection::Column;
    }

    if vertical_flag && matches!(explicit, FlexDirection::Row | FlexDirection::RowReverse) {
        return FlexDirection::Column;
    }

    explicit
}

fn base_classes(
    direction: FlexDirection,
    wrap: FlexWrap,
    justify: FlexJustify,
    align: FlexAlign,
) -> Vec<String> {
    let mut classes = vec!["adui-flex".to_string()];
    match direction {
        FlexDirection::Row => classes.push("adui-flex-horizontal".into()),
        FlexDirection::RowReverse => {
            classes.push("adui-flex-horizontal".into());
            classes.push("adui-flex-row-reverse".into());
        }
        FlexDirection::Column => classes.push("adui-flex-vertical".into()),
        FlexDirection::ColumnReverse => {
            classes.push("adui-flex-vertical".into());
            classes.push("adui-flex-column-reverse".into());
        }
    }

    classes.push(match wrap {
        FlexWrap::NoWrap => "adui-flex-wrap-nowrap".into(),
        FlexWrap::Wrap => "adui-flex-wrap-wrap".into(),
        FlexWrap::WrapReverse => "adui-flex-wrap-wrap-reverse".into(),
    });

    classes.push(match justify {
        FlexJustify::Start => "adui-flex-justify-start".into(),
        FlexJustify::End => "adui-flex-justify-end".into(),
        FlexJustify::Center => "adui-flex-justify-center".into(),
        FlexJustify::Between => "adui-flex-justify-between".into(),
        FlexJustify::Around => "adui-flex-justify-around".into(),
        FlexJustify::Evenly => "adui-flex-justify-evenly".into(),
    });

    classes.push(match align {
        FlexAlign::Start => "adui-flex-align-start".into(),
        FlexAlign::End => "adui-flex-align-end".into(),
        FlexAlign::Center => "adui-flex-align-center".into(),
        FlexAlign::Stretch => "adui-flex-align-stretch".into(),
        FlexAlign::Baseline => "adui-flex-align-baseline".into(),
    });

    classes
}
