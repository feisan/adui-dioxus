//! FloatButton 组件演示
//!
//! 展示 FloatButton 组件的基础用法和高级用法，包括：
//! - 基础悬浮按钮
//! - 不同形状
//! - 按钮组
//! - 带徽标
//! - 自定义位置

use adui_dioxus::{
    BackTop, BadgeConfig, Button, ButtonType, FloatButton, FloatButtonGroup, FloatButtonShape,
    FloatButtonType, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            FloatButtonDemo {}
        }
    }
}

#[component]
fn FloatButtonDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let show_secondary = use_signal(|| true);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 200vh; color: var(--adui-color-text);",

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
                span {
                    style: "margin-left: 16px; font-weight: 600;",
                    "副按钮："
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: {
                        let mut sig = show_secondary;
                        move |_| {
                            let current = *sig.read();
                            sig.set(!current);
                        }
                    },
                    {
                        if *show_secondary.read() {
                            "隐藏"
                        } else {
                            "显示"
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础悬浮按钮
            DemoSection {
                title: "基础悬浮按钮",
                div {
                    style: "border: 1px dashed var(--adui-color-border); padding: 12px; border-radius: var(--adui-radius); background: var(--adui-color-bg-container); min-height: 200px; position: relative;",
                    p { "尝试点击右下角的悬浮按钮。" }
                    FloatButton {
                        r#type: FloatButtonType::Primary,
                        shape: FloatButtonShape::Circle,
                        icon: rsx!(span { "＋" }),
                        tooltip: Some("快速创建".to_string()),
                        onclick: move |_| {
                            println!("primary float button clicked");
                        }
                    }
                }
            }

            // 不同形状
            DemoSection {
                title: "不同形状",
                div {
                    style: "border: 1px dashed var(--adui-color-border); padding: 12px; border-radius: var(--adui-radius); background: var(--adui-color-bg-container); min-height: 200px; position: relative;",
                    p { "圆形和方形悬浮按钮。" }
                    FloatButton {
                        r#type: FloatButtonType::Default,
                        shape: FloatButtonShape::Circle,
                        icon: rsx!(span { "○" }),
                        tooltip: Some("圆形按钮".to_string()),
                        right: Some(24.0),
                        bottom: Some(80.0),
                    }
                    FloatButton {
                        r#type: FloatButtonType::Default,
                        shape: FloatButtonShape::Square,
                        icon: rsx!(span { "□" }),
                        content: Some("方形".to_string()),
                        tooltip: Some("方形按钮".to_string()),
                        right: Some(24.0),
                        bottom: Some(24.0),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 按钮组
            DemoSection {
                title: "按钮组",
                div {
                    style: "border: 1px dashed var(--adui-color-border); padding: 12px; border-radius: var(--adui-radius); background: var(--adui-color-bg-container); min-height: 200px; position: relative;",
                    p { "主/副浮动按钮组，副按钮可在上方控制开关。" }
                    FloatButtonGroup {
                        right: Some(24.0),
                        bottom: Some(120.0),
                        FloatButton {
                            r#type: FloatButtonType::Primary,
                            shape: FloatButtonShape::Circle,
                            icon: rsx!(span { "＋" }),
                            tooltip: Some("快速创建".to_string()),
                            onclick: move |_| {
                                println!("primary float button clicked");
                            }
                        }
                        if *show_secondary.read() {
                            FloatButton {
                                r#type: FloatButtonType::Default,
                                shape: FloatButtonShape::Square,
                                icon: rsx!(span { "?" }),
                                content: Some("帮助".to_string()),
                                tooltip: Some("查看帮助".to_string()),
                                badge: Some(BadgeConfig { content: Some("New".to_string()), dot: false, class: None }),
                                onclick: move |_| {
                                    println!("secondary float button clicked");
                                }
                            }
                        }
                        FloatButton {
                            r#type: FloatButtonType::Default,
                            shape: FloatButtonShape::Circle,
                            icon: rsx!(span { "i" }),
                            tooltip: Some("更多信息".to_string()),
                            badge: Some(BadgeConfig { dot: true, content: None, class: None }),
                        }
                    }
                }
            }

            // 带徽标
            DemoSection {
                title: "带徽标",
                div {
                    style: "border: 1px dashed var(--adui-color-border); padding: 12px; border-radius: var(--adui-radius); background: var(--adui-color-bg-container); min-height: 200px; position: relative;",
                    p { "悬浮按钮可以带数字徽标或点状徽标。" }
                    FloatButton {
                        r#type: FloatButtonType::Primary,
                        shape: FloatButtonShape::Circle,
                        icon: rsx!(span { "🔔" }),
                        tooltip: Some("通知".to_string()),
                        badge: Some(BadgeConfig { content: Some("5".to_string()), dot: false, class: None }),
                        right: Some(24.0),
                        bottom: Some(80.0),
                    }
                    FloatButton {
                        r#type: FloatButtonType::Default,
                        shape: FloatButtonShape::Circle,
                        icon: rsx!(span { "💬" }),
                        tooltip: Some("消息".to_string()),
                        badge: Some(BadgeConfig { dot: true, content: None, class: None }),
                        right: Some(24.0),
                        bottom: Some(24.0),
                    }
                }
            }

            // 回到顶部
            DemoSection {
                title: "回到顶部",
                div {
                    style: "border: 1px dashed var(--adui-color-border); padding: 12px; border-radius: var(--adui-radius); background: var(--adui-color-bg-container); min-height: 200px; position: relative;",
                    p { "滚动后可点击 BackTop 返回顶部。" }
                }
            }
        }

        // 回到顶部按钮
        BackTop {
            tooltip: Some("返回顶部".to_string()),
            content: Some("TOP".to_string()),
            shape: FloatButtonShape::Square,
            right: Some(24.0),
            bottom: Some(24.0),
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
