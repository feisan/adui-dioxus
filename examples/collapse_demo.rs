//! Collapse 组件演示
//!
//! 展示 Collapse 组件的基础用法和高级用法，包括：
//! - 基础折叠面板
//! - 手风琴模式
//! - 无边框和幽灵模式
//! - 自定义图标位置
//! - 嵌套面板

use adui_dioxus::{
    Button, ButtonType, Collapse, CollapsePanel, ExpandIconPlacement, ThemeMode, ThemeProvider,
    Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            CollapseDemo {}
        }
    }
}

#[component]
fn CollapseDemo() -> Element {
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

            // 基础折叠面板
            DemoSection {
                title: "基础折叠面板",
                Collapse {
                    items: vec![
                        CollapsePanel::new(
                            "1",
                            rsx! { span { style: "font-weight: 500;", "📋 产品信息" } },
                            rsx! {
                                div {
                                    style: "padding: 16px; line-height: 1.8;",
                                    div { style: "margin-bottom: 8px;", "名称：Ant Design of Dioxus" }
                                    div { style: "margin-bottom: 8px;", "版本：v0.1.0" }
                                    div { "描述：基于 Dioxus 的 Ant Design 风格组件库" }
                                }
                            },
                        ),
                        CollapsePanel::new(
                            "2",
                            rsx! { span { style: "font-weight: 500;", "⚙️ 技术栈" } },
                            rsx! {
                                div {
                                    style: "padding: 16px;",
                                    div { "• Dioxus - 现代化的 Rust UI 框架" }
                                    div { "• Rust - 安全高性能的系统编程语言" }
                                    div { "• WebAssembly - 浏览器中的原生性能" }
                                    div { "• Ant Design - 企业级设计语言" }
                                }
                            },
                        ),
                        CollapsePanel::new(
                            "3",
                            rsx! { span { style: "font-weight: 500; color: #999;", "🚫 功能特性（禁用状态）" } },
                            rsx! {
                                div {
                                    style: "padding: 16px;",
                                    div { "这个面板被禁用了，无法展开。" }
                                }
                            },
                        )
                        .disabled(true),
                    ],
                    default_active_key: vec!["1".to_string()],
                }
            }

            // 手风琴模式
            DemoSection {
                title: "手风琴模式",
                Collapse {
                    items: vec![
                        CollapsePanel::new(
                            "1",
                            rsx! { "🎯 什么是手风琴模式？" },
                            rsx! {
                                div {
                                    style: "padding: 16px;",
                                    div { "手风琴模式下，同一时间只能展开一个面板。" }
                                    div { "当您展开一个新面板时，之前展开的面板会自动折叠。" }
                                }
                            },
                        ),
                        CollapsePanel::new(
                            "2",
                            rsx! { "💡 使用场景" },
                            rsx! {
                                div {
                                    style: "padding: 16px;",
                                    div { style: "font-weight: 500; margin-bottom: 8px;", "适用场景：" }
                                    div { "• FAQ 常见问题列表" }
                                    div { "• 产品功能介绍" }
                                    div { "• 设置项分类" }
                                }
                            },
                        ),
                        CollapsePanel::new(
                            "3",
                            rsx! { "⚙️ 如何启用？" },
                            rsx! {
                                div {
                                    style: "padding: 16px;",
                                    div { "只需设置 accordion=true 属性即可。" }
                                }
                            },
                        ),
                    ],
                    accordion: true,
                    default_active_key: vec!["1".to_string()],
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 无边框和幽灵模式
            DemoSection {
                title: "无边框和幽灵模式",
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "默认样式（有边框）："
                        }
                        Collapse {
                            items: vec![
                                CollapsePanel::new(
                                    "d1",
                                    rsx! { "默认样式（有边框）" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "默认样式带有边框和背景色，适合独立使用。" }
                                        }
                                    },
                                ),
                                CollapsePanel::new(
                                    "d2",
                                    rsx! { "第二个面板" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "这是默认样式的第二个面板。" }
                                        }
                                    },
                                ),
                            ],
                            default_active_key: vec!["d1".to_string()],
                        }
                    }
                    div {
                        style: "padding: 16px; background: var(--adui-color-fill-quaternary); border-radius: 8px;",
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "无边框模式（bordered=false）："
                        }
                        Collapse {
                            items: vec![
                                CollapsePanel::new(
                                    "b1",
                                    rsx! { "无边框模式" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "无边框模式去掉了外层边框，但保留面板背景。" }
                                        }
                                    },
                                ),
                                CollapsePanel::new(
                                    "b2",
                                    rsx! { "第二个面板" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "这是无边框样式的第二个面板。" }
                                        }
                                    },
                                ),
                            ],
                            bordered: false,
                            default_active_key: vec!["b1".to_string()],
                        }
                    }
                    div {
                        style: "padding: 16px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px;",
                        span {
                            style: "font-size: 14px; color: white; margin-bottom: 8px; display: block;",
                            "幽灵模式（ghost=true）- 渐变背景下完全透明："
                        }
                        Collapse {
                            items: vec![
                                CollapsePanel::new(
                                    "g1",
                                    rsx! { span { style: "color: white;", "幽灵模式（透明）" } },
                                    rsx! {
                                        div {
                                            style: "padding: 16px; color: white;",
                                            div { "幽灵模式完全透明，无边框无背景。" }
                                        }
                                    },
                                ),
                                CollapsePanel::new(
                                    "g2",
                                    rsx! { span { style: "color: white;", "第二个面板" } },
                                    rsx! {
                                        div {
                                            style: "padding: 16px; color: white;",
                                            div { "这是幽灵模式的第二个面板，完全透明。" }
                                        }
                                    },
                                ),
                            ],
                            ghost: true,
                            default_active_key: vec!["g1".to_string()],
                        }
                    }
                }
            }

            // 自定义图标位置
            DemoSection {
                title: "自定义图标位置",
                div {
                    style: "display: flex; gap: 24px;",
                    div {
                        style: "flex: 1;",
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "← 图标在左侧（默认）"
                        }
                        Collapse {
                            items: vec![
                                CollapsePanel::new(
                                    "s1",
                                    rsx! { "图标在开始位置（默认）" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "展开图标默认在标题左侧（起始位置）。" }
                                        }
                                    },
                                ),
                                CollapsePanel::new(
                                    "s2",
                                    rsx! { "第二个面板" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "所有面板都使用相同的图标位置。" }
                                        }
                                    },
                                ),
                            ],
                            expand_icon_placement: ExpandIconPlacement::Start,
                            default_active_key: vec!["s1".to_string()],
                        }
                    }
                    div {
                        style: "flex: 1;",
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "图标在右侧 →"
                        }
                        Collapse {
                            items: vec![
                                CollapsePanel::new(
                                    "e1",
                                    rsx! { "图标在结束位置（右侧）" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "展开图标放在标题右侧（结束位置）。" }
                                        }
                                    },
                                ),
                                CollapsePanel::new(
                                    "e2",
                                    rsx! { "第二个面板" },
                                    rsx! {
                                        div {
                                            style: "padding: 16px;",
                                            div { "图标在右侧，更加优雅。" }
                                        }
                                    },
                                ),
                            ],
                            expand_icon_placement: ExpandIconPlacement::End,
                            default_active_key: vec!["e1".to_string()],
                        }
                    }
                }
            }

            // 嵌套面板
            DemoSection {
                title: "嵌套面板",
                Collapse {
                    items: vec![
                        CollapsePanel::new(
                            "outer1",
                            rsx! { "📂 父面板 1 - 包含嵌套面板" },
                            rsx! {
                                div {
                                    style: "padding: 16px;",
                                    div {
                                        style: "margin-bottom: 12px;",
                                        "这是外层面板的内容。"
                                    }
                                    Collapse {
                                        items: vec![
                                            CollapsePanel::new(
                                                "inner1-1",
                                                rsx! { "子面板 1-1" },
                                                rsx! {
                                                    div {
                                                        style: "padding: 16px;",
                                                        div { "这是第一层嵌套的内容。" }
                                                    }
                                                },
                                            ),
                                            CollapsePanel::new(
                                                "inner1-2",
                                                rsx! { "子面板 1-2" },
                                                rsx! {
                                                    div {
                                                        style: "padding: 16px;",
                                                        div { "嵌套的第二个子面板。" }
                                                    }
                                                },
                                            ),
                                        ],
                                        bordered: false,
                                        default_active_key: vec!["inner1-1".to_string()],
                                    }
                                }
                            },
                        ),
                        CollapsePanel::new(
                            "outer2",
                            rsx! { "📄 父面板 2 - 普通内容" },
                            rsx! {
                                div {
                                    style: "padding: 16px;",
                                    div { "这个父面板不包含嵌套，只有普通内容。" }
                                }
                            },
                        ),
                    ],
                    default_active_key: vec!["outer1".to_string()],
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
