//! Badge 组件演示
//!
//! 展示 Badge 组件的基础用法和高级用法，包括：
//! - 数字角标
//! - 小红点
//! - 状态角标
//! - 溢出计数
//! - 自定义内容

use adui_dioxus::{
    Badge, BadgeStatus, Button, ButtonType, Icon, IconKind, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            BadgeDemo {}
        }
    }
}

#[component]
fn BadgeDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);

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

            // 数字角标
            DemoSection {
                title: "数字角标",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Badge {
                        count_number: Some(5),
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "消息" }
                        )),
                    }
                    Badge {
                        count_number: Some(25),
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "通知" }
                        )),
                    }
                    Badge {
                        count_number: Some(99),
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "提醒" }
                        )),
                    }
                }
            }

            // 溢出计数
            DemoSection {
                title: "溢出计数",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Badge {
                        count_number: Some(120),
                        overflow_count: 99,
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "通知" }
                        )),
                    }
                    Badge {
                        count_number: Some(1000),
                        overflow_count: 999,
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "消息" }
                        )),
                    }
                }
            }

            // 小红点
            DemoSection {
                title: "小红点",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Badge {
                        dot: true,
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "待处理" }
                        )),
                    }
                    Badge {
                        dot: true,
                        children: Some(rsx!(
                            Icon { kind: IconKind::Info, size: 20.0 }
                        )),
                    }
                    Badge {
                        dot: true,
                        children: Some(rsx!(
                            span { style: "padding: 8px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);", "自定义内容" }
                        )),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 状态角标
            DemoSection {
                title: "状态角标",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Badge {
                        count_number: Some(1),
                        status: Some(BadgeStatus::Success),
                        children: Some(rsx!("成功")),
                    }
                    Badge {
                        count_number: Some(2),
                        status: Some(BadgeStatus::Warning),
                        children: Some(rsx!("警告")),
                    }
                    Badge {
                        count_number: Some(3),
                        status: Some(BadgeStatus::Error),
                        children: Some(rsx!("错误")),
                    }
                    Badge {
                        dot: true,
                        status: Some(BadgeStatus::Success),
                        children: Some(rsx!("在线")),
                    }
                }
            }

            // 自定义内容
            DemoSection {
                title: "自定义内容",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Badge {
                        count_number: Some(0),
                        show_zero: true,
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "显示零" }
                        )),
                    }
                    Badge {
                        count_number: Some(0),
                        show_zero: false,
                        children: Some(rsx!(
                            Button { r#type: ButtonType::Default, "隐藏零" }
                        )),
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
