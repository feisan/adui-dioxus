//! Tree component demonstration
//!
//! Run with: cargo run --example tree_demo

use adui_dioxus::{
    Button, ButtonType, Card, DirectoryTree, THEME_BASE_STYLE, Tag, TagColor, Theme, ThemeMode,
    ThemeProvider, Tree, TreeNode, use_theme,
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
                style: "background: linear-gradient(135deg, #1677ff 0%, #0958d9 100%); padding: 48px 24px; color: white;",
                div { style: "max-width: 1200px; margin: 0 auto;",
                    div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",
                        div { style: "display: flex; align-items: center; gap: 12px;",
                            span { style: "font-size: 32px;", "🌳" }
                            h1 { style: "margin: 0; font-size: 28px; font-weight: 600;", "Tree 树形控件" }
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
                        "多层次的结构列表，通过缩进和树形展示数据的层级关系，支持展开、选择、勾选等丰富的交互功能。"
                    }
                }
            }

            // Content
            div { style: "max-width: 1200px; margin: 0 auto; padding: 32px 24px;",
                // Basic Tree
                DemoSection {
                    title: "基础用法",
                    description: "最简单的用法，点击节点可以展开/收起，支持键盘导航。",
                    BasicTreeDemo {}
                }

                // Checkable Tree
                DemoSection {
                    title: "可勾选",
                    description: "带勾选框的树形控件，支持级联选择和半选状态。",
                    CheckableTreeDemo {}
                }

                // Controlled Expand
                DemoSection {
                    title: "受控展开",
                    description: "通过外部状态控制展开的节点，实现程序化控制。",
                    ControlledExpandDemo {}
                }

                // Show Line & Icon
                DemoSection {
                    title: "显示连接线和图标",
                    description: "树形控件可以显示连接线或自定义图标。",
                    ShowLineTreeDemo {}
                }

                // Directory Tree
                DemoSection {
                    title: "目录树",
                    description: "内置的目录样式，适合展示文件系统结构。",
                    DirectoryTreeDemo {}
                }
            }
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
fn BasicTreeDemo() -> Element {
    let tree_data = get_sample_tree_data();
    let mut selected = use_signal(Vec::<String>::new);

    rsx! {
        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
            div {
                style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                Tree {
                    tree_data: tree_data.clone(),
                    default_expanded_keys: vec!["0-0".to_string()],
                    on_select: move |keys: Vec<String>| {
                        selected.set(keys);
                    },
                }
            }
            div { style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px;",
                    span { style: "font-size: 16px;", "📋" }
                    span { style: "font-weight: 600; color: var(--adui-color-text);", "选中的节点" }
                }
                if selected.read().is_empty() {
                    div { style: "color: var(--adui-color-text-secondary); font-style: italic;", "点击节点进行选择" }
                } else {
                    div { style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        {
                            let items = selected.read().clone();
                            items.into_iter().map(|key| {
                                rsx! {
                                    Tag {
                                        key: "{key}",
                                        color: Some(TagColor::Primary),
                                        children: rsx! { "{key}" }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CheckableTreeDemo() -> Element {
    let tree_data = get_sample_tree_data();
    let mut checked = use_signal(Vec::<String>::new);

    rsx! {
        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
            div {
                style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                Tree {
                    tree_data: tree_data.clone(),
                    checkable: true,
                    default_expand_all: true,
                    on_check: move |keys: Vec<String>| {
                        checked.set(keys);
                    },
                }
            }
            div { style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px;",
                    span { style: "font-size: 16px;", "✅" }
                    span { style: "font-weight: 600; color: var(--adui-color-text);", "勾选的节点" }
                    if !checked.read().is_empty() {
                        span {
                            style: "padding: 2px 8px; background: var(--adui-color-primary); color: white; border-radius: 10px; font-size: 12px;",
                            "{checked.read().len()}"
                        }
                    }
                }
                if checked.read().is_empty() {
                    div { style: "color: var(--adui-color-text-secondary); font-style: italic;", "点击复选框进行勾选" }
                } else {
                    div { style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        {
                            let items = checked.read().clone();
                            items.into_iter().map(|key| {
                                rsx! {
                                    Tag {
                                        key: "{key}",
                                        color: Some(TagColor::Success),
                                        children: rsx! { "{key}" }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ControlledExpandDemo() -> Element {
    let tree_data = get_sample_tree_data();
    let mut expanded_keys = use_signal(|| vec!["0-0".to_string()]);

    rsx! {
        div {
            div { style: "display: flex; gap: 12px; margin-bottom: 16px;",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: move |_| {
                        expanded_keys.set(vec![
                            "0-0".to_string(),
                            "0-0-0".to_string(),
                            "0-0-1".to_string(),
                            "0-1".to_string(),
                        ]);
                    },
                    "📂 展开全部"
                }
                Button {
                    onclick: move |_| {
                        expanded_keys.set(vec![]);
                    },
                    "📁 收起全部"
                }
            }
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
                div {
                    style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                    Tree {
                        tree_data: tree_data.clone(),
                        expanded_keys: expanded_keys.read().clone(),
                        on_expand: move |keys: Vec<String>| {
                            expanded_keys.set(keys);
                        },
                    }
                }
                div { style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                    div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px;",
                        span { style: "font-size: 16px;", "📂" }
                        span { style: "font-weight: 600; color: var(--adui-color-text);", "展开的节点" }
                    }
                    if expanded_keys.read().is_empty() {
                        div { style: "color: var(--adui-color-text-secondary); font-style: italic;", "所有节点已收起" }
                    } else {
                        div { style: "display: flex; flex-wrap: wrap; gap: 8px;",
                            {
                                let items = expanded_keys.read().clone();
                                items.into_iter().map(|key| {
                                    rsx! {
                                    Tag {
                                        key: "{key}",
                                        color: Some(TagColor::Warning),
                                        children: rsx! { "{key}" }
                                    }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ShowLineTreeDemo() -> Element {
    let tree_data = get_sample_tree_data();

    rsx! {
        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
            div {
                div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px;",
                    span { style: "font-size: 16px;", "🔗" }
                    span { style: "font-weight: 600; color: var(--adui-color-text);", "带连接线" }
                }
                div {
                    style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                    Tree {
                        tree_data: tree_data.clone(),
                        show_line: true,
                        default_expand_all: true,
                    }
                }
            }
            div {
                div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px;",
                    span { style: "font-size: 16px;", "🎨" }
                    span { style: "font-weight: 600; color: var(--adui-color-text);", "带图标" }
                }
                div {
                    style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                    Tree {
                        tree_data: tree_data.clone(),
                        show_icon: true,
                        default_expand_all: true,
                    }
                }
            }
        }
    }
}

#[component]
fn DirectoryTreeDemo() -> Element {
    let tree_data = get_directory_tree_data();
    let mut selected = use_signal(Vec::<String>::new);

    rsx! {
        div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px;",
            div {
                style: "padding: 20px; background: var(--adui-color-bg-elevated); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                div { style: "display: flex; align-items: center; gap: 8px; padding-bottom: 12px; margin-bottom: 12px; border-bottom: 1px solid var(--adui-color-border);",
                    span { style: "font-size: 14px;", "📁" }
                    span { style: "font-weight: 500; color: var(--adui-color-text-secondary); font-size: 13px;", "EXPLORER" }
                }
                DirectoryTree {
                    tree_data: tree_data.clone(),
                    default_expand_all: true,
                    multiple: true,
                    on_select: move |keys: Vec<String>| {
                        selected.set(keys);
                    },
                }
            }
            div { style: "padding: 20px; background: var(--adui-color-bg-base); border-radius: 8px; border: 1px solid var(--adui-color-border);",
                div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px;",
                    span { style: "font-size: 16px;", "📄" }
                    span { style: "font-weight: 600; color: var(--adui-color-text);", "选中的文件" }
                }
                if selected.read().is_empty() {
                    div { style: "color: var(--adui-color-text-secondary); font-style: italic;", "点击文件进行选择" }
                } else {
                    div { style: "display: flex; flex-direction: column; gap: 8px;",
                        {
                            let items = selected.read().clone();
                            items.into_iter().map(|key| {
                                rsx! {
                                    div {
                                        key: "{key}",
                                        style: "padding: 8px 12px; background: var(--adui-color-bg-layout); border-radius: 6px; font-family: 'SF Mono', 'Consolas', monospace; font-size: 13px; color: var(--adui-color-text);",
                                        "📄 {key}"
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}
