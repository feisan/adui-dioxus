//! Flex 组件演示
//!
//! 展示 Flex 组件的基础用法和高级用法，包括：
//! - 方向和对齐
//! - 换行
//! - 间距
//! - ConfigProvider

use adui_dioxus::{
    Button, ButtonType, Flex, FlexAlign, FlexComponent, FlexConfigProvider, FlexDirection, FlexGap,
    FlexJustify, FlexSharedConfig, FlexWrap, ThemeMode, ThemeProvider, Title, TitleLevel,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            FlexDemo {}
        }
    }
}

#[component]
fn FlexDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let flex_config = FlexSharedConfig {
        class: Some("demo-flex-shared".into()),
        style: Some("gap: 8px;".into()),
        vertical: Some(false),
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

            // 方向
            DemoSection {
                title: "方向（Direction）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 8px;",
                        "Row（水平）"
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        gap: Some(8.0),
                        SampleBox { "A" }
                        SampleBox { "B" }
                        SampleBox { "C" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "Column（垂直）"
                    }
                    Flex {
                        direction: FlexDirection::Column,
                        gap: Some(8.0),
                        SampleBox { "A" }
                        SampleBox { "B" }
                        SampleBox { "C" }
                    }
                }
            }

            // 对齐方式
            DemoSection {
                title: "对齐方式（Justify & Align）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 8px;",
                        "Justify: Start / Center / End / SpaceBetween"
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        justify: FlexJustify::Start,
                        gap: Some(8.0),
                        SampleBox { "Start" }
                        SampleBox { "Start" }
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        justify: FlexJustify::Center,
                        gap: Some(8.0),
                        SampleBox { "Center" }
                        SampleBox { "Center" }
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        justify: FlexJustify::End,
                        gap: Some(8.0),
                        SampleBox { "End" }
                        SampleBox { "End" }
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        justify: FlexJustify::Between,
                        gap: Some(8.0),
                        SampleBox { "Between" }
                        SampleBox { "Between" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "Align: Start / Center / End"
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        align: FlexAlign::Start,
                        gap: Some(8.0),
                        style: Some("height: 100px;".into()),
                        SampleBox { "Start" }
                        SampleBox { "Center" }
                        SampleBox { "End" }
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        align: FlexAlign::Center,
                        gap: Some(8.0),
                        style: Some("height: 100px;".into()),
                        SampleBox { "Start" }
                        SampleBox { "Center" }
                        SampleBox { "End" }
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        align: FlexAlign::End,
                        gap: Some(8.0),
                        style: Some("height: 100px;".into()),
                        SampleBox { "Start" }
                        SampleBox { "Center" }
                        SampleBox { "End" }
                    }
                }
            }

            // 换行
            DemoSection {
                title: "换行（Wrap）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Flex {
                        direction: FlexDirection::Row,
                        wrap: FlexWrap::Wrap,
                        gap: Some(8.0),
                        {
                            (0..8).map(|i| {
                                let label = format!("Item {}", i + 1);
                                rsx! {
                                    SampleBox { {label} }
                                }
                            })
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 间距
            DemoSection {
                title: "间距（Gap）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 8px;",
                        "Small Gap"
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        gap_size: Some(FlexGap::Small),
                        SampleBox { "A" }
                        SampleBox { "B" }
                        SampleBox { "C" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "Middle Gap"
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        gap_size: Some(FlexGap::Middle),
                        SampleBox { "A" }
                        SampleBox { "B" }
                        SampleBox { "C" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "Large Gap"
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        gap_size: Some(FlexGap::Large),
                        SampleBox { "A" }
                        SampleBox { "B" }
                        SampleBox { "C" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "自定义 Gap (24px)"
                    }
                    Flex {
                        direction: FlexDirection::Row,
                        gap: Some(24.0),
                        SampleBox { "A" }
                        SampleBox { "B" }
                        SampleBox { "C" }
                    }
                }
            }

            // ConfigProvider
            DemoSection {
                title: "ConfigProvider（配置提供者）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    FlexConfigProvider {
                        value: flex_config.clone(),
                        Flex {
                            component: FlexComponent::Section,
                            justify: FlexJustify::Between,
                            align: FlexAlign::Center,
                            wrap: FlexWrap::Wrap,
                            gap_size: Some(FlexGap::Large),
                            SampleBox { "继承配置" }
                            SampleBox { "继承配置" }
                            SampleBox { "继承配置" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SampleBoxProps {
    children: Element,
}

#[component]
fn SampleBox(props: SampleBoxProps) -> Element {
    rsx! {
        div {
            style: "padding: 12px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); min-width: 80px; text-align: center;",
            {props.children}
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
