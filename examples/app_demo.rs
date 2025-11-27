use adui_dioxus::{App, ComponentSize, ConfigProvider, use_message, use_modal, use_notification};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        // For this demo we wrap the whole tree in a ConfigProvider so
        // components can pick up global size/disabled if needed.
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            disabled: Some(false),
            App {
                class: Some("demo-app-root".into()),
                DemoShell {}
            }
        }
    }
}

#[component]
fn DemoShell() -> Element {
    let msg = use_message();
    let notice = use_notification();
    let modal = use_modal();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "App & use_* API demo" }
            p { "本示例展示如何在 App 子树中使用 `use_message` / `use_notification` / `use_modal` 获取全局反馈 API。" }

            div { style: "display: flex; flex-direction: column; gap: 8px; max-width: 360px;",
                {
                    let msg_api = msg.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(api) = msg_api.clone() {
                                api.info("来自 App 的 message");
                            }
                        },
                        "调用 message.info()",
                    })
                }
                {
                    let notice_api = notice.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(api) = notice_api.clone() {
                                let _ = api.info("示例通知", Some("来自 App 的 notification".into()));
                            }
                        },
                        "调用 notification.info()",
                    })
                }
                {
                    let modal_api = modal.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(api) = modal_api.clone() {
                                api.open();
                            }
                        },
                        "调用 modal.open()",
                    })
                }
                p {
                    "当前阶段 Message/Notification/Modal 仅完成 API 与 Overlay 管理接线，"
                    "视觉层与更完整的行为将在后续 E/F 小节中补充。"
                }
            }
        }
    }
}
