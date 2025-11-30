//! 注册页面场景演示
//!
//! 模拟真实的用户注册场景，综合使用：
//! - Layout 布局
//! - Form 表单
//! - Input 输入框
//! - Password 密码框
//! - Select 选择器
//! - Button 按钮
//! - Card 卡片

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Card, Form, FormItem, Input, Layout, Password, Select,
    SelectOption, ThemeProvider, Title, TitleLevel,
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
            RegisterDemo {}
        }
    }
}

#[component]
fn RegisterDemo() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "".to_string());

    let country_options = vec![
        SelectOption {
            key: "cn".into(),
            label: "中国".into(),
            disabled: false,
        },
        SelectOption {
            key: "us".into(),
            label: "美国".into(),
            disabled: false,
        },
        SelectOption {
            key: "jp".into(),
            label: "日本".into(),
            disabled: false,
        },
    ];

    rsx! {
        Layout {
            style: "min-height: 100vh; background: var(--adui-color-bg-base);",
            div {
                style: "display: flex; align-items: center; justify-content: center; padding: 24px; min-height: 100vh;",
                Card {
                    style: "width: 100%; max-width: 500px;",
                    title: Some(rsx!(
                        div {
                            style: "text-align: center;",
                            Title { level: TitleLevel::H2, "用户注册" }
                        }
                    )),
                    Form {
                        form: Some(form_handle.read().clone()),
                        on_finish: {
                            let mut sig = submit_message;
                            move |evt: FormFinishEvent| {
                                sig.set("注册成功！".to_string());
                            }
                        },
                        on_finish_failed: {
                            let mut sig = submit_message;
                            move |evt: FormFinishFailedEvent| {
                                sig.set("注册失败，请检查输入".to_string());
                            }
                        },
                        FormItem {
                            name: Some("username".into()),
                            label: Some("用户名".into()),
                            rules: Some(vec![
                                FormRule {
                                    required: true,
                                    message: Some("请输入用户名".into()),
                                    ..FormRule::default()
                                },
                                FormRule {
                                    min: Some(3),
                                    message: Some("用户名至少3个字符".into()),
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
                                placeholder: Some("example@email.com".into()),
                            }
                        }
                        FormItem {
                            name: Some("password".into()),
                            label: Some("密码".into()),
                            rules: Some(vec![
                                FormRule {
                                    required: true,
                                    message: Some("请输入密码".into()),
                                    ..FormRule::default()
                                },
                                FormRule {
                                    min: Some(6),
                                    message: Some("密码至少6个字符".into()),
                                    ..FormRule::default()
                                },
                            ]),
                            Password {
                                placeholder: Some("请输入密码".into()),
                            }
                        }
                        FormItem {
                            name: Some("confirm_password".into()),
                            label: Some("确认密码".into()),
                            rules: Some(vec![FormRule {
                                required: true,
                                message: Some("请再次输入密码".into()),
                                ..FormRule::default()
                            }]),
                            Password {
                                placeholder: Some("请再次输入密码".into()),
                            }
                        }
                        FormItem {
                            name: Some("country".into()),
                            label: Some("国家/地区".into()),
                            Select {
                                options: country_options.clone(),
                                placeholder: Some("请选择国家/地区".into()),
                            }
                        }
                        FormItem {
                            name: None,
                            label: None,
                            div {
                                style: "display: flex; gap: 12px;",
                                Button {
                                    r#type: ButtonType::Primary,
                                    html_type: ButtonHtmlType::Submit,
                                    "注册"
                                }
                                Button {
                                    r#type: ButtonType::Default,
                                    onclick: {
                                        let handle = form_handle.read().clone();
                                        move |_| handle.reset_fields()
                                    },
                                    "重置"
                                }
                            }
                        }
                    }
                    {
                        let msg = submit_message.read().clone();
                        if !msg.is_empty() {
                            rsx! {
                                div {
                                    style: "margin-top: 16px; padding: 12px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); text-align: center; color: var(--adui-color-primary);",
                                    {msg}
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }
                    }
                }
            }
        }
    }
}
