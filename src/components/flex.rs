use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Row
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexJustify {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

impl Default for FlexJustify {
    fn default() -> Self {
        FlexJustify::Start
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexAlign {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

impl Default for FlexAlign {
    fn default() -> Self {
        FlexAlign::Stretch
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        FlexWrap::NoWrap
    }
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
    pub gap: Option<f32>,
    #[props(optional)]
    pub row_gap: Option<f32>,
    #[props(optional)]
    pub column_gap: Option<f32>,
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
        gap,
        row_gap,
        column_gap,
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-flex".to_string()];
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let style_attr = format!(
        "display:flex;flex-direction:{};justify-content:{};align-items:{};flex-wrap:{};gap:{};row-gap:{};column-gap:{};{}",
        match direction {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Column => "column",
            FlexDirection::ColumnReverse => "column-reverse",
        },
        match justify {
            FlexJustify::Start => "flex-start",
            FlexJustify::End => "flex-end",
            FlexJustify::Center => "center",
            FlexJustify::Between => "space-between",
            FlexJustify::Around => "space-around",
            FlexJustify::Evenly => "space-evenly",
        },
        match align {
            FlexAlign::Start => "flex-start",
            FlexAlign::End => "flex-end",
            FlexAlign::Center => "center",
            FlexAlign::Stretch => "stretch",
            FlexAlign::Baseline => "baseline",
        },
        match wrap {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        },
        gap.map(|g| format!("{g}px")).unwrap_or_default(),
        row_gap.map(|g| format!("{g}px")).unwrap_or_default(),
        column_gap.map(|g| format!("{g}px")).unwrap_or_default(),
        style.unwrap_or_default(),
    );

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}
