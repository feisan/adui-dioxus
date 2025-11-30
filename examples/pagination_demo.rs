//! Pagination 组件演示
//!
//! 展示 Pagination 组件的基础用法和高级用法，包括：
//! - 基础分页
//! - 显示总数
//! - 页面大小切换
//! - 简单模式
//! - 受控模式

use adui_dioxus::{
    Button, ButtonType, Pagination, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            PaginationDemo {}
        }
    }
}

#[component]
fn PaginationDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let current = use_signal(|| 1u32);
    let page_size = use_signal(|| 10u32);

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

            // 基础分页
            DemoSection {
                title: "基础分页",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Pagination {
                        total: 100,
                        current: Some(*current.read()),
                        page_size: Some(*page_size.read()),
                        on_change: {
                            let mut sig_current = current;
                            let mut sig_page_size = page_size;
                            move |(page, size)| {
                                sig_current.set(page);
                                sig_page_size.set(size);
                            }
                        },
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前页: ",
                        {current.read().to_string()},
                        "，每页条数: ",
                        {page_size.read().to_string()}
                    }
                }
            }

            // 显示总数
            DemoSection {
                title: "显示总数",
                Pagination {
                    total: 200,
                    current: Some(5),
                    page_size: Some(10),
                    show_total: true,
                }
            }

            // 页面大小切换
            DemoSection {
                title: "页面大小切换",
                Pagination {
                    total: 200,
                    current: Some(*current.read()),
                    page_size: Some(*page_size.read()),
                    show_size_changer: true,
                    on_change: {
                        let mut sig_current = current;
                        let mut sig_page_size = page_size;
                        move |(page, size)| {
                            sig_current.set(page);
                            sig_page_size.set(size);
                        }
                    },
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 受控模式
            DemoSection {
                title: "受控模式",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Pagination {
                        total: 200,
                        current: Some(*current.read()),
                        page_size: Some(*page_size.read()),
                        show_total: true,
                        show_size_changer: true,
                        on_change: {
                            let mut sig_current = current;
                            let mut sig_page_size = page_size;
                            move |(page, size)| {
                                sig_current.set(page);
                                sig_page_size.set(size);
                            }
                        },
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前页: ",
                        {current.read().to_string()},
                        "，每页条数: ",
                        {page_size.read().to_string()}
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
