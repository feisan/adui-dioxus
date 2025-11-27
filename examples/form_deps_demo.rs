use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Checkbox, Form, FormItem, Input, ThemeProvider,
    components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule, ValuesChangeEvent},
    use_form,
};
use dioxus::prelude::*;
use serde_json::Value;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            FormDepsDemo {}
        }
    }
}

/// 示例：使用 on_values_change 和 dependencies 实现“使用默认邮箱”联动行为。
#[component]
fn FormDepsDemo() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        div {
            style: "padding:24px;max-width:640px;margin:0 auto;display:flex;flex-direction:column;gap:16px;",
            h2 { "Form 字段联动示例" }
            p {
                style: "color:#666;font-size:13px;",
                "当勾选“使用主邮箱”时，联系邮箱会自动同步为上面的邮箱，且输入框禁用。"
            }

            Form {
                form: Some(form_handle.read().clone()),
                on_values_change: {
                    let form_handle = form_handle.clone();
                    move |evt: ValuesChangeEvent| {
                        let changed_keys: Vec<_> = evt.changed_values.keys().cloned().collect();

                        // 如果 email 或 use_default_contact 发生变化，则根据最新状态调整 contact_email。
                        if changed_keys.iter().any(|k| k == "email" || k == "use_default_contact") {
                            let values = form_handle.read().values();
                            let use_default = values
                                .get("use_default_contact")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            let email = values
                                .get("email")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            if use_default {
                                form_handle
                                    .read()
                                    .set_field_value("contact_email", Value::String(email));
                            }
                        }
                    }
                },
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
                    name: Some("email".into()),
                    label: Some("主邮箱".into()),
                    rules: Some(vec![FormRule {
                        required: true,
                        message: Some("请输入主邮箱".into()),
                        ..FormRule::default()
                    }]),
                    Input {
                        placeholder: Some("例如 name@example.com".into()),
                    }
                }

                FormItem {
                    name: Some("use_default_contact".into()),
                    label: Some("联系邮箱".into()),
                    Checkbox {
                        default_checked: true,
                        "使用主邮箱作为联系邮箱"
                    }
                }

                // contact_email 依赖 email 与 use_default_contact，值由 on_values_change 驱动。
                FormItem {
                    name: Some("contact_email".into()),
                    label: Some("联系邮箱地址".into()),
                    dependencies: Some(vec!["email".into(), "use_default_contact".into()]),
                    Input {
                        placeholder: Some("当未勾选上方复选框时可单独设置".into()),
                    }
                }

                div {
                    style: "display:flex;justify-content:flex-end;gap:8px;margin-top:16px;",
                    Button {
                        r#type: ButtonType::Default,
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
                    Button {
                        r#type: ButtonType::Primary,
                        html_type: ButtonHtmlType::Submit,
                        "提交"
                    }
                }
            }

            div {
                strong { "提交结果：" }
                pre {
                    style: "background:#f5f5f5;border:1px solid #ddd;padding:8px;white-space:pre-wrap;font-size:12px;",
                    "{submit_message.read()}"
                }
            }
        }
    }
}
