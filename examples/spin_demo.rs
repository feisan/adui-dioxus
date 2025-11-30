//! Spin 组件演示
//!
//! 展示 Spin 组件的基础用法和高级用法，包括：
//! - 基础指示器
//! - 不同尺寸
//! - 嵌套加载
//! - 自定义提示文字

use adui_dioxus::{
    Button, ButtonType, Card, Spin, SpinSize, ThemeMode, ThemeProvider, Title, TitleLevel,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SpinDemo {}
        }
    }
}

#[component]
fn SpinDemo() -> Element {
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

            // 基础指示器
            DemoSection {
                title: "基础指示器",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Spin { size: Some(SpinSize::Small), tip: Some("小号".to_string()) }
                    Spin { tip: Some("默认".to_string()) }
                    Spin { size: Some(SpinSize::Large), tip: Some("大号".to_string()) }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Spin { size: Some(SpinSize::Small) }
                        span { "Small" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Spin {}
                        span { "Default" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Spin { size: Some(SpinSize::Large) }
                        span { "Large" }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 嵌套加载
            DemoSection {
                title: "嵌套加载",
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
                        if *loading.read() { "停止加载" } else { "开始加载" }
                    }
                    Spin {
                        spinning: Some(*loading.read()),
                        tip: Some("加载中...".to_string()),
                        Card {
                            children: rsx! {
                                div {
                                    style: "padding: 16px;",
                                    h4 { style: "margin: 0 0 8px 0;", "内容卡片" }
                                    p {
                                        style: "margin: 0; color: var(--adui-color-text-secondary);",
                                        "当 Spin 处于加载状态时，会在内容上方显示半透明遮罩和指示器。"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 自定义提示文字
            DemoSection {
                title: "自定义提示文字",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Spin { tip: Some("加载中...".to_string()) }
                    Spin { tip: Some("处理中，请稍候".to_string()) }
                    Spin { tip: Some("正在获取数据".to_string()) }
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
