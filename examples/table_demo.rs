//! Table 组件演示
//!
//! 展示 Table 组件的基础用法和高级用法，包括：
//! - 基础表格
//! - 带边框
//! - 不同尺寸
//! - 加载状态
//! - 空状态

use adui_dioxus::{
    Button, ButtonType, Table, TableColumn, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::config_provider::ComponentSize, use_theme,
};
use dioxus::prelude::*;
use serde_json::json;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TableDemo {}
        }
    }
}

#[component]
fn TableDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let loading = use_signal(|| false);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let columns = vec![
        TableColumn::new("name", "姓名"),
        TableColumn::new("age", "年龄"),
        TableColumn::new("city", "城市"),
        TableColumn::new("email", "邮箱"),
    ];

    let data = vec![
        json!({ "name": "Alice", "age": 28, "city": "上海", "email": "alice@example.com" }),
        json!({ "name": "Bob", "age": 35, "city": "北京", "email": "bob@example.com" }),
        json!({ "name": "Charlie", "age": 42, "city": "深圳", "email": "charlie@example.com" }),
        json!({ "name": "David", "age": 30, "city": "杭州", "email": "david@example.com" }),
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
                span { style: "margin-left: 16px; font-weight: 600;", "加载状态：" }
                Button {
                    r#type: if *loading.read() { ButtonType::Primary } else { ButtonType::Default },
                    onclick: {
                        let mut sig = loading;
                        move |_| {
                            let current = *sig.read();
                            sig.set(!current);
                        }
                    },
                    {
                        if *loading.read() {
                            "停止加载"
                        } else {
                            "开始加载"
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础表格
            DemoSection {
                title: "基础表格",
                Table {
                    columns: columns.clone(),
                    data: data.clone(),
                }
            }

            // 带边框
            DemoSection {
                title: "带边框",
                Table {
                    columns: columns.clone(),
                    data: data.clone(),
                    bordered: true,
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "Small 尺寸："
                        }
                        Table {
                            columns: columns.clone(),
                            data: data.clone(),
                            size: Some(ComponentSize::Small),
                            bordered: true,
                        }
                    }
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "Middle 尺寸（默认）："
                        }
                        Table {
                            columns: columns.clone(),
                            data: data.clone(),
                            size: Some(ComponentSize::Middle),
                            bordered: true,
                        }
                    }
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "Large 尺寸："
                        }
                        Table {
                            columns: columns.clone(),
                            data: data.clone(),
                            size: Some(ComponentSize::Large),
                            bordered: true,
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 加载状态
            DemoSection {
                title: "加载状态",
                Table {
                    columns: columns.clone(),
                    data: data.clone(),
                    loading: *loading.read(),
                    bordered: true,
                }
            }

            // 空状态
            DemoSection {
                title: "空状态",
                Table {
                    columns: columns.clone(),
                    data: vec![],
                    is_empty: Some(true),
                    bordered: true,
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
