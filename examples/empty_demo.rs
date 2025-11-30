//! Empty 组件演示
//!
//! 展示 Empty 组件的基础用法和高级用法，包括：
//! - 基础空状态
//! - 自定义描述
//! - 带操作按钮
//! - 自定义图片

use adui_dioxus::{
    Button, ButtonType, Empty, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            EmptyDemo {}
        }
    }
}

#[component]
fn EmptyDemo() -> Element {
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

            // 基础空状态
            DemoSection {
                title: "基础空状态",
                Empty {}
            }

            // 自定义描述
            DemoSection {
                title: "自定义描述",
                Empty {
                    description: Some("当前没有任何记录".to_string()),
                }
            }

            // 带操作按钮
            DemoSection {
                title: "带操作按钮",
                Empty {
                    description: Some("还没有创建任何项目".to_string()),
                    footer: Some(rsx! {
                        Button {
                            r#type: ButtonType::Primary,
                            "创建第一个项目"
                        }
                    }),
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 多个操作按钮
            DemoSection {
                title: "多个操作按钮",
                Empty {
                    description: Some("暂无数据，请先创建内容".to_string()),
                    footer: Some(rsx! {
                        div {
                            style: "display: flex; gap: 8px; justify-content: center;",
                            Button {
                                r#type: ButtonType::Primary,
                                "创建内容"
                            }
                            Button {
                                r#type: ButtonType::Default,
                                "导入数据"
                            }
                        }
                    }),
                }
            }

            // 自定义图片
            DemoSection {
                title: "自定义图片",
                Empty {
                    description: Some("自定义空状态图片".to_string()),
                    image: Some(adui_dioxus::EmptyImage::Custom(
                        "https://via.placeholder.com/200x200?text=Empty".to_string()
                    )),
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
