//! 登录页面场景演示
//!
//! 模拟真实的用户登录场景，综合使用：
//! - Layout 布局
//! - Form 表单
//! - Input 输入框
//! - Password 密码框
//! - Checkbox 复选框
//! - Button 按钮
//! - Card 卡片

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Card, Checkbox, Form, FormItem, Input, Layout, Password,
    ThemeProvider, Title, TitleLevel,
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
            LoginDemo {}
        }
    }
}

#[component]
fn LoginDemo() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "".to_string());

    rsx! {
        Layout {
            style: "min-height: 100vh; background: var(--adui-color-bg-base);",
            div {
                style: "display: flex; align-items: center; justify-content: center; padding: 24px; min-height: 100vh;",
                Card {
                    style: "width: 100%; max-width: 400px;",
                    title: Some(rsx!(
                        div {
                            style: "text-align: center;",
                            Title { level: TitleLevel::H2, "用户登录" }
                        }
                    )),
                    Form {
                        form: Some(form_handle.read().clone()),
                        on_finish: {
                            let mut sig = submit_message;
                            move |evt: FormFinishEvent| {
                                sig.set("登录成功！".to_string());
                            }
                        },
                        on_finish_failed: {
                            let mut sig = submit_message;
                            move |evt: FormFinishFailedEvent| {
                                sig.set("登录失败，请检查输入".to_string());
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
                                placeholder: Some("请输入用户名或邮箱".into()),
                                prefix: Some(rsx!(span { "👤" })),
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
                            name: Some("remember".into()),
                            FormItem {
                                name: None,
                                label: None,
                                Checkbox {
                                    default_checked: true,
                                    "记住我"
                                }
                            }
                        }
                        FormItem {
                            name: None,
                            label: None,
                            Button {
                                r#type: ButtonType::Primary,
                                html_type: ButtonHtmlType::Submit,
                                block: true,
                                "登录"
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
