//! App 组件演示
//!
//! 展示 App 组件的基础用法和高级用法，包括：
//! - use_message API
//! - use_notification API
//! - use_modal API
//! - 全局反馈API使用

use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_message, use_modal, use_notification, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ConfigProvider {
                size: Some(ComponentSize::Middle),
                disabled: Some(false),
                App {
                    class: Some("demo-app-root".into()),
                    AppDemo {}
                }
            }
        }
    }
}

#[component]
fn AppDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let msg = use_message();
    let notice = use_notification();
    let modal = use_modal();

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

            // use_message API
            DemoSection {
                title: "use_message API",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; max-width: 360px;",
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = msg.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    api.info("来自 App 的 message");
                                }
                            }
                        },
                        "调用 message.info()"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = msg.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    api.success("操作成功");
                                }
                            }
                        },
                        "调用 message.success()"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = msg.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    api.warning("警告提示");
                                }
                            }
                        },
                        "调用 message.warning()"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = msg.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    api.error("错误信息");
                                }
                            }
                        },
                        "调用 message.error()"
                    }
                }
            }

            // use_notification API
            DemoSection {
                title: "use_notification API",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; max-width: 360px;",
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = notice.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    let _ = api.info("信息", Some("来自 App 的 notification".into()));
                                }
                            }
                        },
                        "调用 notification.info()"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = notice.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    let _ = api.success("成功", Some("操作成功的通知".into()));
                                }
                            }
                        },
                        "调用 notification.success()"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = notice.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    let _ = api.warning("警告", Some("需要注意的事项".into()));
                                }
                            }
                        },
                        "调用 notification.warning()"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = notice.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    let _ = api.error("错误", Some("发生了一些错误".into()));
                                }
                            }
                        },
                        "调用 notification.error()"
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // use_modal API
            DemoSection {
                title: "use_modal API",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; max-width: 360px;",
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let api = modal.clone();
                            move |_| {
                                if let Some(api) = api.clone() {
                                    api.open();
                                }
                            }
                        },
                        "调用 modal.open()"
                    }
                    p {
                        style: "margin-top: 8px; color: var(--adui-color-text-secondary); font-size: 12px;",
                        "当前阶段 Message/Notification/Modal 仅完成 API 与 Overlay 管理接线，"
                        "视觉层与更完整的行为将在后续补充。"
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
