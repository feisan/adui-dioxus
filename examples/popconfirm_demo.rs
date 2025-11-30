//! Popconfirm 组件演示
//!
//! 展示 Popconfirm 组件的基础用法和高级用法，包括：
//! - 基础确认框
//! - 自定义文字
//! - 危险操作确认
//! - 与Message集成

use adui_dioxus::{
    App, Button, ButtonType, Popconfirm, ThemeMode, ThemeProvider, Title, TitleLevel, use_message,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {
                PopconfirmDemo {}
            }
        }
    }
}

#[component]
fn PopconfirmDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let api = use_message();

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

            // 基础确认框
            DemoSection {
                title: "基础确认框",
                Popconfirm {
                    title: "确定要执行此操作吗？".to_string(),
                    on_confirm: {
                        let api = api.clone();
                        move |_| {
                            if let Some(msg) = api.clone() {
                                msg.success("操作已确认");
                            }
                        }
                    },
                    on_cancel: {
                        let api = api.clone();
                        move |_| {
                            if let Some(msg) = api.clone() {
                                msg.info("操作已取消");
                            }
                        }
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "确认操作"
                        }
                    },
                }
            }

            // 自定义文字
            DemoSection {
                title: "自定义文字",
                Popconfirm {
                    title: "确定要删除这条记录吗？".to_string(),
                    description: Some("删除后无法恢复，请谨慎操作。".to_string()),
                    ok_text: Some("删除".to_string()),
                    cancel_text: Some("取消".to_string()),
                    on_confirm: {
                        let api = api.clone();
                        move |_| {
                            if let Some(msg) = api.clone() {
                                msg.success("已删除");
                            }
                        }
                    },
                    on_cancel: {
                        let api = api.clone();
                        move |_| {
                            if let Some(msg) = api.clone() {
                                msg.info("已取消删除");
                            }
                        }
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "删除记录"
                        }
                    },
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 危险操作确认
            DemoSection {
                title: "危险操作确认",
                Popconfirm {
                    title: "确定要删除这条记录吗？".to_string(),
                    description: Some("删除后无法恢复，请谨慎操作。".to_string()),
                    ok_text: Some("删除".to_string()),
                    cancel_text: Some("取消".to_string()),
                    ok_type: Some(ButtonType::Primary),
                    ok_danger: true,
                    on_confirm: {
                        let api = api.clone();
                        move |_| {
                            if let Some(msg) = api.clone() {
                                msg.success("已删除");
                            }
                        }
                    },
                    on_cancel: {
                        let api = api.clone();
                        move |_| {
                            if let Some(msg) = api.clone() {
                                msg.info("已取消删除");
                            }
                        }
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            danger: true,
                            "删除"
                        }
                    },
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
