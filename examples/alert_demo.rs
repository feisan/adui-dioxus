use adui_dioxus::{Alert, AlertType, App, Button, ButtonType, ComponentSize, ConfigProvider};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { AlertDemoShell {} }
        }
    }
}

#[component]
fn AlertDemoShell() -> Element {
    let show_success = use_signal(|| true);

    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Alert demo" }
            p { "基础四种类型和可关闭行为示例。" }

            div { style: "margin-bottom: 16px; display: flex; gap: 8px;",
                Button {
                    r#type: ButtonType::Default,
                    onclick: {
                        let mut show_success = show_success;
                        move |_| show_success.set(true)
                    },
                    "重置成功提示"
                }
            }

            if *show_success.read() {
                Alert {
                    r#type: AlertType::Success,
                    show_icon: true,
                    closable: true,
                    on_close: {
                        let mut show_success = show_success;
                        move |_| show_success.set(false)
                    },
                    message: rsx!("操作成功"),
                    description: Some(rsx!("用于展示表单提交或任务完成后的成功提示。")),
                }
            }

            Alert {
                r#type: AlertType::Info,
                show_icon: true,
                message: rsx!("信息提示"),
                description: Some(rsx!("普通信息提示，可用于说明当前页面状态。")),
            }

            Alert {
                r#type: AlertType::Warning,
                show_icon: true,
                message: rsx!("警告"),
                description: Some(rsx!("用于提醒潜在风险或需要用户留意的设置。")),
            }

            Alert {
                r#type: AlertType::Error,
                show_icon: true,
                message: rsx!("错误"),
                description: Some(rsx!("用于展示严重错误或阻断性问题。")),
                banner: true,
            }
        }
    }
}
