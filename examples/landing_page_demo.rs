//! 落地页场景演示
//!
//! 模拟真实的落地页场景，综合使用：
//! - Layout 布局
//! - Carousel 轮播
//! - Card 卡片
//! - Button 按钮
//! - Typography 排版
//! - Space 间距

use adui_dioxus::{
    Button, ButtonType, Card, Content, Footer, Header, Layout, Space, SpaceDirection,
    ThemeProvider, Title, TitleLevel,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            LandingPageDemo {}
        }
    }
}

#[component]
fn LandingPageDemo() -> Element {
    rsx! {
        Layout {
            style: "min-height: 100vh; background: var(--adui-color-bg-base);",
            Header {
                div {
                    style: "padding: 16px 24px; background: var(--adui-color-bg-container); border-bottom: 1px solid var(--adui-color-border); display: flex; justify-content: space-between; align-items: center;",
                    Title { level: TitleLevel::H3, "产品名称" }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        Button {
                            r#type: ButtonType::Text,
                            "登录"
                        }
                        Button {
                            r#type: ButtonType::Primary,
                            "注册"
                        }
                    }
                }
            }
            Content {
                // 轮播图占位
                div {
                    style: "margin-bottom: 48px; height: 400px; background: linear-gradient(135deg, var(--adui-color-primary), var(--adui-color-primary-active)); display: flex; align-items: center; justify-content: center; color: white; font-size: 48px; font-weight: 600; border-radius: var(--adui-radius);",
                    "轮播图区域"
                }

                // 特性介绍
                div {
                    style: "padding: 0 24px 48px 24px; max-width: 1200px; margin: 0 auto;",
                    Title {
                        level: TitleLevel::H2,
                        style: "text-align: center; margin-bottom: 48px;",
                        "产品特性"
                    }
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 24px;",
                        {
                            (1..=3).map(|i| {
                                let title_text = format!("特性 {}", i);
                                let desc_text = format!("这是特性 {} 的详细描述。这里可以展示产品的核心功能和优势。", i);
                                rsx! {
                                    Card {
                                        title: Some(rsx!({title_text})),
                                        hoverable: true,
                                        div {
                                            style: "padding: 16px 0;",
                                            {desc_text}
                                        }
                                        Button {
                                            r#type: ButtonType::Primary,
                                            "了解更多"
                                        }
                                    }
                                }
                            })
                        }
                    }
                }

                // 行动号召
                div {
                    style: "padding: 48px 24px; background: var(--adui-color-bg-container); text-align: center;",
                    Title {
                        level: TitleLevel::H2,
                        style: "margin-bottom: 24px;",
                        "立即开始使用"
                    }
                    div {
                        style: "margin-bottom: 24px; color: var(--adui-color-text-secondary);",
                        "加入我们，体验全新的产品功能"
                    }
                    Space {
                        direction: SpaceDirection::Horizontal,
                        gap: Some(16.0),
                        Button {
                            r#type: ButtonType::Primary,
                            size: adui_dioxus::ButtonSize::Large,
                            "立即注册"
                        }
                        Button {
                            r#type: ButtonType::Default,
                            size: adui_dioxus::ButtonSize::Large,
                            "了解更多"
                        }
                    }
                }
            }
            Footer {
                div {
                    style: "padding: 24px; text-align: center; color: var(--adui-color-text-secondary);",
                    "© 2024 产品名称. All rights reserved."
                }
            }
        }
    }
}
