//! TimePicker 组件演示
//!
//! 展示 TimePicker 组件的基础用法和高级用法，包括：
//! - 基础时间选择
//! - 步进控制
//! - 禁用状态
//! - 与Form集成

use adui_dioxus::{
    Button, ButtonHtmlType, ButtonType, Form, FormItem, FormLayout, ThemeMode, ThemeProvider,
    TimePicker, TimeValue, Title, TitleLevel,
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
            TimePickerDemo {}
        }
    }
}

#[component]
fn TimePickerDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| None);
    let stepped_value = use_signal(|| Some(TimeValue::new(9, 30, 0)));

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

            // 基础时间选择
            DemoSection {
                title: "基础时间选择",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "时间：" }
                        TimePicker {
                            value: *basic_value.read(),
                            on_change: {
                                let mut sig = basic_value;
                                move |next| sig.set(next)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        {
                            match *basic_value.read() {
                                Some(time) => format!("{:02}:{:02}:{:02}", time.hour, time.minute, time.second),
                                None => "(未选择)".to_string(),
                            }
                        }
                    }
                }
            }

            // 步进控制
            DemoSection {
                title: "步进控制",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 80px;", "时间：" }
                        TimePicker {
                            value: *stepped_value.read(),
                            hour_step: Some(2),
                            minute_step: Some(15),
                            second_step: Some(30),
                            on_change: {
                                let mut sig = stepped_value;
                                move |next| sig.set(next)
                            },
                        }
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "步进: 小时2, 分钟15, 秒30"
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                TimePicker {
                    value: Some(TimeValue::new(12, 0, 0)),
                    disabled: Some(true),
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 与Form集成
            DemoSection {
                title: "与Form集成",
                FormTimePickerSection {}
            }
        }
    }
}

#[component]
fn FormTimePickerSection() -> Element {
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
                name: Some("time".into()),
                label: Some("时间".into()),
                rules: Some(vec![
                    FormRule {
                        required: true,
                        message: Some("请选择时间".into()),
                        ..FormRule::default()
                    }
                ]),
                TimePicker {}
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
