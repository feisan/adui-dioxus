use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Form, FormItem, Input, TextArea, ThemeProvider,
    components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule},
    use_form,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            FormDemo {}
        }
    }
}

#[component]
fn FormDemo() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        div { style: "padding: 16px; display: flex; flex-direction: column; gap: 16px; max-width: 320px;",
            Form {
                form: Some(form_handle.read().clone()),
                on_finish: {
                    let mut submit_message = submit_message.clone();
                    move |evt: FormFinishEvent| {
                        submit_message.set(format!("提交成功: {:?}", evt.values));
                    }
                },
                on_finish_failed: {
                    let mut submit_message = submit_message.clone();
                    move |evt: FormFinishFailedEvent| {
                        submit_message.set(format!("提交失败: {:?}", evt.errors));
                    }
                },
                FormItem {
                    name: Some("username".into()),
                    label: Some("用户名".into()),
                    rules: Some(vec![FormRule {
                        required: true,
                        message: Some("请输入用户名".into()),
                        ..FormRule::default()
                    }]),
                    Input {
                        placeholder: Some("请输入用户名".into()),
                    }
                }
                FormItem {
                    name: Some("email".into()),
                    label: Some("邮箱".into()),
                    tooltip: Some("用于接收通知".into()),
                    Input {
                        placeholder: Some("请输入邮箱".into()),
                    }
                }
                FormItem {
                    name: Some("bio".into()),
                    label: Some("个人简介".into()),
                    TextArea {
                        rows: Some(4),
                        placeholder: Some("简单介绍一下自己".into()),
                    }
                }
                Button {
                    r#type: ButtonType::Primary,
                    html_type: ButtonHtmlType::Submit,
                    "提交"
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: {
                        let form_handle = form_handle.clone();
                        let mut submit_message = submit_message.clone();
                        move |_| {
                            form_handle.read().reset_fields();
                            submit_message.set("尚未提交".to_string());
                        }
                    },
                    "重置"
                }
            }
            div {
                strong { "提交结果：" }
                pre { style: "background:#f5f5f5;border:1px solid #ddd;padding:8px;", "{submit_message.read()}" }
            }
        }
    }
}
