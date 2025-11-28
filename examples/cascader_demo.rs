use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Cascader, Form, FormItem, FormLayout, ThemeProvider,
    TreeNode,
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
            CascaderDemo {}
        }
    }
}

fn region_options() -> Vec<TreeNode> {
    vec![
        TreeNode {
            key: "zhejiang".into(),
            label: "Zhejiang".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "hangzhou".into(),
                    label: "Hangzhou".into(),
                    disabled: false,
                    children: vec![TreeNode {
                        key: "xihu".into(),
                        label: "West Lake".into(),
                        disabled: false,
                        children: vec![],
                    }],
                },
                TreeNode {
                    key: "ningbo".into(),
                    label: "Ningbo".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
        TreeNode {
            key: "jiangsu".into(),
            label: "Jiangsu".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "nanjing".into(),
                    label: "Nanjing".into(),
                    disabled: false,
                    children: vec![],
                },
                TreeNode {
                    key: "suzhou".into(),
                    label: "Suzhou".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
    ]
}

#[component]
fn CascaderDemo() -> Element {
    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            div { style: "margin: 0 auto; max-width: 680px; display: flex; flex-direction: column; gap: 16px;",
                h2 { "Cascader 级联选择" }
                p { "本示例展示 Cascader 的单选路径选择与 Form 集成行为。" }
                BasicCascaderSection {}
                FormCascaderSection {}
            }
        }
    }
}

#[component]
fn BasicCascaderSection() -> Element {
    let options = region_options();

    let path_value = use_signal(|| {
        vec![
            "zhejiang".to_string(),
            "hangzhou".to_string(),
            "xihu".to_string(),
        ]
    });
    let last_path = use_signal(|| Vec::<String>::new());

    let path_dbg = {
        let v = path_value.read();
        format!("{:?}", *v)
    };
    let last_path_dbg = {
        let v = last_path.read();
        format!("{:?}", *v)
    };

    rsx! {
        div {
            style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; background: var(--adui-color-bg-container); display: flex; flex-direction: column; gap: 12px;",
            h3 { "基础用法" }

            div {
                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                pre {
                    "path_value = {path_dbg}\n",
                    "last_path = {last_path_dbg}"
                }
            }

            div { style: "display: flex; align-items: center; gap: 8px;",
                span { style: "min-width: 80px;", "地区：" }
                Cascader {
                    options: options,
                    value: Some(path_value.read().clone()),
                    placeholder: Some("请选择地区".to_string()),
                    on_change: move |path: Vec<String>| {
                        let mut last = last_path;
                        last.set(path.clone());
                        let mut signal = path_value;
                        signal.set(path);
                    },
                }
            }
        }
    }
}

#[component]
fn FormCascaderSection() -> Element {
    let form_signal = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    rsx! {
        div {
            style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; background: var(--adui-color-bg-container); display: flex; flex-direction: column; gap: 12px;",
            h3 { "与 Form 结合" }
            Form {
                layout: FormLayout::Vertical,
                form: Some(form_signal.read().clone()),
                on_finish: {
                    let mut submit_message = submit_message.clone();
                    move |evt: FormFinishEvent| {
                        submit_message.set(format!("提交成功: {:?}", evt.values));
                    }
                },
                on_finish_failed: {
                    let mut submit_message = submit_message.clone();
                    move |evt: FormFinishFailedEvent| {
                        submit_message.set(format!("提交失败: {:?}", evt.errors));
                    }
                },
                FormItem {
                    name: Some("region".to_string()),
                    label: Some("Region".to_string()),
                    rules: Some(vec![
                        FormRule {
                            required: true,
                            message: Some("请选择地区".into()),
                            ..FormRule::default()
                        }
                    ]),
                    Cascader {
                        options: region_options(),
                        placeholder: Some("请选择地区".to_string()),
                    }
                }
                FormItem {
                    name: None,
                    label: None,
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Primary,
                            html_type: ButtonHtmlType::Submit,
                            "Submit"
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
