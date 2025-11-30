//! Alert 组件演示
//!
//! 展示 Alert 组件的基础用法和高级用法，包括：
//! - 不同类型（成功、信息、警告、错误）
//! - 可关闭
//! - 带描述
//! - 横幅模式

use adui_dioxus::{
    Alert, AlertType, Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            AlertDemo {}
        }
    }
}

#[component]
fn AlertDemo() -> Element {
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

            // 不同类型
            DemoSection {
                title: "不同类型",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Alert {
                        r#type: AlertType::Success,
                        message: rsx!("成功提示信息"),
                    }
                    Alert {
                        r#type: AlertType::Info,
                        message: rsx!("信息提示信息"),
                    }
                    Alert {
                        r#type: AlertType::Warning,
                        message: rsx!("警告提示信息"),
                    }
                    Alert {
                        r#type: AlertType::Error,
                        message: rsx!("错误提示信息"),
                    }
                }
            }

            // 可关闭
            DemoSection {
                title: "可关闭",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Alert {
                        r#type: AlertType::Info,
                        message: rsx!("可关闭的提示信息"),
                        closable: true,
                        on_close: move |_| {
                            println!("Alert closed");
                        },
                    }
                    Alert {
                        r#type: AlertType::Warning,
                        message: rsx!("点击关闭按钮可以关闭此提示"),
                        closable: true,
                        on_close: move |_| {
                            println!("Warning alert closed");
                        },
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 带描述
            DemoSection {
                title: "带描述",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Alert {
                        r#type: AlertType::Info,
                        message: rsx!("提示标题"),
                        description: Some(rsx!("这是提示的详细描述信息，可以包含更多的内容说明。")),
                    }
                    Alert {
                        r#type: AlertType::Success,
                        message: rsx!("操作成功"),
                        description: Some(rsx!("您的操作已成功完成，可以继续下一步操作。")),
                        closable: true,
                    }
                }
            }

            // 无图标
            DemoSection {
                title: "无图标",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Alert {
                        r#type: AlertType::Info,
                        message: rsx!("不显示图标的提示信息"),
                        show_icon: false,
                    }
                    Alert {
                        r#type: AlertType::Warning,
                        message: rsx!("无图标的警告信息"),
                        show_icon: false,
                        closable: true,
                    }
                }
            }

            // 横幅模式
            DemoSection {
                title: "横幅模式",
                Alert {
                    r#type: AlertType::Info,
                    message: rsx!("这是横幅模式的提示信息"),
                    banner: true,
                    closable: true,
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Alert {
                        r#type: AlertType::Success,
                        message: rsx!("操作成功"),
                        description: Some(rsx!("您的数据已成功保存，系统将在3秒后自动刷新。")),
                        closable: true,
                        on_close: move |_| {
                            println!("Success alert closed");
                        },
                    }
                    Alert {
                        r#type: AlertType::Error,
                        message: rsx!("操作失败"),
                        description: Some(rsx!("保存数据时发生错误，请检查网络连接后重试。")),
                        closable: true,
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
