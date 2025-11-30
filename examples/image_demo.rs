//! Image 组件演示
//!
//! 展示 Image 组件的基础用法和高级用法，包括：
//! - 基础图片
//! - 错误回退
//! - 禁用预览
//! - 自定义预览遮罩
//! - 图片画廊
//! - 预览组

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::image::{Image, ImagePreviewGroup, ImagePreviewItem, PreviewConfig},
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ImageDemo {}
        }
    }
}

#[component]
fn ImageDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let mut visible: Signal<bool> = use_signal(|| false);
    let mut current: Signal<usize> = use_signal(|| 0);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let items = vec![
        ImagePreviewItem::new("https://picsum.photos/800/600?random=10").with_alt("Large Image 1"),
        ImagePreviewItem::new("https://picsum.photos/800/600?random=11").with_alt("Large Image 2"),
        ImagePreviewItem::new("https://picsum.photos/800/600?random=12").with_alt("Large Image 3"),
        ImagePreviewItem::new("https://picsum.photos/800/600?random=13").with_alt("Large Image 4"),
    ];

    let thumbnails = [
        "https://picsum.photos/150/150?random=10",
        "https://picsum.photos/150/150?random=11",
        "https://picsum.photos/150/150?random=12",
        "https://picsum.photos/150/150?random=13",
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

            // 基础图片
            DemoSection {
                title: "基础图片",
                Image {
                    src: "https://picsum.photos/300/200".to_string(),
                    alt: Some("Random image".to_string()),
                    width: Some("300px".to_string()),
                    height: Some("200px".to_string()),
                }
            }

            // 错误回退
            DemoSection {
                title: "错误回退",
                div {
                    style: "display: flex; gap: 16px; flex-wrap: wrap;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-size: 12px;", "无效源，带回退：" }
                        Image {
                            src: "https://invalid-url-that-will-fail.jpg".to_string(),
                            fallback: Some("https://picsum.photos/150/150".to_string()),
                            width: Some("150px".to_string()),
                            height: Some("150px".to_string()),
                        }
                    }
                }
            }

            // 禁用预览
            DemoSection {
                title: "禁用预览",
                Image {
                    src: "https://picsum.photos/200/150".to_string(),
                    alt: Some("No preview".to_string()),
                    width: Some("200px".to_string()),
                    height: Some("150px".to_string()),
                    preview: false,
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 自定义预览遮罩
            DemoSection {
                title: "自定义预览遮罩",
                Image {
                    src: "https://picsum.photos/250/180".to_string(),
                    alt: Some("Custom mask".to_string()),
                    width: Some("250px".to_string()),
                    height: Some("180px".to_string()),
                    preview_config: Some(PreviewConfig::new().with_mask("Click to view")),
                }
            }

            // 图片画廊
            DemoSection {
                title: "图片画廊",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 12px;",
                    for (src, alt) in [
                        ("https://picsum.photos/200/200?random=1", "Image 1"),
                        ("https://picsum.photos/200/200?random=2", "Image 2"),
                        ("https://picsum.photos/200/200?random=3", "Image 3"),
                        ("https://picsum.photos/200/200?random=4", "Image 4"),
                    ] {
                        Image {
                            src: src.to_string(),
                            alt: Some(alt.to_string()),
                            width: Some("150px".to_string()),
                            height: Some("150px".to_string()),
                        }
                    }
                }
            }

            // 预览组
            DemoSection {
                title: "预览组",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "点击任意缩略图打开预览组："
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        for (i, thumb) in thumbnails.iter().enumerate() {
                            img {
                                src: "{thumb}",
                                style: "width: 100px; height: 100px; object-fit: cover; cursor: pointer; border-radius: 4px;",
                                onclick: {
                                    let mut current = current;
                                    let mut visible = visible;
                                    move |_| {
                                        current.set(i);
                                        visible.set(true);
                                    }
                                },
                            }
                        }
                    }
                    ImagePreviewGroup {
                        items: items.clone(),
                        visible: *visible.read(),
                        current: *current.read(),
                        on_visible_change: Some(EventHandler::new({
                            let mut visible = visible;
                            move |v| visible.set(v)
                        })),
                        on_change: Some(EventHandler::new({
                            let mut current = current;
                            move |idx| current.set(idx)
                        })),
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
