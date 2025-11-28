use adui_dioxus::{
    AutoComplete, Button, ButtonHtmlType, ButtonType, Cascader, Form, FormItem, FormLayout, Select,
    SelectOption, ThemeProvider, TreeNode, TreeSelect,
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
            SelectFamilyIntegrationDemo {}
        }
    }
}

fn base_select_options() -> Vec<SelectOption> {
    vec![
        SelectOption {
            key: "basic".into(),
            label: "Basic".into(),
            disabled: false,
        },
        SelectOption {
            key: "advanced".into(),
            label: "Advanced".into(),
            disabled: false,
        },
        SelectOption {
            key: "expert".into(),
            label: "Expert".into(),
            disabled: false,
        },
    ]
}

fn region_tree() -> Vec<TreeNode> {
    vec![
        TreeNode {
            key: "zhejiang".into(),
            label: "Zhejiang".into(),
            disabled: false,
            children: vec![TreeNode {
                key: "hangzhou".into(),
                label: "Hangzhou".into(),
                disabled: false,
                children: vec![TreeNode {
                    key: "xihu".into(),
                    label: "West Lake".into(),
                    disabled: false,
                    children: vec![],
                }],
            }],
        },
        TreeNode {
            key: "jiangsu".into(),
            label: "Jiangsu".into(),
            disabled: false,
            children: vec![TreeNode {
                key: "nanjing".into(),
                label: "Nanjing".into(),
                disabled: false,
                children: vec![],
            }],
        },
    ]
}

fn city_options() -> Vec<SelectOption> {
    vec![
        SelectOption {
            key: "hangzhou".into(),
            label: "Hangzhou".into(),
            disabled: false,
        },
        SelectOption {
            key: "shanghai".into(),
            label: "Shanghai".into(),
            disabled: false,
        },
        SelectOption {
            key: "beijing".into(),
            label: "Beijing".into(),
            disabled: false,
        },
        SelectOption {
            key: "shenzhen".into(),
            label: "Shenzhen".into(),
            disabled: false,
        },
    ]
}

#[component]
fn SelectFamilyIntegrationDemo() -> Element {
    let form_handle = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            div { style: "margin: 0 auto; max-width: 720px; display: flex; flex-direction: column; gap: 16px;",
                h2 { "选择器家族集成示例" }
                p { "在同一个 Form 中组合使用 Select / TreeSelect / Cascader / AutoComplete，验证 reset 与校验行为一致性。" }
                Form {
                    layout: FormLayout::Vertical,
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

                    // Select 字段：计划类型（必填）
                    FormItem {
                        name: Some("plan".to_string()),
                        label: Some("Plan".to_string()),
                        rules: Some(vec![
                            FormRule {
                                required: true,
                                message: Some("请选择计划类型".into()),
                                ..FormRule::default()
                            }
                        ]),
                        Select {
                            options: base_select_options(),
                            placeholder: Some("请选择计划类型".to_string()),
                        }
                    }

                    // TreeSelect 字段：地区节点（必填，多选）
                    FormItem {
                        name: Some("regions".to_string()),
                        label: Some("Regions".to_string()),
                        rules: Some(vec![
                            FormRule {
                                required: true,
                                message: Some("请选择至少一个地区节点".into()),
                                ..FormRule::default()
                            }
                        ]),
                        TreeSelect {
                            tree_data: Some(region_tree()),
                            multiple: true,
                            tree_checkable: true,
                            show_search: true,
                            placeholder: Some("请选择地区节点".to_string()),
                        }
                    }

                    // Cascader 字段：地区路径（必填）
                    FormItem {
                        name: Some("regionPath".to_string()),
                        label: Some("Region Path".to_string()),
                        rules: Some(vec![
                            FormRule {
                                required: true,
                                message: Some("请选择地区路径".into()),
                                ..FormRule::default()
                            }
                        ]),
                        Cascader {
                            options: region_tree(),
                            placeholder: Some("请选择地区路径".to_string()),
                        }
                    }

                    // AutoComplete 字段：城市名称（必填）
                    FormItem {
                        name: Some("city".to_string()),
                        label: Some("City".to_string()),
                        rules: Some(vec![
                            FormRule {
                                required: true,
                                message: Some("请输入或选择城市".into()),
                                ..FormRule::default()
                            }
                        ]),
                        AutoComplete {
                            options: Some(city_options()),
                            placeholder: Some("请输入城市".to_string()),
                            allow_clear: true,
                        }
                    }

                    // 按钮行
                    FormItem {
                        name: None,
                        label: None,
                        children: rsx! {
                            div { style: "display:flex; gap: 8px;",
                                Button {
                                    r#type: ButtonType::Primary,
                                    html_type: ButtonHtmlType::Submit,
                                    "Submit"
                                }
                                Button {
                                    r#type: ButtonType::Default,
                                    onclick: {
                                        let mut submit_message = submit_message;
                                        move |_| {
                                            form_handle.read().reset_fields();
                                            submit_message.set("尚未提交".to_string());
                                        }
                                    },
                                    "Reset"
                                }
                            }
                        }
                    }
                }

                div {
                    strong { "提交结果：" }
                    pre {
                        style: "background:#f5f5f5;border:1px solid #ddd;padding:8px;",
                        "{submit_message.read()}"
                    }
                }
            }
        }
    }
}
