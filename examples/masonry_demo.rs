//! Masonry 组件演示
//!
//! 展示 Masonry 组件的基础用法和高级用法，包括：
//! - 基础瀑布流布局
//! - 响应式列数
//! - 自定义间距
//! - 最小列宽

use adui_dioxus::{
    Button, ButtonType, Masonry, MasonryResponsive, ThemeMode, ThemeProvider, Title, TitleLevel,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            MasonryDemo {}
        }
    }
}

#[component]
fn MasonryDemo() -> Element {
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

            // 基础瀑布流
            DemoSection {
                title: "基础瀑布流布局",
                Masonry {
                    columns: 4,
                    gap: Some(12.0),
                    row_gap: Some(16.0),
                    {
                        (0..12).map(|i| {
                            let height = 80 + (i % 5) * 40;
                            rsx! {
                                div {
                                    style: format!("background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; height: {}px; display: flex; align-items: center; justify-content: center;", height),
                                    "卡片 {i + 1}"
                                }
                            }
                        })
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 响应式列数
            DemoSection {
                title: "响应式列数",
                div {
                    style: "margin-bottom: 12px; padding: 8px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                    "在不同屏幕尺寸下自动调整列数：xs(1列) → sm(2列) → md(3列) → lg(4列)"
                }
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
                            let height = 100 + (i % 4) * 30;
                            rsx! {
                                div {
                                    style: format!("background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; height: {}px; display: flex; align-items: center; justify-content: center;", height),
                                    "响应式卡片 {i + 1}"
                                }
                            }
                        })
                    }
                }
            }

            // 自定义间距
            DemoSection {
                title: "自定义间距",
                Masonry {
                    columns: 3,
                    gap: Some(24.0),
                    row_gap: Some(32.0),
                    {
                        (0..9).map(|i| {
                            let height = 120 + (i % 3) * 40;
                            rsx! {
                                div {
                                    style: format!("background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 20px; height: {}px; display: flex; align-items: center; justify-content: center;", height),
                                    "大间距卡片 {i + 1}"
                                }
                            }
                        })
                    }
                }
            }

            // 最小列宽
            DemoSection {
                title: "最小列宽（min_column_width = 200px）",
                div {
                    style: "margin-bottom: 12px; padding: 8px; background: var(--adui-color-bg-base); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                    "当容器宽度不足以容纳设定列数时，会自动减少列数以满足最小列宽要求"
                }
                Masonry {
                    columns: 4,
                    gap: Some(12.0),
                    row_gap: Some(16.0),
                    min_column_width: Some(200.0),
                    {
                        (0..12).map(|i| {
                            let height = 90 + (i % 5) * 35;
                            rsx! {
                                div {
                                    style: format!("background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; height: {}px; display: flex; align-items: center; justify-content: center;", height),
                                    "卡片 {i + 1}"
                                }
                            }
                        })
                    }
                }
            }

            // 组合示例 - 图片墙
            DemoSection {
                title: "组合示例 - 图片墙",
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
                    row_gap: Some(12.0),
                    {
                        (0..16).map(|i| {
                            let height = 150 + (i % 6) * 50;
                            rsx! {
                                div {
                                    style: format!("background: linear-gradient(135deg, var(--adui-color-primary-bg), var(--adui-color-bg-container)); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 16px; height: {}px; display: flex; flex-direction: column; justify-content: space-between;", height),
                                    div {
                                        style: "font-weight: 600; margin-bottom: 8px;",
                                        "图片 {i + 1}"
                                    }
                                    div {
                                        style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                        "描述信息"
                                    }
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
