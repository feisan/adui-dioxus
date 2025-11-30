//! Calendar 组件演示
//!
//! 展示 Calendar 组件的基础用法和高级用法，包括：
//! - 基础日历
//! - 日期选择
//! - 月/年视图切换
//! - 全屏模式

use adui_dioxus::{
    Button, ButtonType, Calendar, CalendarDate, CalendarMode, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            CalendarDemo {}
        }
    }
}

#[component]
fn CalendarDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let selected = use_signal(|| None::<CalendarDate>);
    let calendar_mode = use_signal(|| CalendarMode::Month);

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

            // 基础日历
            DemoSection {
                title: "基础日历",
                Calendar {
                    value: *selected.read(),
                    on_select: {
                        let mut sig = selected;
                        move |date| sig.set(Some(date))
                    },
                    fullscreen: Some(false),
                    mode: Some(*calendar_mode.read()),
                }
            }

            // 日期选择
            DemoSection {
                title: "日期选择",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Calendar {
                        value: *selected.read(),
                        on_select: {
                            let mut sig = selected;
                            move |date| sig.set(Some(date))
                        },
                        fullscreen: Some(false),
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "选中的日期: ",
                        {
                            selected.read().as_ref().map(|d| format!("{:?}", d)).unwrap_or_else(|| "(未选择)".to_string())
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 全屏模式
            DemoSection {
                title: "全屏模式",
                Calendar {
                    value: *selected.read(),
                    on_select: {
                        let mut sig = selected;
                        move |date| sig.set(Some(date))
                    },
                    fullscreen: Some(true),
                }
            }

            // 视图模式切换
            DemoSection {
                title: "视图模式切换",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; gap: 8px;",
                        Button {
                            r#type: if matches!(*calendar_mode.read(), CalendarMode::Month) { ButtonType::Primary } else { ButtonType::Default },
                            onclick: {
                                let mut sig = calendar_mode;
                                move |_| sig.set(CalendarMode::Month)
                            },
                            "月视图"
                        }
                        Button {
                            r#type: if matches!(*calendar_mode.read(), CalendarMode::Year) { ButtonType::Primary } else { ButtonType::Default },
                            onclick: {
                                let mut sig = calendar_mode;
                                move |_| sig.set(CalendarMode::Year)
                            },
                            "年视图"
                        }
                    }
                    Calendar {
                        value: *selected.read(),
                        on_select: {
                            let mut sig = selected;
                            move |date| sig.set(Some(date))
                        },
                        fullscreen: Some(false),
                        mode: Some(*calendar_mode.read()),
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
