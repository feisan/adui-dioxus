//! Grid 组件演示
//!
//! 展示 Grid 组件的基础用法和高级用法，包括：
//! - 24列网格系统
//! - 响应式布局
//! - 偏移和排序
//! - 响应式间距

use adui_dioxus::{
    Button, ButtonType, Col, ColResponsive, ColSize, ResponsiveGutter, ResponsiveValue, Row,
    RowAlign, RowJustify, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            GridDemo {}
        }
    }
}

#[component]
fn GridDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let responsive_gutter = ResponsiveGutter {
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

            // 控制工具栏
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 24px; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border);",
                span { style: "font-weight: 600;", "主题控制：" }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Light,
                    "Light"
                }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Dark,
                    "Dark"
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础网格
            DemoSection {
                title: "基础网格（24列系统）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Row {
                        gutter: Some(12.0),
                        GridCard { span: 6, "6" }
                        GridCard { span: 6, "6" }
                        GridCard { span: 6, "6" }
                        GridCard { span: 6, "6" }
                    }
                    Row {
                        gutter: Some(12.0),
                        GridCard { span: 8, "8" }
                        GridCard { span: 8, "8" }
                        GridCard { span: 8, "8" }
                    }
                    Row {
                        gutter: Some(12.0),
                        GridCard { span: 12, "12" }
                        GridCard { span: 12, "12" }
                    }
                    Row {
                        gutter: Some(12.0),
                        GridCard { span: 24, "24" }
                    }
                }
            }

            // 对齐方式
            DemoSection {
                title: "对齐方式",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "margin-bottom: 8px; font-size: 14px; font-weight: 600;",
                        "justify: Start"
                    }
                    Row {
                        gutter: Some(12.0),
                        justify: RowJustify::Start,
                        GridCard { span: 6, "6" }
                        GridCard { span: 6, "6" }
                    }
                    div {
                        style: "margin: 8px 0; font-size: 14px; font-weight: 600;",
                        "justify: Center"
                    }
                    Row {
                        gutter: Some(12.0),
                        justify: RowJustify::Center,
                        GridCard { span: 6, "6" }
                        GridCard { span: 6, "6" }
                    }
                    div {
                        style: "margin: 8px 0; font-size: 14px; font-weight: 600;",
                        "justify: End"
                    }
                    Row {
                        gutter: Some(12.0),
                        justify: RowJustify::End,
                        GridCard { span: 6, "6" }
                        GridCard { span: 6, "6" }
                    }
                    div {
                        style: "margin: 8px 0; font-size: 14px; font-weight: 600;",
                        "justify: SpaceBetween"
                    }
                    Row {
                        gutter: Some(12.0),
                        justify: RowJustify::SpaceBetween,
                        GridCard { span: 6, "6" }
                        GridCard { span: 6, "6" }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 响应式布局
            DemoSection {
                title: "响应式布局",
                div {
                    style: "margin-bottom: 12px; padding: 8px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                    "在不同屏幕尺寸下自动调整：xs(24列) → md(12列) → xl(12列)"
                }
                Row {
                    gutter: Some(12.0),
                    Col {
                        span: 24,
                        responsive: Some(ColResponsive {
                            md: Some(ColSize {
                                span: Some(12),
                                ..Default::default()
                            }),
                            xl: Some(ColSize {
                                span: Some(12),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        GridCard { span: 24, "响应式列" }
                    }
                    Col {
                        span: 24,
                        responsive: Some(ColResponsive {
                            md: Some(ColSize {
                                span: Some(12),
                                ..Default::default()
                            }),
                            xl: Some(ColSize {
                                span: Some(12),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        GridCard { span: 24, "响应式列" }
                    }
                }
            }

            // 偏移
            DemoSection {
                title: "列偏移",
                Row {
                    gutter: Some(12.0),
                    Col {
                        span: 6,
                        offset: 6,
                        GridCard { span: 6, "offset: 6" }
                    }
                    Col {
                        span: 6,
                        offset: 6,
                        GridCard { span: 6, "offset: 6" }
                    }
                }
                Row {
                    gutter: Some(12.0),
                    Col {
                        span: 8,
                        offset: 8,
                        GridCard { span: 8, "offset: 8" }
                    }
                }
            }

            // 排序
            DemoSection {
                title: "列排序",
                Row {
                    gutter: Some(12.0),
                    Col {
                        span: 6,
                        order: Some(3),
                        GridCard { span: 6, "order: 3" }
                    }
                    Col {
                        span: 6,
                        order: Some(1),
                        GridCard { span: 6, "order: 1" }
                    }
                    Col {
                        span: 6,
                        order: Some(2),
                        GridCard { span: 6, "order: 2" }
                    }
                    Col {
                        span: 6,
                        order: Some(4),
                        GridCard { span: 6, "order: 4" }
                    }
                }
            }

            // 响应式间距
            DemoSection {
                title: "响应式间距",
                div {
                    style: "margin-bottom: 12px; padding: 8px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                    "在不同屏幕尺寸下使用不同的间距：xs(8px) → sm(12px) → md(16px) → lg(24px)"
                }
                Row {
                    responsive_gutter: Some(responsive_gutter.clone()),
                    GridCard { span: 8, "响应式间距" }
                    GridCard { span: 8, "响应式间距" }
                    GridCard { span: 8, "响应式间距" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct GridCardProps {
    span: u16,
    children: Element,
}

#[component]
fn GridCard(props: GridCardProps) -> Element {
    rsx! {
        Col {
            span: props.span,
            div {
                style: "min-height: 80px; padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); text-align: center; display: flex; align-items: center; justify-content: center;",
                {props.children}
            }
        }
    }
}

// 统一的demo section组件
#[derive(Props, Clone, PartialEq)]
struct DemoSectionProps {
    title: &'static str,
    children: Element,
}

#[component]
fn DemoSection(props: DemoSectionProps) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 24px; padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);",
            div {
                style: "font-weight: 600; margin-bottom: 12px; color: var(--adui-color-text); font-size: 14px;",
                {props.title}
            }
            {props.children}
        }
    }
}
