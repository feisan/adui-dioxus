//! Input 组件演示
//!
//! 展示 Input 组件的基础用法和高级用法，包括：
//! - 基础输入框
//! - 密码输入框
//! - 搜索输入框
//! - 前缀和后缀
//! - 状态（成功/警告/错误）
//! - 尺寸和变体
//! - 清除按钮和字数统计

use adui_dioxus::{
    Button, ButtonType, Icon, IconKind, Input, InputSize, Password, Search, ThemeMode,
    ThemeProvider, Title, TitleLevel, Variant, components::control::ControlStatus, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            InputDemo {}
        }
    }
}

#[component]
fn InputDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| String::new());
    let password_value = use_signal(|| String::new());
    let search_value = use_signal(|| String::new());

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

            // 基础输入框
            DemoSection {
                title: "基础输入框",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("请输入内容".into()),
                        on_change: {
                            let mut sig = basic_value;
                            move |value| sig.set(value)
                        },
                    }
                    Input {
                        placeholder: Some("受控输入框".into()),
                        value: Some(basic_value.read().clone()),
                        on_change: {
                            let mut sig = basic_value;
                            move |value| sig.set(value)
                        },
                    }
                    Input {
                        placeholder: Some("默认值".into()),
                        default_value: Some("默认内容".into()),
                    }
                }
            }

            // 密码输入框
            DemoSection {
                title: "密码输入框",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Password {
                        placeholder: Some("请输入密码".into()),
                        on_change: {
                            let mut sig = password_value;
                            move |value| sig.set(value)
                        },
                    }
                    Password {
                        placeholder: Some("密码输入框（可切换显示）".into()),
                        visible: false,
                        on_change: {
                            let mut sig = password_value;
                            move |value| sig.set(value)
                        },
                    }
                }
            }

            // 搜索输入框
            DemoSection {
                title: "搜索输入框",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Search {
                        placeholder: Some("请输入搜索关键词".into()),
                        on_search: {
                            let mut sig = search_value;
                            move |value: String| {
                                sig.set(value.clone());
                                println!("搜索: {}", value);
                            }
                        },
                    }
                    Search {
                        placeholder: Some("带加载状态的搜索框".into()),
                        loading: true,
                        on_search: move |value: String| {
                            println!("搜索: {}", value);
                        },
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 前缀和后缀
            DemoSection {
                title: "前缀和后缀",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("请输入用户名".into()),
                        prefix: Some(rsx!(Icon { kind: IconKind::Info })),
                    }
                    Input {
                        placeholder: Some("请输入邮箱".into()),
                        prefix: Some(rsx!(span { "@" })),
                        suffix: Some(rsx!(Icon { kind: IconKind::Check })),
                    }
                    Input {
                        placeholder: Some("带后缀图标".into()),
                        suffix: Some(rsx!(Icon { kind: IconKind::Close })),
                    }
                }
            }

            // 状态
            DemoSection {
                title: "状态（Status）",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("成功状态".into()),
                        status: Some(ControlStatus::Success),
                        value: Some("成功输入".into()),
                    }
                    Input {
                        placeholder: Some("警告状态".into()),
                        status: Some(ControlStatus::Warning),
                        value: Some("警告输入".into()),
                    }
                    Input {
                        placeholder: Some("错误状态".into()),
                        status: Some(ControlStatus::Error),
                        value: Some("错误输入".into()),
                    }
                }
            }

            // 尺寸
            DemoSection {
                title: "尺寸（Size）",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("Small 尺寸".into()),
                        size: Some(InputSize::Small),
                    }
                    Input {
                        placeholder: Some("Middle 尺寸（默认）".into()),
                        size: Some(InputSize::Middle),
                    }
                    Input {
                        placeholder: Some("Large 尺寸".into()),
                        size: Some(InputSize::Large),
                    }
                }
            }

            // 变体
            DemoSection {
                title: "变体（Variant）",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("Outlined（默认）".into()),
                        variant: Some(Variant::Outlined),
                    }
                    Input {
                        placeholder: Some("Filled".into()),
                        variant: Some(Variant::Filled),
                    }
                    Input {
                        placeholder: Some("Borderless".into()),
                        variant: Some(Variant::Borderless),
                    }
                }
            }

            // 清除按钮和字数统计
            DemoSection {
                title: "清除按钮和字数统计",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("带清除按钮".into()),
                        allow_clear: true,
                        default_value: Some("可清除的内容".into()),
                    }
                    Input {
                        placeholder: Some("字数统计（最大10字符）".into()),
                        max_length: Some(10),
                        show_count: true,
                        default_value: Some("12345".into()),
                    }
                    Input {
                        placeholder: Some("清除按钮 + 字数统计".into()),
                        allow_clear: true,
                        max_length: Some(20),
                        show_count: true,
                        default_value: Some("示例内容".into()),
                    }
                }
            }

            // 禁用状态
            DemoSection {
                title: "禁用状态",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("禁用状态".into()),
                        disabled: true,
                        default_value: Some("禁用内容".into()),
                    }
                    Input {
                        placeholder: Some("禁用状态（带前缀）".into()),
                        disabled: true,
                        prefix: Some(rsx!(Icon { kind: IconKind::Info })),
                        default_value: Some("禁用内容".into()),
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Input {
                        placeholder: Some("用户名".into()),
                        prefix: Some(rsx!(Icon { kind: IconKind::Info })),
                        allow_clear: true,
                        status: Some(ControlStatus::Success),
                    }
                    Password {
                        placeholder: Some("密码".into()),
                    }
                    Search {
                        placeholder: Some("搜索".into()),
                        on_search: move |value: String| {
                            println!("搜索: {}", value);
                        },
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
