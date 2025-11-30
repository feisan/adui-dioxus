//! Result 组件演示
//!
//! 展示 Result 组件的基础用法和高级用法，包括：
//! - 成功状态
//! - 错误状态
//! - 警告状态
//! - 信息状态
//! - 自定义操作按钮

use adui_dioxus::{
    Button, ButtonType, Card, Result, ResultStatus, ThemeMode, ThemeProvider, Title, TitleLevel,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ResultDemo {}
        }
    }
}

#[component]
fn ResultDemo() -> Element {
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

            // 成功状态
            DemoSection {
                title: "成功状态",
                Card {
                    children: rsx! {
                        Result {
                            status: Some(ResultStatus::Success),
                            title: Some(rsx!("提交成功")),
                            sub_title: Some(rsx!("您的操作已成功提交，可以在列表页查看最新状态。")),
                            extra: Some(rsx!(
                                div {
                                    style: "display: inline-flex; gap: 8px;",
                                    Button {
                                        r#type: ButtonType::Primary,
                                        "返回列表"
                                    }
                                    Button {
                                        r#type: ButtonType::Default,
                                        "查看详情"
                                    }
                                }
                            )),
                        }
                    }
                }
            }

            // 错误状态
            DemoSection {
                title: "错误状态",
                Card {
                    children: rsx! {
                        Result {
                            status: Some(ResultStatus::Error),
                            title: Some(rsx!("操作失败")),
                            sub_title: Some(rsx!("请检查您的输入或网络连接，然后重试。")),
                            extra: Some(rsx!(
                                div {
                                    style: "display: inline-flex; gap: 8px;",
                                    Button {
                                        r#type: ButtonType::Primary,
                                        "重新尝试"
                                    }
                                    Button {
                                        r#type: ButtonType::Default,
                                        "返回首页"
                                    }
                                }
                            )),
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 服务器错误
            DemoSection {
                title: "服务器错误",
                Card {
                    children: rsx! {
                        Result {
                            status: Some(ResultStatus::ServerError),
                            title: Some(rsx!("服务器错误")),
                            sub_title: Some(rsx!("请求处理过程中发生了未知错误，请稍后重试，或联系管理员。")),
                            extra: Some(rsx!(
                                div {
                                    style: "display: inline-flex; gap: 8px;",
                                    Button {
                                        r#type: ButtonType::Primary,
                                        "重新尝试"
                                    }
                                    Button {
                                        r#type: ButtonType::Default,
                                        "返回首页"
                                    }
                                }
                            )),
                        }
                    }
                }
            }

            // 警告状态
            DemoSection {
                title: "警告状态",
                Card {
                    children: rsx! {
                        Result {
                            status: Some(ResultStatus::Warning),
                            title: Some(rsx!("警告")),
                            sub_title: Some(rsx!("请注意检查您的操作是否符合要求。")),
                            extra: Some(rsx!(
                                Button {
                                    r#type: ButtonType::Primary,
                                    "我知道了"
                                }
                            )),
                        }
                    }
                }
            }

            // 信息状态
            DemoSection {
                title: "信息状态",
                Card {
                    children: rsx! {
                        Result {
                            status: Some(ResultStatus::Info),
                            title: Some(rsx!("提示")),
                            sub_title: Some(rsx!("这是一条信息提示，请仔细阅读。")),
                            extra: Some(rsx!(
                                Button {
                                    r#type: ButtonType::Primary,
                                    "确定"
                                }
                            )),
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
