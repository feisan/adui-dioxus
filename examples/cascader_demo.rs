//! Cascader 组件演示
//!
//! 展示 Cascader 组件的基础用法和高级用法，包括：
//! - 基础级联选择
//! - 多级选择
//! - 禁用状态
//! - 与Form集成

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Cascader, Form, FormItem, FormLayout, ThemeMode,
    ThemeProvider, Title, TitleLevel,
    components::control::ControlStatus,
    components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule},
    components::select_base::CascaderNode,
    use_form, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            CascaderDemo {}
        }
    }
}

fn region_options() -> Vec<CascaderNode> {
    vec![
        CascaderNode {
            key: "zhejiang".into(),
            label: "浙江".into(),
            disabled: false,
            children: vec![
                CascaderNode {
                    key: "hangzhou".into(),
                    label: "杭州".into(),
                    disabled: false,
                    children: vec![
                        CascaderNode {
                            key: "xihu".into(),
                            label: "西湖区".into(),
                            disabled: false,
                            children: vec![],
                        },
                        CascaderNode {
                            key: "xiacheng".into(),
                            label: "下城区".into(),
                            disabled: false,
                            children: vec![],
                        },
                    ],
                },
                CascaderNode {
                    key: "ningbo".into(),
                    label: "宁波".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
        CascaderNode {
            key: "jiangsu".into(),
            label: "江苏".into(),
            disabled: false,
            children: vec![
                CascaderNode {
                    key: "nanjing".into(),
                    label: "南京".into(),
                    disabled: false,
                    children: vec![],
                },
                CascaderNode {
                    key: "suzhou".into(),
                    label: "苏州".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
        CascaderNode {
            key: "guangdong".into(),
            label: "广东".into(),
            disabled: false,
            children: vec![
                CascaderNode {
                    key: "guangzhou".into(),
                    label: "广州".into(),
                    disabled: false,
                    children: vec![],
                },
                CascaderNode {
                    key: "shenzhen".into(),
                    label: "深圳".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
    ]
}

#[component]
fn CascaderDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| Vec::<String>::new());
    let controlled_value = use_signal(|| {
        vec![
            "zhejiang".to_string(),
            "hangzhou".to_string(),
            "xihu".to_string(),
        ]
    });

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

            // 基础级联选择
            DemoSection {
                title: "基础级联选择",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "地区：" }
                        Cascader {
                            options: region_options(),
                            value: Some(basic_value.read().clone()),
                            placeholder: Some("请选择省/市/区".into()),
                            allow_clear: true,
                            on_change: {
                                let mut sig = basic_value;
                                move |path: Vec<String>| sig.set(path)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前路径: ",
                        if basic_value.read().is_empty() {
                            "(未选择)"
                        } else {
                            {basic_value.read().join(" / ")}
                        }
                    }
                }
            }

            // 受控模式
            DemoSection {
                title: "受控模式",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "地区：" }
                        Cascader {
                            options: region_options(),
                            value: Some(controlled_value.read().clone()),
                            placeholder: Some("请选择省/市/区".into()),
                            allow_clear: true,
                            on_change: {
                                let mut sig = controlled_value;
                                move |path: Vec<String>| sig.set(path)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前路径: ",
                        {controlled_value.read().join(" / ")}
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                Cascader {
                    options: region_options(),
                    placeholder: Some("禁用状态".into()),
                    disabled: true,
                }
            }

            // 不同状态
            DemoSection {
                title: "不同状态",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Cascader {
                        options: region_options(),
                        placeholder: Some("成功状态".into()),
                        status: Some(ControlStatus::Success),
                    }
                    Cascader {
                        options: region_options(),
                        placeholder: Some("警告状态".into()),
                        status: Some(ControlStatus::Warning),
                    }
                    Cascader {
                        options: region_options(),
                        placeholder: Some("错误状态".into()),
                        status: Some(ControlStatus::Error),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 与Form集成
            DemoSection {
                title: "与Form集成",
                FormCascaderSection {}
            }
        }
    }
}

#[component]
fn FormCascaderSection() -> Element {
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
                name: Some("region".into()),
                label: Some("地区".into()),
                rules: Some(vec![
                    FormRule {
                        required: true,
                        message: Some("请选择地区".into()),
                        ..FormRule::default()
                    }
                ]),
                Cascader {
                    options: region_options(),
                    placeholder: Some("请选择省/市/区".into()),
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
