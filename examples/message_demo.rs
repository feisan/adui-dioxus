use adui_dioxus::{App, ComponentSize, ConfigProvider, use_message};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { DemoMessageDemo {} }
        }
    }
}

#[component]
fn DemoMessageDemo() -> Element {
    let api = use_message();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Message demo" }
            p { "展示不同类型的全局 message，使用 App + use_message API。" }
            div { style: "display: flex; flex-wrap: wrap; gap: 8px;",
                {
                    let api_info = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(msg) = api_info.clone() {
                                msg.info("Info message");
                            }
                        },
                        "info",
                    })
                }
                {
                    let api_success = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(msg) = api_success.clone() {
                                msg.success("操作成功");
                            }
                        },
                        "success",
                    })
                }
                {
                    let api_warning = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(msg) = api_warning.clone() {
                                msg.warning("警告提示");
                            }
                        },
                        "warning",
                    })
                }
                {
                    let api_error = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(msg) = api_error.clone() {
                                msg.error("错误信息");
                            }
                        },
                        "error",
                    })
                }
                {
                    let api_loading = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(msg) = api_loading.clone() {
                                msg.loading("加载中……");
                            }
                        },
                        "loading (不自动关闭)",
                    })
                }
                {
                    let api_destroy = api.clone();
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(msg) = api_destroy.clone() {
                                msg.destroy(None);
                            }
                        },
                        "destroy all",
                    })
                }
            }
        }
    }
}
