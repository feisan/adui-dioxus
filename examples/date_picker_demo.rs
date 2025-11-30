//! DatePicker 组件演示
//!
//! 展示 DatePicker 组件的基础用法和高级用法，包括：
//! - 基础日期选择
//! - 日期范围选择
//! - 禁用日期
//! - 与Form集成

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, DatePicker, DateRangeValue, Form, FormItem, FormLayout,
    RangePicker, ThemeMode, ThemeProvider, Title, TitleLevel,
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
            DatePickerDemo {}
        }
    }
}

#[component]
fn DatePickerDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let single_value = use_signal(|| None);
    let range_value = use_signal(|| DateRangeValue {
        start: None,
        end: None,
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

            // 基础日期选择
            DemoSection {
                title: "基础日期选择",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "日期：" }
                        DatePicker {
                            value: *single_value.read(),
                            placeholder: Some("请选择日期".into()),
                            on_change: {
                                let mut sig = single_value;
                                move |next| sig.set(next)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        match *single_value.read() {
                            Some(date) => rsx! { {date.to_ymd_string()} },
                            None => rsx! { "(未选择)" },
                        }
                    }
                }
            }

            // 日期范围选择
            DemoSection {
                title: "日期范围选择",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "范围：" }
                        RangePicker {
                            value: Some(*range_value.read()),
                            placeholder: Some(("开始日期".to_string(), "结束日期".to_string())),
                            on_change: {
                                let mut sig = range_value;
                                move |next| sig.set(next)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前范围: ",
                        match (range_value.read().start, range_value.read().end) {
                            (Some(s), Some(e)) => rsx!({format!("{} ~ {}", s.to_ymd_string(), e.to_ymd_string())}),
                            _ => rsx!("(未选择)"),
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 与Form集成
            DemoSection {
                title: "与Form集成",
                FormDatePickerSection {}
            }
        }
    }
}

#[component]
fn FormDatePickerSection() -> Element {
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
                name: Some("start_date".into()),
                label: Some("开始日期".into()),
                rules: Some(vec![
                    FormRule {
                        required: true,
                        message: Some("请选择开始日期".into()),
                        ..FormRule::default()
                    }
                ]),
                DatePicker {
                    placeholder: Some("请选择开始日期".into()),
                }
            }
            FormItem {
                name: Some("range".into()),
                label: Some("日期区间".into()),
                RangePicker {
                    placeholder: Some(("开始日期".to_string(), "结束日期".to_string())),
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
