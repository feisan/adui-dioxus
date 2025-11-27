use adui_dioxus::{App, ComponentSize, ConfigProvider, NotificationPlacement, use_notification};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { DemoNotificationDemo {} }
        }
    }
}

#[component]
fn DemoNotificationDemo() -> Element {
    let api = use_notification();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Notification demo" }
            p { "展示不同类型和位置的全局 notification。" }
            div { style: "display: flex; flex-wrap: wrap; gap: 8px;",
                {
                    let api_info = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(n) = api_info.clone() {
                                let _ = n.info("信息", Some("这是一条普通通知".into()));
                            }
                        },
                        "info (topRight)",
                    })
                }
                {
                    let api_success = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(n) = api_success.clone() {
                                let _ = n.success("成功", Some("操作成功的通知".into()));
                            }
                        },
                        "success (topRight)",
                    })
                }
                {
                    let api_warning = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(n) = api_warning.clone() {
                                let _ = n.warning("警告", Some("需要注意的事项".into()));
                            }
                        },
                        "warning (topRight)",
                    })
                }
                {
                    let api_error = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(n) = api_error.clone() {
                                let _ = n.error("错误", Some("发生了一些错误".into()));
                            }
                        },
                        "error (topRight)",
                    })
                }
            }
            p { "当前实现默认使用 topRight / bottomRight 两个位置，API 初步与 AntD 保持语义一致。" }
        }
    }
}
