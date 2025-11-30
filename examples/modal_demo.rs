//! Modal 组件演示
//!
//! 展示 Modal 组件的基础用法和高级用法，包括：
//! - 基础对话框
//! - 确认对话框
//! - 不同尺寸
//! - 自定义内容

use adui_dioxus::{
    Button, ButtonType, Input, Modal, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ModalDemo {}
        }
    }
}

#[component]
fn ModalDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_open = use_signal(|| false);
    let confirm_open = use_signal(|| false);
    let form_open = use_signal(|| false);

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

            // 基础对话框
            DemoSection {
                title: "基础对话框",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Button {
                        r#type: ButtonType::Primary,
                        onclick: {
                            let mut sig = basic_open;
                            move |_| sig.set(true)
                        },
                        "打开基础 Modal"
                    }
                    Modal {
                        open: *basic_open.read(),
                        title: Some("基础 Modal".into()),
                        on_cancel: {
                            let mut sig = basic_open;
                            move |_| sig.set(false)
                        },
                        destroy_on_close: true,
                        children: rsx! {
                            p { "这里可以放任意内容，例如表单、说明文本等。" }
                        },
                    }
                }
            }

            // 确认对话框
            DemoSection {
                title: "确认对话框",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Button {
                        r#type: ButtonType::Default,
                        danger: true,
                        onclick: {
                            let mut sig = confirm_open;
                            move |_| sig.set(true)
                        },
                        "打开确认对话框"
                    }
                    Modal {
                        open: *confirm_open.read(),
                        title: Some("确认删除".into()),
                        on_ok: {
                            let mut sig = confirm_open;
                            move |_| {
                                sig.set(false);
                                // 这里可以添加实际的删除逻辑
                            }
                        },
                        on_cancel: {
                            let mut sig = confirm_open;
                            move |_| sig.set(false)
                        },
                        destroy_on_close: true,
                        children: rsx! {
                            p { "确定要删除这条记录吗？删除后无法恢复。" }
                        },
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 自定义内容
            DemoSection {
                title: "自定义内容（表单）",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut sig = form_open;
                        move |_| sig.set(true)
                    },
                    "打开表单 Modal"
                }
            }
        }

        // 表单 Modal
        Modal {
            open: *form_open.read(),
            title: Some("编辑信息".into()),
            on_ok: {
                let mut sig = form_open;
                move |_| {
                    sig.set(false);
                    // 这里可以添加表单提交逻辑
                }
            },
            on_cancel: {
                let mut sig = form_open;
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
