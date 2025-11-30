//! Anchor 组件演示
//!
//! 展示 Anchor 组件的基础用法和高级用法，包括：
//! - 基础锚点导航
//! - 垂直方向
//! - 水平方向
//! - 嵌套链接
//! - 固定模式

use adui_dioxus::{
    Affix, Anchor, AnchorDirection, AnchorLinkItem, Button, ButtonType, Card, Divider, Text,
    TextType, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            AnchorDemo {}
        }
    }
}

#[component]
fn AnchorDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let mut affixed = use_signal(|| false);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let anchor_items = vec![
        AnchorLinkItem::new("intro", "#doc-intro", "简介"),
        AnchorLinkItem::with_children(
            "install",
            "#doc-install",
            "安装指南",
            vec![
                AnchorLinkItem::new("npm", "#doc-npm", "使用 npm"),
                AnchorLinkItem::new("cargo", "#doc-cargo", "使用 Cargo"),
            ],
        ),
        AnchorLinkItem::new("usage", "#doc-usage", "基本用法"),
        AnchorLinkItem::new("api", "#doc-api", "API 参考"),
        AnchorLinkItem::new("faq", "#doc-faq", "常见问题"),
    ];

    let horizontal_items = vec![
        AnchorLinkItem::new("h1", "#h-section-1", "概述"),
        AnchorLinkItem::new("h2", "#h-section-2", "特性"),
        AnchorLinkItem::new("h3", "#h-section-3", "安装"),
        AnchorLinkItem::new("h4", "#h-section-4", "更新日志"),
    ];

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

            // 基础锚点导航
            DemoSection {
                title: "基础锚点导航",
                div {
                    style: "display: flex; gap: 32px; position: relative;",
                    div {
                        style: "flex: 1; min-width: 0;",
                        DocumentSection {
                            id: "doc-intro",
                            title: "简介",
                            content: rsx!(
                                Text { "Affix 组件将页面元素固定在可视区域的特定位置。常见的使用场景包括侧边栏导航、悬浮操作按钮等。" }
                                Text { "Anchor 组件用于实现页面内的锚点导航。它可以自动检测页面滚动位置，并高亮当前可见的章节链接。" }
                            ),
                        }
                        DocumentSection {
                            id: "doc-install",
                            title: "安装指南",
                            content: rsx!(
                                Text { "你可以通过以下方式安装本组件库：" }
                                div {
                                    id: "doc-npm",
                                    style: "margin: 16px 0;",
                                    Text { strong: true, "使用 npm" }
                                    pre {
                                        style: "background: var(--adui-color-bg-base); padding: 12px; border-radius: 6px; margin-top: 8px;",
                                        code { "npm install adui-dioxus" }
                                    }
                                }
                                div {
                                    id: "doc-cargo",
                                    style: "margin: 16px 0;",
                                    Text { strong: true, "使用 Cargo" }
                                    pre {
                                        style: "background: var(--adui-color-bg-base); padding: 12px; border-radius: 6px; margin-top: 8px;",
                                        code { "cargo add adui-dioxus" }
                                    }
                                }
                            ),
                        }
                        DocumentSection {
                            id: "doc-usage",
                            title: "基本用法",
                            content: rsx!(
                                Text { "使用 Anchor 组件非常简单，只需定义锚点项列表并传入即可。" }
                            ),
                        }
                        DocumentSection {
                            id: "doc-api",
                            title: "API 参考",
                            content: rsx!(
                                Text { "Anchor 组件支持多种配置选项，包括方向、偏移量等。" }
                            ),
                        }
                        DocumentSection {
                            id: "doc-faq",
                            title: "常见问题",
                            content: rsx!(
                                Text { "这里是一些常见问题的解答。" }
                            ),
                        }
                    }
                    div {
                        style: "width: 180px; flex-shrink: 0;",
                        Anchor {
                            items: anchor_items,
                            offset_top: Some(100.0),
                            direction: AnchorDirection::Vertical,
                        }
                    }
                }
            }

            // Affix 固钉
            DemoSection {
                title: "Affix 固钉",
                Card {
                    children: rsx!(
                        Text {
                            r#type: TextType::Secondary,
                            "当页面滚动时，下方按钮距顶部 80px 时会固定。当前状态："
                        }
                        Text {
                            r#type: if *affixed.read() { TextType::Success } else { TextType::Secondary },
                            if *affixed.read() { "已固定 ✓" } else { "未固定" }
                        }
                        div {
                            style: "margin-top: 16px;",
                            Affix {
                                offset_top: Some(80.0),
                                on_change: {
                                    let mut sig = affixed;
                                    move |is_affixed: bool| sig.set(is_affixed)
                                },
                                Button {
                                    r#type: ButtonType::Primary,
                                    "固定到顶部（offset: 80px）"
                                }
                            }
                        }
                    ),
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 水平方向
            DemoSection {
                title: "水平方向",
                Card {
                    children: rsx!(
                        Anchor {
                            items: horizontal_items.clone(),
                            direction: AnchorDirection::Horizontal,
                            affix: false,
                        }
                        div {
                            style: "margin-top: 24px; display: flex; gap: 16px; overflow-x: auto;",
                            div {
                                id: "h-section-1",
                                style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                                Text { strong: true, "概述" }
                                Text {
                                    r#type: TextType::Secondary,
                                    "组件库介绍..."
                                }
                            }
                            div {
                                id: "h-section-2",
                                style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                                Text { strong: true, "特性" }
                                Text {
                                    r#type: TextType::Secondary,
                                    "主要功能特点..."
                                }
                            }
                            div {
                                id: "h-section-3",
                                style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                                Text { strong: true, "安装" }
                                Text {
                                    r#type: TextType::Secondary,
                                    "安装说明..."
                                }
                            }
                            div {
                                id: "h-section-4",
                                style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                                Text { strong: true, "更新日志" }
                                Text {
                                    r#type: TextType::Secondary,
                                    "版本历史..."
                                }
                            }
                        }
                    ),
                }
            }

            // 底部固定
            DemoSection {
                title: "底部固定 Affix",
                Card {
                    children: rsx!(
                        Text { "当你滚动页面时，下方按钮将固定在页面底部 20px 的位置。" }
                        div {
                            style: "margin-top: 16px;",
                            Affix {
                                offset_bottom: Some(20.0),
                                div {
                                    style: "display: inline-flex; gap: 12px; padding: 12px 20px; background: var(--adui-color-bg-elevated); border-radius: 8px; box-shadow: var(--adui-shadow);",
                                    Button {
                                        r#type: ButtonType::Default,
                                        "取消"
                                    }
                                    Button {
                                        r#type: ButtonType::Primary,
                                        "保存更改"
                                    }
                                }
                            }
                        }
                    ),
                }
            }
        }
    }
}

// 文档章节组件
#[derive(Props, Clone, PartialEq)]
struct DocumentSectionProps {
    id: String,
    title: String,
    content: Element,
}

#[component]
fn DocumentSection(props: DocumentSectionProps) -> Element {
    rsx! {
        section {
            id: "{props.id}",
            style: "min-height: 280px; padding: 20px; margin-bottom: 24px; background: var(--adui-color-bg-container); border-radius: 8px; border: 1px solid var(--adui-color-border);",
            Title { level: TitleLevel::H4, "{props.title}" }
            div {
                style: "margin-top: 12px; line-height: 1.8;",
                {props.content}
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
