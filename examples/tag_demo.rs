//! Tag 组件演示
//!
//! 展示 Tag 组件的基础用法和高级用法，包括：
//! - 预设颜色
//! - 可关闭标签
//! - 可选中标签
//! - 自定义颜色
//! - 不同尺寸

use adui_dioxus::{
    Button, ButtonType, Card, Tag, TagColor, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TagDemo {}
        }
    }
}

#[component]
fn TagDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let checked = use_signal(|| true);
    let closable_visible = use_signal(|| true);

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

            // 预设颜色
            DemoSection {
                title: "预设颜色",
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    Tag { children: rsx!("Default") }
                    Tag { color: Some(TagColor::Primary), children: rsx!("Primary") }
                    Tag { color: Some(TagColor::Success), children: rsx!("Success") }
                    Tag { color: Some(TagColor::Warning), children: rsx!("Warning") }
                    Tag { color: Some(TagColor::Error), children: rsx!("Error") }
                }
            }

            // 可关闭标签
            DemoSection {
                title: "可关闭标签",
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    {
                        if *closable_visible.read() {
                            rsx! {
                                Tag {
                                    color: Some(TagColor::Primary),
                                    closable: true,
                                    on_close: {
                                        let mut sig = closable_visible;
                                        move |_| sig.set(false)
                                    },
                                    children: rsx!("可关闭标签")
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                    Tag {
                        color: Some(TagColor::Success),
                        closable: true,
                        children: rsx!("点击关闭")
                    }
                }
            }

            // 可选中标签
            DemoSection {
                title: "可选中标签",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "点击标签切换选中状态："
                    }
                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Tag {
                            checkable: true,
                            checked: Some(*checked.read()),
                            on_change: {
                                let mut sig = checked;
                                move |next| sig.set(next)
                            },
                            children: rsx!("可选中标签")
                        }
                        Tag {
                            checkable: true,
                            default_checked: Some(true),
                            children: rsx!("默认选中")
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 状态标签示例
            DemoSection {
                title: "状态标签示例",
                Card {
                    title: Some(rsx!("订单状态")),
                    children: rsx! {
                        div {
                            style: "display: flex; gap: 8px; flex-wrap: wrap;",
                            Tag { color: Some(TagColor::Success), children: rsx!("已完成") }
                            Tag { color: Some(TagColor::Warning), children: rsx!("进行中") }
                            Tag { color: Some(TagColor::Error), children: rsx!("已关闭") }
                            Tag { children: rsx!("待处理") }
                        }
                    }
                }
            }

            // 图标标签
            DemoSection {
                title: "图标标签",
                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    Tag {
                        color: Some(TagColor::Primary),
                        children: rsx! {
                            span { "带图标" }
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
