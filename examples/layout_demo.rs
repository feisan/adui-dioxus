//! Layout 组件演示
//!
//! 展示 Layout 组件的基础用法和高级用法，包括：
//! - Layout 基础布局（Header/Content/Footer）
//! - Sider 侧边栏（可折叠、主题、零宽度触发）
//! - 响应式布局

use adui_dioxus::{
    Button, ButtonType, Content, Footer, Header, Layout, Sider, SiderTheme, ThemeMode,
    ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            LayoutDemo {}
        }
    }
}

#[component]
fn LayoutDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let sider_collapsed = use_signal(|| false);
    let mini_collapsed = use_signal(|| true);

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

            // 基础布局
            DemoSection {
                title: "基础布局（Header + Content + Footer）",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Layout {
                        Header {
                            div {
                                style: "padding: 16px; background: var(--adui-color-primary-bg); border-radius: var(--adui-radius); text-align: center; color: var(--adui-color-text);",
                                "Header - 顶部导航区域"
                            }
                        }
                        Content {
                            div {
                                style: "padding: 24px; min-height: 200px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius);",
                                "Content - 主要内容区域"
                            }
                        }
                        Footer {
                            div {
                                style: "padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); text-align: center; color: var(--adui-color-text-secondary);",
                                "Footer - 底部区域"
                            }
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 带侧边栏的布局
            DemoSection {
                title: "带侧边栏的布局",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; gap: 8px; align-items: center;",
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let mut sig = sider_collapsed;
                                move |_| {
                                    let current = *sig.read();
                                    sig.set(!current);
                                }
                            },
                            if *sider_collapsed.read() { "展开侧边栏" } else { "收起侧边栏" }
                        }
                    }
                    Layout {
                        has_sider: Some(true),
                        Sider {
                            collapsible: true,
                            theme: SiderTheme::Dark,
                            width: Some(220.0),
                            collapsed_width: Some(72.0),
                            collapsed: Some(*sider_collapsed.read()),
                            on_collapse: {
                                let mut sig = sider_collapsed;
                                move |next| sig.set(next)
                            },
                            div {
                                style: "padding: 16px; color: #fff;",
                                "侧边栏内容"
                            }
                        }
                        Layout {
                            Header {
                                div {
                                    style: "padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius);",
                                    "Header"
                                }
                            }
                            Content {
                                div {
                                    style: "padding: 24px; min-height: 200px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius);",
                                    "Content"
                                }
                            }
                            Footer {
                                div {
                                    style: "padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); text-align: center;",
                                    "Footer"
                                }
                            }
                        }
                    }
                }
            }

            // 浅色主题侧边栏
            DemoSection {
                title: "浅色主题侧边栏",
                Layout {
                    has_sider: Some(true),
                    Sider {
                        theme: SiderTheme::Light,
                        collapsible: true,
                        width: Some(200.0),
                        collapsed_width: Some(80.0),
                        collapsed: Some(*sider_collapsed.read()),
                        on_collapse: {
                            let mut sig = sider_collapsed;
                            move |next| sig.set(next)
                        },
                        div {
                            style: "padding: 16px;",
                            "浅色侧边栏"
                        }
                    }
                    Content {
                        div {
                            style: "padding: 24px; min-height: 200px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius);",
                            "主要内容区域"
                        }
                    }
                }
            }

            // 零宽度触发
            DemoSection {
                title: "零宽度触发（collapsed_width = 0）",
                div {
                    style: "display: flex; gap: 8px; align-items: center; margin-bottom: 8px;",
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let mut sig = mini_collapsed;
                            move |_| {
                                let current = *sig.read();
                                sig.set(!current);
                            }
                        },
                        if *mini_collapsed.read() { "展开" } else { "收起" }
                    }
                }
                Layout {
                    has_sider: Some(true),
                    Sider {
                        theme: SiderTheme::Light,
                        collapsible: true,
                        collapsed_width: Some(0.0),
                        width: Some(180.0),
                        zero_width_trigger_style: Some("top: 24px;".into()),
                        collapsed: Some(*mini_collapsed.read()),
                        on_collapse: {
                            let mut sig = mini_collapsed;
                            move |next| sig.set(next)
                        },
                        div {
                            style: "padding: 16px;",
                            "零宽度侧边栏"
                        }
                    }
                    Content {
                        div {
                            style: "padding: 24px; min-height: 200px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius);",
                            "内容区域 - 侧边栏收起时宽度为0"
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
