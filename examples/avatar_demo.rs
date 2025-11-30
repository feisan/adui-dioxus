//! Avatar 组件演示
//!
//! 展示 Avatar 组件的基础用法和高级用法，包括：
//! - 图片头像
//! - 文字头像
//! - 图标头像
//! - 不同尺寸
//! - 不同形状
//! - 头像组

use adui_dioxus::{
    Avatar, AvatarGroup, AvatarShape, AvatarSize, Button, ButtonType, Icon, IconKind, ThemeMode,
    ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            AvatarDemo {}
        }
    }
}

#[component]
fn AvatarDemo() -> Element {
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

            // 图片头像
            DemoSection {
                title: "图片头像",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Avatar {
                        src: Some("https://via.placeholder.com/64".to_string()),
                    }
                    Avatar {
                        src: Some("https://via.placeholder.com/64".to_string()),
                        size: Some(AvatarSize::Large),
                    }
                    Avatar {
                        src: Some("https://via.placeholder.com/40".to_string()),
                        size: Some(AvatarSize::Small),
                    }
                }
            }

            // 文字头像
            DemoSection {
                title: "文字头像",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Avatar {
                        children: Some(rsx!("AD")),
                    }
                    Avatar {
                        children: Some(rsx!("UI")),
                    }
                    Avatar {
                        children: Some(rsx!("DX")),
                    }
                }
            }

            // 图标头像
            DemoSection {
                title: "图标头像",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Avatar {
                        icon: Some(rsx!(Icon { kind: IconKind::Info })),
                    }
                    Avatar {
                        icon: Some(rsx!(Icon { kind: IconKind::Search })),
                    }
                    Avatar {
                        icon: Some(rsx!(Icon { kind: IconKind::Edit })),
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Avatar {
                        children: Some(rsx!("S")),
                        size: Some(AvatarSize::Small),
                    }
                    Avatar {
                        children: Some(rsx!("M")),
                        size: Some(AvatarSize::Default),
                    }
                    Avatar {
                        children: Some(rsx!("L")),
                        size: Some(AvatarSize::Large),
                    }
                }
            }

            // 不同形状
            DemoSection {
                title: "不同形状",
                div {
                    style: "display: flex; gap: 16px; align-items: center; flex-wrap: wrap;",
                    Avatar {
                        children: Some(rsx!("圆")),
                        shape: Some(AvatarShape::Circle),
                    }
                    Avatar {
                        children: Some(rsx!("方")),
                        shape: Some(AvatarShape::Square),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 头像组
            DemoSection {
                title: "头像组",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    AvatarGroup {
                        children: rsx! {
                            Avatar { children: Some(rsx!("A")) }
                            Avatar { children: Some(rsx!("B")) }
                            Avatar { children: Some(rsx!("C")) }
                            Avatar { children: Some(rsx!("D")) }
                        }
                    }
                    div {
                        style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                        "多个头像组合（当前实现不支持max_count限制）"
                    }
                    AvatarGroup {
                        children: rsx! {
                            Avatar { children: Some(rsx!("1")) }
                            Avatar { children: Some(rsx!("2")) }
                            Avatar { children: Some(rsx!("3")) }
                            Avatar { children: Some(rsx!("4")) }
                            Avatar { children: Some(rsx!("5")) }
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
