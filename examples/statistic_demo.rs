//! Statistic 组件演示
//!
//! 展示 Statistic 组件的基础用法和高级用法，包括：
//! - 基础统计数值
//! - 前缀和后缀
//! - 精度控制
//! - 趋势显示

use adui_dioxus::{
    Button, ButtonType, Card, Col, Row, Statistic, Tag, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            StatisticDemo {}
        }
    }
}

#[component]
fn StatisticDemo() -> Element {
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

            // 基础统计数值
            DemoSection {
                title: "基础统计数值",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;",
                    Statistic {
                        title: Some(rsx!("总访问量")),
                        value: Some(123456.0),
                    }
                    Statistic {
                        title: Some(rsx!("今日访问")),
                        value: Some(12345.0),
                    }
                    Statistic {
                        title: Some(rsx!("在线用户")),
                        value: Some(888.0),
                    }
                }
            }

            // 前缀和后缀
            DemoSection {
                title: "前缀和后缀",
                Row {
                    gutter: Some(16.0),
                    Col {
                        span: 8,
                        Card {
                            title: Some(rsx!("今日访问量")),
                            children: rsx! {
                                Statistic {
                                    title: Some(rsx!("Visits")),
                                    value: Some(12345.0),
                                    precision: Some(0),
                                    suffix: Some(rsx!("次")),
                                }
                                Tag { children: rsx!("较昨日 +8%") }
                            }
                        }
                    }
                    Col {
                        span: 8,
                        Card {
                            title: Some(rsx!("转化率")),
                            children: rsx! {
                                Statistic {
                                    title: Some(rsx!("Conversion")),
                                    value: Some(3.14159),
                                    precision: Some(2),
                                    suffix: Some(rsx!("%")),
                                }
                                Tag { children: rsx!("较昨日 -1.2%") }
                            }
                        }
                    }
                    Col {
                        span: 8,
                        Card {
                            title: Some(rsx!("错误率")),
                            children: rsx! {
                                Statistic {
                                    title: Some(rsx!("Error rate")),
                                    value: Some(0.07),
                                    precision: Some(2),
                                    suffix: Some(rsx!("%")),
                                }
                                Tag { children: rsx!("稳定") }
                            }
                        }
                    }
                }
            }

            // 精度控制
            DemoSection {
                title: "精度控制",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;",
                    Statistic {
                        title: Some(rsx!("无精度")),
                        value: Some(1234.567),
                    }
                    Statistic {
                        title: Some(rsx!("精度2位")),
                        value: Some(1234.567),
                        precision: Some(2),
                    }
                    Statistic {
                        title: Some(rsx!("精度0位")),
                        value: Some(1234.567),
                        precision: Some(0),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 前缀示例
            DemoSection {
                title: "前缀示例",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px;",
                    Statistic {
                        title: Some(rsx!("收入")),
                        value: Some(12345.0),
                        prefix: Some(rsx!("¥")),
                    }
                    Statistic {
                        title: Some(rsx!("增长率")),
                        value: Some(12.5),
                        prefix: Some(rsx!("+")),
                        suffix: Some(rsx!("%")),
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
