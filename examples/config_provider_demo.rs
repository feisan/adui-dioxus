//! ConfigProvider 组件演示
//!
//! 展示 ConfigProvider 组件的基础用法和高级用法，包括：
//! - 全局尺寸配置
//! - 全局禁用配置
//! - 动态更新配置
//! - 本地覆盖全局配置

use adui_dioxus::{
    Button, ButtonSize, ButtonType, ComponentSize, ConfigProvider, Input, ThemeMode, ThemeProvider,
    Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            ConfigProviderDemo {}
        }
    }
}

#[component]
fn ConfigProviderDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let global_disabled = use_signal(|| false);
    let global_size = use_signal(|| ComponentSize::Middle);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let size_label = match *global_size.read() {
        ComponentSize::Small => "small",
        ComponentSize::Middle => "middle",
        ComponentSize::Large => "large",
    };

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

            // 全局尺寸配置
            DemoSection {
                title: "全局尺寸配置",
                ConfigProvider {
                    size: Some(*global_size.read()),
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        div {
                            style: "display: flex; gap: 8px; margin-bottom: 8px;",
                            Button {
                                size: ButtonSize::Small,
                                onclick: {
                                    let mut sig = global_size;
                                    move |_| sig.set(ComponentSize::Small)
                                },
                                "small"
                            }
                            Button {
                                size: ButtonSize::Middle,
                                onclick: {
                                    let mut sig = global_size;
                                    move |_| sig.set(ComponentSize::Middle)
                                },
                                "middle"
                            }
                            Button {
                                size: ButtonSize::Large,
                                onclick: {
                                    let mut sig = global_size;
                                    move |_| sig.set(ComponentSize::Large)
                                },
                                "large"
                            }
                        }
                        p {
                            style: "color: var(--adui-color-text-secondary); font-size: 14px;",
                            "当前全局 size: ",
                            {size_label},
                            "，disabled: ",
                            {if *global_disabled.read() { "true" } else { "false" }}
                        }
                        p {
                            style: "color: var(--adui-color-text-secondary); font-size: 14px;",
                            "未显式指定 size / disabled 的 Button 和 Input 会继承上方 ConfigProvider 的配置。"
                        }
                        Button { "继承全局 Config 的按钮" }
                        Input { placeholder: Some("继承 Config 的输入框".to_string()) }
                    }
                }
            }

            // 全局禁用配置
            DemoSection {
                title: "全局禁用配置",
                ConfigProvider {
                    disabled: Some(*global_disabled.read()),
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        Button {
                            r#type: ButtonType::Primary,
                            onclick: {
                                let mut sig = global_disabled;
                                move |_| {
                                    let next = !*sig.read();
                                    sig.set(next);
                                }
                            },
                            "切换全局 disabled"
                        }
                        Button { "继承全局 disabled 的按钮" }
                        Input { placeholder: Some("继承全局 disabled 的输入框".to_string()) }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 本地覆盖全局配置
            DemoSection {
                title: "本地覆盖全局配置",
                ConfigProvider {
                    size: Some(*global_size.read()),
                    disabled: Some(*global_disabled.read()),
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        p {
                            style: "color: var(--adui-color-text-secondary); font-size: 14px;",
                            "仍然可以在本地 props 中覆盖全局配置："
                        }
                        Button {
                            size: ButtonSize::Small,
                            disabled: false,
                            "本地 size = small，忽略全局 size/disabled"
                        }
                        Input {
                            disabled: false,
                            placeholder: Some("本地 disabled=false，忽略全局 disabled".to_string()),
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
