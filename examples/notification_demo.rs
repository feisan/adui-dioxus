//! Notification 组件演示
//!
//! 展示 Notification 组件的基础用法和高级用法，包括：
//! - 不同类型通知
//! - 不同位置
//! - 自定义持续时间
//! - 全局API使用

use adui_dioxus::{
    App, Button, ButtonType, NotificationPlacement, ThemeMode, ThemeProvider, Title, TitleLevel,
    use_notification, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {
                NotificationDemo {}
            }
        }
    }
}

#[component]
fn NotificationDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let api = use_notification();

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

            // 不同类型通知
            DemoSection {
                title: "不同类型通知",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = api.clone();
                            move |_| {
                                if let Some(n) = api.clone() {
                                    let _ = n.info("信息", Some("这是一条普通通知".into()));
                                }
                            }
                        },
                        "Info"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = api.clone();
                            move |_| {
                                if let Some(n) = api.clone() {
                                    let _ = n.success("成功", Some("操作成功的通知".into()));
                                }
                            }
                        },
                        "Success"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = api.clone();
                            move |_| {
                                if let Some(n) = api.clone() {
                                    let _ = n.warning("警告", Some("需要注意的事项".into()));
                                }
                            }
                        },
                        "Warning"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = api.clone();
                            move |_| {
                                if let Some(n) = api.clone() {
                                    let _ = n.error("错误", Some("发生了一些错误".into()));
                                }
                            }
                        },
                        "Error"
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 不同位置
            DemoSection {
                title: "不同位置",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "当前实现支持 topRight 和 bottomRight 两个位置："
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let api = api.clone();
                                move |_| {
                                    if let Some(n) = api.clone() {
                                        let _ = n.info("Top Right", Some("右上角通知".into()));
                                    }
                                }
                            },
                            "Top Right"
                        }
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let api = api.clone();
                                move |_| {
                                    if let Some(n) = api.clone() {
                                        let _ = n.info("Bottom Right", Some("右下角通知".into()));
                                    }
                                }
                            },
                            "Bottom Right"
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
