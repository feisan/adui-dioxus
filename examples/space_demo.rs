//! Space 组件演示
//!
//! 展示 Space 组件的基础用法和高级用法，包括：
//! - 水平和垂直方向
//! - 间距大小
//! - 分割线
//! - 紧凑模式
//! - 换行

use adui_dioxus::{
    Button, ButtonType, Divider, Space, SpaceDirection, SpaceSize, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SpaceDemo {}
        }
    }
}

#[component]
fn SpaceDemo() -> Element {
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

            // 方向
            DemoSection {
                title: "方向（Direction）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 8px;",
                        "水平方向"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        gap: Some(12.0),
                        Button { r#type: ButtonType::Primary, "Button 1" }
                        Button { r#type: ButtonType::Default, "Button 2" }
                        Button { r#type: ButtonType::Default, "Button 3" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "垂直方向"
                    }
                    Space {
                        direction: SpaceDirection::Vertical,
                        gap: Some(12.0),
                        Button { r#type: ButtonType::Primary, "Button 1" }
                        Button { r#type: ButtonType::Default, "Button 2" }
                        Button { r#type: ButtonType::Default, "Button 3" }
                    }
                }
            }

            // 间距大小
            DemoSection {
                title: "间距大小（Size）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 8px;",
                        "Small"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        size: SpaceSize::Small,
                        Button { r#type: ButtonType::Primary, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "Middle（默认）"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        size: SpaceSize::Middle,
                        Button { r#type: ButtonType::Primary, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "Large"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        size: SpaceSize::Large,
                        Button { r#type: ButtonType::Primary, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "自定义间距 (24px)"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        gap: Some(24.0),
                        Button { r#type: ButtonType::Primary, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                        Button { r#type: ButtonType::Default, "Button" }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 分割线
            DemoSection {
                title: "分割线（Split）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 8px;",
                        "水平分割线"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        gap: Some(12.0),
                        split: Some(rsx!(Divider { vertical: true })),
                        Button { r#type: ButtonType::Primary, "Button 1" }
                        Button { r#type: ButtonType::Default, "Button 2" }
                        Button { r#type: ButtonType::Default, "Button 3" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "自定义分割符"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        gap: Some(12.0),
                        split: Some(rsx!(span { style: "color: var(--adui-color-text-secondary);", "·" })),
                        Button { r#type: ButtonType::Primary, "Button 1" }
                        Button { r#type: ButtonType::Default, "Button 2" }
                        Button { r#type: ButtonType::Default, "Button 3" }
                    }
                }
            }

            // 换行
            DemoSection {
                title: "换行（Wrap）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Space {
                        direction: SpaceDirection::Horizontal,
                        gap: Some(12.0),
                        wrap: Some(true),
                        {
                            (0..8).map(|i| {
                                let label = format!("Button {}", i + 1);
                                rsx! {
                                    Button {
                                        r#type: ButtonType::Default,
                                        {label}
                                    }
                                }
                            })
                        }
                    }
                }
            }

            // 紧凑模式
            DemoSection {
                title: "紧凑模式（Compact）",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 8px;",
                        "普通模式"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        gap: Some(12.0),
                        Button { r#type: ButtonType::Primary, "Button 1" }
                        Button { r#type: ButtonType::Default, "Button 2" }
                        Button { r#type: ButtonType::Default, "Button 3" }
                    }
                    div {
                        style: "font-size: 14px; font-weight: 600; margin: 16px 0 8px 0;",
                        "紧凑模式"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        compact: true,
                        gap: Some(8.0),
                        Button { r#type: ButtonType::Primary, "Button 1" }
                        Button { r#type: ButtonType::Default, "Button 2" }
                        Button { r#type: ButtonType::Default, "Button 3" }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Space {
                        direction: SpaceDirection::Horizontal,
                        size: SpaceSize::Large,
                        split: Some(rsx!(Divider { vertical: true })),
                        wrap: Some(true),
                        Button { r#type: ButtonType::Primary, "确定" }
                        Button { r#type: ButtonType::Default, "取消" }
                        Button { r#type: ButtonType::Text, "更多" }
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
