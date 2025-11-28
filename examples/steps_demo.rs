use adui_dioxus::{
    App, Button, ButtonType, Card, ComponentSize, ConfigProvider, Result, ResultStatus, StepItem,
    Steps,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { StepsDemoShell {} }
        }
    }
}

#[component]
fn StepsDemoShell() -> Element {
    let mut current = use_signal(|| 0usize);

    let items = vec![
        StepItem::new("step1", rsx!("填写信息")),
        StepItem::new("step2", rsx!("确认")),
        StepItem::new("step3", rsx!("完成")),
    ];

    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Steps demo" }
            p { "基础步骤条示例，演示受控 current 与简单结果视图。" }

            Steps {
                items: items.clone(),
                current: Some(*current.read()),
                on_change: move |next| current.set(next),
            }

            div { style: "margin-top: 24px; max-width: 480px;",
                match *current.read() {
                    0 => rsx! {
                        Card {
                            title: Some(rsx!("步骤一：填写信息")),
                            children: rsx!(
                                p { "在这一步填写基本信息（此处仅为示例文案）。" }
                                Button { r#type: ButtonType::Primary, onclick: move |_| current.set(1), "下一步" }
                            ),
                        }
                    },
                    1 => rsx! {
                        Card {
                            title: Some(rsx!("步骤二：确认")),
                            children: rsx!(
                                p { "请确认信息无误后继续。" }
                                Button { r#type: ButtonType::Default, onclick: move |_| current.set(0), "上一步" }
                                Button { r#type: ButtonType::Primary, onclick: move |_| current.set(2), "提交" }
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
                                        Button { r#type: ButtonType::Primary, onclick: move |_| current.set(0), "重新开始" }
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
