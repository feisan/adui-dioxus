//! 布局组件综合演示
//!
//! 展示所有布局组件的综合使用，包括：
//! - Layout + Sider 组合
//! - Grid 响应式布局
//! - Flex 和 Space 组合
//! - Splitter 分割面板
//! - Masonry 瀑布流

use adui_dioxus::{
    Button, ButtonType, Col, Content, Flex, FlexDirection, FlexJustify, Header, Layout, Masonry,
    MasonryResponsive, Row, Sider, SiderTheme, Space, SpaceDirection, Splitter,
    SplitterOrientation, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            LayoutComponentsDemo {}
        }
    }
}

#[component]
fn LayoutComponentsDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let sider_collapsed = use_signal(|| false);
    let split_ratio = use_signal(|| 0.3f32);

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

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "布局组件综合展示" }

            // 完整页面布局
            DemoSection {
                title: "完整页面布局（Layout + Sider + Grid）",
                Layout {
                    has_sider: Some(true),
                    Sider {
                        collapsible: true,
                        theme: SiderTheme::Dark,
                        width: Some(200.0),
                        collapsed_width: Some(80.0),
                        collapsed: Some(*sider_collapsed.read()),
                        on_collapse: {
                            let mut sig = sider_collapsed;
                            move |next| sig.set(next)
                        },
                        div {
                            style: "padding: 16px; color: #fff;",
                            "导航菜单"
                        }
                    }
                    Layout {
                        Header {
                            div {
                                style: "padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius);",
                                "Header - 顶部导航栏"
                            }
                        }
                        Content {
                            div {
                                style: "padding: 24px;",
                                Row {
                                    gutter: Some(16.0),
                                    Col {
                                        span: 12,
                                        div {
                                            style: "padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); min-height: 200px;",
                                            "左侧内容区域"
                                        }
                                    }
                                    Col {
                                        span: 12,
                                        div {
                                            style: "padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); min-height: 200px;",
                                            "右侧内容区域"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Flex 和 Space 组合
            DemoSection {
                title: "Flex 和 Space 组合使用",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);",
                        div {
                            style: "font-weight: 600; margin-bottom: 12px;",
                            "Flex 布局"
                        }
                        Flex {
                            direction: FlexDirection::Row,
                            justify: FlexJustify::Between,
                            gap: Some(12.0),
                            div {
                                style: "padding: 12px; background: var(--adui-color-primary-bg); border-radius: var(--adui-radius);",
                                "Flex Item 1"
                            }
                            div {
                                style: "padding: 12px; background: var(--adui-color-primary-bg); border-radius: var(--adui-radius);",
                                "Flex Item 2"
                            }
                            div {
                                style: "padding: 12px; background: var(--adui-color-primary-bg); border-radius: var(--adui-radius);",
                                "Flex Item 3"
                            }
                        }
                    }
                    div {
                        style: "padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);",
                        div {
                            style: "font-weight: 600; margin-bottom: 12px;",
                            "Space 布局"
                        }
                        Space {
                            direction: SpaceDirection::Horizontal,
                            gap: Some(12.0),
                            Button { r#type: ButtonType::Primary, "确定" }
                            Button { r#type: ButtonType::Default, "取消" }
                            Button { r#type: ButtonType::Text, "更多" }
                        }
                    }
                }
            }

            // Splitter 分割面板
            DemoSection {
                title: "Splitter 分割面板",
                div {
                    style: "height: 300px;",
                    Splitter {
                        orientation: SplitterOrientation::Horizontal,
                        split: Some(*split_ratio.read()),
                        on_change: {
                            let mut sig = split_ratio;
                            move |v| sig.set(v)
                        },
                        first: rsx! {
                            div {
                                style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                "左侧面板"
                            }
                        },
                        second: rsx! {
                            div {
                                style: "height: 100%; padding: 16px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); display: flex; align-items: center; justify-content: center;",
                                "右侧面板"
                            }
                        },
                    }
                }
            }

            // Masonry 瀑布流
            DemoSection {
                title: "Masonry 瀑布流布局",
                Masonry {
                    columns: 4,
                    responsive: Some(MasonryResponsive {
                        xs: Some(1),
                        sm: Some(2),
                        md: Some(3),
                        lg: Some(4),
                        ..Default::default()
                    }),
                    gap: Some(12.0),
                    row_gap: Some(16.0),
                    {
                        (0..12).map(|i| {
                            let height = 100 + (i % 5) * 40;
                            let style_str = format!("background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; height: {}px; display: flex; align-items: center; justify-content: center;", height);
                            let label = format!("卡片 {}", i + 1);
                            rsx! {
                                div {
                                    style: {style_str},
                                    {label}
                                }
                            }
                        })
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
