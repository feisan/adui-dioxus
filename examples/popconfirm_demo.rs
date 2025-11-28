use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Popconfirm, use_message,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { PopconfirmDemoShell {} }
        }
    }
}

#[component]
fn PopconfirmDemoShell() -> Element {
    let api = use_message();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Popconfirm demo" }
            p { "展示 Popconfirm 的基础用法：危险操作二次确认，并与 Message 集成反馈结果。" }

            div { style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                Popconfirm {
                    title: "确定要删除这条记录吗？".to_string(),
                    description: Some("删除后无法恢复，请谨慎操作。".to_string()),
                    ok_text: Some("删除".to_string()),
                    cancel_text: Some("取消".to_string()),
                    ok_type: Some(ButtonType::Primary),
                    ok_danger: true,
                    on_confirm: {
                        let api_confirm = api.clone();
                        move |_| {
                            if let Some(msg) = api_confirm.clone() {
                                msg.success("已删除");
                            }
                        }
                    },
                    on_cancel: {
                        let api_cancel = api.clone();
                        move |_| {
                            if let Some(msg) = api_cancel.clone() {
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
