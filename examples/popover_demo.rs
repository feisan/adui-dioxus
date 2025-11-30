//! Popover 组件演示
//!
//! 展示 Popover 组件的基础用法和高级用法，包括：
//! - 基础气泡卡片
//! - 不同触发方式
//! - 不同位置
//! - 受控模式

use adui_dioxus::{
    App, Button, ButtonType, Popover, ThemeMode, ThemeProvider, Title, TitleLevel,
    TooltipPlacement, TooltipTrigger, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {
                PopoverDemo {}
            }
        }
    }
}

#[component]
fn PopoverDemo() -> Element {
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

            // 基础气泡卡片
            DemoSection {
                title: "基础气泡卡片",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: flex-start;",
                    Popover {
                        title: Some(rsx! { b { "标题" } }),
                        content: Some(rsx! { p { "这是一个简单的 Popover 内容区域，可以放任意元素。" } }),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Click Popover"
                            }
                        },
                    }
                }
            }

            // 不同触发方式
            DemoSection {
                title: "不同触发方式",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: flex-start;",
                    Popover {
                        title: Some(rsx! { b { "Click 触发" } }),
                        content: Some(rsx! { p { "点击按钮时展示的 Popover。" } }),
                        trigger: TooltipTrigger::Click,
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Click"
                            }
                        },
                    }
                    Popover {
                        title: Some(rsx! { b { "Hover 触发" } }),
                        content: Some(rsx! { p { "鼠标悬停时展示的 Popover。" } }),
                        trigger: TooltipTrigger::Hover,
                        placement: Some(TooltipPlacement::Right),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Hover"
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
                    Popover {
                        title: Some(rsx! { b { "Top" } }),
                        content: Some(rsx! { p { "顶部位置" } }),
                        placement: Some(TooltipPlacement::Top),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Top"
                            }
                        },
                    }
                    Popover {
                        title: Some(rsx! { b { "Right" } }),
                        content: Some(rsx! { p { "右侧位置" } }),
                        placement: Some(TooltipPlacement::Right),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Right"
                            }
                        },
                    }
                    Popover {
                        title: Some(rsx! { b { "Bottom" } }),
                        content: Some(rsx! { p { "底部位置" } }),
                        placement: Some(TooltipPlacement::Bottom),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Bottom"
                            }
                        },
                    }
                    Popover {
                        title: Some(rsx! { b { "Left" } }),
                        content: Some(rsx! { p { "左侧位置" } }),
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
                            "打开 Popover"
                        }
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let mut sig = controlled_open;
                                move |_| sig.set(false)
                            },
                            "关闭 Popover"
                        }
                    }
                    Popover {
                        title: Some(rsx! { b { "受控 Popover" } }),
                        content: Some(rsx! { p { "由外部按钮完全控制 open 状态。" } }),
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
