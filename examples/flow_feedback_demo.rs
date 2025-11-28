use adui_dioxus::{
    Alert, AlertType, App, Button, ButtonHtmlType, ButtonType, Card, ComponentSize, ConfigProvider,
    Content, Form, FormItem, Input, Layout, Progress, ProgressStatus, ProgressType, Result,
    ResultStatus, Statistic, StepItem, Steps,
    components::form::{FormFinishEvent, FormFinishFailedEvent, FormRule},
    use_form, use_message,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { FlowFeedbackDemoShell {} }
        }
    }
}

#[component]
fn FlowFeedbackDemoShell() -> Element {
    let mut current_step = use_signal(|| 0usize);
    let form_handle = use_signal(use_form);
    let mut progress = use_signal(|| 0.0f32);
    let mut last_error = use_signal(|| None::<String>);
    let message_api = use_message();

    let items = vec![
        StepItem::new("basic", rsx!("填写信息")),
        StepItem::new("confirm", rsx!("确认")),
        StepItem::new("done", rsx!("完成")),
    ];

    let current = *current_step.read();

    rsx! {
        Layout {
            Content {
                style: Some("padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);".into()),
                children: rsx! {
                    h2 { "Flow feedback demo" }
                    p { "示例：使用 Steps + Form + Progress + Alert + Result/Statistic 组合一个三步流程。" }

                    Alert {
                        r#type: AlertType::Info,
                        show_icon: true,
                        message: rsx!("本示例仅用于演示组件组合，不会真实提交数据。"),
                    }

                    div { style: "margin-top: 16px; margin-bottom: 16px;",
                        Steps {
                            items: items.clone(),
                            current: Some(current),
                            on_change: move |next| {
                                // 仅允许回退，不允许跳过未完成步骤
                                if next <= *current_step.read() {
                                    current_step.set(next);
                                }
                            },
                        }
                    }

                    match current {
                        0 => rsx! {
                            Card {
                                title: Some(rsx!("步骤一：填写基本信息")),
                                children: rsx!(
                                    if let Some(err) = last_error.read().clone() {
                                        Alert {
                                            r#type: AlertType::Error,
                                            show_icon: true,
                                            closable: true,
                                            message: rsx!("提交失败"),
                                            description: Some(rsx!("{err}")),
                                        }
                                    }

                                    Form {
                                        form: Some(form_handle.read().clone()),
                                        on_finish: {
                                            let mut current_step = current_step.clone();
                                            let mut progress = progress.clone();
                                            let mut last_error = last_error.clone();
                                            move |_evt: FormFinishEvent| {
                                                last_error.set(None);
                                                progress.set(40.0);
                                                current_step.set(1);
                                            }
                                        },
                                        on_finish_failed: {
                                            let mut last_error = last_error.clone();
                                            move |evt: FormFinishFailedEvent| {
                                                last_error.set(Some(format!("请检查字段: {:?}", evt.errors)));
                                            }
                                        },

                                        FormItem {
                                            name: Some("name".into()),
                                            label: Some("用户名".into()),
                                            rules: Some(vec![FormRule {
                                                required: true,
                                                message: Some("请输入用户名".into()),
                                                ..FormRule::default()
                                            }]),
                                            Input { placeholder: Some("请输入用户名".into()) }
                                        }

                                        FormItem {
                                            name: Some("email".into()),
                                            label: Some("邮箱".into()),
                                            rules: Some(vec![FormRule {
                                                required: true,
                                                message: Some("请输入邮箱".into()),
                                                ..FormRule::default()
                                            }]),
                                            Input { placeholder: Some("请输入邮箱".into()) }
                                        }

                                        Button {
                                            r#type: ButtonType::Primary,
                                            html_type: ButtonHtmlType::Submit,
                                            "下一步"
                                        }
                                    }
                                ),
                            }
                        },
                        1 => rsx! {
                            Card {
                                title: Some(rsx!("步骤二：确认与处理")),
                                children: rsx!(
                                    p { "此处模拟一个需要一定时间的处理过程，可通过按钮增加进度。" }
                                    Progress {
                                        percent: *progress.read(),
                                        show_info: true,
                                        r#type: ProgressType::Line,
                                        status: Some(ProgressStatus::Active),
                                    }
                                    div { style: "margin-top: 12px; display: flex; gap: 8px;",
                                        Button {
                                            r#type: ButtonType::Default,
                                            onclick: {
                                                let mut progress = progress.clone();
                                                move |_| {
                                                    let next = (*progress.read() + 20.0).min(100.0);
                                                    progress.set(next);
                                                }
                                            },
                                            "增加进度 20%"
                                        }
                                        Button {
                                            r#type: ButtonType::Default,
                                            onclick: {
                                                let mut current_step = current_step.clone();
                                                move |_| current_step.set(0)
                                            },
                                            "上一步"
                                        }
                                        Button {
                                            r#type: ButtonType::Primary,
                                            onclick: {
                                                let mut current_step = current_step.clone();
                                                let mut progress = progress.clone();
                                                let api = message_api.clone();
                                                move |_| {
                                                    progress.set(100.0);
                                                    current_step.set(2);
                                                    if let Some(msg) = api.clone() {
                                                        msg.success("流程已完成");
                                                    }
                                                }
                                            },
                                            "完成"
                                        }
                                    }
                                ),
                            }
                        },
                        _ => rsx! {
                            Card {
                                title: Some(rsx!("步骤三：完成")),
                                children: rsx!(
                                    Result {
                                        status: Some(ResultStatus::Success),
                                        title: Some(rsx!("提交成功")),
                                        sub_title: Some(rsx!("可以返回第一步重新填写，或关闭页面。")),
                                        extra: Some(rsx!(
                                            Button { r#type: ButtonType::Primary, onclick: move |_| current_step.set(0), "重新开始" }
                                        )),
                                    }
                                    div { style: "margin-top: 16px;",
                                        Statistic {
                                            title: Some(rsx!("完成进度")),
                                            value: Some(100.0),
                                            precision: Some(0),
                                            suffix: Some(rsx!("%")),
                                        }
                                    }
                                ),
                            }
                        },
                    }
                }
            }
        }
    }
}
