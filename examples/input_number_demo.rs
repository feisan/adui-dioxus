//! InputNumber 组件演示
//!
//! 展示 InputNumber 组件的基础用法和高级用法，包括：
//! - 基础数字输入
//! - 步进控制
//! - 范围限制
//! - 精度控制
//! - 前缀和后缀

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::control::ControlStatus, components::input_number::InputNumber, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            InputNumberDemo {}
        }
    }
}

#[component]
fn InputNumberDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| Some(1.0));
    let step_value = use_signal(|| Some(0.5));
    let range_value = use_signal(|| Some(50.0));
    let precision_value = use_signal(|| Some(std::f64::consts::PI));

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

            // 基础数字输入
            DemoSection {
                title: "基础数字输入",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 100px;", "数字：" }
                        InputNumber {
                            value: *basic_value.read(),
                            on_change: {
                                let mut sig = basic_value;
                                move |v: Option<f64>| sig.set(v)
                            },
                        }
                        span {
                            style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                            "当前值: ",
                            {basic_value.read().as_ref().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        }
                    }
                }
            }

            // 步进控制
            DemoSection {
                title: "步进控制",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 100px;", "步进0.5：" }
                        InputNumber {
                            value: *step_value.read(),
                            step: Some(0.5),
                            min: Some(0.0),
                            max: Some(10.0),
                            on_change: {
                                let mut sig = step_value;
                                move |v: Option<f64>| sig.set(v)
                            },
                        }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 100px;", "步进10：" }
                        InputNumber {
                            value: *range_value.read(),
                            step: Some(10.0),
                            min: Some(0.0),
                            max: Some(100.0),
                            on_change: {
                                let mut sig = range_value;
                                move |v: Option<f64>| sig.set(v)
                            },
                        }
                    }
                }
            }

            // 精度控制
            DemoSection {
                title: "精度控制",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 100px;", "精度2位：" }
                        InputNumber {
                            value: *precision_value.read(),
                            precision: Some(2),
                            step: Some(0.01),
                            on_change: {
                                let mut sig = precision_value;
                                move |v: Option<f64>| sig.set(v)
                            },
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                InputNumber {
                    value: Some(42.0),
                    disabled: true,
                }
            }

            // 不同状态
            DemoSection {
                title: "不同状态",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    InputNumber {
                        value: Some(100.0),
                        status: Some(ControlStatus::Success),
                    }
                    InputNumber {
                        value: Some(50.0),
                        status: Some(ControlStatus::Warning),
                    }
                    InputNumber {
                        value: Some(0.0),
                        status: Some(ControlStatus::Error),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 无控制按钮
            DemoSection {
                title: "无控制按钮",
                InputNumber {
                    value: Some(10.0),
                    controls: false,
                }
            }

            // 范围限制
            DemoSection {
                title: "范围限制",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 100px;", "0-100：" }
                        InputNumber {
                            value: Some(50.0),
                            min: Some(0.0),
                            max: Some(100.0),
                        }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "min-width: 100px;", "-10到10：" }
                        InputNumber {
                            value: Some(0.0),
                            min: Some(-10.0),
                            max: Some(10.0),
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
