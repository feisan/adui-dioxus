//! Tree 组件演示
//!
//! 展示 Tree 组件的基础用法和高级用法，包括：
//! - 基础树形控件
//! - 可勾选
//! - 受控展开
//! - 显示连接线和图标
//! - 目录树

use adui_dioxus::{
    Button, ButtonType, Card, DirectoryTree, Tag, TagColor, ThemeMode, ThemeProvider, Title,
    TitleLevel, Tree, TreeNode, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TreeDemo {}
        }
    }
}

fn get_sample_tree_data() -> Vec<TreeNode> {
    vec![
        TreeNode {
            key: "0-0".into(),
            label: "parent 1".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "0-0-0".into(),
                    label: "parent 1-0".into(),
                    disabled: false,
                    children: vec![
                        TreeNode {
                            key: "0-0-0-0".into(),
                            label: "leaf".into(),
                            disabled: false,
                            children: vec![],
                        },
                        TreeNode {
                            key: "0-0-0-1".into(),
                            label: "leaf".into(),
                            disabled: false,
                            children: vec![],
                        },
                    ],
                },
                TreeNode {
                    key: "0-0-1".into(),
                    label: "parent 1-1".into(),
                    disabled: false,
                    children: vec![TreeNode {
                        key: "0-0-1-0".into(),
                        label: "leaf".into(),
                        disabled: false,
                        children: vec![],
                    }],
                },
            ],
        },
        TreeNode {
            key: "0-1".into(),
            label: "parent 2".into(),
            disabled: false,
            children: vec![TreeNode {
                key: "0-1-0".into(),
                label: "leaf".into(),
                disabled: false,
                children: vec![],
            }],
        },
    ]
}

