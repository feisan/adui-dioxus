//! Breadcrumb 组件演示
//!
//! 展示 Breadcrumb 组件的基础用法和高级用法，包括：
//! - 基础面包屑
//! - 自定义分隔符
//! - 点击事件
//! - 带图标

use adui_dioxus::{
    App, Breadcrumb, BreadcrumbItem, Button, ButtonType, Icon, IconKind, ThemeMode, ThemeProvider,
    Title, TitleLevel, use_message, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {
                BreadcrumbDemo {}
            }
        }
    }
}

fn demo_items() -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem::with_href("home", "首页", "/"),
        BreadcrumbItem::with_href("list", "列表", "/list"),
        BreadcrumbItem::new("detail", "详情"),
    ]
}

#[component]
fn BreadcrumbDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let api = use_message();

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

            // 基础面包屑
            DemoSection {
                title: "基础面包屑",
                Breadcrumb {
                    items: demo_items(),
                }
            }

            // 自定义分隔符
            DemoSection {
                title: "自定义分隔符",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Breadcrumb {
                        items: demo_items(),
                        separator: Some(" > ".to_string()),
                    }
                    Breadcrumb {
                        items: demo_items(),
                        separator: Some(" / ".to_string()),
                    }
                    Breadcrumb {
                        items: demo_items(),
                        separator: Some(" · ".to_string()),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 点击事件
            DemoSection {
                title: "点击事件",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Breadcrumb {
                        items: demo_items(),
                        separator: Some(" > ".to_string()),
                        on_item_click: {
                            let api = api.clone();
                            move |id: String| {
                                if let Some(msg) = api.clone() {
                                    msg.info(format!("点击了节点: {}", id));
                                }
                            }
                        },
                    }
                    span {
                        style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                        "点击前两级会触发 message 提示"
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
