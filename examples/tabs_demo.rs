//! Tabs 组件演示
//!
//! 展示 Tabs 组件的基础用法和高级用法，包括：
//! - 基础标签页
//! - 不同类型（Line、Card、EditableCard）
//! - 不同位置（Top、Right、Bottom、Left）
//! - 可编辑标签页

use adui_dioxus::{
    Button, ButtonType, Icon, IconKind, TabItem, Tabs, TabsType, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TabsDemo {}
        }
    }
}

#[component]
fn TabsDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let active_key = use_signal(|| "1".to_string());
    let tabs = use_signal(|| {
        vec![
            TabItem::new("1", "标签页 1", Some(rsx!("内容 1"))),
            TabItem::new("2", "标签页 2", Some(rsx!("内容 2"))),
            TabItem::new("3", "标签页 3", Some(rsx!("内容 3"))),
        ]
    });

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

            // 基础标签页
            DemoSection {
                title: "基础标签页",
                Tabs {
                    items: vec![
                        TabItem::new("1", "标签页 1", Some(rsx!("这是标签页 1 的内容"))),
                        TabItem::new("2", "标签页 2", Some(rsx!("这是标签页 2 的内容"))),
                        TabItem::new("3", "标签页 3", Some(rsx!("这是标签页 3 的内容"))),
                    ],
                    active_key: Some(active_key.read().clone()),
                    on_change: {
                        let mut sig = active_key;
                        move |key| {
                            sig.set(key);
                        }
                    },
                }
            }

            // 不同类型
            DemoSection {
                title: "不同类型",
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "Line 类型（默认）" }
                        Tabs {
                            r#type: TabsType::Line,
                            items: vec![
                                TabItem::new("1", "标签页 1", Some(rsx!("内容 1"))),
                                TabItem::new("2", "标签页 2", Some(rsx!("内容 2"))),
                            ],
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "Card 类型" }
                        Tabs {
                            r#type: TabsType::Card,
                            items: vec![
                                TabItem::new("1", "标签页 1", Some(rsx!("内容 1"))),
                                TabItem::new("2", "标签页 2", Some(rsx!("内容 2"))),
                            ],
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 带图标的标签页
            DemoSection {
                title: "带图标的标签页",
                Tabs {
                    items: vec![
                        TabItem {
                            key: "1".into(),
                            label: "标签页 1".into(),
                            icon: Some(rsx!(Icon { kind: IconKind::Info })),
                            disabled: false,
                            closable: false,
                            content: Some(rsx!("内容 1")),
                        },
                        TabItem {
                            key: "2".into(),
                            label: "标签页 2".into(),
                            icon: Some(rsx!(Icon { kind: IconKind::Search })),
                            disabled: false,
                            closable: false,
                            content: Some(rsx!("内容 2")),
                        },
                        TabItem {
                            key: "3".into(),
                            label: "标签页 3".into(),
                            icon: Some(rsx!(Icon { kind: IconKind::Edit })),
                            disabled: false,
                            closable: false,
                            content: Some(rsx!("内容 3")),
                        },
                    ],
                }
            }

            // 禁用标签页
            DemoSection {
                title: "禁用标签页",
                Tabs {
                    items: vec![
                        TabItem::new("1", "标签页 1", Some(rsx!("内容 1"))),
                        TabItem::disabled("2", "标签页 2"),
                        TabItem::new("3", "标签页 3", Some(rsx!("内容 3"))),
                    ],
                }
            }

            // 可编辑标签页
            DemoSection {
                title: "可编辑标签页",
                Tabs {
                    r#type: TabsType::EditableCard,
                    items: tabs.read().clone(),
                    active_key: Some(active_key.read().clone()),
                    on_change: {
                        let mut sig = active_key;
                        move |key| {
                            sig.set(key);
                        }
                    },
                    on_edit: {
                        let mut t = tabs;
                        let mut a = active_key;
                        move |action| {
                            match action {
                                adui_dioxus::TabEditAction::Add => {
                                    let new_key = format!("{}", t.read().len() + 1);
                                    let new_tab = TabItem::new(
                                        new_key.clone(),
                                        format!("新标签页 {}", t.read().len() + 1),
                                        Some(rsx!("新内容")),
                                    );
                                    let mut current = t.read().clone();
                                    current.push(new_tab);
                                    t.set(current);
                                    a.set(new_key);
                                }
                                adui_dioxus::TabEditAction::Remove(key) => {
                                    let mut current = t.read().clone();
                                    current.retain(|tab| tab.key != key);
                                    t.set(current);
                                    if *a.read() == key {
                                        a.set("1".to_string());
                                    }
                                }
                            }
                        }
                    },
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                Tabs {
                    r#type: TabsType::Card,
                    items: vec![
                        TabItem {
                            key: "1".into(),
                            label: "首页".into(),
                            icon: Some(rsx!(Icon { kind: IconKind::Info })),
                            disabled: false,
                            closable: false,
                            content: Some(rsx!(
                                div {
                                    style: "padding: 24px;",
                                    "这是首页内容"
                                }
                            )),
                        },
                        TabItem {
                            key: "2".into(),
                            label: "产品".into(),
                            icon: Some(rsx!(Icon { kind: IconKind::Search })),
                            disabled: false,
                            closable: false,
                            content: Some(rsx!(
                                div {
                                    style: "padding: 24px;",
                                    "这是产品页面内容"
                                }
                            )),
                        },
                        TabItem {
                            key: "3".into(),
                            label: "关于".into(),
                            icon: Some(rsx!(Icon { kind: IconKind::Edit })),
                            disabled: false,
                            closable: false,
                            content: Some(rsx!(
                                div {
                                    style: "padding: 24px;",
                                    "这是关于页面内容"
                                }
                            )),
                        },
                    ],
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
