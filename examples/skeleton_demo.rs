//! Skeleton 组件演示
//!
//! 展示 Skeleton 组件的基础用法和高级用法，包括：
//! - 基础骨架屏
//! - 包裹真实内容
//! - 动画效果
//! - 自定义段落行数

use adui_dioxus::{
    Button, ButtonType, Card, Skeleton, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SkeletonDemo {}
        }
    }
}

#[component]
fn SkeletonDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let loading = use_signal(|| true);

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

            // 基础骨架屏
            DemoSection {
                title: "基础骨架屏",
                Skeleton {}
            }

            // 包裹真实内容
            DemoSection {
                title: "包裹真实内容",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = loading;
                            move |_| {
                                let current = *sig.read();
                                sig.set(!current);
                            }
                        },
                        if *loading.read() { "加载完成" } else { "重新加载" }
                    }
                    Skeleton {
                        loading: Some(*loading.read()),
                        active: true,
                        paragraph_rows: Some(3),
                        content: Some(rsx! {
                            Card {
                                children: rsx! {
                                    div {
                                        style: "padding: 16px;",
                                        h4 { style: "margin: 0 0 8px 0;", "真实内容" }
                                        p {
                                            style: "margin: 0; color: var(--adui-color-text-secondary);",
                                            "当 loading=false 时，Skeleton 渲染子内容而不再显示骨架。"
                                        }
                                    }
                                }
                            }
                        }),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 动画效果
            DemoSection {
                title: "动画效果",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "有动画："
                        }
                        Skeleton {
                            active: true,
                            paragraph_rows: Some(2),
                        }
                    }
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "无动画："
                        }
                        Skeleton {
                            active: false,
                            paragraph_rows: Some(2),
                        }
                    }
                }
            }

            // 自定义段落行数
            DemoSection {
                title: "自定义段落行数",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "1行："
                        }
                        Skeleton { paragraph_rows: Some(1) }
                    }
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "3行："
                        }
                        Skeleton { paragraph_rows: Some(3) }
                    }
                    div {
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 8px; display: block;",
                            "5行："
                        }
                        Skeleton { paragraph_rows: Some(5) }
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
