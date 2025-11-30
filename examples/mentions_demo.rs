//! Mentions 组件演示
//!
//! 展示 Mentions 组件的基础用法和高级用法，包括：
//! - 基础提及
//! - 多个前缀
//! - 不同尺寸
//! - 位置控制

use adui_dioxus::{
    Button, ButtonType, ThemeMode, ThemeProvider, Title, TitleLevel,
    components::config_provider::ComponentSize,
    components::mentions::{MentionOption, MentionPlacement, Mentions},
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            MentionsDemo {}
        }
    }
}

#[component]
fn MentionsDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let basic_value = use_signal(|| String::new());
    let multi_prefix_value = use_signal(|| String::new());

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let user_options = vec![
        MentionOption::new("john", "John Doe"),
        MentionOption::new("jane", "Jane Smith"),
        MentionOption::new("bob", "Bob Wilson"),
        MentionOption::new("alice", "Alice Brown"),
    ];

    let tag_options = vec![
        MentionOption::new("react", "React"),
        MentionOption::new("vue", "Vue"),
        MentionOption::new("angular", "Angular"),
        MentionOption::new("svelte", "Svelte"),
    ];

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

            // 基础提及
            DemoSection {
                title: "基础提及",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "输入 @ 来提及用户："
                    }
                    Mentions {
                        value: basic_value.read().clone(),
                        options: user_options.clone(),
                        placeholder: Some("Type @ to mention...".into()),
                        rows: 3,
                        on_change: {
                            let mut sig = basic_value;
                            move |v: String| sig.set(v)
                        },
                    }
                    div {
                        style: "padding: 8px; background: var(--adui-color-fill-quaternary); border-radius: var(--adui-radius); font-size: 12px; color: var(--adui-color-text-secondary);",
                        "当前值: ",
                        if basic_value.read().is_empty() {
                            "(空)"
                        } else {
                            {basic_value.read().clone()}
                        }
                    }
                }
            }

            // 多个前缀
            DemoSection {
                title: "多个前缀",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "使用 @ 提及用户或 # 添加标签："
                    }
                    Mentions {
                        value: multi_prefix_value.read().clone(),
                        options: tag_options.clone(),
                        prefix: vec!['@', '#'],
                        placeholder: Some("Type @ or # ...".into()),
                        rows: 3,
                        on_change: {
                            let mut sig = multi_prefix_value;
                            move |v: String| sig.set(v)
                        },
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Mentions {
                        options: user_options.clone(),
                        size: ComponentSize::Small,
                        placeholder: Some("Small size...".into()),
                    }
                    Mentions {
                        options: user_options.clone(),
                        size: ComponentSize::Middle,
                        placeholder: Some("Middle size (default)...".into()),
                    }
                    Mentions {
                        options: user_options.clone(),
                        size: ComponentSize::Large,
                        placeholder: Some("Large size...".into()),
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 顶部位置
            DemoSection {
                title: "顶部位置",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "下拉菜单在输入框上方显示："
                    }
                    div {
                        style: "margin-top: 120px;",
                        Mentions {
                            options: user_options.clone(),
                            placement: MentionPlacement::Top,
                            placeholder: Some("Type @ ...".into()),
                            rows: 2,
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
