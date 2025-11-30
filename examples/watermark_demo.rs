//! Watermark 组件演示
//!
//! 展示 Watermark 组件的基础用法和高级用法，包括：
//! - 基础文字水印
//! - 多行文字水印
//! - 自定义字体样式
//! - 调整间距
//! - 图片水印

use adui_dioxus::{
    App, Button, ButtonType, Card, ComponentSize, ConfigProvider, ThemeMode, ThemeProvider, Title,
    TitleLevel, Watermark, WatermarkFont, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ConfigProvider {
                size: Some(ComponentSize::Middle),
                App {
                    WatermarkDemo {}
                }
            }
        }
    }
}

#[component]
fn WatermarkDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let show_watermark = use_signal(|| true);

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
                span {
                    style: "margin-left: 16px; font-weight: 600;",
                    "水印控制："
                }
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut sig = show_watermark;
                        move |_| {
                            let current = *sig.read();
                            sig.set(!current);
                        }
                    },
                    if *show_watermark.read() { "隐藏所有水印" } else { "显示所有水印" }
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础文字水印
            DemoSection {
                title: "基础文字水印",
                {
                    if *show_watermark.read() {
                        rsx! {
                            Watermark {
                                content: Some(vec!["Ant Design".to_string()]),
                                Card {
                                    title: Some(rsx!("卡片标题")),
                                    style: Some("height: 200px;".to_string()),
                                    children: rsx! {
                                        p { "这是一段受水印保护的内容。" }
                                        p { "水印会覆盖整个容器区域。" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            Card {
                                title: Some(rsx!("卡片标题")),
                                style: Some("height: 200px;".to_string()),
                                children: rsx! {
                                    p { "这是一段受水印保护的内容。" }
                                    p { "水印会覆盖整个容器区域。" }
                                }
                            }
                        }
                    }
                }
            }

            // 多行文字水印
            DemoSection {
                title: "多行文字水印",
                {
                    if *show_watermark.read() {
                        rsx! {
                            Watermark {
                                content: Some(vec![
                                    "Confidential".to_string(),
                                    "2024-01-01".to_string(),
                                ]),
                                Card {
                                    style: Some("height: 200px;".to_string()),
                                    children: rsx! {
                                        p { "多行水印可以显示更多信息，例如日期、用户名等。" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            Card {
                                style: Some("height: 200px;".to_string()),
                                children: rsx! {
                                    p { "多行水印可以显示更多信息，例如日期、用户名等。" }
                                }
                            }
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 自定义字体样式
            DemoSection {
                title: "自定义字体样式",
                {
                    if *show_watermark.read() {
                        rsx! {
                            Watermark {
                                content: Some(vec!["Custom Style".to_string()]),
                                font: Some(WatermarkFont {
                                    color: "rgba(255, 0, 0, 0.15)".to_string(),
                                    font_size: 20.0,
                                    font_weight: "bold".to_string(),
                                    font_style: "normal".to_string(),
                                    font_family: "sans-serif".to_string(),
                                    text_align: "center".to_string(),
                                }),
                                rotate: -30.0,
                                Card {
                                    style: Some("height: 200px;".to_string()),
                                    children: rsx! {
                                        p { "自定义红色水印，更大的字体和不同的旋转角度。" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            Card {
                                style: Some("height: 200px;".to_string()),
                                children: rsx! {
                                    p { "自定义红色水印，更大的字体和不同的旋转角度。" }
                                }
                            }
                        }
                    }
                }
            }

            // 调整间距
            DemoSection {
                title: "调整水印间距",
                {
                    if *show_watermark.read() {
                        rsx! {
                            Watermark {
                                content: Some(vec!["Dense".to_string()]),
                                gap: Some([50.0, 50.0]),
                                Card {
                                    style: Some("height: 200px;".to_string()),
                                    children: rsx! {
                                        p { "更密集的水印分布，间距设置为 50x50 像素。" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            Card {
                                style: Some("height: 200px;".to_string()),
                                children: rsx! {
                                    p { "更密集的水印分布，间距设置为 50x50 像素。" }
                                }
                            }
                        }
                    }
                }
            }

            // 图片水印
            DemoSection {
                title: "图片水印",
                {
                    if *show_watermark.read() {
                        rsx! {
                            Watermark {
                                image: Some("https://gw.alipayobjects.com/zos/bmw-prod/59a18171-ae17-4571-bbbd-07a66b520b46/k7cjl1fa_w192_h106.png".to_string()),
                                width: Some(100.0),
                                height: Some(55.0),
                                Card {
                                    style: Some("height: 200px;".to_string()),
                                    children: rsx! {
                                        p { "使用图片作为水印，适合添加公司 Logo 等。" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            Card {
                                style: Some("height: 200px;".to_string()),
                                children: rsx! {
                                    p { "使用图片作为水印，适合添加公司 Logo 等。" }
                                }
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
