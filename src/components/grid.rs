use dioxus::prelude::*;

/// Horizontal justification for a grid row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowJustify {
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

impl Default for RowJustify {
    fn default() -> Self {
        RowJustify::Start
    }
}

/// Cross-axis alignment for row items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowAlign {
    Top,
    Middle,
    Bottom,
    Stretch,
}

impl Default for RowAlign {
    fn default() -> Self {
        RowAlign::Top
    }
}

/// Layout props for a row container.
#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    #[props(optional)]
    pub gutter: Option<f32>,
    #[props(default)]
    pub justify: RowJustify,
    #[props(default)]
    pub align: RowAlign,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Flex-based grid row with 24-column gutter system.
#[component]
pub fn Row(props: RowProps) -> Element {
    let RowProps {
        gutter,
        justify,
        align,
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-row".to_string()];
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");
    let gutter_val = gutter.unwrap_or(0.0);
    let half = gutter_val / 2.0;
    let style_attr = format!(
        "display:flex;flex-wrap:wrap;row-gap:{gutter_val}px;margin-left:-{half}px;margin-right:-{half}px;justify-content:{};align-items:{};--adui-row-gutter:{gutter_val}px;{}",
        match justify {
            RowJustify::Start => "flex-start",
            RowJustify::End => "flex-end",
            RowJustify::Center => "center",
            RowJustify::SpaceAround => "space-around",
            RowJustify::SpaceBetween => "space-between",
            RowJustify::SpaceEvenly => "space-evenly",
        },
        match align {
            RowAlign::Top => "flex-start",
            RowAlign::Middle => "center",
            RowAlign::Bottom => "flex-end",
            RowAlign::Stretch => "stretch",
        },
        style.unwrap_or_default(),
        half = half
    );

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}

/// Column sizing options within a row.
#[derive(Props, Clone, PartialEq)]
pub struct ColProps {
    #[props(default = 24)]
    pub span: u16,
    #[props(default)]
    pub offset: u16,
    #[props(optional)]
    pub flex: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Grid column in a 24-part system with optional flex sizing.
#[component]
pub fn Col(props: ColProps) -> Element {
    let ColProps {
        span,
        offset,
        flex,
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-col".to_string()];
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let width_percent = (span as f32 / 24.0) * 100.0;
    let offset_percent = (offset as f32 / 24.0) * 100.0;

    let style_attr = if let Some(flex_val) = flex {
        format!(
            "flex:{flex_val};max-width:100%;padding:0 calc(var(--adui-row-gutter, 0px)/2);box-sizing:border-box;{}",
            style.unwrap_or_default()
        )
    } else {
        format!(
            "flex:0 0 {width_percent}%;max-width:{width_percent}%;margin-left:{offset_percent}%;padding:0 calc(var(--adui-row-gutter, 0px)/2);box-sizing:border-box;{}",
            style.unwrap_or_default()
        )
    };

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}
