//! Tour component demonstration
//!
//! Run with: cargo run --example tour_demo

use adui_dioxus::{
    Button, ButtonColor, ButtonType, Card, THEME_BASE_STYLE, Tag, TagColor, Theme, ThemeMode,
    ThemeProvider, TooltipPlacement, Tour, TourStep, TourType, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {}
        }
    }
}

#[component]
fn App() -> Element {
    let theme = use_theme();
    let mut theme_mode = use_signal(|| ThemeMode::Light);

    use_effect(move || {
        let mode_val = theme_mode();
        let next = match mode_val {
            ThemeMode::Light => Theme::light(),
            ThemeMode::Dark => Theme::dark(),
            ThemeMode::Custom => Theme::light(),
        };
        theme.set_theme(next);
    });

    rsx! {
        style { {THEME_BASE_STYLE} }
        div {
            style: "min-height: 100vh; background: linear-gradient(135deg, var(--adui-color-bg-layout) 0%, var(--adui-color-bg-container) 100%);",
            // Hero Header
            div {
                style: "background: linear-gradient(135deg, #722ed1 0%, #531dab 100%); padding: 48px 24px; color: white;",
                div { style: "max-width: 1200px; margin: 0 auto;",
                    div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",
                        div { style: "display: flex; align-items: center; gap: 12px;",
                            span { style: "font-size: 32px;", "🎯" }
                            h1 { style: "margin: 0; font-size: 28px; font-weight: 600;", "Tour 漫游式引导" }
                        }
                        Button {
                            ghost: true,
                            onclick: move |_| {
                                theme_mode.set(match theme_mode() {
                                    ThemeMode::Light => ThemeMode::Dark,
                                    ThemeMode::Dark => ThemeMode::Light,
                                    ThemeMode::Custom => ThemeMode::Light,
                                });
                            },
                            if theme_mode() == ThemeMode::Dark { "☀️ 切换亮色" } else { "🌙 切换暗色" }
                        }
                    }
                    p { style: "margin: 0; opacity: 0.9; font-size: 16px; max-width: 600px;",
                        "用于分步引导用户了解产品功能的组件，提供友好的新手引导体验，支持多种定位方式和丰富的自定义选项。"
                    }
                }
            }

            // Content
            div { style: "max-width: 1200px; margin: 0 auto; padding: 32px 24px;",
                // Feature Cards
                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 16px; margin-bottom: 40px;",
                    FeatureCard { icon: "🚀", title: "简单易用", description: "开箱即用的步骤引导" }
                    FeatureCard { icon: "🎨", title: "样式丰富", description: "支持多种主题风格" }
                    FeatureCard { icon: "📍", title: "灵活定位", description: "上下左右四种位置" }
                    FeatureCard { icon: "⌨️", title: "键盘支持", description: "完善的键盘导航" }
                }

                // Basic Tour
                DemoSection {
                    title: "基础用法",
                    description: "最简单的用法，点击按钮开始引导，支持键盘操作（← → Enter Esc）。",
                    BasicTourDemo {}
                }

                // Primary Tour
                DemoSection {
                    title: "主要类型",
                    description: "使用主色调背景，更加醒目突出，适合重要的功能引导。",
                    PrimaryTourDemo {}
                }

                // Placement Tour
                DemoSection {
                    title: "位置变换",
                    description: "支持上、下、左、右四种定位方式，根据内容和目标元素选择合适的位置。",
                    PlacementTourDemo {}
                }

                // Custom Buttons
                DemoSection {
                    title: "自定义按钮",
                    description: "可以为每个步骤自定义按钮文字，适应不同的引导场景。",
                    CustomButtonsTourDemo {}
                }

                // With Cover
                DemoSection {
                    title: "带封面图",
                    description: "添加封面图片展示更多视觉信息，适合功能介绍和产品展示。",
                    CoverTourDemo {}
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div {
            style: "padding: 20px; background: var(--adui-color-bg-container); border-radius: 12px; border: 1px solid var(--adui-color-border); text-align: center; transition: all 0.2s; cursor: default;",
            div { style: "font-size: 28px; margin-bottom: 8px;", "{icon}" }
            div { style: "font-weight: 600; color: var(--adui-color-text); margin-bottom: 4px;", "{title}" }
            div { style: "font-size: 12px; color: var(--adui-color-text-secondary);", "{description}" }
        }
    }
}

#[component]
fn DemoSection(title: &'static str, description: &'static str, children: Element) -> Element {
    rsx! {
        div { style: "margin-bottom: 40px;",
            div { style: "margin-bottom: 16px;",
                h3 { style: "margin: 0 0 8px 0; font-size: 18px; font-weight: 600; color: var(--adui-color-text);", "{title}" }
                p { style: "margin: 0; color: var(--adui-color-text-secondary); font-size: 14px;", "{description}" }
            }
            Card {
                children: rsx! { {children} }
            }
        }
    }
}

#[component]
fn BasicTourDemo() -> Element {
    let mut open = use_signal(|| false);
    let mut completed_count = use_signal(|| 0u32);

    let steps = vec![
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
    ];

    rsx! {
        div { style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
            Button {
                r#type: ButtonType::Primary,
                onclick: move |_| {
                    open.set(true);
                },
                "🚀 开始引导"
            }
            if *completed_count.read() > 0 {
                Tag {
                    color: Some(TagColor::Success),
                    children: rsx! { "已完成 {completed_count.read()} 次" }
                }
            }
            div { style: "flex: 1;", }
            div { style: "display: flex; gap: 8px;",
                Tag { children: rsx! { "← → 切换步骤" } }
                Tag { children: rsx! { "Enter 下一步" } }
                Tag { children: rsx! { "Esc 关闭" } }
            }
        }
        Tour {
            open: open(),
            steps: steps,
            on_close: move |_| {
                open.set(false);
            },
            on_finish: move |_| {
                open.set(false);
                let current = *completed_count.read();
                completed_count.set(current + 1);
            },
        }
    }
}

#[component]
fn PrimaryTourDemo() -> Element {
    let mut open = use_signal(|| false);

    let steps = vec![
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
    ];

    rsx! {
        div { style: "display: flex; align-items: center; gap: 16px;",
            Button {
                color: Some(ButtonColor::Primary),
                onclick: move |_| {
                    open.set(true);
                },
                "💜 主要风格引导"
            }
            div { style: "padding: 8px 16px; background: linear-gradient(135deg, #722ed1 0%, #531dab 100%); border-radius: 6px; color: white; font-size: 12px;",
                "主色调背景 · 更醒目"
            }
        }
        Tour {
            open: open(),
            steps: steps,
            r#type: TourType::Primary,
            on_close: move |_| {
                open.set(false);
            },
            on_finish: move |_| {
                open.set(false);
            },
        }
    }
}

#[component]
fn PlacementTourDemo() -> Element {
    let mut open = use_signal(|| false);

    let steps = vec![
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
    ];

    rsx! {
        div { style: "display: flex; align-items: center; gap: 16px;",
            Button {
                r#type: ButtonType::Primary,
                onclick: move |_| {
                    open.set(true);
                },
                "🧭 查看不同位置"
            }
            div { style: "display: flex; gap: 8px;",
                Tag { color: Some(TagColor::Primary), children: rsx! { "Top" } }
                Tag { color: Some(TagColor::Success), children: rsx! { "Right" } }
                Tag { color: Some(TagColor::Warning), children: rsx! { "Bottom" } }
                Tag { color: Some(TagColor::Error), children: rsx! { "Left" } }
            }
        }
        Tour {
            open: open(),
            steps: steps,
            on_close: move |_| {
                open.set(false);
            },
            on_finish: move |_| {
                open.set(false);
            },
        }
    }
}

#[component]
fn CustomButtonsTourDemo() -> Element {
    let mut open = use_signal(|| false);

    let steps = vec![
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
    ];

    rsx! {
        div { style: "display: flex; align-items: center; gap: 16px;",
            Button {
                r#type: ButtonType::Primary,
                onclick: move |_| {
                    open.set(true);
                },
                "✏️ 自定义按钮文字"
            }
            div { style: "color: var(--adui-color-text-secondary); font-size: 13px;",
                "按钮文字: 继续探索 → / ← 返回上一步"
            }
        }
        Tour {
            open: open(),
            steps: steps,
            finish_button_text: "完成引导 ✓".to_string(),
            on_close: move |_| {
                open.set(false);
            },
            on_finish: move |_| {
                open.set(false);
            },
        }
    }
}

#[component]
fn CoverTourDemo() -> Element {
    let mut open = use_signal(|| false);

    let steps = vec![
        TourStep {
            key: "step1".into(),
            title: Some("🎨 封面引导".into()),
            description: Some(rsx! {
                div { style: "line-height: 1.6;",
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
                div { style: "line-height: 1.6;",
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
        TourStep {
            key: "step3".into(),
            title: Some("🎉 引导完成".into()),
            description: Some(rsx! {
                div { style: "line-height: 1.6;",
                    "恭喜你完成了带封面的引导体验！现在你可以在自己的项目中使用这个功能了。"
                }
            }),
            cover: Some(rsx! {
                div {
                    style: "width: 100%; height: 160px; background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); border-radius: 8px; display: flex; flex-direction: column; align-items: center; justify-content: center; color: white;",
                    div { style: "font-size: 40px; margin-bottom: 8px;", "🎉" }
                    div { style: "font-size: 14px; opacity: 0.9;", "恭喜完成！" }
                }
            }),
            placement: None,
            target: None,
            next_button_text: None,
            prev_button_text: None,
        },
    ];

    rsx! {
        div { style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
            Button {
                r#type: ButtonType::Primary,
                onclick: move |_| {
                    open.set(true);
                },
                "🖼️ 带封面图的引导"
            }
            // Preview cards
            div { style: "display: flex; gap: 8px;",
                div { style: "width: 40px; height: 40px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 16px;", "🎨" }
                div { style: "width: 40px; height: 40px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 16px;", "📊" }
                div { style: "width: 40px; height: 40px; background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 16px;", "🎉" }
            }
        }
        Tour {
            open: open(),
            steps: steps,
            on_close: move |_| {
                open.set(false);
            },
            on_finish: move |_| {
                open.set(false);
            },
        }
    }
}
