//! Switch 组件演示
//!
//! 展示 Switch 组件的基础用法和高级用法，包括：
//! - 基础开关
//! - 大小
//! - 文字
//! - 禁用状态

use adui_dioxus::{
    Button, ButtonType, Switch, SwitchSize, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SwitchDemo {}
        }
    }
}

#[component]
fn SwitchDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_checked = use_signal(|| false);
    let text_checked = use_signal(|| false);

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

            // 基础开关
            DemoSection {
                title: "基础开关",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            checked: Some(*basic_checked.read()),
                            on_change: {
                                let mut sig = basic_checked;
                                move |checked| sig.set(checked)
                            },
                        }
                        span { "基础开关" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            default_checked: true,
                        }
                        span { "默认开启" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            disabled: true,
                        }
                        span { "禁用状态" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            disabled: true,
                            default_checked: true,
                        }
                        span { "禁用且开启" }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 大小
            DemoSection {
                title: "大小（Size）",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            size: SwitchSize::Default,
                        }
                        span { "Default 尺寸" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            size: SwitchSize::Small,
                        }
                        span { "Small 尺寸" }
                    }
                }
            }

            // 文字
            DemoSection {
                title: "文字（Children）",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            checked: Some(*text_checked.read()),
                            on_change: {
                                let mut sig = text_checked;
                                move |checked| sig.set(checked)
                            },
                            checked_children: Some(rsx!("开")),
                            un_checked_children: Some(rsx!("关")),
                        }
                        span { "带文字的开关" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Switch {
                            checked_children: Some(rsx!("ON")),
                            un_checked_children: Some(rsx!("OFF")),
                        }
                        span { "英文文字" }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "通知设置：" }
                        div {
                            style: "display: flex; align-items: center; justify-content: space-between;",
                            span { "接收邮件通知" }
                            Switch {
                                default_checked: true,
                            }
                        }
                        div {
                            style: "display: flex; align-items: center; justify-content: space-between;",
                            span { "接收短信通知" }
                            Switch {}
                        }
                        div {
                            style: "display: flex; align-items: center; justify-content: space-between;",
                            span { "接收推送通知" }
                            Switch {
                                default_checked: true,
                            }
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
