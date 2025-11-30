//! Segmented 组件演示
//!
//! 展示 Segmented 组件的基础用法和高级用法，包括：
//! - 基础分段控制器
//! - Block模式
//! - 禁用状态
//! - 带图标

use adui_dioxus::{
    Button, ButtonType, Icon, IconKind, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::segmented::{Segmented, SegmentedOption},
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SegmentedDemo {}
        }
    }
}

#[component]
fn SegmentedDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let value = use_signal(|| Some("daily".to_string()));
    let icon_value = use_signal(|| Some("list".to_string()));

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let options = vec![
        SegmentedOption {
            label: "日".into(),
            value: "daily".into(),
            icon: None,
            tooltip: Some("每日".into()),
            disabled: false,
        },
        SegmentedOption {
            label: "周".into(),
            value: "weekly".into(),
            icon: None,
            tooltip: Some("每周".into()),
            disabled: false,
        },
        SegmentedOption {
            label: "月".into(),
            value: "monthly".into(),
            icon: None,
            tooltip: Some("每月".into()),
            disabled: false,
        },
    ];

    let icon_options = vec![
        SegmentedOption {
            label: "列表".into(),
            value: "list".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Search
            })),
            tooltip: None,
            disabled: false,
        },
        SegmentedOption {
            label: "网格".into(),
            value: "grid".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Info
            })),
            tooltip: None,
            disabled: false,
        },
        SegmentedOption {
            label: "卡片".into(),
            value: "card".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Edit
            })),
            tooltip: None,
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

            // 基础分段控制器
            DemoSection {
                title: "基础分段控制器",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Segmented {
                        options: options.clone(),
                        value: value.read().clone(),
                        on_change: {
                            let mut sig = value;
                            move |v| sig.set(Some(v))
                        },
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        {
                            let val = value.read();
                            val.as_ref().map(|v| v.as_str()).unwrap_or("(未选择)").to_string()
                        }
                    }
                }
            }

            // Block模式
            DemoSection {
                title: "Block模式",
                Segmented {
                    options: options.clone(),
                    value: value.read().clone(),
                    block: true,
                    on_change: {
                        let mut sig = value;
                        move |v| sig.set(Some(v))
                    },
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                Segmented {
                    options: options.clone(),
                    value: Some("daily".into()),
                    disabled: true,
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 带图标
            DemoSection {
                title: "带图标",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Segmented {
                        options: icon_options.clone(),
                        value: icon_value.read().clone(),
                        on_change: {
                            let mut sig = icon_value;
                            move |v| sig.set(Some(v))
                        },
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        {
                            let val = icon_value.read();
                            val.as_ref().map(|v| v.as_str()).unwrap_or("(未选择)").to_string()
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
