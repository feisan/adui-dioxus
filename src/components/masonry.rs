use dioxus::prelude::*;

/// Properties for a simple masonry layout.
#[derive(Props, Clone, PartialEq)]
pub struct MasonryProps {
    #[props(default = 3)]
    pub columns: u16,
    #[props(optional)]
    pub gap: Option<f32>,
    #[props(optional)]
    pub min_column_width: Option<f32>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Masonry layout using CSS columns for simple waterfall.
#[component]
pub fn Masonry(props: MasonryProps) -> Element {
    let MasonryProps {
        columns,
        gap,
        min_column_width,
        class,
        style,
        children,
    } = props;

    let class_attr = format!("adui-masonry {}", class.unwrap_or_default());
    let gap_val = gap.unwrap_or(16.0);
    let style_attr = format!(
        "column-count:{};column-gap:{}px;--adui-masonry-gap:{}px;{}{}",
        columns,
        gap_val,
        gap_val,
        min_column_width
            .map(|w| format!("column-width:{}px;", w))
            .unwrap_or_default(),
        style.unwrap_or_default()
    );

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}
