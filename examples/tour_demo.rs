//! Tour 组件演示
//!
//! 展示 Tour 组件的基础用法和高级用法，包括：
//! - 基础引导
//! - 主要类型
//! - 位置变换
//! - 自定义按钮
//! - 带封面图

use adui_dioxus::{
    Button, ButtonType, Card, Tag, TagColor, ThemeMode, ThemeProvider, Title, TitleLevel,
    TooltipPlacement, Tour, TourStep, TourType, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TourDemo {}
        }
    }
}

#[component]
fn TourDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_open = use_signal(|| false);
    let primary_open = use_signal(|| false);
    let placement_open = use_signal(|| false);
    let custom_open = use_signal(|| false);
    let cover_open = use_signal(|| false);
    let completed_count = use_signal(|| 0u32);

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

            // 基础引导
            DemoSection {
                title: "基础引导",
                div {
                    style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = basic_open;
                            move |_| sig.set(true)
                        },
                        "🚀 开始引导"
                    }
                    if *completed_count.read() > 0 {
                        Tag {
                            color: Some(TagColor::Success),
                            children: rsx! { "已完成 {completed_count.read()} 次" }
                        }
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        Tag { children: rsx! { "← → 切换步骤" } }
                        Tag { children: rsx! { "Enter 下一步" } }
                        Tag { children: rsx! { "Esc 关闭" } }
                    }
                }
            }

            // 主要类型
            DemoSection {
                title: "主要类型",
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = primary_open;
                            move |_| sig.set(true)
                        },
                        "💜 主要风格引导"
                    }
                }
            }

            // 位置变换
            DemoSection {
                title: "位置变换",
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = placement_open;
                            move |_| sig.set(true)
                        },
                        "🧭 查看不同位置"
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        Tag { color: Some(TagColor::Primary), children: rsx! { "Top" } }
                        Tag { color: Some(TagColor::Success), children: rsx! { "Right" } }
                        Tag { color: Some(TagColor::Warning), children: rsx! { "Bottom" } }
                        Tag { color: Some(TagColor::Error), children: rsx! { "Left" } }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 自定义按钮
            DemoSection {
                title: "自定义按钮",
                div {
                    style: "display: flex; align-items: center; gap: 16px;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = custom_open;
                            move |_| sig.set(true)
                        },
                        "✏️ 自定义按钮文字"
                    }
                }
            }

            // 带封面图
            DemoSection {
                title: "带封面图",
                div {
                    style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = cover_open;
                            move |_| sig.set(true)
                        },
                        "🖼️ 带封面图的引导"
                    }
                }
            }
        }

        // 基础引导 Tour
        Tour {
            open: *basic_open.read(),
            steps: vec![
                TourStep::new(
                    "step1",
                    "👋 欢迎",
                    "欢迎使用 ADUI 组件库！这是一个基于 Dioxus 的 Ant Design 风格组件库。",
                ),
                TourStep::new(
                    "step2",
                    "🎨 主题系统",
                    "支持亮色和暗色主题切换，以及自定义主色调，让你的应用更加个性化。",
                ),
                TourStep::new(
                    "step3",
                    "🧩 丰富的组件",
                    "包含 70+ 常用组件，涵盖布局、表单、数据展示等场景，助力快速开发。",
                ),
                TourStep::new("step4", "🎉 开始探索", "现在就开始探索这些精美的组件吧！"),
            ],
            on_close: {
                let mut sig = basic_open;
                move |_| sig.set(false)
            },
            on_finish: {
                let mut sig_open = basic_open;
                let mut sig_count = completed_count;
                move |_| {
                    sig_open.set(false);
                    let current = *sig_count.read();
                    sig_count.set(current + 1);
                }
            },
        }

        // 主要类型 Tour
        Tour {
            open: *primary_open.read(),
            steps: vec![
                TourStep::new(
                    "step1",
                    "✨ 主要风格",
                    "这是主要风格的引导组件，使用主色调作为背景，视觉效果更强烈。",
                ),
                TourStep::new(
                    "step2",
                    "👁️ 更醒目",
                    "主要风格更加醒目，适合重要的引导场景和关键功能介绍。",
                ),
                TourStep::new("step3", "🎯 体验完成", "你已经体验了主要风格的引导组件！"),
            ],
            r#type: TourType::Primary,
            on_close: {
                let mut sig = primary_open;
                move |_| sig.set(false)
            },
            on_finish: {
                let mut sig = primary_open;
                move |_| sig.set(false)
            },
        }

        // 位置变换 Tour
        Tour {
            open: *placement_open.read(),
            steps: vec![
                TourStep::new(
                    "top",
                    "⬆️ 顶部位置",
                    "引导面板显示在目标元素的上方，适合底部有更多内容的场景。",
                )
                .placement(TooltipPlacement::Top),
                TourStep::new(
                    "right",
                    "➡️ 右侧位置",
                    "引导面板显示在目标元素的右侧，适合左侧有重要内容需要保持可见。",
                )
                .placement(TooltipPlacement::Right),
                TourStep::new(
                    "bottom",
                    "⬇️ 底部位置",
                    "引导面板显示在目标元素的下方，这是最常用的默认位置。",
                )
                .placement(TooltipPlacement::Bottom),
                TourStep::new(
                    "left",
                    "⬅️ 左侧位置",
                    "引导面板显示在目标元素的左侧，适合右侧有重要内容需要保持可见。",
                )
                .placement(TooltipPlacement::Left),
            ],
            on_close: {
                let mut sig = placement_open;
                move |_| sig.set(false)
            },
            on_finish: {
                let mut sig = placement_open;
                move |_| sig.set(false)
            },
        }

        // 自定义按钮 Tour
        Tour {
            open: *custom_open.read(),
            steps: vec![
                TourStep::new(
                    "step1",
                    "🎨 自定义按钮",
                    "你可以为每个步骤自定义按钮文字，让引导更加贴合你的产品风格。",
                )
                .next_button("继续探索 →"),
                TourStep::new(
                    "step2",
                    "📝 第二步",
                    "这里的按钮文字都是自定义的，你可以根据步骤内容设置合适的文案。",
                )
                .prev_button("← 返回上一步")
                .next_button("继续前进 →"),
                TourStep::new(
                    "step3",
                    "🏁 最后一步",
                    "完成所有步骤后，可以设置专属的完成按钮文字。",
                )
                .prev_button("← 回头看看"),
            ],
            finish_button_text: "完成引导 ✓".to_string(),
            on_close: {
                let mut sig = custom_open;
                move |_| sig.set(false)
            },
            on_finish: {
                let mut sig = custom_open;
                move |_| sig.set(false)
            },
        }

        // 带封面图 Tour
        Tour {
            open: *cover_open.read(),
            steps: vec![
                TourStep {
                    key: "step1".into(),
                    title: Some("🎨 封面引导".into()),
                    description: Some(rsx! {
                        div {
                            style: "line-height: 1.6;",
                            "封面图可以展示更多视觉信息，帮助用户更好地理解功能和特性。"
                        }
                    }),
                    cover: Some(rsx! {
                        div {
                            style: "width: 100%; height: 160px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px; display: flex; flex-direction: column; align-items: center; justify-content: center; color: white;",
                            div { style: "font-size: 40px; margin-bottom: 8px;", "🎨" }
                            div { style: "font-size: 14px; opacity: 0.9;", "精美的视觉设计" }
                        }
                    }),
                    placement: None,
                    target: None,
                    next_button_text: None,
                    prev_button_text: None,
                },
                TourStep {
                    key: "step2".into(),
                    title: Some("📊 数据可视化".into()),
                    description: Some(rsx! {
                        div {
                            style: "line-height: 1.6;",
                            "你可以放置任何内容作为封面，包括图片、图表、动画等丰富的媒体内容。"
                        }
                    }),
                    cover: Some(rsx! {
                        div {
                            style: "width: 100%; height: 160px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); border-radius: 8px; display: flex; flex-direction: column; align-items: center; justify-content: center; color: white;",
                            div { style: "font-size: 40px; margin-bottom: 8px;", "📊" }
                            div { style: "font-size: 14px; opacity: 0.9;", "丰富的数据展示" }
                        }
                    }),
                    placement: None,
                    target: None,
                    next_button_text: None,
                    prev_button_text: None,
                },
            ],
            on_close: {
                let mut sig = cover_open;
                move |_| sig.set(false)
            },
            on_finish: {
                let mut sig = cover_open;
                move |_| sig.set(false)
            },
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
