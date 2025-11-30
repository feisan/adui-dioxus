//! Icon 组件演示
//!
//! 展示 Icon 组件的基础用法和高级用法，包括：
//! - 内置图标类型
//! - 图标大小
//! - 图标颜色
//! - 图标旋转
//! - 自定义样式

use adui_dioxus::{
    Button, ButtonType, Icon, IconKind, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

const ICON_LIST: &[(IconKind, &str)] = &[
    (IconKind::Plus, "Plus"),
    (IconKind::Minus, "Minus"),
    (IconKind::Check, "Check"),
    (IconKind::Close, "Close"),
    (IconKind::Info, "Info"),
    (IconKind::Question, "Question"),
    (IconKind::Search, "Search"),
    (IconKind::ArrowLeft, "ArrowLeft"),
    (IconKind::ArrowRight, "ArrowRight"),
    (IconKind::Loading, "Loading"),
];

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            IconDemo {}
        }
    }
}

#[component]
fn IconDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let mut size = use_signal(|| 24f32);
    let mut accent = use_signal(|| "#1677ff".to_string());
    let spin_all = use_signal(|| false);

    use_effect(move || {
        let m = *mode.read();
        theme.set_mode(m);
        if matches!(m, ThemeMode::Dark) {
            let mut tokens = adui_dioxus::ThemeTokens::dark();
            tokens.color_primary = accent.read().clone();
            theme.set_theme(adui_dioxus::Theme { mode: m, tokens });
        } else {
            let mut tokens = adui_dioxus::ThemeTokens::light();
            tokens.color_primary = accent.read().clone();
            theme.set_theme(adui_dioxus::Theme { mode: m, tokens });
        }
    });

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",

            // 控制工具栏
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 24px; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border);",
                span { style: "font-weight: 600;", "控制：" }
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
                span { style: "margin-left: 12px;", "大小：" }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| {
                        let mut v = *size.read();
                        v = (v - 4.0).max(12.0);
                        size.set(v);
                    },
                    "−"
                }
                span { "{size.read():.0}px" }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| {
                        let mut v = *size.read();
                        v = (v + 4.0).min(48.0);
                        size.set(v);
                    },
                    "+"
                }
                span { style: "margin-left: 12px;", "颜色：" }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#1677ff".into()),
                    "蓝"
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#fa8c16".into()),
                    "橙"
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#13c2c2".into()),
                    "青"
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#ff4d4f".into()),
                    "红"
                }
                {
                    let mut spin_sig = spin_all;
                    let spin_label = format!("旋转 {}", if *spin_all.read() { "ON" } else { "OFF" });
                    rsx!(Button {
                        r#type: ButtonType::Text,
                        onclick: move |_| {
                            let cur = *spin_sig.read();
                            spin_sig.set(!cur);
                        },
                        {spin_label}
                    })
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 内置图标列表
            DemoSection {
                title: "内置图标",
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 12px;",
                    {
                        ICON_LIST.iter().map(|(kind, label)| {
                            let color = accent.read().clone();
                            let spinning = *spin_all.read() || matches!(kind, IconKind::Loading);
                            let size_val = *size.read();
                            let label_str = *label;
                            rsx! {
                                div {
                                    style: "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 16px; border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); background: var(--adui-color-bg-container);",
                                    Icon {
                                        kind: *kind,
                                        size: size_val,
                                        color: Some(color.clone()),
                                        spin: spinning,
                                        aria_label: Some(label_str.to_string()),
                                    }
                                    span {
                                        style: "color: var(--adui-color-text); font-size: 12px;",
                                        {label_str}
                                    }
                                }
                            }
                        })
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 不同尺寸
            DemoSection {
                title: "图标尺寸",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Search, size: 16.0, aria_label: Some("Small".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "16px" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Search, size: 24.0, aria_label: Some("Middle".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "24px" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Search, size: 32.0, aria_label: Some("Large".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "32px" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Search, size: 48.0, aria_label: Some("XLarge".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "48px" }
                    }
                }
            }

            // 不同颜色
            DemoSection {
                title: "图标颜色",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Info, size: 24.0, color: Some("#1677ff".into()), aria_label: Some("Primary".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Primary" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Check, size: 24.0, color: Some("#52c41a".into()), aria_label: Some("Success".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Success" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Question, size: 24.0, color: Some("#faad14".into()), aria_label: Some("Warning".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Warning" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Close, size: 24.0, color: Some("#ff4d4f".into()), aria_label: Some("Danger".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Danger" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Info, size: 24.0, color: Some("var(--adui-color-text-secondary)".into()), aria_label: Some("Secondary".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Secondary" }
                    }
                }
            }

            // 旋转动画
            DemoSection {
                title: "旋转动画",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Loading, size: 24.0, spin: true, aria_label: Some("Loading".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Loading (自动旋转)" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::Search, size: 24.0, spin: *spin_all.read(), aria_label: Some("Search".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Search (手动控制)" }
                    }
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 8px;",
                        Icon { kind: IconKind::ArrowRight, size: 24.0, spin: *spin_all.read(), aria_label: Some("Arrow".into()) }
                        span { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "Arrow (手动控制)" }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
                        Icon { kind: IconKind::Search, size: 20.0, color: Some(accent.read().clone()), aria_label: Some("Search icon".into()) }
                        span { "搜索功能" }
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
                        Icon { kind: IconKind::Check, size: 20.0, color: Some("#52c41a".into()), aria_label: Some("Check icon".into()) }
                        span { "操作成功" }
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
                        Icon { kind: IconKind::Close, size: 20.0, color: Some("#ff4d4f".into()), aria_label: Some("Close icon".into()) }
                        span { "操作失败" }
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 12px; align-items: center;",
                        Icon { kind: IconKind::Loading, size: 20.0, spin: true, aria_label: Some("Loading icon".into()) }
                        span { "加载中..." }
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
