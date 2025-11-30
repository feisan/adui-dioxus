//! Drawer 组件演示
//!
//! 展示 Drawer 组件的基础用法和高级用法，包括：
//! - 基础抽屉
//! - 不同位置
//! - 不同尺寸
//! - 自定义内容

use adui_dioxus::{
    Button, ButtonType, Drawer, DrawerPlacement, Input, ThemeMode, ThemeProvider, Title,
    TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DrawerDemo {}
        }
    }
}

#[component]
fn DrawerDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let mut right_open = use_signal(|| false);
    let mut left_open = use_signal(|| false);
    let mut top_open = use_signal(|| false);
    let mut bottom_open = use_signal(|| false);

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

            // 基础抽屉
            DemoSection {
                title: "基础抽屉",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = right_open;
                            move |_| sig.set(true)
                        },
                        "打开右侧 Drawer"
                    }
                    Drawer {
                        open: *right_open.read(),
                        title: Some("右侧 Drawer".into()),
                        placement: DrawerPlacement::Right,
                        size: Some(320.0),
                        on_close: {
                            let mut sig = right_open;
                            move |_| sig.set(false)
                        },
                        destroy_on_close: true,
                        children: rsx! {
                            p { "用于展示表单、详情或导航内容的侧边抽屉。" }
                        },
                    }
                }
            }

            // 不同位置
            DemoSection {
                title: "不同位置",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let mut sig = right_open;
                            move |_| sig.set(true)
                        },
                        "右侧"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let mut sig = left_open;
                            move |_| sig.set(true)
                        },
                        "左侧"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let mut sig = top_open;
                            move |_| sig.set(true)
                        },
                        "顶部"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: {
                            let mut sig = bottom_open;
                            move |_| sig.set(true)
                        },
                        "底部"
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 自定义内容
            DemoSection {
                title: "自定义内容",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut sig = right_open;
                        move |_| sig.set(true)
                    },
                    "打开带表单的 Drawer"
                }
            }
        }

        // 不同位置的 Drawer
        Drawer {
            open: *left_open.read(),
            title: Some("左侧 Drawer".into()),
            placement: DrawerPlacement::Left,
            size: Some(320.0),
            on_close: {
                let mut sig = left_open;
                move |_| sig.set(false)
            },
            destroy_on_close: true,
            children: rsx! {
                p { "从左侧滑出的抽屉。" }
            },
        }

        Drawer {
            open: *top_open.read(),
            title: Some("顶部 Drawer".into()),
            placement: DrawerPlacement::Top,
            size: Some(200.0),
            on_close: {
                let mut sig = top_open;
                move |_| sig.set(false)
            },
            destroy_on_close: true,
            children: rsx! {
                p { "从顶部滑出的抽屉。" }
            },
        }

        Drawer {
            open: *bottom_open.read(),
            title: Some("底部 Drawer".into()),
            placement: DrawerPlacement::Bottom,
            size: Some(200.0),
            on_close: {
                let mut sig = bottom_open;
                move |_| sig.set(false)
            },
            destroy_on_close: true,
            children: rsx! {
                p { "从底部滑出的抽屉。" }
            },
        }

        // 带表单的 Drawer
        Drawer {
            open: *right_open.read(),
            title: Some("表单 Drawer".into()),
            placement: DrawerPlacement::Right,
            size: Some(400.0),
            on_close: {
                let mut sig = right_open;
                move |_| sig.set(false)
            },
            destroy_on_close: true,
            children: rsx! {
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { "用户名" }
                        Input {
                            placeholder: Some("请输入用户名".to_string()),
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { "邮箱" }
                        Input {
                            placeholder: Some("请输入邮箱".to_string()),
                        }
                    }
                    div {
                        style: "display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px;",
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let mut sig = right_open;
                                move |_| sig.set(false)
                            },
                            "取消"
                        }
                        Button {
                            r#type: ButtonType::Primary,
                            "确定"
                        }
                    }
                }
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
