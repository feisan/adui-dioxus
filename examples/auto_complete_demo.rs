use adui_dioxus::{
    AutoComplete, Button, ButtonHtmlType, ButtonType, Form, FormItem, FormLayout, ThemeProvider,
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
            AutoCompleteDemo {}
        }
    }
}

fn city_options() -> Vec<adui_dioxus::SelectOption> {
    vec![
        adui_dioxus::SelectOption {
            key: "hangzhou".into(),
            label: "Hangzhou".into(),
            disabled: false,
        },
        adui_dioxus::SelectOption {
            key: "shanghai".into(),
            label: "Shanghai".into(),
            disabled: false,
        },
        adui_dioxus::SelectOption {
            key: "beijing".into(),
            label: "Beijing".into(),
            disabled: false,
        },
        adui_dioxus::SelectOption {
            key: "shenzhen".into(),
            label: "Shenzhen".into(),
            disabled: false,
        },
    ]
}

#[component]
fn AutoCompleteDemo() -> Element {
    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            div { style: "margin: 0 auto; max-width: 680px; display: flex; flex-direction: column; gap: 16px;",
                h2 { "AutoComplete 输入联想" }
                p { "本示例展示 AutoComplete 的基础用法以及与 Form 的集成行为。" }
                BasicAutoCompleteSection {}
                FormAutoCompleteSection {}
            }
        }
    }
}

#[component]
fn BasicAutoCompleteSection() -> Element {
    let options = city_options();

    let mut value = use_signal(|| "Hangzhou".to_string());
    let mut last_search = use_signal(|| String::new());
    let mut last_select = use_signal(|| String::new());

    let value_dbg = {
        let v = value.read();
        format!("{:?}", *v)
    };
    let search_dbg = {
        let v = last_search.read();
        format!("{:?}", *v)
    };
    let select_dbg = {
        let v = last_select.read();
        format!("{:?}", *v)
    };

    rsx! {
        div {
            style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; background: var(--adui-color-bg-container); display: flex; flex-direction: column; gap: 12px;",
            h3 { "基础用法" }

            div {
                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                pre {
                    "value = {value_dbg}\n",
                    "last_search = {search_dbg}\n",
                    "last_select = {select_dbg}"
                }
            }

            div { style: "display: flex; align-items: center; gap: 8px;",
                span { style: "min-width: 80px;", "城市：" }
                AutoComplete {
                    options: Some(options),
                    value: Some(value.read().clone()),
                    placeholder: Some("请输入城市".to_string()),
                    on_change: move |txt: String| {
                        value.set(txt);
                    },
                    on_search: move |txt: String| {
                        last_search.set(txt);
                    },
                    on_select: move |key: String| {
                        last_select.set(key);
                    },
                }
            }
        }
    }
}

#[component]
fn FormAutoCompleteSection() -> Element {
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
