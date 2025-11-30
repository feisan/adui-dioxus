//! AutoComplete 组件演示
//!
//! 展示 AutoComplete 组件的基础用法和高级用法，包括：
//! - 基础自动完成
//! - 自定义数据源
//! - 异步搜索
//! - 与Form集成

use adui_dioxus::{
    AutoComplete, Button, ButtonHtmlType, ButtonType, Form, FormItem, FormLayout, SelectOption,
    ThemeMode, ThemeProvider, Title, TitleLevel,
    components::control::ControlStatus,
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
            AutoCompleteDemo {}
        }
    }
}

fn city_options() -> Vec<SelectOption> {
    vec![
        SelectOption {
            key: "hangzhou".into(),
            label: "杭州".into(),
            disabled: false,
        },
        SelectOption {
            key: "shanghai".into(),
            label: "上海".into(),
            disabled: false,
        },
        SelectOption {
            key: "beijing".into(),
            label: "北京".into(),
            disabled: false,
        },
        SelectOption {
            key: "shenzhen".into(),
            label: "深圳".into(),
            disabled: false,
        },
        SelectOption {
            key: "guangzhou".into(),
            label: "广州".into(),
            disabled: false,
        },
        SelectOption {
            key: "chengdu".into(),
            label: "成都".into(),
            disabled: false,
        },
    ]
}

#[component]
fn AutoCompleteDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| String::new());
    let async_value = use_signal(|| String::new());
    let async_options = use_signal(|| Vec::<SelectOption>::new());

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

            // 基础自动完成
            DemoSection {
                title: "基础自动完成",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "城市：" }
                        AutoComplete {
                            options: Some(city_options()),
                            value: Some(basic_value.read().clone()),
                            placeholder: Some("请输入城市名称".into()),
                            allow_clear: true,
                            on_change: {
                                let mut sig = basic_value;
                                move |txt: String| sig.set(txt)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        if basic_value.read().is_empty() {
                            "(空)"
                        } else {
                            {basic_value.read().clone()}
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                AutoComplete {
                    options: Some(city_options()),
                    placeholder: Some("禁用状态".into()),
                    disabled: true,
                }
            }

            // 不同状态
            DemoSection {
                title: "不同状态",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    AutoComplete {
                        options: Some(city_options()),
                        placeholder: Some("成功状态".into()),
                        status: Some(ControlStatus::Success),
                    }
                    AutoComplete {
                        options: Some(city_options()),
                        placeholder: Some("警告状态".into()),
                        status: Some(ControlStatus::Warning),
                    }
                    AutoComplete {
                        options: Some(city_options()),
                        placeholder: Some("错误状态".into()),
                        status: Some(ControlStatus::Error),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 异步搜索
            DemoSection {
                title: "异步搜索",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "搜索：" }
                        AutoComplete {
                            options: if async_options.read().is_empty() { None } else { Some(async_options.read().clone()) },
                            value: Some(async_value.read().clone()),
                            placeholder: Some("输入关键词进行搜索".into()),
                            allow_clear: true,
                            on_change: {
                                let mut sig = async_value;
                                move |txt: String| sig.set(txt)
                            },
                            on_search: {
                                let mut sig = async_options;
                                move |query: String| {
                                    // 模拟异步搜索
                                    let filtered: Vec<SelectOption> = city_options()
                                        .into_iter()
                                        .filter(|opt| opt.label.to_lowercase().contains(&query.to_lowercase()))
                                        .collect();
                                    sig.set(filtered);
                                }
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "搜索结果数量: ",
                        {async_options.read().len().to_string()}
                    }
                }
            }

            // 与Form集成
            DemoSection {
                title: "与Form集成",
                FormAutoCompleteSection {}
            }
        }
    }
}

#[component]
fn FormAutoCompleteSection() -> Element {
    let form_signal = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        Form {
            layout: FormLayout::Vertical,
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
            FormItem {
                name: Some("city".into()),
                label: Some("城市".into()),
                rules: Some(vec![
                    FormRule {
                        required: true,
                        message: Some("请选择或输入城市".into()),
                        ..FormRule::default()
                    }
                ]),
                AutoComplete {
                    options: Some(city_options()),
                    placeholder: Some("请输入或选择城市".into()),
                }
            }
            FormItem {
                name: None,
                label: None,
                children: rsx! {
                    Button {
                        r#type: ButtonType::Primary,
                        html_type: ButtonHtmlType::Submit,
                        "提交"
                    }
                }
            }
            div {
                style: "padding: 12px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 14px;",
                strong { "提交结果：" }
                span { {submit_message.read().clone()} }
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
