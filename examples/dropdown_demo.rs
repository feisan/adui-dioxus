//! Dropdown 组件演示
//!
//! 展示 Dropdown 组件的基础用法和高级用法，包括：
//! - 基础下拉菜单
//! - 不同触发方式
//! - 不同位置
//! - 禁用项
//! - 点击事件

use adui_dioxus::{
    App, Button, ButtonType, Dropdown, DropdownItem, DropdownPlacement, DropdownTrigger, ThemeMode,
    ThemeProvider, Title, TitleLevel, use_message, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {
                DropdownDemo {}
            }
        }
    }
}

fn default_items() -> Vec<DropdownItem> {
    vec![
        DropdownItem::new("new", "新建文档"),
        DropdownItem::new("open", "打开..."),
        DropdownItem::new("share", "分享"),
        DropdownItem {
            key: "disabled".into(),
            label: "禁用项".into(),
            disabled: true,
        },
    ]
}

#[component]
fn DropdownDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let api = use_message();
    let last_click = use_signal(String::new);

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

            // 基础下拉菜单
            DemoSection {
                title: "基础下拉菜单",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: flex-start;",
                    Dropdown {
                        items: default_items(),
                        trigger: DropdownTrigger::Click,
                        placement: Some(DropdownPlacement::BottomLeft),
                        on_click: {
                            let mut sig = last_click;
                            let api = api.clone();
                            move |key: String| {
                                sig.set(key.clone());
                                if let Some(msg) = api.clone() {
                                    msg.info(format!("选择菜单项: {}", key));
                                }
                            }
                        },
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "基础下拉菜单"
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
                    Dropdown {
                        items: default_items(),
                        trigger: DropdownTrigger::Click,
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Click 触发"
                            }
                        },
                    }
                    Dropdown {
                        items: default_items(),
                        trigger: DropdownTrigger::Hover,
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Hover 触发"
                            }
                        },
                    }
                }
            }

            // 不同位置
            DemoSection {
                title: "不同位置",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: flex-start;",
                    Dropdown {
                        items: default_items(),
                        trigger: DropdownTrigger::Click,
                        placement: Some(DropdownPlacement::BottomLeft),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Bottom Left"
                            }
                        },
                    }
                    Dropdown {
                        items: default_items(),
                        trigger: DropdownTrigger::Click,
                        placement: Some(DropdownPlacement::BottomRight),
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Default,
                                "Bottom Right"
                            }
                        },
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 禁用项
            DemoSection {
                title: "禁用项",
                Dropdown {
                    items: default_items(),
                    trigger: DropdownTrigger::Click,
                    on_click: {
                        let mut sig = last_click;
                        let api = api.clone();
                        move |key: String| {
                            sig.set(key.clone());
                            if let Some(msg) = api.clone() {
                                msg.info(format!("选择菜单项: {}", key));
                            }
                        }
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "包含禁用项的下拉菜单"
                        }
                    },
                }
            }

            // 点击反馈
            DemoSection {
                title: "点击反馈",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Dropdown {
                        items: default_items(),
                        trigger: DropdownTrigger::Click,
                        on_click: {
                            let mut sig = last_click;
                            let api = api.clone();
                            move |key: String| {
                                sig.set(key.clone());
                                if let Some(msg) = api.clone() {
                                    msg.success(format!("已选择: {}", key));
                                }
                            }
                        },
                        children: rsx! {
                            Button {
                                r#type: ButtonType::Primary,
                                "点击查看反馈"
                            }
                        },
                    }
                    if !last_click.read().is_empty() {
                        div {
                            style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                            "最近一次点击的 key: ",
                            {last_click.read().clone()}
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
