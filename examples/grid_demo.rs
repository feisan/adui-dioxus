use adui_dioxus::{
    Col, ColResponsive, ColSize, ResponsiveGutter, ResponsiveValue, Row, RowAlign, RowJustify,
    ThemeProvider,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DemoPage {}
        }
    }
}

#[component]
fn DemoPage() -> Element {
    let gutter = ResponsiveGutter {
        horizontal: ResponsiveValue {
            xs: Some(8.0),
            sm: Some(12.0),
            md: Some(16.0),
            lg: Some(24.0),
            ..Default::default()
        },
        vertical: Some(ResponsiveValue {
            sm: Some(12.0),
            md: Some(16.0),
            ..Default::default()
        }),
    };

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",
            h1 { "Grid Row & Col" }
            p { "下方 Row 在不同断点有不同 gutter，Col 则根据 `ColResponsive` 设置调整 span/offset/order。" }

            Row {
                gutter: Some(24.0),
                gutter_vertical: Some(16.0),
                responsive_gutter: Some(gutter.clone()),
                justify: RowJustify::SpaceBetween,
                align: RowAlign::Top,
                GridCard { title: "xs 24", body: "span=24, md 12", responsive: Some(col_size(Some(12), None)) },
                GridCard { title: "order", body: "在 lg 以上 order=1", responsive: Some(order_size(Some(8), Some(1))) },
                GridCard { title: "offset", body: "sm 12, offset 6", responsive: Some(offset_size(Some((12, 6)))) },
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct GridCardProps {
    title: &'static str,
    body: &'static str,
    #[props(optional)]
    responsive: Option<ColResponsive>,
}

#[component]
fn GridCard(props: GridCardProps) -> Element {
    let span = props.responsive.clone().unwrap_or_default();
    rsx! {
        Col {
            span: 24,
            responsive: Some(span),
            div {
                style: "min-height: 120px; border: 1px dashed var(--adui-color-border); padding: 12px; border-radius: var(--adui-radius); background: var(--adui-color-bg-container);",
                h3 { style: "margin: 0 0 8px 0;", "{props.title}" }
                p { style: "margin: 0; color: var(--adui-color-text-secondary);", "{props.body}" }
            }
        }
    }
}

fn col_size(md_span: Option<u16>, xl_span: Option<u16>) -> ColResponsive {
    ColResponsive {
        md: md_span.map(|span| ColSize {
            span: Some(span),
            ..Default::default()
        }),
        xl: xl_span.map(|span| ColSize {
            span: Some(span),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn order_size(lg_span: Option<u16>, order: Option<i16>) -> ColResponsive {
    ColResponsive {
        lg: Some(ColSize {
            span: lg_span,
            order,
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn offset_size(sm: Option<(u16, u16)>) -> ColResponsive {
    ColResponsive {
        sm: sm.map(|(span, offset)| ColSize {
            span: Some(span),
            offset: Some(offset),
            ..Default::default()
        }),
        ..Default::default()
    }
}
