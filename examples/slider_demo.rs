//! Slider 组件演示
//!
//! 展示 Slider 组件的基础用法和高级用法，包括：
//! - 基础滑块
//! - 范围滑块
//! - 标记
//! - 垂直方向
//! - 步进控制

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::slider::{Slider, SliderMark, SliderValue},
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SliderDemo {}
        }
    }
}

#[component]
fn SliderDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let single_value = use_signal(|| SliderValue::Single(30.0));
    let range_value = use_signal(|| SliderValue::Range(20.0, 60.0));
    let marked_value = use_signal(|| SliderValue::Single(50.0));

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let marks = vec![
        SliderMark {
            value: 0.0,
            label: "0".into(),
        },
        SliderMark {
            value: 50.0,
            label: "50".into(),
        },
        SliderMark {
            value: 100.0,
            label: "100".into(),
        },
    ];

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

            // 基础滑块
            DemoSection {
                title: "基础滑块",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Slider {
                            value: Some(single_value.read().clone()),
                            min: 0.0,
                            max: 100.0,
                            on_change: {
                                let mut sig = single_value;
                                move |v| sig.set(v)
                            },
                        }
                        span {
                            style: "min-width: 60px; font-size: 14px; color: var(--adui-color-text-secondary);",
                            {
                                let val = single_value.read().clone();
                                match val {
                                    SliderValue::Single(v) => format!("{:.0}", v),
                                    SliderValue::Range(s, e) => format!("{:.0}-{:.0}", s, e),
                                }
                            }
                        }
                    }
                }
            }

            // 范围滑块
            DemoSection {
                title: "范围滑块",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Slider {
                            value: Some(range_value.read().clone()),
                            range: true,
                            min: 0.0,
                            max: 100.0,
                            on_change: {
                                let mut sig = range_value;
                                move |v| sig.set(v)
                            },
                        }
                        span {
                            style: "min-width: 80px; font-size: 14px; color: var(--adui-color-text-secondary);",
                            {
                                let val = range_value.read().clone();
                                match val {
                                    SliderValue::Single(v) => format!("{:.0}", v),
                                    SliderValue::Range(s, e) => format!("{:.0} - {:.0}", s, e),
                                }
                            }
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
                        style: "display: flex; align-items: center; gap: 12px;",
                        Slider {
                            value: Some(SliderValue::Single(30.0)),
                            min: 0.0,
                            max: 100.0,
                            step: Some(5.0),
                        }
                        span {
                            style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                            "步进: 5"
                        }
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                Slider {
                    value: Some(SliderValue::Single(40.0)),
                    disabled: true,
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 标记
            DemoSection {
                title: "标记",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        Slider {
                            value: Some(marked_value.read().clone()),
                            min: 0.0,
                            max: 100.0,
                            marks: Some(marks.clone()),
                            on_change: {
                                let mut sig = marked_value;
                                move |v| sig.set(v)
                            },
                        }
                    }
                }
            }

            // 带点的标记
            DemoSection {
                title: "带点的标记",
                Slider {
                    value: Some(SliderValue::Single(50.0)),
                    min: 0.0,
                    max: 100.0,
                    marks: Some(marks.clone()),
                    dots: true,
                }
            }

            // 垂直方向
            DemoSection {
                title: "垂直方向",
                div {
                    style: "display: flex; gap: 24px; align-items: flex-start;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        span { style: "font-size: 12px;", "垂直滑块" }
                        Slider {
                            value: Some(SliderValue::Single(50.0)),
                            vertical: true,
                            style: Some("height: 180px;".into()),
                        }
                    }
                }
            }

            // 反向
            DemoSection {
                title: "反向",
                Slider {
                    value: Some(SliderValue::Single(30.0)),
                    reverse: true,
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
