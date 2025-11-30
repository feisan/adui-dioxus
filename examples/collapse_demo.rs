use adui_dioxus::{
    Button, Collapse, CollapsePanel, Divider, ExpandIconPlacement, THEME_BASE_STYLE, Text,
    TextType, Theme, ThemeMode, ThemeProvider, Title as AduiTitle, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {}
        }
    }
}

#[component]
fn App() -> Element {
    let theme = use_theme();
    let mut theme_mode = use_signal(|| ThemeMode::Light);
    let mut primary_color = use_signal(|| "#1890ff".to_string());

    use_effect(move || {
        let mode_val = theme_mode();
        let color = primary_color();
        let mut next = match mode_val {
            ThemeMode::Light => Theme::light(),
            ThemeMode::Dark => Theme::dark(),
            ThemeMode::Custom => Theme::light(),
        };
        next.tokens.color_primary = color.clone();
        next.tokens.color_primary_hover = color.clone();
        next.tokens.color_primary_active = color;
        theme.set_theme(next);
    });

    rsx! {
        style { {THEME_BASE_STYLE} },
        div { style: "min-height: 100vh; padding: 24px; background: var(--adui-color-bg-container);",
            div { style: "max-width: 1200px; margin: 0 auto;",
                // Header controls
                div { style: "margin-bottom: 32px; display: flex; gap: 16px; align-items: center;",
                    Button {
                        onclick: move |_| {
                            theme_mode.set(match theme_mode() {
                                ThemeMode::Light => ThemeMode::Dark,
                                ThemeMode::Dark => ThemeMode::Light,
                                ThemeMode::Custom => ThemeMode::Light,
                            });
                        },
                        "切换主题 ({theme_mode():?})"
                    },
                    select {
                        value: "{primary_color()}",
                        onchange: move |evt| primary_color.set(evt.value()),
                        option { value: "#1890ff", "蓝色 (默认)" },
                        option { value: "#52c41a", "绿色" },
                        option { value: "#f5222d", "红色" },
                        option { value: "#fa8c16", "橙色" },
                    }
                },

                AduiTitle { level: TitleLevel::H2, "Collapse 折叠面板" }
                Divider {}

                // Basic Collapse
                AduiTitle { level: TitleLevel::H3, "基础折叠面板" }
                BasicCollapseDemo {}
                Divider {}

                // Accordion Mode
                AduiTitle { level: TitleLevel::H3, "手风琴模式" }
                AccordionDemo {}
                Divider {}

                // Borderless & Ghost
                AduiTitle { level: TitleLevel::H3, "无边框和幽灵模式" }
                BorderlessGhostDemo {}
                Divider {}

                // Custom Icon Placement
                AduiTitle { level: TitleLevel::H3, "自定义图标位置" }
                IconPlacementDemo {}
                Divider {}

                // Nested Panels
                AduiTitle { level: TitleLevel::H3, "嵌套面板" }
                NestedCollapseDemo {}
            }
        }
    }
}

#[component]
fn BasicCollapseDemo() -> Element {
    let panels = vec![
        CollapsePanel::new(
            "1",
            rsx! {
                span { style: "font-weight: 500;", "📋 产品信息" }
            },
            rsx! {
                div { style: "padding: 16px; line-height: 1.8;",
                    div { style: "margin-bottom: 8px;",
                        Text { style: "font-weight: 500;", "名称：" }
                        Text { "Ant Design of Dioxus" }
                    }
                    div { style: "margin-bottom: 8px;",
                        Text { style: "font-weight: 500;", "版本：" }
                        Text { r#type: TextType::Secondary, "v0.1.0" }
                    }
                    div {
                        Text { style: "font-weight: 500;", "描述：" }
                        Text { r#type: TextType::Secondary, "基于 Dioxus 的 Ant Design 风格组件库" }
                    }
                }
            },
        ),
        CollapsePanel::new(
            "2",
            rsx! {
                span { style: "font-weight: 500;", "⚙️ 技术栈" }
            },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "• Dioxus - 现代化的 Rust UI 框架" }
                    Text { "• Rust - 安全高性能的系统编程语言" }
                    Text { "• WebAssembly - 浏览器中的原生性能" }
                    Text { "• Ant Design - 企业级设计语言" }
                }
            },
        ),
        CollapsePanel::new(
            "3",
            rsx! {
                span { style: "font-weight: 500; color: #999;", "🚫 功能特性（禁用状态）" }
            },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "这个面板被禁用了，无法展开。" }
                    Text { r#type: TextType::Secondary, "可以用于展示暂时不可用的功能。" }
                }
            },
        )
        .disabled(true),
    ];

    rsx! {
        div {
            Text { r#type: TextType::Secondary, style: "margin-bottom: 8px; display: block;",
                "基础用法：可同时展开多个面板，第一个默认展开"
            }
            Collapse {
                items: panels,
                default_active_key: vec!["1".to_string()],
            }
        }
    }
}

