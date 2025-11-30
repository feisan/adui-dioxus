//! TreeSelect 组件演示
//!
//! 展示 TreeSelect 组件的基础用法和高级用法，包括：
//! - 基础树形选择
//! - 多选模式
//! - 搜索功能
//! - 与Form集成

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Form, FormItem, FormLayout, ThemeMode, ThemeProvider,
    Title, TitleLevel, TreeNode, TreeSelect,
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
            TreeSelectDemo {}
        }
    }
}

fn demo_tree_data() -> Vec<TreeNode> {
    vec![
        TreeNode {
            key: "zhejiang".into(),
            label: "浙江".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "hangzhou".into(),
                    label: "杭州".into(),
                    disabled: false,
                    children: vec![TreeNode {
                        key: "xihu".into(),
                        label: "西湖区".into(),
                        disabled: false,
                        children: vec![],
                    }],
                },
                TreeNode {
                    key: "ningbo".into(),
                    label: "宁波".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
        TreeNode {
            key: "jiangsu".into(),
            label: "江苏".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "nanjing".into(),
                    label: "南京".into(),
                    disabled: false,
                    children: vec![],
                },
                TreeNode {
                    key: "suzhou".into(),
                    label: "苏州".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
    ]
}

#[component]
fn TreeSelectDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let single_value = use_signal(|| Some("xihu".to_string()));
    let multi_values = use_signal(|| vec!["hangzhou".to_string(), "nanjing".to_string()]);

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

            // 基础树形选择
            DemoSection {
                title: "基础树形选择",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "地点：" }
                        TreeSelect {
                            tree_data: Some(demo_tree_data()),
                            value: single_value.read().as_ref().cloned(),
                            placeholder: Some("请选择地点".into()),
                            on_change: {
                                let mut sig = single_value;
                                move |keys: Vec<String>| sig.set(keys.into_iter().next())
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        {
                            single_value.read().as_ref().map(|v| v.clone()).unwrap_or_else(|| "(未选择)".to_string())
                        }
                    }
                }
            }

            // 多选模式
            DemoSection {
                title: "多选模式",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "地点：" }
                        TreeSelect {
                            tree_data: Some(demo_tree_data()),
                            values: multi_values.read().clone(),
                            multiple: true,
                            tree_checkable: true,
                            show_search: true,
                            placeholder: Some("请选择多个地点".into()),
                            on_change: {
                                let mut sig = multi_values;
                                move |keys: Vec<String>| sig.set(keys)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        {
                            let vals = multi_values.read().clone();
                            if vals.is_empty() {
                                "(未选择)".to_string()
                            } else {
                                vals.join(", ")
                            }
                        }
                    }
                }
            }

            // 搜索功能
            DemoSection {
                title: "搜索功能",
                TreeSelect {
                    tree_data: Some(demo_tree_data()),
                    show_search: true,
                    placeholder: Some("输入关键词搜索地点".into()),
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 与Form集成
            DemoSection {
                title: "与Form集成",
                FormTreeSelectSection {}
            }
        }
    }
}

#[component]
fn FormTreeSelectSection() -> Element {
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
                name: Some("location".into()),
                label: Some("地点".into()),
                rules: Some(vec![
                    FormRule {
                        required: true,
                        message: Some("请选择地点".into()),
                        ..FormRule::default()
                    }
                ]),
                TreeSelect {
                    tree_data: Some(demo_tree_data()),
                    placeholder: Some("请选择地点".into()),
                    show_search: true,
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
