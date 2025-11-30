//! 导航组件综合演示
//!
//! 展示所有导航组件的综合使用，包括：
//! - Menu 菜单
//! - Tabs 标签页
//! - Steps 步骤条
//! - 组合使用场景

use adui_dioxus::{
    Icon, IconKind, Layout, Menu, MenuItemNode, MenuMode, Sider, SiderTheme, StepItem, StepStatus,
    Steps, TabItem, Tabs, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            NavigationComponentsDemo {}
        }
    }
}

#[component]
fn NavigationComponentsDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let selected_keys = use_signal(|| vec!["1".to_string()]);
    let active_tab = use_signal(|| "1".to_string());

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let menu_items = vec![
        MenuItemNode {
            id: "1".into(),
            label: "首页".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Info
            })),
            disabled: false,
            children: None,
        },
        MenuItemNode {
            id: "2".into(),
            label: "产品".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Search
            })),
            disabled: false,
            children: None,
        },
        MenuItemNode {
            id: "3".into(),
            label: "关于".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Edit
            })),
            disabled: false,
            children: None,
        },
    ];

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",

            // 控制工具栏
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 24px; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border);",
                span { style: "font-weight: 600;", "主题控制：" }
                button {
                    style: "padding: 4px 12px; border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); background: var(--adui-color-bg-container); cursor: pointer;",
                    onclick: move |_| *mode.write() = ThemeMode::Light,
                    "Light"
                }
                button {
                    style: "padding: 4px 12px; border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); background: var(--adui-color-bg-container); cursor: pointer;",
                    onclick: move |_| *mode.write() = ThemeMode::Dark,
                    "Dark"
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "导航组件综合展示" }

            // 完整布局示例
            DemoSection {
                title: "完整布局示例（Layout + Sider + Menu + Tabs）",
                Layout {
                    has_sider: Some(true),
                    Sider {
                        theme: SiderTheme::Dark,
                        width: Some(200.0),
                        div {
                            style: "padding: 16px;",
                            Menu {
                                items: menu_items.clone(),
                                mode: MenuMode::Inline,
                                selected_keys: Some(selected_keys.read().clone()),
                                on_select: {
                                    let mut sig = selected_keys;
                                    move |key: String| {
                                        sig.set(vec![key]);
                                    }
                                },
                            }
                        }
                    }
                    Layout {
                        div {
                            style: "padding: 24px;",
                            Tabs {
                                items: vec![
                                    TabItem::new("1", "标签页 1", Some(rsx!("内容 1"))),
                                    TabItem::new("2", "标签页 2", Some(rsx!("内容 2"))),
                                    TabItem::new("3", "标签页 3", Some(rsx!("内容 3"))),
                                ],
                                active_key: Some(active_tab.read().clone()),
                                on_change: {
                                    let mut sig = active_tab;
                                    move |key: String| {
                                        sig.set(key);
                                    }
                                },
                            }
                        }
                    }
                }
            }

            // Steps 步骤条
            DemoSection {
                title: "Steps 步骤条",
                Steps {
                    current: Some(1usize),
                    items: vec![
                        StepItem {
                            key: "1".into(),
                            title: rsx!("步骤 1"),
                            description: Some(rsx!("完成基本信息")),
                            status: Some(StepStatus::Finish),
                            disabled: false,
                        },
                        StepItem {
                            key: "2".into(),
                            title: rsx!("步骤 2"),
                            description: Some(rsx!("进行中")),
                            status: Some(StepStatus::Process),
                            disabled: false,
                        },
                        StepItem {
                            key: "3".into(),
                            title: rsx!("步骤 3"),
                            description: Some(rsx!("等待处理")),
                            status: Some(StepStatus::Wait),
                            disabled: false,
                        },
                    ],
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "顶部导航菜单" }
                        Menu {
                            items: menu_items.clone(),
                            mode: MenuMode::Horizontal,
                            selected_keys: Some(selected_keys.read().clone()),
                            on_select: {
                                let mut sig = selected_keys;
                                move |key: String| {
                                    sig.set(vec![key]);
                                }
                            },
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "标签页导航" }
                        Tabs {
                            items: vec![
                                TabItem::new("1", "首页", Some(rsx!("首页内容"))),
                                TabItem::new("2", "产品", Some(rsx!("产品内容"))),
                                TabItem::new("3", "关于", Some(rsx!("关于内容"))),
                            ],
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
