//! 表单组件综合演示
//!
//! 展示所有表单组件的综合使用，包括：
//! - Input、TextArea、Select 组合
//! - Checkbox、Radio、Switch 组合
//! - DatePicker、TimePicker 组合
//! - InputNumber、Slider、Rate 组合
//! - Upload 文件上传

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Checkbox, CheckboxGroup, Form, FormItem, Input, Radio,
    RadioGroup, Select, SelectOption, Switch, TextArea, ThemeMode, ThemeProvider, Title,
    TitleLevel,
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
            FormComponentsDemo {}
        }
    }
}

#[component]
fn FormComponentsDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let select_options = vec![
        SelectOption {
            key: "option1".into(),
            label: "选项 1".into(),
            disabled: false,
        },
        SelectOption {
            key: "option2".into(),
            label: "选项 2".into(),
            disabled: false,
        },
        SelectOption {
            key: "option3".into(),
            label: "选项 3".into(),
            disabled: false,
        },
    ];

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

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "表单组件综合展示" }

            // 完整表单示例
            DemoSection {
                title: "完整表单示例",
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
                        name: Some("description".into()),
                        label: Some("描述".into()),
                        TextArea {
                            rows: Some(4),
                            placeholder: Some("请输入描述".into()),
                        }
                    }
                    FormItem {
                        name: Some("category".into()),
                        label: Some("分类".into()),
                        rules: Some(vec![FormRule {
                            required: true,
                            message: Some("请选择分类".into()),
                            ..FormRule::default()
                        }]),
                        Select {
                            options: select_options.clone(),
                            placeholder: Some("请选择分类".into()),
                        }
                    }
                    FormItem {
                        name: Some("hobbies".into()),
                        label: Some("爱好".into()),
                        CheckboxGroup {
                            Checkbox {
                                value: Some("reading".into()),
                                "阅读"
                            }
                            Checkbox {
                                value: Some("music".into()),
                                "音乐"
                            }
                            Checkbox {
                                value: Some("sports".into()),
                                "运动"
                            }
                        }
                    }
                    FormItem {
                        name: Some("gender".into()),
                        label: Some("性别".into()),
                        RadioGroup {
                            Radio {
                                value: "male".to_string(),
                                "男"
                            }
                            Radio {
                                value: "female".to_string(),
                                "女"
                            }
                        }
                    }
                    FormItem {
                        name: Some("notifications".into()),
                        label: Some("接收通知".into()),
                        Switch {
                            default_checked: true,
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
