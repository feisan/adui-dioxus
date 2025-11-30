//! Menu 组件演示
//!
//! 展示 Menu 组件的基础用法和高级用法，包括：
//! - 水平菜单
//! - 垂直菜单
//! - 子菜单
//! - 选中状态
//! - 折叠状态

use adui_dioxus::{
    Button, ButtonType, Icon, IconKind, Menu, MenuItemNode, MenuMode, ThemeMode, ThemeProvider,
    Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            MenuDemo {}
        }
    }
}

#[component]
fn MenuDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let selected_keys = use_signal(|| vec!["1".to_string()]);
    let open_keys = use_signal(|| vec!["sub1".to_string()]);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let menu_items = vec![
        MenuItemNode {
            id: "1".into(),
            label: "导航一".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Info
            })),
            disabled: false,
            children: None,
        },
        MenuItemNode {
            id: "2".into(),
            label: "导航二".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Search
            })),
            disabled: false,
            children: None,
        },
        MenuItemNode {
            id: "sub1".into(),
            label: "导航三".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Edit
            })),
            disabled: false,
            children: Some(vec![
                MenuItemNode {
                    id: "3".into(),
                    label: "选项 1".into(),
                    icon: None,
                    disabled: false,
                    children: None,
                },
                MenuItemNode {
                    id: "4".into(),
                    label: "选项 2".into(),
                    icon: None,
                    disabled: false,
                    children: None,
                },
            ]),
        },
        MenuItemNode {
            id: "5".into(),
            label: "导航四".into(),
            icon: Some(rsx!(Icon {
                kind: IconKind::Copy
            })),
            disabled: true,
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

            // 水平菜单
            DemoSection {
                title: "水平菜单",
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

            // 垂直菜单
            DemoSection {
                title: "垂直菜单",
                div {
                    style: "width: 200px;",
                    Menu {
                        items: menu_items.clone(),
                        mode: MenuMode::Inline,
                        selected_keys: Some(selected_keys.read().clone()),
                        open_keys: Some(open_keys.read().clone()),
                        on_select: {
                            let mut sig = selected_keys;
                            move |key: String| {
                                sig.set(vec![key]);
                            }
                        },
                        on_open_change: {
                            let mut sig = open_keys;
                            move |keys| {
                                sig.set(keys);
                            }
                        },
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 默认选中
            DemoSection {
                title: "默认选中",
                div {
                    style: "width: 200px;",
                    Menu {
                        items: menu_items.clone(),
                        mode: MenuMode::Inline,
                        default_selected_keys: Some(vec!["2".to_string()]),
                        default_open_keys: Some(vec!["sub1".to_string()]),
                    }
                }
            }

            // 折叠菜单
            DemoSection {
                title: "折叠菜单",
                div {
                    style: "width: 200px;",
                    Menu {
                        items: menu_items.clone(),
                        mode: MenuMode::Inline,
                        inline_collapsed: true,
                    }
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
                        span { style: "font-weight: 600;", "侧边栏菜单" }
                        div {
                            style: "width: 200px; border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 8px;",
                            Menu {
                                items: menu_items.clone(),
                                mode: MenuMode::Inline,
                                selected_keys: Some(selected_keys.read().clone()),
                                open_keys: Some(open_keys.read().clone()),
                        on_select: {
                            let mut sig = selected_keys;
                            move |key: String| {
                                sig.set(vec![key]);
                            }
                        },
                                on_open_change: {
                                    let mut sig = open_keys;
                                    move |keys| {
                                        sig.set(keys);
                                    }
                                },
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
