//! List 组件演示
//!
//! 展示 List 组件的基础用法和高级用法，包括：
//! - 基础列表
//! - 带头部和底部的列表
//! - 加载状态
//! - 空状态
//! - 分页

use adui_dioxus::{
    Button, ButtonType, ComponentSize, List, Pagination, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ListDemo {}
        }
    }
}

#[component]
fn ListDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let current_page = use_signal(|| 1u32);
    let page_size = use_signal(|| 10u32);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let items = (1..=20)
        .map(|i| format!("列表项 {}", i))
        .collect::<Vec<_>>();

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

            // 基础列表
            DemoSection {
                title: "基础列表",
                List {
                    {
                        items.iter().take(5).map(|item| {
                            let item_str = item.clone();
                            rsx! {
                                div {
                                    class: "adui-list-item",
                                    style: "padding: 12px 0;",
                                    {item_str}
                                }
                            }
                        })
                    }
                }
            }

            // 带边框的列表
            DemoSection {
                title: "带边框的列表",
                List {
                    bordered: true,
                    {
                        items.iter().take(5).map(|item| {
                            let item_str = item.clone();
                            rsx! {
                                div {
                                    class: "adui-list-item",
                                    style: "padding: 12px 0;",
                                    {item_str}
                                }
                            }
                        })
                    }
                }
            }

            // 带头部和底部
            DemoSection {
                title: "带头部和底部的列表",
                List {
                    header: Some(rsx!(
                        div {
                            style: "font-weight: 600; padding: 8px 0;",
                            "列表头部"
                        }
                    )),
                    footer: Some(rsx!(
                        div {
                            style: "padding: 8px 0; color: var(--adui-color-text-secondary);",
                            "列表底部"
                        }
                    )),
                    bordered: true,
                    {
                        items.iter().take(5).map(|item| {
                            let item_str = item.clone();
                            rsx! {
                                div {
                                    class: "adui-list-item",
                                    style: "padding: 12px 0;",
                                    {item_str}
                                }
                            }
                        })
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 加载状态
            DemoSection {
                title: "加载状态",
                List {
                    loading: true,
                    bordered: true,
                    {
                        items.iter().take(5).map(|item| {
                            let item_str = item.clone();
                            rsx! {
                                div {
                                    class: "adui-list-item",
                                    style: "padding: 12px 0;",
                                    {item_str}
                                }
                            }
                        })
                    }
                }
            }

            // 空状态
            DemoSection {
                title: "空状态",
                List {
                    is_empty: Some(true),
                    bordered: true,
                    div { style: "display: none;", "" }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "Small 尺寸" }
                        List {
                            size: Some(ComponentSize::Small),
                            bordered: true,
                            {
                                items.iter().take(3).map(|item| {
                                    let item_str = item.clone();
                                    rsx! {
                                        div {
                                            class: "adui-list-item",
                                            style: "padding: 12px 0;",
                                            {item_str}
                                        }
                                    }
                                })
                            }
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "Middle 尺寸（默认）" }
                        List {
                            size: Some(ComponentSize::Middle),
                            bordered: true,
                            {
                                items.iter().take(3).map(|item| {
                                    let item_str = item.clone();
                                    rsx! {
                                        div {
                                            class: "adui-list-item",
                                            style: "padding: 12px 0;",
                                            {item_str}
                                        }
                                    }
                                })
                            }
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "Large 尺寸" }
                        List {
                            size: Some(ComponentSize::Large),
                            bordered: true,
                            {
                                items.iter().take(3).map(|item| {
                                    let item_str = item.clone();
                                    rsx! {
                                        div {
                                            class: "adui-list-item",
                                            style: "padding: 12px 0;",
                                            {item_str}
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }

            // 分页
            DemoSection {
                title: "分页列表",
                List {
                    bordered: true,
                    pagination_total: Some(100),
                    pagination_current: Some(*current_page.read()),
                    pagination_page_size: Some(*page_size.read()),
                    pagination_on_change: {
                        let mut page = current_page;
                        let mut size = page_size;
                        move |(p, s)| {
                            page.set(p);
                            size.set(s);
                        }
                    },
                    {
                        let start = ((*current_page.read() - 1) * *page_size.read()) as usize;
                        let end = (start + *page_size.read() as usize).min(items.len());
                        items.iter().skip(start).take(end - start).map(|item| {
                            let item_str = item.clone();
                            rsx! {
                                div {
                                    class: "adui-list-item",
                                    style: "padding: 12px 0;",
                                    {item_str}
                                }
                            }
                        })
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                List {
                    header: Some(rsx!(
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0;",
                            span { style: "font-weight: 600;", "任务列表" }
                            Button {
                                r#type: ButtonType::Text,
                                size: adui_dioxus::ButtonSize::Small,
                                "添加"
                            }
                        }
                    )),
                    bordered: true,
                    {
                        items.iter().take(5).map(|item| {
                            let item_str = item.clone();
                            rsx! {
                                div {
                                    class: "adui-list-item",
                                    style: "padding: 12px 0; display: flex; justify-content: space-between; align-items: center;",
                                    span { {item_str} }
                                    Button {
                                        r#type: ButtonType::Text,
                                        size: adui_dioxus::ButtonSize::Small,
                                        "操作"
                                    }
                                }
                            }
                        })
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
