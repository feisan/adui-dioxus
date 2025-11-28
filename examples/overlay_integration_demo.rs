use adui_dioxus::components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule};
use adui_dioxus::{
    App, Button, ButtonHtmlType, ButtonType, ComponentSize, ConfigProvider, Form, FormItem, Input,
    Modal, TextArea, use_form, use_message,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { OverlayIntegrationShell {} }
        }
    }
}

#[component]
fn OverlayIntegrationShell() -> Element {
    let modal_open = use_signal(|| false);
    let form_signal = use_signal(use_form);
    let message_api = use_message();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Overlay integration demo" }
            p { "演示 ConfigProvider + App + Form + Modal + Message 的组合使用。" }

            Button {
                r#type: ButtonType::Primary,
                onclick: {
                    let mut modal_open = modal_open;
                    move |_| modal_open.set(true)
                },
                "在 Modal 中填写表单",
            }

            Modal {
                open: *modal_open.read(),
                title: Some("创建条目".into()),
                on_cancel: {
                    let mut modal_open = modal_open;
                    move |_| modal_open.set(false)
                },
                destroy_on_close: true,
                footer: None,
                children: rsx! {
                    Form {
                        form: Some(form_signal.read().clone()),
                        on_finish: {
                            let mut modal_open = modal_open;
                            let msg = message_api.clone();
                            move |_evt: FormFinishEvent| {
                                if let Some(api) = msg.clone() {
                                    let _ = api.success("提交成功");
                                }
                                modal_open.set(false);
                                // 简单起见，提交成功后清空表单。
                                let handle = form_signal.read().clone();
                                handle.reset_fields();
                            }
                        },
                        on_finish_failed: {
                            let msg = message_api.clone();
                            move |_evt: FormFinishFailedEvent| {
                                if let Some(api) = msg.clone() {
                                    let _ = api.error("请检查必填项");
                                }
                            }
                        },

                        FormItem {
                            name: Some("title".into()),
                            label: Some("标题".into()),
                            rules: Some(vec![FormRule {
                                required: true,
                                message: Some("请输入标题".into()),
                                ..FormRule::default()
                            }]),
                            Input { placeholder: Some("请输入标题".into()) }
                        }

                        FormItem {
                            name: Some("description".into()),
                            label: Some("描述".into()),
                            TextArea {
                                rows: Some(3),
                                placeholder: Some("请输入描述".into()),
                            }
                        }

                        div {
                            style: "margin-top: 12px; display: flex; justify-content: flex-end; gap: 8px;",
                            Button {
                                r#type: ButtonType::Default,
                                onclick: {
                                    let mut modal_open = modal_open;
                                    move |_| modal_open.set(false)
                                },
                                "取消",
                            }
                            Button {
                                r#type: ButtonType::Primary,
                                html_type: ButtonHtmlType::Submit,
                                "提交",
                            }
                        }
                    }
                },
            }
        }
    }
}
