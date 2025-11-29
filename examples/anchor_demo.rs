//! Affix & Anchor 组件演示
//!
//! 运行命令: cargo run --example anchor_demo

use adui_dioxus::{
    Affix, Anchor, AnchorDirection, AnchorLinkItem, Button, ButtonType, Card, Divider, Text,
    TextType, Theme, ThemeMode, ThemeProvider, Title as AduiTitle, TitleLevel, use_theme,
    THEME_BASE_STYLE,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider { App {} }
    }
}

#[component]
fn App() -> Element {
    let theme = use_theme();
    let mut theme_mode = use_signal(|| ThemeMode::Light);

    use_effect(move || {
        let mode_val = theme_mode();
        let next = match mode_val {
            ThemeMode::Light => Theme::light(),
            ThemeMode::Dark => Theme::dark(),
            ThemeMode::Custom => Theme::light(),
        };
        theme.set_theme(next);
    });

    rsx! {
        style { {THEME_BASE_STYLE} }
        div { style: "min-height: 100vh; background: var(--adui-color-bg-base);",
            // 顶部工具栏
            div { style: "position: sticky; top: 0; z-index: 100; padding: 12px 24px; background: var(--adui-color-bg-container); border-bottom: 1px solid var(--adui-color-border); display: flex; justify-content: space-between; align-items: center;",
                AduiTitle { level: TitleLevel::H4, "Affix & Anchor 演示" }
                Button {
                    onclick: move |_| {
                        theme_mode.set(match theme_mode() {
                            ThemeMode::Light => ThemeMode::Dark,
                            ThemeMode::Dark => ThemeMode::Light,
                            ThemeMode::Custom => ThemeMode::Light,
                        });
                    },
                    if matches!(theme_mode(), ThemeMode::Dark) { "☀️ 浅色模式" } else { "🌙 深色模式" }
                }
            }

            // 主内容区
            div { style: "max-width: 1200px; margin: 0 auto; padding: 24px;",
                AduiTitle { level: TitleLevel::H2, "Affix 固钉 & Anchor 锚点" }
                Text { r#type: TextType::Secondary,
                    "Affix 用于将页面元素固定在可视区域，Anchor 用于页面内锚点导航。"
                }

                Divider {}

                // 基础 Affix 演示
                AduiTitle { level: TitleLevel::H3, "基础 Affix" }
                BasicAffixDemo {}

                Divider {}

                // 带锚点的文档演示
                AduiTitle { level: TitleLevel::H3, "Anchor 锚点导航" }
                Text { r#type: TextType::Secondary,
                    "右侧导航会固定在页面上，并随滚动高亮当前章节。"
                }
                DocumentWithAnchorDemo {}

                Divider {}

                // 水平方向 Anchor
                AduiTitle { level: TitleLevel::H3, "水平方向 Anchor" }
                HorizontalAnchorDemo {}

                Divider {}

                // 底部固定 Affix
                AduiTitle { level: TitleLevel::H3, "底部固定 Affix" }
                BottomAffixDemo {}
            }
        }
    }
}

/// 基础 Affix 演示
#[component]
fn BasicAffixDemo() -> Element {
    let mut affixed = use_signal(|| false);

    rsx! {
        Card {
            title: Some(rsx!("顶部固定示例")),
            children: rsx!(
                Text { r#type: TextType::Secondary,
                    "当页面滚动时，下方按钮距顶部 80px 时会固定。当前状态：" }
                Text { r#type: if *affixed.read() { TextType::Success } else { TextType::Secondary },
                    if *affixed.read() { "已固定 ✓" } else { "未固定" }
                }
                div { style: "margin-top: 16px;",
                    Affix {
                        offset_top: Some(80.0),
                        on_change: move |is_affixed: bool| {
                            affixed.set(is_affixed);
                        },
                        Button { r#type: ButtonType::Primary,
                            "固定到顶部（offset: 80px）"
                        }
                    }
                }
            ),
        }
    }
}

