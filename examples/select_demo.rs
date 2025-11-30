//! Select 组件演示
//!
//! 展示 Select 组件的基础用法和高级用法，包括：
//! - 单选和多选
//! - 搜索功能
//! - 分组选项
//! - 禁用状态
//! - 与 Form 集成

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Form, FormItem, Select, SelectOption, ThemeMode,
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
            SelectDemo {}
        }
    }
}

#[component]
fn SelectDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let basic_options = vec![
        SelectOption {
            key: "apple".into(),
            label: "Apple".into(),
            disabled: false,
        },
        SelectOption {
            key: "banana".into(),
            label: "Banana".into(),
            disabled: false,
        },
        SelectOption {
            key: "cherry".into(),
            label: "Cherry".into(),
            disabled: false,
        },
        SelectOption {
            key: "orange".into(),
            label: "Orange".into(),
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

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 单选
            BasicSelectSection { options: basic_options.clone() }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 与 Form 集成
            FormSelectSection {}
        }
    }
}

#[component]
fn BasicSelectSection(options: Vec<SelectOption>) -> Element {
    let single_value = use_signal(|| Some("banana".to_string()));
    let multi_values = use_signal(|| vec!["apple".to_string(), "cherry".to_string()]);

    rsx! {
        DemoSection {
            title: "基础选择器",
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    span { style: "font-weight: 600;", "单选：" }
                    Select {
                        value: single_value.read().clone(),
                        options: options.clone(),
                        placeholder: Some("请选择".into()),
                        on_change: {
                            let mut sig = single_value;
                            move |values: Vec<String>| {
                                sig.set(values.into_iter().next());
                            }
                        },
                    }
                    {
                        let value_text = format!("当前值: {:?}", *single_value.read());
                        rsx! {
                            div {
                                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                {value_text}
                            }
                        }
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    span { style: "font-weight: 600;", "多选：" }
                    Select {
                        mode: adui_dioxus::SelectMode::Multiple,
                        values: Some(multi_values.read().clone()),
                        options: options.clone(),
                        placeholder: Some("请选择多个".into()),
                        on_change: {
                            let mut sig = multi_values;
                            move |values: Vec<String>| {
                                sig.set(values);
                            }
                        },
                    }
                    {
                        let values_text = format!("当前值: {:?}", *multi_values.read());
                        rsx! {
                            div {
                                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                {values_text}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FormSelectSection() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    let fruit_options = vec![
        SelectOption {
            key: "apple".into(),
            label: "Apple".into(),
            disabled: false,
        },
        SelectOption {
            key: "banana".into(),
            label: "Banana".into(),
            disabled: false,
        },
        SelectOption {
            key: "cherry".into(),
            label: "Cherry".into(),
            disabled: false,
        },
    ];

    rsx! {
        DemoSection {
            title: "与 Form 集成",
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
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
                        name: Some("fruit".into()),
                        label: Some("选择水果".into()),
                        rules: Some(vec![FormRule {
                            required: true,
                            message: Some("请选择水果".into()),
                            ..FormRule::default()
                        }]),
                        Select {
                            options: fruit_options.clone(),
                            placeholder: Some("请选择".into()),
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
                            style: "padding: 12px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 14px;",
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
