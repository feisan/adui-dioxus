//! Checkbox 组件演示
//!
//! 展示 Checkbox 组件的基础用法和高级用法，包括：
//! - 基础复选框
//! - 复选框组
//! - 全选/半选
//! - 禁用状态

use adui_dioxus::{
    Button, ButtonType, Checkbox, CheckboxGroup, ThemeMode, ThemeProvider, Title, TitleLevel,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            CheckboxDemo {}
        }
    }
}

#[component]
fn CheckboxDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let single_checked = use_signal(|| false);
    let group_values = use_signal(|| vec!["apple".to_string()]);
    let all_options = vec!["apple", "banana", "cherry", "orange"];

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    let all_selected = group_values.read().len() == all_options.len();
    let some_selected = !group_values.read().is_empty() && !all_selected;

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

            // 基础复选框
            DemoSection {
                title: "基础复选框",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Checkbox {
                        checked: Some(*single_checked.read()),
                        on_change: {
                            let mut sig = single_checked;
                            move |checked| sig.set(checked)
                        },
                        "基础复选框"
                    }
                    Checkbox {
                        default_checked: true,
                        "默认选中"
                    }
                    Checkbox {
                        disabled: true,
                        "禁用状态"
                    }
                    Checkbox {
                        disabled: true,
                        default_checked: true,
                        "禁用且选中"
                    }
                }
            }

            // 复选框组
            DemoSection {
                title: "复选框组",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    CheckboxGroup {
                        value: Some(group_values.read().clone()),
                        on_change: {
                            let mut sig = group_values;
                            move |values| sig.set(values)
                        },
                        Checkbox {
                            value: Some("apple".into()),
                            "Apple"
                        }
                        Checkbox {
                            value: Some("banana".into()),
                            "Banana"
                        }
                        Checkbox {
                            value: Some("cherry".into()),
                            "Cherry"
                        }
                        Checkbox {
                            value: Some("orange".into()),
                            "Orange"
                        }
                    }
                    {
                        let selected_text = format!("已选择: {:?}", *group_values.read());
                        rsx! {
                            div {
                                style: "font-size: 12px; color: var(--adui-color-text-secondary);",
                                {selected_text}
                            }
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 全选/半选
            DemoSection {
                title: "全选/半选",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Checkbox {
                        checked: Some(all_selected),
                        indeterminate: some_selected,
                        on_change: {
                            let mut sig = group_values;
                            let options = all_options.clone();
                            move |checked| {
                                if checked {
                                    sig.set(options.iter().map(|s| s.to_string()).collect());
                                } else {
                                    sig.set(vec![]);
                                }
                            }
                        },
                        "全选"
                    }
                    CheckboxGroup {
                        value: Some(group_values.read().clone()),
                        on_change: {
                            let mut sig = group_values;
                            move |values| sig.set(values)
                        },
                        {
                            all_options.iter().map(|&option| {
                                rsx! {
                                    Checkbox {
                                        value: Some(option.to_string()),
                                        {option}
                                    }
                                }
                            })
                        }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600;", "选择您喜欢的水果：" }
                        CheckboxGroup {
                            value: Some(group_values.read().clone()),
                            on_change: {
                                let mut sig = group_values;
                                move |values| sig.set(values)
                            },
                            Checkbox {
                                value: Some("apple".into()),
                                "🍎 Apple"
                            }
                            Checkbox {
                                value: Some("banana".into()),
                                "🍌 Banana"
                            }
                            Checkbox {
                                value: Some("cherry".into()),
                                "🍒 Cherry"
                            }
                            Checkbox {
                                value: Some("orange".into()),
                                "🍊 Orange"
                            }
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
