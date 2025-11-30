//! Transfer 组件演示
//!
//! 展示 Transfer 组件的基础用法和高级用法，包括：
//! - 基础穿梭框
//! - 搜索功能
//! - 单向模式
//! - 带描述

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::transfer::{Transfer, TransferDirection, TransferItem},
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TransferDemo {}
        }
    }
}

#[component]
fn TransferDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let target_keys = use_signal(|| vec!["3".to_string()]);
    let searchable_target = use_signal(|| Vec::<String>::new());
    let one_way_target = use_signal(|| Vec::<String>::new());
    let desc_target = use_signal(|| vec!["react".to_string()]);

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

            // 基础穿梭框
            DemoSection {
                title: "基础穿梭框",
                Transfer {
                    data_source: vec![
                        TransferItem::new("1", "Item 1"),
                        TransferItem::new("2", "Item 2"),
                        TransferItem::new("3", "Item 3"),
                        TransferItem::new("4", "Item 4"),
                        TransferItem::new("5", "Item 5"),
                    ],
                    target_keys: target_keys.read().clone(),
                    titles: ("Source".into(), "Target".into()),
                    on_change: {
                        let mut sig = target_keys;
                        move |(keys, _direction, _moved): (Vec<String>, TransferDirection, Vec<String>)| {
                            sig.set(keys);
                        }
                    },
                }
            }

            // 搜索功能
            DemoSection {
                title: "搜索功能",
                Transfer {
                    data_source: (1..=10)
                        .map(|i| TransferItem::new(format!("{}", i), format!("Content {}", i)))
                        .collect(),
                    target_keys: searchable_target.read().clone(),
                    show_search: true,
                    search_placeholder: Some("Search items...".to_string()),
                    on_change: {
                        let mut sig = searchable_target;
                        move |(keys, _, _): (Vec<String>, TransferDirection, Vec<String>)| {
                            sig.set(keys);
                        }
                    },
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 单向模式
            DemoSection {
                title: "单向模式",
                div {
                    style: "margin-bottom: 12px; color: var(--adui-color-text-secondary); font-size: 14px;",
                    "Items can only be moved from left to right."
                }
                Transfer {
                    data_source: vec![
                        TransferItem::new("a", "Apple"),
                        TransferItem::new("b", "Banana"),
                        TransferItem::new("c", "Cherry"),
                        TransferItem::new("d", "Date"),
                    ],
                    target_keys: one_way_target.read().clone(),
                    one_way: true,
                    titles: ("Available".into(), "Selected".into()),
                    operations: ("Add →".into(), "".into()),
                    on_change: {
                        let mut sig = one_way_target;
                        move |(keys, _, _): (Vec<String>, TransferDirection, Vec<String>)| {
                            sig.set(keys);
                        }
                    },
                }
            }

            // 带描述
            DemoSection {
                title: "带描述",
                Transfer {
                    data_source: vec![
                        TransferItem::new("react", "React")
                            .with_description("A JavaScript library for building UIs"),
                        TransferItem::new("vue", "Vue").with_description("The progressive JavaScript framework"),
                        TransferItem::new("angular", "Angular")
                            .with_description("Platform for building mobile & desktop apps"),
                        TransferItem::new("svelte", "Svelte").with_description("Cybernetically enhanced web apps"),
                        TransferItem::new("solid", "Solid").with_disabled(true),
                    ],
                    target_keys: desc_target.read().clone(),
                    show_search: true,
                    titles: ("Frameworks".into(), "My Stack".into()),
                    on_change: {
                        let mut sig = desc_target;
                        move |(keys, _, _): (Vec<String>, TransferDirection, Vec<String>)| {
                            sig.set(keys);
                        }
                    },
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
