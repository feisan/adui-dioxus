use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Form, FormItem, FormList, Input, ThemeProvider,
    components::form::{
        FormFinishEvent, FormFinishFailedEvent, FormRule, form_list_get, form_list_set,
    },
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
            FormListDemo {}
        }
    }
}

/// 简单示例：使用 FormList 管理一个动态邮箱列表字段。
#[component]
fn FormListDemo() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        div {
            style: "padding:24px;max-width:640px;margin:0 auto;display:flex;flex-direction:column;gap:16px;",
            h2 { "FormList 动态字段列表示例" }
            p {
                style: "color:#666;font-size:13px;",
                "此示例演示如何在 Form 中使用 FormList 管理一组可增删的邮箱地址（底层存储为 Value::Array）。"
            }

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

                // 用一个普通字段做对比
                FormItem {
                    name: Some("owner".into()),
                    label: Some("联系人".into()),
                    rules: Some(vec![FormRule {
                        required: true,
                        message: Some("请输入联系人".into()),
                        ..FormRule::default()
                    }]),
                    Input { placeholder: Some("请输入联系人姓名".into()) }
                }

                // 动态邮箱列表：emails 字段存储为 Value::Array
                FormList {
                    name: "emails", // 字段名使用简单字符串键
                    initial_count: Some(1),
                    EmailList {}
                }

                // 提交与重置按钮
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

#[component]
fn EmailList() -> Element {
    use adui_dioxus::use_form_list;

    let list_ctx = use_form_list().expect("EmailList must be used inside FormList");

    let length = list_ctx.len();

    rsx! {
        div {
            style: "display:flex;flex-direction:column;gap:8px;",
            for index in 0..length {
                EmailRow { index }
            }
            Button {
                r#type: ButtonType::Default,
                onclick: move |_| {
                    // 在列表末尾追加一个空元素。
                    let len = list_ctx.len();
                    list_ctx.insert(len, Value::Null);
                },
                "新增邮箱"
            }
        }
    }
}

#[component]
fn EmailRow(index: usize) -> Element {
    use adui_dioxus::use_form_list;

    let list_ctx = use_form_list().expect("EmailRow must be used inside FormList");
    let list_for_remove = list_ctx.clone();

    // 读取当前列表值，获取该索引对应的字符串（若不存在则为空）。
    let items = form_list_get(&list_ctx.handle, &list_ctx.name);
    let current = items
        .get(index)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    rsx! {
        div {
            style: "display:flex;gap:8px;align-items:center;",
            Input {
                value: Some(current),
                placeholder: Some("请输入邮箱地址".into()),
                on_change: move |next: String| {
                    let mut items = form_list_get(&list_ctx.handle, &list_ctx.name);
                    if index >= items.len() {
                        items.resize(index + 1, Value::Null);
                    }
                    items[index] = Value::String(next);
                    form_list_set(&list_ctx.handle, &list_ctx.name, items);
                }
            }
            Button {
                r#type: ButtonType::Text,
                onclick: move |_| {
                    if list_for_remove.len() > 1 {
                        list_for_remove.remove(index);
                    }
                },
                "删除"
            }
        }
    }
}
