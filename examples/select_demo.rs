use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Form, FormItem, FormLayout, Select, SelectOption,
    ThemeProvider, components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule}, use_form,
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
    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            div { style: "margin: 0 auto; max-width: 640px; display: flex; flex-direction: column; gap: 16px;",
                h2 { "Select 选择器" }
                p { "本示例展示 Select 的基础用法以及与 Form 的集成行为。" }
                BasicSelectSection {}
                FormSelectSection {}
            }
        }
    }
}

#[component]
fn BasicSelectSection() -> Element {
    let options = vec![
        SelectOption { key: "apple".into(), label: "Apple".into(), disabled: false },
        SelectOption { key: "banana".into(), label: "Banana".into(), disabled: false },
        SelectOption { key: "cherry".into(), label: "Cherry".into(), disabled: false },
    ];

    let single_value = use_signal(|| Some("banana".to_string()));
    let multi_values = use_signal(|| vec!["apple".to_string(), "cherry".to_string()]);

    // Debug: record last keys received from on_change
    let last_single_keys = use_signal(|| Vec::<String>::new());
    let last_multi_keys = use_signal(|| Vec::<String>::new());

    // Debug snapshot for display
    let single_dbg = {
        let v = single_value.read();
        format!("{:?}", *v)
    };
    let multi_dbg = {
        let v = multi_values.read();
        format!("{:?}", *v)
    };
    let last_single_dbg = {
        let v = last_single_keys.read();
        format!("{:?}", *v)
    };
    let last_multi_dbg = {
        let v = last_multi_keys.read();
        format!("{:?}", *v)
    };

    rsx! {
        div {
            style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; background: var(--adui-color-bg-container); display: flex; flex-direction: column; gap: 12px;",
            h3 { "基础用法" }

            // Debug 当前受控值，方便排查选中是否写回到父组件。
            div {
                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                pre {
                    "single_value = {single_dbg}\n",
                    "last_single_keys = {last_single_dbg}\n",
                    "multi_values = {multi_dbg}\n",
                    "last_multi_keys = {last_multi_dbg}"
                }
            }

            div { style: "display: flex; align-items: center; gap: 8px;",
                span { style: "min-width: 80px;", "单选：" }
                Select {
                    value: single_value.read().clone(),
                    options: options.clone(),
                    placeholder: "Choose a fruit".to_string(),
                    on_change: move |keys: Vec<String>| {
                        let mut last = last_single_keys;
                        last.set(keys.clone());
                        let mut signal = single_value;
                        signal.set(keys.into_iter().next());
                    },
                }
            }
            div { style: "display: flex; align-items: center; gap: 8px;",
                span { style: "min-width: 80px;", "多选/搜索：" }
                Select {
                    values: multi_values.read().clone(),
                    options,
                    multiple: true,
                    allow_clear: true,
                    placeholder: "Choose fruits".to_string(),
                    show_search: true,
                    on_change: move |keys: Vec<String>| {
                        let mut last = last_multi_keys;
                        last.set(keys.clone());
                        let mut signal = multi_values;
                        signal.set(keys);
                    },
                }
            }
        }
    }
}

#[component]
fn FormSelectSection() -> Element {
    let form_signal = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    let options = vec![
        SelectOption { key: "weekly".into(), label: "Weekly".into(), disabled: false },
        SelectOption { key: "monthly".into(), label: "Monthly".into(), disabled: false },
        SelectOption { key: "yearly".into(), label: "Yearly".into(), disabled: false },
    ];

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
                    name: Some("plan".to_string()),
                    label: Some("Plan".to_string()),
                    rules: Some(vec![
                        FormRule {
                            required: true,
                            message: Some("Please select plan".into()),
                            ..FormRule::default()
                        }
                    ]),
                    Select {
                        options: options.clone(),
                        placeholder: Some("Select plan".to_string()),
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
