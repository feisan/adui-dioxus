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

/// 最小可复现的基础表单示例。
/// - 外部通过 `use_form` 持有 FormHandle（与 AntD 的 Form.useForm 对齐）
/// - Form 内部完全依赖 FormStore 作为唯一数据源
/// - 额外渲染当前 values/errors 便于观察 reset + submit 的行为
#[component]
fn FormDemo() -> Element {
    // 与 AntD 类似：外部持有一个可复用的 Form 实例。
    let form_signal = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    // 方便调试：每次渲染时展示当前表单值和错误。
    let form_snapshot = form_signal.read().clone();
    let current_values = format!("{:?}", form_snapshot.values());
    let current_errors = format!("{:?}", form_snapshot.errors());

    rsx! {
        div {
            style: "padding: 16px; display: flex; flex-direction: column; gap: 16px; max-width: 360px;",
            Form {
                form: Some(form_signal.read().clone()),
                on_finish: {
                    let mut submit_message = submit_message;
                    move |evt: FormFinishEvent| {
                        submit_message.set(format!("提交成功: {:?}", evt.values));
                    }
                },
                on_finish_failed: {
                    let mut submit_message = submit_message;
                    move |evt: FormFinishFailedEvent| {
                        submit_message.set(format!("提交失败: {:?}", evt.errors));
                    }
                },

                // 用户名：必填
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

                // 邮箱：非必填，仅做展示
                FormItem {
                    name: Some("email".into()),
                    label: Some("邮箱".into()),
                    tooltip: Some("用于接收通知".into()),
                    Input {
                        placeholder: Some("请输入邮箱".into()),
                    }
                }

                // 个人简介：非必填
                FormItem {
                    name: Some("bio".into()),
                    label: Some("个人简介".into()),
                    TextArea {
                        rows: Some(4),
                        placeholder: Some("简单介绍一下自己".into()),
                    }
                }

                // 提交按钮
                Button {
                    r#type: ButtonType::Primary,
                    html_type: ButtonHtmlType::Submit,
                    "提交"
                }

                // 重置按钮：调用 FormHandle.reset_fields，并重置提交结果提示
                Button {
                    r#type: ButtonType::Text,
                    onclick: {
                        let mut submit_message = submit_message;
                        move |_| {
                            let handle = form_signal.read().clone();
                            handle.reset_fields();
                            submit_message.set("尚未提交".to_string());
                        }
                    },
                    "重置"
                }
            }

            // 提交结果
            div {
                strong { "提交结果：" }
                pre {
                    style: "background:#f5f5f5;border:1px solid #ddd;padding:8px;",
                    "{submit_message.read()}"
                }
            }

            // 当前表单内部状态（仅用于调试）
            div {
                strong { "当前 values：" }
                pre {
                    style: "background:#fafafa;border:1px dashed #ccc;padding:8px;font-size:12px;",
                    "{current_values}"
                }
            }
            div {
                strong { "当前 errors：" }
                pre {
                    style: "background:#fafafa;border:1px dashed #ccc;padding:8px;font-size:12px;",
                    "{current_errors}"
                }
            }
        }
    }
}
