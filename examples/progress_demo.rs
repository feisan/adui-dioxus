//! Progress 组件演示
//!
//! 展示 Progress 组件的基础用法和高级用法，包括：
//! - 基础进度条
//! - 不同状态
//! - 圆形进度条
//! - 自定义样式

use adui_dioxus::{
    Button, ButtonType, Progress, ProgressStatus, ProgressType, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ProgressDemo {}
        }
    }
}

#[component]
fn ProgressDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let percent = use_signal(|| 75.0f32);

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
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut p = percent;
                        move |_| {
                            let current = *p.read();
                            p.set((current + 10.0).min(100.0));
                        }
                    },
                    "增加进度"
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础进度条
            DemoSection {
                title: "基础进度条",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Progress {
                        percent: 30.0,
                    }
                    Progress {
                        percent: 50.0,
                    }
                    Progress {
                        percent: 80.0,
                    }
                    Progress {
                        percent: 100.0,
                    }
                }
            }

            // 不同状态
            DemoSection {
                title: "不同状态",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Progress {
                        percent: 50.0,
                        status: Some(ProgressStatus::Normal),
                    }
                    Progress {
                        percent: 100.0,
                        status: Some(ProgressStatus::Success),
                    }
                    Progress {
                        percent: 50.0,
                        status: Some(ProgressStatus::Exception),
                    }
                    Progress {
                        percent: 50.0,
                        status: Some(ProgressStatus::Active),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 圆形进度条
            DemoSection {
                title: "圆形进度条",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 24px; align-items: center;",
                    Progress {
                        r#type: ProgressType::Circle,
                        percent: 30.0,
                    }
                    Progress {
                        r#type: ProgressType::Circle,
                        percent: 50.0,
                    }
                    Progress {
                        r#type: ProgressType::Circle,
                        percent: 80.0,
                    }
                    Progress {
                        r#type: ProgressType::Circle,
                        percent: 100.0,
                        status: Some(ProgressStatus::Success),
                    }
                }
            }

            // 不显示百分比
            DemoSection {
                title: "不显示百分比",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Progress {
                        percent: 50.0,
                        show_info: false,
                    }
                    Progress {
                        r#type: ProgressType::Circle,
                        percent: 75.0,
                        show_info: false,
                    }
                }
            }

            // 自定义宽度
            DemoSection {
                title: "自定义宽度",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    Progress {
                        percent: 50.0,
                        stroke_width: Some(8.0),
                    }
                    Progress {
                        percent: 70.0,
                        stroke_width: Some(12.0),
                    }
                }
            }

            // 动态进度
            DemoSection {
                title: "动态进度（点击按钮增加）",
                Progress {
                    percent: *percent.read(),
                    status: if *percent.read() >= 100.0 {
                        Some(ProgressStatus::Success)
                    } else {
                        Some(ProgressStatus::Active)
                    },
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "文件上传进度" }
                        Progress {
                            percent: 65.0,
                            status: Some(ProgressStatus::Active),
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "处理完成" }
                        Progress {
                            percent: 100.0,
                            status: Some(ProgressStatus::Success),
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "处理失败" }
                        Progress {
                            percent: 45.0,
                            status: Some(ProgressStatus::Exception),
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
