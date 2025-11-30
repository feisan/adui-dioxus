//! ColorPicker 组件演示
//!
//! 展示 ColorPicker 组件的基础用法和高级用法，包括：
//! - 基础颜色选择
//! - 受控模式
//! - 清除功能
//! - 禁用状态

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::color_picker::ColorPicker, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ColorPickerDemo {}
        }
    }
}

#[component]
fn ColorPickerDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| Some("#1677ff".to_string()));
    let controlled_value = use_signal(|| Some("#52c41a".to_string()));

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

            // 基础颜色选择
            DemoSection {
                title: "基础颜色选择",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        ColorPicker {
                            value: basic_value.read().clone(),
                            on_change: {
                                let mut sig = basic_value;
                                move |hex: String| sig.set(if hex.is_empty() { None } else { Some(hex) })
                            },
                        }
                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            span {
                                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                "当前颜色: ",
                                {
                                    let val = basic_value.read();
                                    val.as_ref().map(|v| v.clone()).unwrap_or_else(|| "(未选择)".to_string())
                                }
                            }
                            {
                                let val = basic_value.read();
                                if let Some(color) = val.as_ref() {
                                    let style_str = format!("width: 60px; height: 30px; background: {}; border: 1px solid var(--adui-color-border); border-radius: 4px;", color);
                                    rsx! {
                                        div {
                                            style: style_str,
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
                            }
                        }
                    }
                }
            }

            // 受控模式
            DemoSection {
                title: "受控模式",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        ColorPicker {
                            value: controlled_value.read().clone(),
                            on_change: {
                                let mut sig = controlled_value;
                                move |hex: String| sig.set(if hex.is_empty() { None } else { Some(hex) })
                            },
                        }
                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            span {
                                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                "当前颜色: ",
                                {
                                    let val = controlled_value.read();
                                    val.as_ref().map(|v| v.clone()).unwrap_or_else(|| "(未选择)".to_string())
                                }
                            }
                            {
                                let val = controlled_value.read();
                                if let Some(color) = val.as_ref() {
                                    let style_str = format!("width: 60px; height: 30px; background: {}; border: 1px solid var(--adui-color-border); border-radius: 4px;", color);
                                    rsx! {
                                        div {
                                            style: style_str,
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
                            }
                        }
                    }
                }
            }

            // 允许清除
            DemoSection {
                title: "允许清除",
                ColorPicker {
                    default_value: Some("#faad14".to_string()),
                    allow_clear: true,
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                ColorPicker {
                    value: Some("#faad14".to_string()),
                    disabled: true,
                    allow_clear: false,
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 预设颜色示例
            DemoSection {
                title: "预设颜色示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        for preset in ["#1677ff", "#52c41a", "#faad14", "#f5222d", "#722ed1"] {
                            div {
                                style: format!("width: 40px; height: 40px; background: {}; border: 1px solid var(--adui-color-border); border-radius: 4px; cursor: pointer;", preset),
                                title: preset,
                            }
                        }
                    }
                    span {
                        style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                        "点击上面的颜色块可以查看效果（实际应用中可以通过onChange设置）"
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
