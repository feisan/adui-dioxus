//! Divider 组件演示
//!
//! 展示 Divider 组件的基础用法和高级用法，包括：
//! - 水平分割线
//! - 垂直分割线
//! - 文字分割线
//! - 虚线样式
//! - 方向对齐

use adui_dioxus::{
    Button, ButtonType, Divider, DividerOrientation, ThemeMode, ThemeProvider, Title, TitleLevel,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DividerDemo {}
        }
    }
}

#[component]
fn DividerDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);

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

            // 水平分割线
            DemoSection {
                title: "水平分割线",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    p { "分割线上方的内容" }
                    Divider {}
                    p { "分割线下方的内容" }
                }
            }

            // 垂直分割线
            DemoSection {
                title: "垂直分割线",
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    span { "左侧内容" }
                    Divider { vertical: true }
                    span { "中间内容" }
                    Divider { vertical: true }
                    span { "右侧内容" }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 文字分割线
            DemoSection {
                title: "文字分割线",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Divider {
                        orientation: DividerOrientation::Left,
                        content: Some(rsx!("左侧文字")),
                    }
                    Divider {
                        orientation: DividerOrientation::Center,
                        content: Some(rsx!("中间文字")),
                    }
                    Divider {
                        orientation: DividerOrientation::Right,
                        content: Some(rsx!("右侧文字")),
                    }
                }
            }

            // 虚线样式
            DemoSection {
                title: "虚线样式",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    p { "实线分割线（默认）" }
                    Divider {}
                    p { "虚线分割线" }
                    Divider { dashed: true }
                    Divider {
                        dashed: true,
                        orientation: DividerOrientation::Center,
                        content: Some(rsx!("虚线文字分割线")),
                    }
                }
            }

            // 方向对齐
            DemoSection {
                title: "方向对齐",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Divider {
                        orientation: DividerOrientation::Left,
                        orientation_margin: Some("0px".into()),
                        content: Some(rsx!("左对齐（无边距）")),
                    }
                    Divider {
                        orientation: DividerOrientation::Left,
                        orientation_margin: Some("24px".into()),
                        content: Some(rsx!("左对齐（24px边距）")),
                    }
                    Divider {
                        orientation: DividerOrientation::Right,
                        orientation_margin: Some("0px".into()),
                        content: Some(rsx!("右对齐（无边距）")),
                    }
                    Divider {
                        orientation: DividerOrientation::Right,
                        orientation_margin: Some("24px".into()),
                        content: Some(rsx!("右对齐（24px边距）")),
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
                        h3 { style: "margin: 0; font-size: 16px; font-weight: 600;", "文章标题" }
                        span { style: "color: var(--adui-color-text-secondary); font-size: 14px;", "2024-01-01" }
                        Divider { style: Some("margin: 8px 0;".into()) }
                        p { style: "margin: 0;", "这是文章内容的第一段。分割线用于区分标题和正文。" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Button { r#type: ButtonType::Primary, "确定" }
                        Divider { vertical: true }
                        Button { r#type: ButtonType::Default, "取消" }
                        Divider { vertical: true }
                        Button { r#type: ButtonType::Text, "更多" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        Divider {
                            dashed: true,
                            orientation: DividerOrientation::Center,
                            content: Some(rsx!("章节分隔")),
                        }
                        p { style: "margin: 0;", "使用虚线分割线来分隔不同的章节或内容区域。" }
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
