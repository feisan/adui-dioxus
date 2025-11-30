//! Radio 组件演示
//!
//! 展示 Radio 组件的基础用法和高级用法，包括：
//! - 基础单选
//! - 单选组
//! - 按钮样式
//! - 禁用状态

use adui_dioxus::{
    Button, ButtonType, Radio, RadioGroup, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            RadioDemo {}
        }
    }
}

#[component]
fn RadioDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let single_value = use_signal(|| "option1".to_string());
    let group_value = use_signal(|| Some("apple".to_string()));
    let button_value = use_signal(|| Some("small".to_string()));

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

            // 基础单选
            DemoSection {
                title: "基础单选",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Radio {
                        value: "option1".to_string(),
                        checked: Some(*single_value.read() == "option1"),
                        on_change: {
                            let mut sig = single_value;
                            move |_| sig.set("option1".to_string())
                        },
                        "选项 1"
                    }
                    Radio {
                        value: "option2".to_string(),
                        checked: Some(*single_value.read() == "option2"),
                        on_change: {
                            let mut sig = single_value;
                            move |_| sig.set("option2".to_string())
                        },
                        "选项 2"
                    }
                    Radio {
                        value: "option3".to_string(),
                        disabled: true,
                        "禁用选项"
                    }
                }
            }

            // 单选组
            DemoSection {
                title: "单选组",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    RadioGroup {
                        value: group_value.read().clone(),
                        on_change: {
                            let mut sig = group_value;
                            move |value| sig.set(Some(value))
                        },
                        Radio {
                            value: "apple".to_string(),
                            "Apple"
                        }
                        Radio {
                            value: "banana".to_string(),
                            "Banana"
                        }
                        Radio {
                            value: "cherry".to_string(),
                            "Cherry"
                        }
                        Radio {
                            value: "orange".to_string(),
                            "Orange"
                        }
                    }
                    {
                        let selected_text = format!("已选择: {:?}", *group_value.read());
                        rsx! {
                            div {
                                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                {selected_text}
                            }
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 按钮样式
            DemoSection {
                title: "按钮样式",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    RadioGroup {
                        value: button_value.read().clone(),
                        on_change: {
                            let mut sig = button_value;
                            move |value| sig.set(Some(value))
                        },
                        Radio {
                            value: "small".to_string(),
                            button: true,
                            "Small"
                        }
                        Radio {
                            value: "middle".to_string(),
                            button: true,
                            "Middle"
                        }
                        Radio {
                            value: "large".to_string(),
                            button: true,
                            "Large"
                        }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "选择您喜欢的水果：" }
                        RadioGroup {
                            value: group_value.read().clone(),
                            on_change: {
                                let mut sig = group_value;
                                move |value| sig.set(Some(value))
                            },
                            Radio {
                                value: "apple".to_string(),
                                "🍎 Apple"
                            }
                            Radio {
                                value: "banana".to_string(),
                                "🍌 Banana"
                            }
                            Radio {
                                value: "cherry".to_string(),
                                "🍒 Cherry"
                            }
                            Radio {
                                value: "orange".to_string(),
                                "🍊 Orange"
                            }
                        }
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
