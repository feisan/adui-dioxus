//! Steps 组件演示
//!
//! 展示 Steps 组件的基础用法和高级用法，包括：
//! - 基础步骤条
//! - 受控步骤
//! - 不同状态
//! - 垂直方向
//! - 自定义图标

use adui_dioxus::{
    Button, ButtonType, Card, Result, ResultStatus, StepItem, StepStatus, Steps, StepsDirection,
    ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            StepsDemo {}
        }
    }
}

#[component]
fn StepsDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let current = use_signal(|| 0usize);
    let status_current = use_signal(|| 1usize);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let items = vec![
        StepItem::new("step1", rsx!("填写信息")),
        StepItem::new("step2", rsx!("确认")),
        StepItem::new("step3", rsx!("完成")),
    ];

    let status_items = vec![
        StepItem {
            key: "step1".into(),
            title: rsx!("已完成"),
            description: None,
            status: Some(StepStatus::Finish),
            disabled: false,
        },
        StepItem {
            key: "step2".into(),
            title: rsx!("进行中"),
            description: None,
            status: Some(StepStatus::Process),
            disabled: false,
        },
        StepItem {
            key: "step3".into(),
            title: rsx!("等待"),
            description: None,
            status: Some(StepStatus::Wait),
            disabled: false,
        },
        StepItem {
            key: "step4".into(),
            title: rsx!("错误"),
            description: None,
            status: Some(StepStatus::Error),
            disabled: false,
        },
    ];

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

            // 基础步骤条
            DemoSection {
                title: "基础步骤条",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Steps {
                        items: items.clone(),
                        current: Some(*current.read()),
                        on_change: {
                            let mut sig = current;
                            move |next| sig.set(next)
                        },
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        Button {
                            r#type: ButtonType::Default,
                            disabled: *current.read() == 0,
                            onclick: {
                                let mut sig = current;
                                move |_| {
                                    let c = *sig.read();
                                    if c > 0 {
                                        sig.set(c - 1);
                                    }
                                }
                            },
                            "上一步"
                        }
                        Button {
                            r#type: ButtonType::Primary,
                            disabled: *current.read() >= items.len() - 1,
                            onclick: {
                                let mut sig = current;
                                let max = items.len();
                                move |_| {
                                    let c = *sig.read();
                                    if c < max - 1 {
                                        sig.set(c + 1);
                                    }
                                }
                            },
                            "下一步"
                        }
                    }
                }
            }

            // 不同状态
            DemoSection {
                title: "不同状态",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Steps {
                        items: status_items.clone(),
                        current: Some(*status_current.read()),
                        on_change: {
                            let mut sig = status_current;
                            move |next| sig.set(next)
                        },
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        Button {
                            r#type: ButtonType::Default,
                            disabled: *status_current.read() == 0,
                            onclick: {
                                let mut sig = status_current;
                                move |_| {
                                    let c = *sig.read();
                                    if c > 0 {
                                        sig.set(c - 1);
                                    }
                                }
                            },
                            "上一步"
                        }
                        Button {
                            r#type: ButtonType::Primary,
                            disabled: *status_current.read() >= status_items.len() - 1,
                            onclick: {
                                let mut sig = status_current;
                                let max = status_items.len();
                                move |_| {
                                    let c = *sig.read();
                                    if c < max - 1 {
                                        sig.set(c + 1);
                                    }
                                }
                            },
                            "下一步"
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 垂直方向
            DemoSection {
                title: "垂直方向",
                Steps {
                    items: items.clone(),
                    current: Some(*current.read()),
                    direction: StepsDirection::Vertical,
                    on_change: {
                        let mut sig = current;
                        move |next| sig.set(next)
                    },
                }
            }

            // 完整流程示例
            DemoSection {
                title: "完整流程示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Steps {
                        items: items.clone(),
                        current: Some(*current.read()),
                        on_change: {
                            let mut sig = current;
                            move |next| sig.set(next)
                        },
                    }
                    div {
                        style: "margin-top: 16px;",
                        match *current.read() {
                            0 => rsx! {
                                Card {
                                    title: Some(rsx!("步骤一：填写信息")),
                                    children: rsx!(
                                        p { "在这一步填写基本信息（此处仅为示例文案）。" }
                                        Button {
                                            r#type: ButtonType::Primary,
                                            onclick: {
                                                let mut sig = current;
                                                move |_| sig.set(1)
                                            },
                                            "下一步"
                                        }
                                    ),
                                }
                            },
                            1 => rsx! {
                                Card {
                                    title: Some(rsx!("步骤二：确认")),
                                    children: rsx!(
                                        p { "请确认信息无误后继续。" }
                                        div {
                                            style: "display: flex; gap: 8px;",
                                            Button {
                                                r#type: ButtonType::Default,
                                                onclick: {
                                                    let mut sig = current;
                                                    move |_| sig.set(0)
                                                },
                                                "上一步"
                                            }
                                            Button {
                                                r#type: ButtonType::Primary,
                                                onclick: {
                                                    let mut sig = current;
                                                    move |_| sig.set(2)
                                                },
                                                "提交"
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
                                            sub_title: Some(rsx!("可以返回首页或继续操作。")),
                                            extra: Some(rsx!(
                                                Button {
                                                    r#type: ButtonType::Primary,
                                                    onclick: {
                                                        let mut sig = current;
                                                        move |_| sig.set(0)
                                                    },
                                                    "重新开始"
                                                }
                                            )),
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
