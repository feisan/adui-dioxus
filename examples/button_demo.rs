//! Button 组件演示
//!
//! 展示 Button 组件的基础用法和高级用法，包括：
//! - 基础类型（Primary、Default、Dashed、Text、Link）
//! - 尺寸和形状
//! - 状态（Loading、Danger、Ghost、Block）
//! - 图标按钮
//! - 按钮组

use adui_dioxus::{
    Button, ButtonColor, ButtonGroup, ButtonIconPlacement, ButtonShape, ButtonSize, ButtonType,
    ButtonVariant, Icon, IconKind, Theme, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

const PRIMARY_PRESETS: [(&str, &str, &str, &str); 3] = [
    ("Blue", "#1677ff", "#4096ff", "#0958d9"),
    ("Cyan", "#13c2c2", "#36cfc9", "#08979c"),
    ("Orange", "#fa8c16", "#ffa940", "#d46b08"),
];

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ButtonDemo {}
        }
    }
}

#[component]
fn ButtonDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let preset = use_signal(|| 0usize);

    use_effect(move || {
        let idx = *preset.read();
        let mode_val = *mode.read();
        let (_label, base, hover, active) = PRIMARY_PRESETS[idx];
        let mut next = Theme::for_mode(mode_val);
        next.tokens.color_primary = base.to_string();
        next.tokens.color_primary_hover = hover.to_string();
        next.tokens.color_primary_active = active.to_string();
        next.tokens.color_link = base.to_string();
        next.tokens.color_link_hover = hover.to_string();
        next.tokens.color_link_active = active.to_string();
        if matches!(mode_val, ThemeMode::Dark) {
            next.tokens.color_bg_base = "#0f0f0f".into();
            next.tokens.color_bg_container = "#1b1b1b".into();
        }
        theme.set_theme(next);
    });

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",

            // 主题控制工具栏
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 24px; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border);",
                span { style: "font-weight: 600;", "主题控制：" }
                ButtonGroup {
                    size: Some(ButtonSize::Small),
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
                span { style: "margin-left: 12px;", "主色：" }
                {
                    PRIMARY_PRESETS
                        .iter()
                        .enumerate()
                        .map(|(idx, (label, ..))| {
                            let mut preset = preset;
                            rsx!(
                                Button {
                                    r#type: ButtonType::Text,
                                    size: ButtonSize::Small,
                                    onclick: move |_| *preset.write() = idx,
                                    class: if idx == *preset.read() { Some("adui-btn-active".into()) } else { None },
                                    "{label}"
                                }
                            )
                        })
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础类型
            DemoSection {
                title: "按钮类型",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    Button { r#type: ButtonType::Primary, "Primary" }
                    Button { r#type: ButtonType::Default, "Default" }
                    Button { r#type: ButtonType::Dashed, "Dashed" }
                    Button { r#type: ButtonType::Text, "Text" }
                    Button { r#type: ButtonType::Link, "Link" }
                }
            }

            // 尺寸
            DemoSection {
                title: "按钮尺寸",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center;",
                    Button { r#type: ButtonType::Primary, size: ButtonSize::Small, "Small" }
                    Button { r#type: ButtonType::Primary, size: ButtonSize::Middle, "Middle" }
                    Button { r#type: ButtonType::Primary, size: ButtonSize::Large, "Large" }
                }
            }

            // 形状
            DemoSection {
                title: "按钮形状",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center;",
                    Button { r#type: ButtonType::Primary, shape: ButtonShape::Default, "Default" }
                    Button { r#type: ButtonType::Primary, shape: ButtonShape::Round, "Round" }
                    Button {
                        r#type: ButtonType::Primary,
                        shape: ButtonShape::Circle,
                        icon: rsx!(Icon { kind: IconKind::Search }),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 状态：Loading
            DemoSection {
                title: "加载状态",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    Button { r#type: ButtonType::Primary, loading: true, "Loading" }
                    Button { r#type: ButtonType::Default, loading: true, "Loading" }
                    Button { r#type: ButtonType::Primary, loading: true, "Click me" }
                }
            }

            // 状态：Danger
            DemoSection {
                title: "危险按钮",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    Button { r#type: ButtonType::Primary, danger: true, "Danger Primary" }
                    Button { r#type: ButtonType::Default, danger: true, "Danger Default" }
                    Button { r#type: ButtonType::Dashed, danger: true, "Danger Dashed" }
                    Button { r#type: ButtonType::Text, danger: true, "Danger Text" }
                }
            }

            // 状态：Ghost
            DemoSection {
                title: "幽灵按钮",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px; padding: 16px; background: var(--adui-color-primary-bg); border-radius: var(--adui-radius);",
                    Button { r#type: ButtonType::Primary, ghost: true, "Ghost Primary" }
                    Button { r#type: ButtonType::Default, ghost: true, "Ghost Default" }
                    Button { r#type: ButtonType::Dashed, ghost: true, "Ghost Dashed" }
                }
            }

            // 状态：Block
            DemoSection {
                title: "块级按钮",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; max-width: 300px;",
                    Button { r#type: ButtonType::Primary, block: true, "Block Button" }
                    Button { r#type: ButtonType::Default, block: true, "Block Default" }
                }
            }

            // 图标按钮
            DemoSection {
                title: "图标按钮",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    Button {
                        r#type: ButtonType::Primary,
                        icon: rsx!(Icon { kind: IconKind::Search }),
                        "Search"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        icon: rsx!(Icon { kind: IconKind::Plus }),
                        "Add"
                    }
                    Button {
                        r#type: ButtonType::Primary,
                        icon_placement: ButtonIconPlacement::End,
                        icon: rsx!(Icon { kind: IconKind::ArrowRight }),
                        "Next"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        shape: ButtonShape::Circle,
                        icon: rsx!(Icon { kind: IconKind::Info }),
                    }
                }
            }

            // 按钮组
            DemoSection {
                title: "按钮组",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 12px;",
                        ButtonGroup {
                            size: Some(ButtonSize::Small),
                            variant: Some(ButtonVariant::Solid),
                            color: Some(ButtonColor::Primary),
                            Button {
                                icon: rsx!(Icon { kind: IconKind::ArrowLeft }),
                                label: Some("上一页".into()),
                            }
                            Button {
                                icon: rsx!(Icon { kind: IconKind::Check }),
                                label: Some("刷新".into()),
                            }
                            Button {
                                icon_placement: ButtonIconPlacement::End,
                                icon: rsx!(Icon { kind: IconKind::ArrowRight }),
                                label: Some("下一页".into()),
                            }
                        }
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 12px;",
                        ButtonGroup {
                            size: Some(ButtonSize::Middle),
                            Button { r#type: ButtonType::Primary, "Left" }
                            Button { r#type: ButtonType::Default, "Middle" }
                            Button { r#type: ButtonType::Default, "Right" }
                        }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        Button {
                            r#type: ButtonType::Primary,
                            danger: true,
                            loading: true,
                            icon: rsx!(Icon { kind: IconKind::Close }),
                            "Danger Loading"
                        }
                        Button {
                            r#type: ButtonType::Primary,
                            ghost: true,
                            block: true,
                            icon: rsx!(Icon { kind: IconKind::Check }),
                            "Ghost Block"
                        }
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        Button {
                            r#type: ButtonType::Link,
                            href: Some("https://ant.design".to_string()),
                            icon: rsx!(Icon { kind: IconKind::ArrowRight }),
                            "Visit Ant Design"
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
