//! Affix 组件演示
//!
//! 展示 Affix 组件的基础用法和高级用法，包括：
//! - 顶部固定
//! - 底部固定
//! - 偏移量设置
//! - 状态变化回调

use adui_dioxus::{
    Affix, Button, ButtonType, Card, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            AffixDemo {}
        }
    }
}

#[component]
fn AffixDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let affixed = use_signal(|| false);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 200vh; color: var(--adui-color-text);",

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

            // 顶部固定
            DemoSection {
                title: "顶部固定",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Affix {
                        offset_top: Some(20.0),
                        on_change: {
                            let mut sig = affixed;
                            move |is_affixed: bool| {
                                sig.set(is_affixed);
                            }
                        },
                        Card {
                            title: Some(rsx!("固定在顶部")),
                            "当页面滚动时，这个卡片会固定在距离顶部 20px 的位置。向下滚动查看效果。"
                        }
                    }
                    {
                        let status_text = if *affixed.read() { "已固定" } else { "未固定" };
                        let display_text = format!("当前状态: {}", status_text);
                        rsx! {
                            div {
                                style: "padding: 12px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 14px;",
                                {display_text}
                            }
                        }
                    }
                }
            }

            // 底部固定
            DemoSection {
                title: "底部固定",
                div {
                    style: "margin-top: 800px;",
                    Affix {
                        offset_bottom: Some(20.0),
                        Card {
                            title: Some(rsx!("固定在底部")),
                            "当页面滚动时，这个卡片会固定在距离底部 20px 的位置。"
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 不同偏移量
            DemoSection {
                title: "不同偏移量",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Affix {
                        offset_top: Some(10.0),
                        Card {
                            title: Some(rsx!("偏移量 10px")),
                            "距离顶部 10px"
                        }
                    }
                    Affix {
                        offset_top: Some(50.0),
                        Card {
                            title: Some(rsx!("偏移量 50px")),
                            "距离顶部 50px"
                        }
                    }
                    Affix {
                        offset_top: Some(100.0),
                        Card {
                            title: Some(rsx!("偏移量 100px")),
                            "距离顶部 100px"
                        }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Affix {
                        offset_top: Some(80.0),
                        on_change: {
                            let mut sig = affixed;
                            move |is_affixed: bool| {
                                sig.set(is_affixed);
                            }
                        },
                        div {
                            style: "padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);",
                            div {
                                style: "font-weight: 600; margin-bottom: 8px;",
                                "导航栏"
                            }
                            div {
                                style: "display: flex; gap: 12px;",
                                Button {
                                    r#type: ButtonType::Default,
                                    "首页"
                                }
                                Button {
                                    r#type: ButtonType::Default,
                                    "产品"
                                }
                                Button {
                                    r#type: ButtonType::Default,
                                    "关于"
                                }
                            }
                        }
                    }
                    div {
                        style: "height: 1500px; padding: 16px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius);",
                        "这是页面内容区域，向下滚动可以看到导航栏固定在顶部。"
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