fn get_directory_tree_data() -> Vec<TreeNode> {
    vec![
        TreeNode {
            key: "src".into(),
            label: "src".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "components".into(),
                    label: "components".into(),
                    disabled: false,
                    children: vec![
                        TreeNode {
                            key: "button.rs".into(),
                            label: "button.rs".into(),
                            disabled: false,
                            children: vec![],
                        },
                        TreeNode {
                            key: "tree.rs".into(),
                            label: "tree.rs".into(),
                            disabled: false,
                            children: vec![],
                        },
                        TreeNode {
                            key: "tour.rs".into(),
                            label: "tour.rs".into(),
                            disabled: false,
                            children: vec![],
                        },
                    ],
                },
                TreeNode {
                    key: "lib.rs".into(),
                    label: "lib.rs".into(),
                    disabled: false,
                    children: vec![],
                },
                TreeNode {
                    key: "theme.rs".into(),
                    label: "theme.rs".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
        TreeNode {
            key: "examples".into(),
            label: "examples".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "tree_demo.rs".into(),
                    label: "tree_demo.rs".into(),
                    disabled: false,
                    children: vec![],
                },
                TreeNode {
                    key: "tour_demo.rs".into(),
                    label: "tour_demo.rs".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        },
        TreeNode {
            key: "Cargo.toml".into(),
            label: "Cargo.toml".into(),
            disabled: false,
            children: vec![],
        },
    ]
}

#[component]
fn TreeDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let selected = use_signal(|| Vec::<String>::new());
    let checked = use_signal(|| Vec::<String>::new());
    let expanded_keys = use_signal(|| vec!["0-0".to_string()]);

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

            // 基础树形控件
            DemoSection {
                title: "基础树形控件",
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                    div {
                        style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                        Tree {
                            tree_data: get_sample_tree_data(),
                            default_expanded_keys: vec!["0-0".to_string()],
                            on_select: {
                                let mut sig = selected;
                                move |keys: Vec<String>| sig.set(keys)
                            },
                        }
                    }
                    div {
                        style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                        span {
                            style: "font-weight: 600; color: var(--adui-color-text); margin-bottom: 12px; display: block;",
                            "选中的节点："
                        }
                        if selected.read().is_empty() {
                            div {
                                style: "color: var(--adui-color-text-secondary); font-style: italic;",
                                "点击节点进行选择"
                            }
                        } else {
                            div {
                                style: "display: flex; flex-wrap: wrap; gap: 8px;",
                                for key in selected.read().iter() {
                                    Tag {
                                        color: Some(TagColor::Primary),
                                        children: rsx! { {key.clone()} }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 可勾选
            DemoSection {
                title: "可勾选",
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                    div {
                        style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                        Tree {
                            tree_data: get_sample_tree_data(),
                            checkable: true,
                            default_expand_all: true,
                            on_check: {
                                let mut sig = checked;
                                move |keys: Vec<String>| sig.set(keys)
                            },
                        }
                    }
                    div {
                        style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                        span {
                            style: "font-weight: 600; color: var(--adui-color-text); margin-bottom: 12px; display: block;",
                            "勾选的节点："
                        }
                        if checked.read().is_empty() {
                            div {
                                style: "color: var(--adui-color-text-secondary); font-style: italic;",
                                "点击复选框进行勾选"
                            }
                        } else {
                            div {
                                style: "display: flex; flex-wrap: wrap; gap: 8px;",
                                for key in checked.read().iter() {
                                    Tag {
                                        color: Some(TagColor::Success),
                                        children: rsx! { {key.clone()} }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 受控展开
            DemoSection {
                title: "受控展开",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; gap: 12px;",
                        Button {
                            r#type: ButtonType::Primary,
                            onclick: {
                                let mut sig = expanded_keys;
                                move |_| sig.set(vec![
                                    "0-0".to_string(),
                                    "0-0-0".to_string(),
                                    "0-0-1".to_string(),
                                    "0-1".to_string(),
                                ])
                            },
                            "展开全部"
                        }
                        Button {
                            onclick: {
                                let mut sig = expanded_keys;
                                move |_| sig.set(vec![])
                            },
                            "收起全部"
                        }
                    }
                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                        div {
                            style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                            Tree {
                                tree_data: get_sample_tree_data(),
                                expanded_keys: expanded_keys.read().clone(),
                                on_expand: {
                                    let mut sig = expanded_keys;
                                    move |keys: Vec<String>| sig.set(keys)
                                },
                            }
                        }
                        div {
                            style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                            span {
                                style: "font-weight: 600; color: var(--adui-color-text); margin-bottom: 12px; display: block;",
                                "展开的节点："
                            }
                            if expanded_keys.read().is_empty() {
                                div {
                                    style: "color: var(--adui-color-text-secondary); font-style: italic;",
                                    "所有节点已收起"
                                }
                            } else {
                                div {
                                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                                    for key in expanded_keys.read().iter() {
                                        Tag {
                                            color: Some(TagColor::Warning),
                                            children: rsx! { {key.clone()} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 显示连接线和图标
            DemoSection {
                title: "显示连接线和图标",
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                    div {
                        span {
                            style: "font-weight: 600; color: var(--adui-color-text); margin-bottom: 12px; display: block;",
                            "带连接线："
                        }
                        div {
                            style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                            Tree {
                                tree_data: get_sample_tree_data(),
                                show_line: true,
                                default_expand_all: true,
                            }
                        }
                    }
                    div {
                        span {
                            style: "font-weight: 600; color: var(--adui-color-text); margin-bottom: 12px; display: block;",
                            "带图标："
                        }
                        div {
                            style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                            Tree {
                                tree_data: get_sample_tree_data(),
                                show_icon: true,
                                default_expand_all: true,
                            }
                        }
                    }
                }
            }

            // 目录树
            DemoSection {
                title: "目录树",
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                    div {
                        style: "padding: 20px; background: var(--adui-color-bg-elevated); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                        DirectoryTree {
                            tree_data: get_directory_tree_data(),
                            default_expand_all: true,
                            multiple: true,
                            on_select: {
                                let mut sig = selected;
                                move |keys: Vec<String>| sig.set(keys)
                            },
                        }
                    }
                    div {
                        style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                        span {
                            style: "font-weight: 600; color: var(--adui-color-text); margin-bottom: 12px; display: block;",
                            "选中的文件："
                        }
                        if selected.read().is_empty() {
                            div {
                                style: "color: var(--adui-color-text-secondary); font-style: italic;",
                                "点击文件进行选择"
                            }
                        } else {
                            div {
                                style: "display: flex; flex-direction: column; gap: 8px;",
                                for key in selected.read().iter() {
                                    div {
                                        style: "padding: 8px 12px; background: var(--adui-color-bg-layout); border-radius: 6px; font-family: 'SF Mono', 'Consolas', monospace; font-size: 13px; color: var(--adui-color-text);",
                                        {format!("📄 {}", key)}
                                    }
                                }
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
