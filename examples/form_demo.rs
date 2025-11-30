//! Form 组件演示
//!
//! 展示 Form 组件的基础用法和高级用法，包括：
//! - 基础表单
//! - 表单验证规则
//! - 表单布局
//! - 表单重置和提交

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Form, FormItem, FormLayout, Input, TextArea, ThemeMode,
    ThemeProvider, Title, TitleLevel,
    components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule},
    use_form, use_theme,
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
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",

            // 控制工具栏
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 24px; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border);",
                span { style: "font-weight: 600;", "主题控制：" }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Light,
                    "Light"
                }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Dark,
                    "Dark"
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础表单
            DemoSection {
                title: "基础表单",
                Form {
                    form: Some(form_handle.read().clone()),
                    on_finish: {
                        let mut sig = submit_message;
                        move |evt: FormFinishEvent| {
                            sig.set(format!("提交成功: {:?}", evt.values));
                        }
                    },
                    on_finish_failed: {
                        let mut sig = submit_message;
                        move |evt: FormFinishFailedEvent| {
                            sig.set(format!("提交失败: {:?}", evt.errors));
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
                            let mut sig = submit_message;
                            move |_| {
                                let handle = form_handle.read().clone();
                                handle.reset_fields();
                                sig.set("尚未提交".to_string());
                            }
                        },
                        "重置"
                    }
                }
                {
                    let msg = submit_message.read().clone();
                    rsx! {
                        div {
                            style: "margin-top: 16px; padding: 12px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 14px;",
                            strong { "提交结果：" }
                            span { {msg} }
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 表单验证
            ValidationFormSection {}

            // 表单布局
            LayoutFormSection {}
        }
    }
}

#[component]
fn ValidationFormSection() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        DemoSection {
            title: "表单验证规则",
            Form {
                form: Some(form_handle.read().clone()),
                on_finish: {
                    let mut sig = submit_message;
                    move |evt: FormFinishEvent| {
                        sig.set(format!("提交成功: {:?}", evt.values));
                    }
                },
                on_finish_failed: {
                    let mut sig = submit_message;
                    move |evt: FormFinishFailedEvent| {
                        sig.set(format!("提交失败: {:?}", evt.errors));
                    }
                },
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
                        placeholder: Some("至少3个字符".into()),
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
                            min: Some(6),
                            message: Some("密码至少 6 个字符".into()),
                            ..FormRule::default()
                        },
                    ]),
                    Input {
                        placeholder: Some("至少6个字符".into()),
                    }
                }
                Button {
                    r#type: ButtonType::Primary,
                    html_type: ButtonHtmlType::Submit,
                    "提交"
                }
            }
            {
                let msg = submit_message.read().clone();
                rsx! {
                    div {
                        style: "margin-top: 16px; padding: 12px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 14px;",
                        strong { "提交结果：" }
                        span { {msg} }
                    }
                }
            }
        }
    }
}

#[component]
fn LayoutFormSection() -> Element {
    let form_handle = use_signal(use_form);

    rsx! {
        DemoSection {
            title: "表单布局",
            div {
                style: "display: flex; flex-direction: column; gap: 24px;",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    span { style: "font-weight: 600;", "垂直布局（默认）" }
                    Form {
                        form: Some(form_handle.read().clone()),
                        layout: FormLayout::Vertical,
                        FormItem {
                            name: Some("field1".into()),
                            label: Some("字段1".into()),
                            Input {
                                placeholder: Some("输入内容".into()),
                            }
                        }
                        FormItem {
                            name: Some("field2".into()),
                            label: Some("字段2".into()),
                            Input {
                                placeholder: Some("输入内容".into()),
                            }
                        }
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    span { style: "font-weight: 600;", "水平布局" }
                    Form {
                        form: Some(form_handle.read().clone()),
                        layout: FormLayout::Horizontal,
                        FormItem {
                            name: Some("field3".into()),
                            label: Some("字段3".into()),
                            Input {
                                placeholder: Some("输入内容".into()),
                            }
                        }
                        FormItem {
                            name: Some("field4".into()),
                            label: Some("字段4".into()),
                            Input {
                                placeholder: Some("输入内容".into()),
                            }
                        }
                    }
                }
            }
        }
    }
}

// 统一的demo section组件
#[derive(Props, Clone, PartialEq)]
struct DemoSectionProps {
    title: &'static str,
    children: Element,
}

#[component]
fn DemoSection(props: DemoSectionProps) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 24px; padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);",
            div {
                style: "font-weight: 600; margin-bottom: 12px; color: var(--adui-color-text); font-size: 14px;",
                {props.title}
            }
            {props.children}
        }
    }
}
