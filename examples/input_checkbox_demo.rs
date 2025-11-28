use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Checkbox, CheckboxGroup, Flex, FlexDirection, FlexGap,
    FlexJustify, Form, FormItem, Input, Radio, RadioGroup, Switch, TextArea, ThemeProvider,
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
            InputCheckboxDemo {}
        }
    }
}

/// Showcase for Input / TextArea / Checkbox / Radio / Switch working together inside Form.
#[component]
fn InputCheckboxDemo() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        div {
            style: "padding:24px;max-width:640px;margin:0 auto;display:flex;flex-direction:column;gap:16px;",
            h2 { "账户设置示例（Input / Checkbox / Radio / Switch）" }
            p {
                style: "color:#666;font-size:13px;",
                "此示例演示：文本输入、单选组、复选组和开关与 Form 的集成与重置行为。"
            }

            Form {
                form: Some(form_handle.read().clone()),
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

                // 基本信息
                FormItem {
                    name: Some("username".into()),
                    label: Some("用户名".into()),
                    rules: Some(vec![
                        FormRule {
                            required: true,
                            min: Some(3),
                            message: Some("用户名至少 3 个字符".into()),
                            ..FormRule::default()
                        },
                    ]),
                    Input {
                        placeholder: Some("请输入用户名".into()),
                    }
                }
                FormItem {
                    name: Some("email".into()),
                    label: Some("邮箱".into()),
                    rules: Some(vec![
                        FormRule {
                            required: true,
                            message: Some("请输入邮箱".into()),
                            ..FormRule::default()
                        },
                        FormRule {
                            pattern: Some(r"^[^@\s]+@[^@\s]+\.[^@\s]+$".into()),
                            message: Some("邮箱格式不正确".into()),
                            ..FormRule::default()
                        },
                    ]),
                    Input {
                        placeholder: Some("例如 name@example.com".into()),
                    }
                }
                FormItem {
                    name: Some("gender".into()),
                    label: Some("性别".into()),
                    rules: Some(vec![FormRule {
                        required: true,
                        message: Some("请选择性别".into()),
                        ..FormRule::default()
                    }]),
                    RadioGroup {
                        default_value: Some("male".into()),
                        Radio { value: "male".to_string(), "男" }
                        Radio { value: "female".to_string(), "女" }
                    }
                }

                // 偏好设置
                FormItem {
                    name: Some("newsletter".into()),
                    label: Some("订阅邮件".into()),
                    Switch {
                        default_checked: true,
                        checked_children: Some(rsx!("开")),
                        un_checked_children: Some(rsx!("关")),
                    }
                }
                FormItem {
                    name: Some("tags".into()),
                    label: Some("技术标签".into()),
                    CheckboxGroup {
                        default_value: vec!["rust".into()],
                        Checkbox { value: Some("rust".into()), "Rust" }
                        Checkbox { value: Some("dioxus".into()), "Dioxus" }
                        Checkbox { value: Some("web".into()), "Web" }
                    }
                }
                FormItem {
                    name: Some("bio".into()),
                    label: Some("个人简介".into()),
                    TextArea {
                        rows: Some(3),
                        placeholder: Some("可以简单介绍一下自己和当前的技术栈".into()),
                    }
                }
                FormItem {
                    name: Some("agree".into()),
                    label: Some("协议".into()),
                    rules: Some(vec![FormRule {
                        required: true,
                        message: Some("请先勾选同意条款".into()),
                        ..FormRule::default()
                    }]),
                    Checkbox {
                        default_checked: false,
                        "我已阅读并同意相关使用条款"
                    }
                }

                // 操作区
                Flex {
                    direction: FlexDirection::Row,
                    gap_size: Some(FlexGap::Middle),
                    justify: FlexJustify::End,
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let mut submit_message = submit_message;
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
