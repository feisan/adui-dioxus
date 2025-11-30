//! Splitter 组件演示
//!
//! 展示 Splitter 组件的基础用法和高级用法，包括：
//! - 水平分割
//! - 垂直分割
//! - 受控分割比例
//! - 最小/最大宽度限制

use adui_dioxus::{
    Button, ButtonType, Splitter, SplitterOrientation, Text, TextType, ThemeMode, ThemeProvider,
    Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SplitterDemo {}
        }
    }
}

#[component]
fn SplitterDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let horizontal_split = use_signal(|| 0.4f32);
    let vertical_split = use_signal(|| 0.5f32);

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

            // 水平分割
            DemoSection {
                title: "水平分割",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Text {
                        r#type: TextType::Secondary,
                        style: Some("font-size: 12px;".into()),
                        "拖动中间的分割线可以调整左右两个面板的宽度比例"
                    }
                    div {
                        style: "height: 300px;",
                        Splitter {
                            orientation: SplitterOrientation::Horizontal,
                            split: Some(*horizontal_split.read()),
                            on_change: {
                                let mut sig = horizontal_split;
                                move |v| sig.set(v)
                            },
                            first: rsx! {
                                div {
                                    style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                    "左侧面板"
                                }
                            },
                            second: rsx! {
                                div {
                                    style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                    "右侧面板"
                                }
                            },
                        }
                    }
                    {
                        let ratio_text = format!("当前比例: {:.0}% / {:.0}%", *horizontal_split.read() * 100.0, (1.0 - *horizontal_split.read()) * 100.0);
                        rsx! {
                            Text {
                                r#type: TextType::Secondary,
                                style: Some("font-size: 12px;".into()),
                                {ratio_text}
                            }
                        }
                    }
                }
            }

            // 垂直分割
            DemoSection {
                title: "垂直分割",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Text {
                        r#type: TextType::Secondary,
                        style: Some("font-size: 12px;".into()),
                        "拖动中间的分割线可以调整上下两个面板的高度比例"
                    }
                    div {
                        style: "height: 300px;",
                        Splitter {
                            orientation: SplitterOrientation::Vertical,
                            split: Some(*vertical_split.read()),
                            on_change: {
                                let mut sig = vertical_split;
                                move |v| sig.set(v)
                            },
                            first: rsx! {
                                div {
                                    style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                    "上方面板"
                                }
                            },
                            second: rsx! {
                                div {
                                    style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                    "下方面板"
                                }
                            },
                        }
                    }
                    {
                        let ratio_text = format!("当前比例: {:.0}% / {:.0}%", *vertical_split.read() * 100.0, (1.0 - *vertical_split.read()) * 100.0);
                        rsx! {
                            Text {
                                r#type: TextType::Secondary,
                                style: Some("font-size: 12px;".into()),
                                {ratio_text}
                            }
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 嵌套分割
            DemoSection {
                title: "嵌套分割",
                div {
                    style: "height: 400px;",
                    Splitter {
                        orientation: SplitterOrientation::Horizontal,
                        split: Some(0.5),
                        first: rsx! {
                            div {
                                style: "height: 100%;",
                                Splitter {
                                    orientation: SplitterOrientation::Vertical,
                                    split: Some(0.5),
                                    first: rsx! {
                                        div {
                                            style: "height: 100%; padding: 16px; background: var(--adui-color-primary-bg); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center; color: var(--adui-color-text);",
                                            "左上"
                                        }
                                    },
                                    second: rsx! {
                                        div {
                                            style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                            "左下"
                                        }
                                    },
                                }
                            }
                        },
                        second: rsx! {
                            div {
                                style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                "右侧面板"
                            }
                        },
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例 - 代码编辑器布局",
                div {
                    style: "height: 400px;",
                    Splitter {
                        orientation: SplitterOrientation::Horizontal,
                        split: Some(0.3),
                        first: rsx! {
                            div {
                                style: "height: 100%; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; flex-direction: column;",
                                div {
                                    style: "font-weight: 600; margin-bottom: 8px;",
                                    "文件树"
                                }
                                div {
                                    style: "flex: 1; padding: 8px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 12px;",
                                    "src/\n  main.rs\n  lib.rs\n  components/\n    button.rs"
                                }
                            }
                        },
                        second: rsx! {
                            div {
                                style: "height: 100%;",
                                Splitter {
                                    orientation: SplitterOrientation::Vertical,
                                    split: Some(0.7),
                                    first: rsx! {
                                        div {
                                            style: "height: 100%; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; flex-direction: column;",
                                            div {
                                                style: "font-weight: 600; margin-bottom: 8px;",
                                                "编辑器"
                                            }
                                            div {
                                                style: "flex: 1; padding: 8px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-family: monospace; font-size: 12px; white-space: pre;",
                                                {format!("fn main() {{\n    println!(\"Hello, world!\");\n}}")}
                                            }
                                        }
                                    },
                                    second: rsx! {
                                        div {
                                            style: "height: 100%; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; flex-direction: column;",
                                            div {
                                                style: "font-weight: 600; margin-bottom: 8px;",
                                                "终端"
                                            }
                                            div {
                                                style: "flex: 1; padding: 8px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-family: monospace; font-size: 12px; white-space: pre;",
                                                {"$ cargo run\n   Compiling...\n   Finished\n   Running..."}
                                            }
                                        }
                                    },
                                }
                            }
                        },
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
