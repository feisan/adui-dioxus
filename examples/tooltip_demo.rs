//! Tooltip 组件演示
//!
//! 展示 Tooltip 组件的基础用法和高级用法，包括：
//! - 基础提示
//! - 不同触发方式
//! - 不同位置
//! - 受控模式

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel, Tooltip, TooltipPlacement,
    TooltipTrigger, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TooltipDemo {}
        }
    }
}

#[component]
fn TooltipDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let mut controlled_open = use_signal(|| false);

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

            // 基础提示
            DemoSection {
                title: "基础提示",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                    Tooltip {
                        title: Some("默认 hover 提示".to_string()),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Hover me"
                            }
                        },
                    }
                    Tooltip {
                        title: Some("这是一个很长的提示文字，用来测试 Tooltip 的自动换行和最大宽度限制。".to_string()),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "长文本提示"
                            }
                        },
                    }
                }
            }

            // 不同触发方式
            DemoSection {
                title: "不同触发方式",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                    Tooltip {
                        title: Some("Hover 触发（默认）".to_string()),
                        trigger: TooltipTrigger::Hover,
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Hover"
                            }
                        },
                    }
                    Tooltip {
                        title: Some("Click 触发".to_string()),
                        trigger: TooltipTrigger::Click,
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Click"
                            }
                        },
                    }
                }
            }

            // 不同位置
            DemoSection {
                title: "不同位置",
                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; max-width: 600px;",
                    Tooltip {
                        title: Some("Top".to_string()),
                        placement: Some(TooltipPlacement::Top),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Top"
                            }
                        },
                    }
                    Tooltip {
                        title: Some("Right".to_string()),
                        placement: Some(TooltipPlacement::Right),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Right"
                            }
                        },
                    }
                    Tooltip {
                        title: Some("Bottom".to_string()),
                        placement: Some(TooltipPlacement::Bottom),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Bottom"
                            }
                        },
                    }
                    Tooltip {
                        title: Some("Left".to_string()),
                        placement: Some(TooltipPlacement::Left),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Left"
                            }
                        },
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 受控模式
            DemoSection {
                title: "受控模式",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Button {
                            r#type: ButtonType::Primary,
                            onclick: {
                                let mut sig = controlled_open;
                                move |_| sig.set(true)
                            },
                            "打开 Tooltip"
                        }
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let mut sig = controlled_open;
                                move |_| sig.set(false)
                            },
                            "关闭 Tooltip"
                        }
                    }
                    Tooltip {
                        title: Some("受控 Tooltip（由外部按钮控制 open）".to_string()),
                        trigger: TooltipTrigger::Click,
                        open: Some(*controlled_open.read()),
                        on_open_change: {
                            let mut sig = controlled_open;
                            move |next: bool| sig.set(next)
                        },
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Controlled"
                            }
                        },
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
