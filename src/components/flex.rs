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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flex_direction_variants() {
        assert_eq!(FlexDirection::default(), FlexDirection::Row);
        assert_ne!(FlexDirection::Row, FlexDirection::Column);
        assert_ne!(FlexDirection::RowReverse, FlexDirection::ColumnReverse);
    }

    #[test]
    fn flex_justify_variants() {
        assert_eq!(FlexJustify::default(), FlexJustify::Start);
        assert_ne!(FlexJustify::Start, FlexJustify::End);
        assert_ne!(FlexJustify::Center, FlexJustify::Between);
    }

    #[test]
    fn flex_align_variants() {
        assert_eq!(FlexAlign::default(), FlexAlign::Stretch);
        assert_ne!(FlexAlign::Start, FlexAlign::End);
        assert_ne!(FlexAlign::Center, FlexAlign::Baseline);
    }

    #[test]
    fn flex_wrap_variants() {
        assert_eq!(FlexWrap::default(), FlexWrap::NoWrap);
        assert_ne!(FlexWrap::NoWrap, FlexWrap::Wrap);
        assert_ne!(FlexWrap::Wrap, FlexWrap::WrapReverse);
    }

    #[test]
    fn flex_component_variants() {
        assert_eq!(FlexComponent::default(), FlexComponent::Div);
        assert_ne!(FlexComponent::Div, FlexComponent::Section);
        assert_ne!(FlexComponent::Article, FlexComponent::Nav);
    }

    #[test]
    fn flex_orientation_variants() {
        assert_eq!(FlexOrientation::default(), FlexOrientation::Horizontal);
        assert_ne!(FlexOrientation::Horizontal, FlexOrientation::Vertical);
    }

    #[test]
    fn flex_gap_conversion() {
        // Test that conversion works without panicking
        let _small: GapPreset = FlexGap::Small.into();
        let _middle: GapPreset = FlexGap::Middle.into();
        let _large: GapPreset = FlexGap::Large.into();
    }

    #[test]
    fn flex_shared_config_defaults() {
        let config = FlexSharedConfig::default();
        assert_eq!(config.class, None);
        assert_eq!(config.style, None);
        assert_eq!(config.vertical, None);
    }

    #[test]
    fn compute_direction_with_orientation() {
        // Orientation takes priority
        assert_eq!(
            compute_direction(
                FlexDirection::Row,
                Some(FlexOrientation::Vertical),
                false,
                None
            ),
            FlexDirection::Column
        );
        assert_eq!(
            compute_direction(
                FlexDirection::Column,
                Some(FlexOrientation::Horizontal),
                true,
                None
            ),
            FlexDirection::Row
        );
    }

    #[test]
    fn compute_direction_with_vertical_flag() {
        // Vertical flag converts Row to Column
        assert_eq!(
            compute_direction(FlexDirection::Row, None, true, None),
            FlexDirection::Column
        );
        // But doesn't affect Column
        assert_eq!(
            compute_direction(FlexDirection::Column, None, true, None),
            FlexDirection::Column
        );
    }

    #[test]
    fn compute_direction_with_inherited_vertical() {
        // Inherited vertical flag converts Row to Column
        assert_eq!(
            compute_direction(FlexDirection::Row, None, false, Some(true)),
            FlexDirection::Column
        );
        // But doesn't affect Column
        assert_eq!(
            compute_direction(FlexDirection::Column, None, false, Some(true)),
            FlexDirection::Column
        );
    }

    #[test]
    fn base_classes_includes_flex_class() {
        let classes = base_classes(
            FlexDirection::Row,
            FlexWrap::NoWrap,
            FlexJustify::Start,
            FlexAlign::Stretch,
        );
        assert!(classes.contains(&"adui-flex".to_string()));
    }

    #[test]
    fn base_classes_direction_mapping() {
        let row_classes = base_classes(
            FlexDirection::Row,
            FlexWrap::NoWrap,
            FlexJustify::Start,
            FlexAlign::Stretch,
        );
        assert!(row_classes.contains(&"adui-flex-horizontal".to_string()));

        let col_classes = base_classes(
            FlexDirection::Column,
            FlexWrap::NoWrap,
            FlexJustify::Start,
            FlexAlign::Stretch,
        );
        assert!(col_classes.contains(&"adui-flex-vertical".to_string()));
    }

    #[test]
    fn base_classes_wrap_mapping() {
        let nowrap_classes = base_classes(
            FlexDirection::Row,
            FlexWrap::NoWrap,
            FlexJustify::Start,
            FlexAlign::Stretch,
        );
        assert!(nowrap_classes.contains(&"adui-flex-wrap-nowrap".to_string()));

        let wrap_classes = base_classes(
            FlexDirection::Row,
            FlexWrap::Wrap,
            FlexJustify::Start,
            FlexAlign::Stretch,
        );
        assert!(wrap_classes.contains(&"adui-flex-wrap-wrap".to_string()));
    }

    #[test]
    fn base_classes_justify_mapping() {
        let start_classes = base_classes(
            FlexDirection::Row,
            FlexWrap::NoWrap,
            FlexJustify::Start,
            FlexAlign::Stretch,
        );
        assert!(start_classes.contains(&"adui-flex-justify-start".to_string()));

        let center_classes = base_classes(
            FlexDirection::Row,
            FlexWrap::NoWrap,
            FlexJustify::Center,
            FlexAlign::Stretch,
        );
        assert!(center_classes.contains(&"adui-flex-justify-center".to_string()));
    }

    #[test]
    fn base_classes_align_mapping() {
        let stretch_classes = base_classes(
            FlexDirection::Row,
            FlexWrap::NoWrap,
            FlexJustify::Start,
            FlexAlign::Stretch,
        );
        assert!(stretch_classes.contains(&"adui-flex-align-stretch".to_string()));

        let center_classes = base_classes(
            FlexDirection::Row,
            FlexWrap::NoWrap,
            FlexJustify::Start,
            FlexAlign::Center,
        );
        assert!(center_classes.contains(&"adui-flex-align-center".to_string()));
    }
}