#[component]
fn AccordionDemo() -> Element {
    let panels = vec![
        CollapsePanel::new(
            "1",
            rsx! { "🎯 什么是手风琴模式？" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "手风琴模式下，同一时间只能展开一个面板。" }
                    Text { r#type: TextType::Secondary, "当您展开一个新面板时，之前展开的面板会自动折叠。" }
                    Text { r#type: TextType::Secondary, "试试点击其他面板查看效果！" }
                }
            },
        ),
        CollapsePanel::new(
            "2",
            rsx! { "💡 使用场景" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { style: "font-weight: 500; margin-bottom: 8px;", "适用场景：" }
                    Text { "• FAQ 常见问题列表" }
                    Text { "• 产品功能介绍" }
                    Text { "• 设置项分类" }
                    Text { "• 表单分步填写" }
                }
            },
        ),
        CollapsePanel::new(
            "3",
            rsx! { "⚙️ 如何启用？" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "只需设置 accordion=true 属性：" }
                    div { style: "background: #f5f5f5; padding: 12px; border-radius: 4px; margin-top: 8px; font-family: monospace;",
                        "Collapse {{"
                        br {}
                        "    accordion: true,"
                        br {}
                        "    items: panels,"
                        br {}
                        "}}"
                    }
                }
            },
        ),
    ];

    rsx! {
        div {
            Text { r#type: TextType::Secondary, style: "margin-bottom: 8px; display: block;",
                "手风琴模式：一次只展开一个，点击试试"
            }
            Collapse {
                items: panels,
                accordion: true,
                default_active_key: vec!["1".to_string()],
            }
        }
    }
}

#[component]
fn BorderlessGhostDemo() -> Element {
    let default_panels = vec![
        CollapsePanel::new(
            "d1",
            rsx! { "默认样式（有边框）" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "默认样式带有边框和背景色，适合独立使用。" }
                    Text { r#type: TextType::Secondary, "• 有外边框" }
                    Text { r#type: TextType::Secondary, "• 有背景色" }
                    Text { r#type: TextType::Secondary, "• 面板之间有分隔线" }
                }
            },
        ),
        CollapsePanel::new(
            "d2",
            rsx! { "第二个面板" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "这是默认样式的第二个面板。" }
                }
            },
        ),
    ];

    let borderless_panels = vec![
        CollapsePanel::new(
            "b1",
            rsx! { "无边框模式" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "无边框模式去掉了外层边框，但保留面板背景。" }
                    Text { r#type: TextType::Secondary, "• 无外边框" }
                    Text { r#type: TextType::Secondary, "• 保留背景色" }
                    Text { r#type: TextType::Secondary, "• 适合嵌入卡片等容器" }
                }
            },
        ),
        CollapsePanel::new(
            "b2",
            rsx! { "第二个面板" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "这是无边框样式的第二个面板。" }
                }
            },
        ),
    ];

    let ghost_panels = vec![
        CollapsePanel::new(
            "g1",
            rsx! { "幽灵模式（透明）" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "幽灵模式完全透明，无边框无背景。" }
                    Text { r#type: TextType::Secondary, "• 无外边框" }
                    Text { r#type: TextType::Secondary, "• 无背景色（完全透明）" }
                    Text { r#type: TextType::Secondary, "• 最轻量的视觉效果" }
                    Text { r#type: TextType::Secondary, "• 适合需要融入页面背景的场景" }
                }
            },
        ),
        CollapsePanel::new(
            "g2",
            rsx! { "第二个面板" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "这是幽灵模式的第二个面板，完全透明。" }
                }
            },
        ),
    ];

    rsx! {
        // 用带背景色的容器包裹，让透明效果更明显
        div { style: "display: flex; flex-direction: column; gap: 24px;",
            // 默认样式
            div {
                Text { r#type: TextType::Secondary, "默认样式（bordered=true, default）：" }
                Collapse {
                    items: default_panels,
                    default_active_key: vec!["d1".to_string()],
                }
            },

            // 无边框模式 - 用浅色背景突出显示
            div { style: "padding: 16px; background: var(--adui-color-fill-quaternary); border-radius: 8px;",
                Text { r#type: TextType::Secondary, "无边框模式（bordered=false）：" }
                div { style: "margin-top: 8px;",
                    Collapse {
                        items: borderless_panels,
                        bordered: false,
                        default_active_key: vec!["b1".to_string()],
                    }
                }
            },

            // 幽灵模式 - 用彩色背景突出透明效果
            div { style: "padding: 16px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px;",
                Text { r#type: TextType::Secondary, style: "color: white;", "幽灵模式（ghost=true）- 渐变背景下完全透明：" }
                div { style: "margin-top: 8px;",
                    Collapse {
                        items: ghost_panels,
                        ghost: true,
                        default_active_key: vec!["g1".to_string()],
                    }
                }
            }
        }
    }
}

#[component]
fn IconPlacementDemo() -> Element {
    let start_panels = vec![
        CollapsePanel::new(
            "s1",
            rsx! { "图标在开始位置（默认）" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "展开图标默认在标题左侧（起始位置）。" }
                    Text { r#type: TextType::Secondary, "这是传统的展开样式，符合用户习惯。" }
                }
            },
        ),
        CollapsePanel::new(
            "s2",
            rsx! { "第二个面板" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "所有面板都使用相同的图标位置。" }
                }
            },
        ),
    ];

    let end_panels = vec![
        CollapsePanel::new(
            "e1",
            rsx! { "图标在结束位置（右侧）" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "展开图标放在标题右侧（结束位置）。" }
                    Text { r#type: TextType::Secondary, "适合需要突出标题文字的场景。" }
                    Text { r#type: TextType::Secondary, "类似于移动端常见的展开样式。" }
                }
            },
        ),
        CollapsePanel::new(
            "e2",
            rsx! { "第二个面板" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "图标在右侧，更加优雅。" }
                }
            },
        ),
    ];

    rsx! {
        div { style: "display: flex; gap: 24px;",
            div { style: "flex: 1;",
                Text { r#type: TextType::Secondary, style: "margin-bottom: 8px; display: block;",
                    "← 图标在左侧（默认）"
                }
                Collapse {
                    items: start_panels,
                    expand_icon_placement: ExpandIconPlacement::Start,
                    default_active_key: vec!["s1".to_string()],
                }
            },
            div { style: "flex: 1;",
                Text { r#type: TextType::Secondary, style: "margin-bottom: 8px; display: block;",
                    "图标在右侧 →"
                }
                Collapse {
                    items: end_panels,
                    expand_icon_placement: ExpandIconPlacement::End,
                    default_active_key: vec!["e1".to_string()],
                }
            }
        }
    }
}

