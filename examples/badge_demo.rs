use adui_dioxus::{
    App, Badge, BadgeStatus, Button, ButtonType, ComponentSize, ConfigProvider, FloatButton,
    FloatButtonType, Icon, IconKind,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { BadgeDemoShell {} }
        }
    }
}

#[component]
fn BadgeDemoShell() -> Element {
    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Badge demo" }
            p { "展示基础 Badge 用法，包括数字角标、小红点与状态色。" }

            // 挂在按钮上的数字角标
            h3 { style: "margin-top: 16px;", "按钮上的角标" }
            div { style: "display: flex; gap: 16px; align-items: center;",
                Badge {
                    count: Some(5),
                    children: Some(rsx!(
                        Button { r#type: ButtonType::Default, "消息" }
                    )),
                }
                Badge {
                    count: Some(120),
                    overflow_count: 99,
                    children: Some(rsx!(
                        Button { r#type: ButtonType::Default, "通知" }
                    )),
                }
            }

            // 小红点
            h3 { style: "margin-top: 24px;", "小红点" }
            div { style: "display: flex; gap: 16px; align-items: center;",
                Badge {
                    dot: true,
                    children: Some(rsx!(
                        Button { r#type: ButtonType::Default, "待处理" }
                    )),
                }
                Badge {
                    dot: true,
                    status: Some(BadgeStatus::Success),
                    children: Some(rsx!(
                        Icon { kind: IconKind::Info, size: 20.0 }
                    )),
                }
            }

            // 状态色示例
            h3 { style: "margin-top: 24px;", "状态角标" }
            div { style: "display: flex; gap: 16px; align-items: center;",
                Badge {
                    count: Some(1),
                    status: Some(BadgeStatus::Success),
                    children: Some(rsx!("成功")),
                }
                Badge {
                    count: Some(2),
                    status: Some(BadgeStatus::Warning),
                    children: Some(rsx!("警告")),
                }
                Badge {
                    count: Some(3),
                    status: Some(BadgeStatus::Error),
                    children: Some(rsx!("错误")),
                }
            }

            // 与 FloatButton 组合的示例
            h3 { style: "margin-top: 24px;", "与 FloatButton 组合" }
            div { style: "position: relative; height: 120px;",
                Badge {
                    count: Some(8),
                    children: Some(rsx!(
                        FloatButton {
                            r#type: FloatButtonType::Default,
                            icon: rsx!(Icon { kind: IconKind::Info, size: 20.0 }),
                        }
                    )),
                }
            }
        }
    }
}
