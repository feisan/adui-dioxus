use adui_dioxus::{
    Button, ButtonType, Form, FormItem, ThemeProvider,
    components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule},
    use_form, use_form_item_control,
};
use dioxus::prelude::*;
use serde_json::Value;

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
                    FormTextInput {}
                }
                FormItem {
                    name: Some("email".into()),
                    label: Some("邮箱".into()),
                    tooltip: Some("用于接收通知".into()),
                    FormTextInput {}
                }
                button { r#type: "submit", "提交" }
                Button {
                    r#type: ButtonType::Text,
                    onclick: {
                        let form_handle = form_handle.clone();
                        move |_| {
                            form_handle.read().reset_fields();
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

#[component]
fn FormTextInput() -> Element {
    let control = use_form_item_control();
    let text_value = control
        .as_ref()
        .and_then(|ctrl| ctrl.value())
        .and_then(|val| match val {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(if b { "true".into() } else { "false".into() }),
            _ => None,
        })
        .unwrap_or_default();
    let disabled = control
        .as_ref()
        .map(|ctrl| ctrl.is_disabled())
        .unwrap_or(false);
    let control_for_change = control.clone();
    rsx! {
        input {
            value: "{text_value}",
            disabled: disabled,
            oninput: move |evt| {
                if let Some(ctrl) = control_for_change.as_ref() {
                    ctrl.set_string(evt.value());
                }
            },
            class: "adui-input",
        }
    }
}