#[component]
fn NestedCollapseDemo() -> Element {
    let inner_panels1 = vec![
        CollapsePanel::new(
            "inner1-1",
            rsx! { "子面板 1-1" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "这是第一层嵌套的内容。" }
                    Text { r#type: TextType::Secondary, "可以嵌套任意层级的 Collapse。" }
                }
            },
        ),
        CollapsePanel::new(
            "inner1-2",
            rsx! { "子面板 1-2" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "嵌套的第二个子面板。" }
                }
            },
        ),
    ];

    let inner_panels2 = vec![CollapsePanel::new(
        "inner2-1",
        rsx! { "子面板 2-1" },
        rsx! {
            div { style: "padding: 16px;",
                Text { "这个父面板也包含嵌套的 Collapse。" }
            }
        },
    )];

    let outer_panels = vec![
        CollapsePanel::new(
            "outer1",
            rsx! { "📂 父面板 1 - 包含嵌套面板" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { style: "margin-bottom: 12px; display: block;", "这是外层面板的内容。" }
                    Text { r#type: TextType::Secondary, style: "margin-bottom: 12px; display: block;",
                        "下面是嵌套的 Collapse 组件（使用无边框样式）："
                    }
                    Collapse {
                        items: inner_panels1,
                        bordered: false,
                        default_active_key: vec!["inner1-1".to_string()],
                    }
                }
            },
        ),
        CollapsePanel::new(
            "outer2",
            rsx! { "📂 父面板 2 - 也有嵌套" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { style: "margin-bottom: 12px; display: block;", "另一个包含嵌套的父面板。" }
                    Collapse {
                        items: inner_panels2,
                        bordered: false,
                    }
                }
            },
        ),
        CollapsePanel::new(
            "outer3",
            rsx! { "📄 父面板 3 - 普通内容" },
            rsx! {
                div { style: "padding: 16px;",
                    Text { "这个父面板不包含嵌套，只有普通内容。" }
                    Text { r#type: TextType::Secondary, "嵌套使用场景：" }
                    Text { r#type: TextType::Secondary, "• 多级分类展示" }
                    Text { r#type: TextType::Secondary, "• 复杂的设置项分组" }
                    Text { r#type: TextType::Secondary, "• 层级化的内容组织" }
                }
            },
        ),
    ];

    rsx! {
        div {
            Text { r#type: TextType::Secondary, style: "margin-bottom: 8px; display: block;",
                "嵌套用法：Collapse 内可以再嵌套 Collapse，建议内层使用无边框样式"
            }
            Collapse {
                items: outer_panels,
                default_active_key: vec!["outer1".to_string()],
            }
        }
    }
}
