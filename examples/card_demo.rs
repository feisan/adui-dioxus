//! Card 组件演示
//!
//! 展示 Card 组件的基础用法和高级用法，包括：
//! - 基础卡片
//! - 带标题和额外内容的卡片
//! - 加载状态
//! - 悬停效果
//! - 不同尺寸

use adui_dioxus::{
    Button, ButtonType, Card, ComponentSize, Icon, IconKind, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            CardDemo {}
        }
    }
}

#[component]
fn CardDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

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

            // 基础卡片
            DemoSection {
                title: "基础卡片",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px;",
                    Card {
                        "这是基础卡片内容，可以包含任何内容。"
                    }
                    Card {
                        bordered: false,
                        "无边框卡片"
                    }
                }
            }

            // 带标题的卡片
            DemoSection {
                title: "带标题和额外内容的卡片",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px;",
                    Card {
                        title: Some(rsx!("卡片标题")),
                        "这是卡片内容。"
                    }
                    Card {
                        title: Some(rsx!("卡片标题")),
                        extra: Some(rsx!(
                            Button {
                                r#type: ButtonType::Text,
                                size: adui_dioxus::ButtonSize::Small,
                                "更多"
                            }
                        )),
                        "带标题和额外操作的卡片。"
                    }
                    Card {
                        title: Some(rsx!("卡片标题")),
                        extra: Some(rsx!(Icon { kind: IconKind::Info })),
                        "带图标的额外内容。"
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 加载状态
            DemoSection {
                title: "加载状态",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px;",
                    Card {
                        title: Some(rsx!("加载中的卡片")),
                        loading: true,
                        "这个内容在加载时不会显示。"
                    }
                    Card {
                        title: Some(rsx!("正常卡片")),
                        loading: false,
                        "这是正常显示的卡片内容。"
                    }
                }
            }

            // 悬停效果
            DemoSection {
                title: "悬停效果",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px;",
                    Card {
                        title: Some(rsx!("可悬停卡片")),
                        hoverable: true,
                        "鼠标悬停时会有阴影效果。"
                    }
                    Card {
                        title: Some(rsx!("普通卡片")),
                        hoverable: false,
                        "没有悬停效果。"
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px;",
                    Card {
                        title: Some(rsx!("Small 尺寸")),
                        size: Some(ComponentSize::Small),
                        "这是小尺寸卡片。"
                    }
                    Card {
                        title: Some(rsx!("Middle 尺寸（默认）")),
                        size: Some(ComponentSize::Middle),
                        "这是中等尺寸卡片。"
                    }
                    Card {
                        title: Some(rsx!("Large 尺寸")),
                        size: Some(ComponentSize::Large),
                        "这是大尺寸卡片。"
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 16px;",
                    Card {
                        title: Some(rsx!("产品卡片")),
                        extra: Some(rsx!(
                            Button {
                                r#type: ButtonType::Text,
                                size: adui_dioxus::ButtonSize::Small,
                                "查看详情"
                            }
                        )),
                        hoverable: true,
                        div {
                            style: "display: flex; flex-direction: column; gap: 8px;",
                            div {
                                style: "font-size: 20px; font-weight: 600;",
                                "产品名称"
                            }
                            div {
                                style: "color: var(--adui-color-text-secondary);",
                                "这是产品描述信息。"
                            }
                            div {
                                style: "font-size: 18px; color: var(--adui-color-primary);",
                                "¥99.00"
                            }
                        }
                    }
                    Card {
                        title: Some(rsx!("用户卡片")),
                        hoverable: true,
                        div {
                            style: "display: flex; flex-direction: column; gap: 12px;",
                            div {
                                style: "display: flex; align-items: center; gap: 12px;",
                                div {
                                    style: "width: 48px; height: 48px; border-radius: 50%; background: var(--adui-color-primary-bg); display: flex; align-items: center; justify-content: center;",
                                    Icon { kind: IconKind::Info }
                                }
                                div {
                                    style: "display: flex; flex-direction: column;",
                                    div { style: "font-weight: 600;", "用户名" }
                                    div {
                                        style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                        "user@example.com"
                                    }
                                }
                            }
                            div {
                                style: "padding-top: 12px; border-top: 1px solid var(--adui-color-border);",
                                "这是用户的详细信息。"
                            }
                        }
                    }
                }
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
