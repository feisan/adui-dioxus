//! Rate 组件演示
//!
//! 展示 Rate 组件的基础用法和高级用法，包括：
//! - 基础评分
//! - 半星评分
//! - 自定义数量
//! - 只读模式
//! - 自定义字符

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel, components::rate::Rate,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            RateDemo {}
        }
    }
}

#[component]
fn RateDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| Some(3.0));
    let half_value = use_signal(|| Some(2.5));
    let custom_count_value = use_signal(|| Some(4.0));

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

            // 基础评分
            DemoSection {
                title: "基础评分",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Rate {
                            value: *basic_value.read(),
                            on_change: {
                                let mut sig = basic_value;
                                move |v| sig.set(v)
                            },
                        }
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                            "当前评分: ",
                            {basic_value.read().as_ref().map(|v| format!("{:.1}", v)).unwrap_or_else(|| "未评分".to_string())}
                        }
                    }
                }
            }

            // 半星评分
            DemoSection {
                title: "半星评分",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Rate {
                            value: *half_value.read(),
                            allow_half: true,
                            on_change: {
                                let mut sig = half_value;
                                move |v| sig.set(v)
                            },
                        }
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                            "当前评分: ",
                            {half_value.read().as_ref().map(|v| format!("{:.1}", v)).unwrap_or_else(|| "未评分".to_string())}
                        }
                    }
                }
            }

            // 自定义数量
            DemoSection {
                title: "自定义数量",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Rate {
                            value: *custom_count_value.read(),
                            count: 10,
                            on_change: {
                                let mut sig = custom_count_value;
                                move |v| sig.set(v)
                            },
                        }
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                            "10星评分"
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                Rate {
                    value: Some(3.0),
                    disabled: true,
                }
            }

            // 只读模式（通过禁用实现）
            DemoSection {
                title: "只读模式",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Rate {
                        value: Some(4.5),
                        allow_half: true,
                        disabled: true,
                    }
                    Rate {
                        value: Some(2.0),
                        disabled: true,
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 不允许清除
            DemoSection {
                title: "不允许清除",
                Rate {
                    value: Some(3.0),
                    allow_clear: false,
                }
            }

            // 自定义字符
            DemoSection {
                title: "自定义字符",
                Rate {
                    value: Some(3.0),
                    character: Some(rsx! { span { style: "font-size: 20px;", "❤" } }),
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
