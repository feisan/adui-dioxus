//! Message 组件演示
//!
//! 展示 Message 组件的基础用法和高级用法，包括：
//! - 不同类型消息
//! - 自定义持续时间
//! - 手动关闭
//! - 全局API使用

use adui_dioxus::{
    App, Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel, use_message, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {
                MessageDemo {}
            }
        }
    }
}

#[component]
fn MessageDemo() -> Element {
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

            // 不同类型消息
            DemoSection {
                title: "不同类型消息",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = api.clone();
                            move |_| {
                                if let Some(msg) = api.clone() {
                                    msg.info("Info message");
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
                                if let Some(msg) = api.clone() {
                                    msg.success("操作成功");
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
                                if let Some(msg) = api.clone() {
                                    msg.warning("警告提示");
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
                                if let Some(msg) = api.clone() {
                                    msg.error("错误信息");
                                }
                            }
                        },
                        "Error"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = api.clone();
                            move |_| {
                                if let Some(msg) = api.clone() {
                                    msg.loading("加载中……");
                                }
                            }
                        },
                        "Loading"
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 手动关闭
            DemoSection {
                title: "手动关闭",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "点击按钮关闭所有消息："
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = api.clone();
                            move |_| {
                                if let Some(msg) = api.clone() {
                                    msg.destroy(None);
                                }
                            }
                        },
                        "Destroy All"
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
