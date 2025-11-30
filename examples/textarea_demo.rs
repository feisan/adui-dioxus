//! TextArea 组件演示
//!
//! 展示 TextArea 组件的基础用法和高级用法，包括：
//! - 基础多行输入
//! - 自动高度
//! - 字数统计
//! - 禁用状态

use adui_dioxus::{
    Button, ButtonType, TextArea, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TextAreaDemo {}
        }
    }
}

#[component]
fn TextAreaDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let textarea_value = use_signal(|| String::new());

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

            // 基础多行输入
            DemoSection {
                title: "基础多行输入",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    TextArea {
                        placeholder: Some("请输入内容".into()),
                        rows: Some(4),
                        on_change: {
                            let mut sig = textarea_value;
                            move |value| sig.set(value)
                        },
                    }
                    TextArea {
                        placeholder: Some("默认值示例".into()),
                        rows: Some(4),
                        default_value: Some("这是默认内容\n可以多行显示".into()),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 不同行数
            DemoSection {
                title: "不同行数",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    TextArea {
                        placeholder: Some("2行".into()),
                        rows: Some(2),
                    }
                    TextArea {
                        placeholder: Some("4行（默认）".into()),
                        rows: Some(4),
                    }
                    TextArea {
                        placeholder: Some("6行".into()),
                        rows: Some(6),
                    }
                }
            }

            // 字数统计
            DemoSection {
                title: "字数统计",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    TextArea {
                        placeholder: Some("显示字数统计（最大100字符）".into()),
                        rows: Some(4),
                        max_length: Some(100),
                        show_count: true,
                        default_value: Some("已输入的内容".into()),
                    }
                    TextArea {
                        placeholder: Some("字数统计（6行）".into()),
                        rows: Some(6),
                        max_length: Some(200),
                        show_count: true,
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    TextArea {
                        placeholder: Some("禁用状态".into()),
                        rows: Some(4),
                        disabled: true,
                        default_value: Some("禁用内容\n无法编辑".into()),
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    TextArea {
                        placeholder: Some("评论输入框（带字数统计）".into()),
                        rows: Some(4),
                        max_length: Some(500),
                        show_count: true,
                        default_value: Some("请输入您的评论...".into()),
                    }
                    TextArea {
                        placeholder: Some("大输入框（8行）".into()),
                        rows: Some(8),
                        max_length: Some(1000),
                        show_count: true,
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