/// 带锚点的文档演示
#[component]
fn DocumentWithAnchorDemo() -> Element {
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

    rsx! {
        div { style: "display: flex; gap: 32px; position: relative;",
            // 左侧文档内容
            div { style: "flex: 1; min-width: 0;",
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

                        div { id: "doc-npm", style: "margin: 16px 0;",
                            Text { strong: true, "使用 npm" }
                            pre { style: "background: var(--adui-color-bg-base); padding: 12px; border-radius: 6px; margin-top: 8px;",
                                code { "npm install adui-dioxus" }
                            }
                        }

                        div { id: "doc-cargo", style: "margin: 16px 0;",
                            Text { strong: true, "使用 Cargo" }
                            pre { style: "background: var(--adui-color-bg-base); padding: 12px; border-radius: 6px; margin-top: 8px;",
                                code { "cargo add adui-dioxus" }
                            }
                        }
                    ),
                }

                DocumentSection {
                    id: "doc-usage",
                    title: "基本用法",
                    content: rsx!(
                        Text { "使用 Anchor 组件非常简单，只需定义锚点项列表并传入即可：" }
                        pre { style: "background: var(--adui-color-bg-base); padding: 12px; border-radius: 6px; margin-top: 12px; overflow-x: auto;",
                            code {
                                "use adui_dioxus::{{Anchor, AnchorLinkItem}};\n\nlet items = vec![\n    AnchorLinkItem::new(\"1\", \"#section-1\", \"章节一\"),\n    AnchorLinkItem::new(\"2\", \"#section-2\", \"章节二\"),\n];\n\nrsx! {{\n    Anchor {{\n        items: items,\n        offset_top: Some(80.0),\n    }}\n}}"
                            }
                        }
                    ),
                }

                DocumentSection {
                    id: "doc-api",
                    title: "API 参考",
                    content: rsx!(
                        Text { strong: true, "Affix 属性" }
                        div { style: "margin: 12px 0; padding: 12px; background: var(--adui-color-bg-base); border-radius: 6px;",
                            ApiItem { name: "offset_top", r#type: "Option<f64>", desc: "距离窗口顶部触发固定的偏移量（像素）" }
                            ApiItem { name: "offset_bottom", r#type: "Option<f64>", desc: "距离窗口底部触发固定的偏移量（像素）" }
                            ApiItem { name: "on_change", r#type: "EventHandler<bool>", desc: "固定状态改变时的回调" }
                        }

                        Text { strong: true, "Anchor 属性" }
                        div { style: "margin: 12px 0; padding: 12px; background: var(--adui-color-bg-base); border-radius: 6px;",
                            ApiItem { name: "items", r#type: "Vec<AnchorLinkItem>", desc: "锚点链接列表" }
                            ApiItem { name: "affix", r#type: "bool", desc: "是否固定（默认 true）" }
                            ApiItem { name: "offset_top", r#type: "Option<f64>", desc: "固定时距顶部的偏移量" }
                            ApiItem { name: "direction", r#type: "AnchorDirection", desc: "排列方向：Vertical | Horizontal" }
                            ApiItem { name: "on_change", r#type: "EventHandler<String>", desc: "当前锚点改变时的回调" }
                        }
                    ),
                }

                DocumentSection {
                    id: "doc-faq",
                    title: "常见问题",
                    content: rsx!(
                        div { style: "margin-bottom: 16px;",
                            Text { strong: true, "Q: 如何禁用 Anchor 的固定功能？" }
                            Text { r#type: TextType::Secondary,
                                "A: 将 affix 属性设置为 false 即可。"
                            }
                        }
                        div { style: "margin-bottom: 16px;",
                            Text { strong: true, "Q: 如何自定义锚点的滚动偏移？" }
                            Text { r#type: TextType::Secondary,
                                "A: 使用 target_offset 属性可以指定滚动到锚点时的偏移量。"
                            }
                        }
                        div { style: "margin-bottom: 16px;",
                            Text { strong: true, "Q: Anchor 支持嵌套链接吗？" }
                            Text { r#type: TextType::Secondary,
                                "A: 是的，垂直模式下支持嵌套的子链接，使用 AnchorLinkItem::with_children() 创建。"
                            }
                        }
                    ),
                }
            }

            // 右侧锚点导航
            div { style: "width: 180px; flex-shrink: 0;",
                Anchor {
                    items: anchor_items,
                    offset_top: Some(100.0),
                    direction: AnchorDirection::Vertical,
                }
            }
        }
    }
}

/// 文档章节组件
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
            AduiTitle { level: TitleLevel::H4, "{props.title}" }
            div { style: "margin-top: 12px; line-height: 1.8;",
                {props.content}
            }
        }
    }
}

/// API 项组件
#[derive(Props, Clone, PartialEq)]
struct ApiItemProps {
    name: &'static str,
    r#type: &'static str,
    desc: &'static str,
}

#[component]
fn ApiItem(props: ApiItemProps) -> Element {
    rsx! {
        div { style: "display: flex; gap: 12px; padding: 8px 0; border-bottom: 1px solid var(--adui-color-border-secondary);",
            code { style: "color: var(--adui-color-primary); min-width: 120px;", "{props.name}" }
            Text { r#type: TextType::Secondary, code: true, "{props.r#type}" }
            Text { "{props.desc}" }
        }
    }
}

/// 水平方向 Anchor 演示
#[component]
fn HorizontalAnchorDemo() -> Element {
    let horizontal_items = vec![
        AnchorLinkItem::new("h1", "#h-section-1", "概述"),
        AnchorLinkItem::new("h2", "#h-section-2", "特性"),
        AnchorLinkItem::new("h3", "#h-section-3", "安装"),
        AnchorLinkItem::new("h4", "#h-section-4", "更新日志"),
    ];

    rsx! {
        Card {
            title: Some(rsx!("水平导航模式")),
            children: rsx!(
                Anchor {
                    items: horizontal_items,
                    direction: AnchorDirection::Horizontal,
                    affix: false,
                }
                div { style: "margin-top: 24px; display: flex; gap: 16px; overflow-x: auto;",
                    div { id: "h-section-1", style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                        Text { strong: true, "概述" }
                        Text { r#type: TextType::Secondary, "组件库介绍..." }
                    }
                    div { id: "h-section-2", style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                        Text { strong: true, "特性" }
                        Text { r#type: TextType::Secondary, "主要功能特点..." }
                    }
                    div { id: "h-section-3", style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                        Text { strong: true, "安装" }
                        Text { r#type: TextType::Secondary, "安装说明..." }
                    }
                    div { id: "h-section-4", style: "min-width: 200px; padding: 16px; background: var(--adui-color-bg-base); border-radius: 6px;",
                        Text { strong: true, "更新日志" }
                        Text { r#type: TextType::Secondary, "版本历史..." }
                    }
                }
            ),
        }
    }
}

/// 底部固定 Affix 演示
#[component]
fn BottomAffixDemo() -> Element {
    rsx! {
        Card {
            title: Some(rsx!("底部固定示例")),
            children: rsx!(
                Text { "当你滚动页面时，下方按钮将固定在页面底部 20px 的位置。" }
                div { style: "margin-top: 16px;",
                    Affix {
                        offset_bottom: Some(20.0),
                        div { style: "display: inline-flex; gap: 12px; padding: 12px 20px; background: var(--adui-color-bg-elevated); border-radius: 8px; box-shadow: var(--adui-shadow);",
                            Button { r#type: ButtonType::Default, "取消" }
                            Button { r#type: ButtonType::Primary, "保存更改" }
                        }
                    }
                }
            ),
        }
    }
}
