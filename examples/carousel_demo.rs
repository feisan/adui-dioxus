//! Carousel 组件演示
//!
//! 展示 Carousel 组件的基础用法和高级用法，包括：
//! - 基础轮播
//! - 箭头控制
//! - 淡入淡出效果
//! - 垂直指示器
//! - 自动播放

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::carousel::{Carousel, CarouselEffect, CarouselItem, DotPlacement},
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            CarouselDemo {}
        }
    }
}

#[component]
fn CarouselDemo() -> Element {
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

            // 基础轮播
            DemoSection {
                title: "基础轮播",
                div {
                    style: "border-radius: 8px; overflow: hidden;",
                    Carousel {
                        items: vec![
                            CarouselItem::new("Slide 1").with_background("#364d79"),
                            CarouselItem::new("Slide 2").with_background("#3d5a80"),
                            CarouselItem::new("Slide 3").with_background("#456990"),
                            CarouselItem::new("Slide 4").with_background("#4e7da6"),
                        ],
                    }
                }
            }

            // 箭头控制
            DemoSection {
                title: "箭头控制",
                div {
                    style: "border-radius: 8px; overflow: hidden;",
                    Carousel {
                        items: vec![
                            CarouselItem::new("🍎 Apple").with_background("#4a7c59"),
                            CarouselItem::new("🍌 Banana").with_background("#5a8c69"),
                            CarouselItem::new("🍒 Cherry").with_background("#6a9c79"),
                        ],
                        arrows: true,
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 淡入淡出效果
            DemoSection {
                title: "淡入淡出效果",
                div {
                    style: "border-radius: 8px; overflow: hidden;",
                    Carousel {
                        items: vec![
                            CarouselItem::new("Fade Slide 1").with_background("#7c4a6c"),
                            CarouselItem::new("Fade Slide 2").with_background("#8c5a7c"),
                            CarouselItem::new("Fade Slide 3").with_background("#9c6a8c"),
                        ],
                        effect: CarouselEffect::Fade,
                        arrows: true,
                    }
                }
            }

            // 垂直指示器
            DemoSection {
                title: "垂直指示器",
                div {
                    style: "border-radius: 8px; overflow: hidden;",
                    Carousel {
                        items: vec![
                            CarouselItem::new("Vertical Dots 1").with_background("#6c4a7c"),
                            CarouselItem::new("Vertical Dots 2").with_background("#7c5a8c"),
                            CarouselItem::new("Vertical Dots 3").with_background("#8c6a9c"),
                        ],
                        dot_placement: DotPlacement::Right,
                        arrows: true,
                    }
                }
            }

            // 自动播放
            DemoSection {
                title: "自动播放",
                div {
                    style: "border-radius: 8px; overflow: hidden;",
                    Carousel {
                        items: vec![
                            CarouselItem::new("Auto Play 1").with_background("#5a7c4a"),
                            CarouselItem::new("Auto Play 2").with_background("#6a8c5a"),
                            CarouselItem::new("Auto Play 3").with_background("#7a9c6a"),
                        ],
                        autoplay: true,
                        arrows: true,
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
