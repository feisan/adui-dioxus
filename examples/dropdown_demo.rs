use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Dropdown, DropdownItem,
    DropdownPlacement, DropdownTrigger, use_message,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { DropdownDemoShell {} }
        }
    }
}

fn default_items() -> Vec<DropdownItem> {
    vec![
        DropdownItem::new("new", "新建文档"),
        DropdownItem::new("open", "打开..."),
        DropdownItem::new("share", "分享"),
        DropdownItem {
            key: "disabled".into(),
            label: "禁用项".into(),
            disabled: true,
        },
    ]
}

#[component]
fn DropdownDemoShell() -> Element {
    let api = use_message();
    let last_click = use_signal(String::new);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Dropdown demo" }
            p { "展示 Dropdown 的基础用法：点击按钮弹出菜单，选择后关闭并给出反馈。" }

            div { style: "display: flex; flex-wrap: wrap; gap: 24px; align-items: flex-start;",
                // 基础下拉菜单（Click 触发）
                Dropdown {
                    items: default_items(),
                    trigger: DropdownTrigger::Click,
                    placement: Some(DropdownPlacement::BottomLeft),
                    on_click: {
                        let mut last_click = last_click;
                        let api = api.clone();
                        move |key: String| {
                            last_click.set(key.clone());
                            if let Some(msg) = api.clone() {
                                msg.info(format!("选择菜单项: {key}"));
                            }
                        }
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "基础下拉菜单"
                        }
                    },
                }

                // 右对齐下拉菜单
                Dropdown {
                    items: default_items(),
                    trigger: DropdownTrigger::Click,
                    placement: Some(DropdownPlacement::BottomRight),
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "右对齐菜单"
                        }
                    },
                }

                // Hover 下拉菜单
                Dropdown {
                    items: default_items(),
                    trigger: DropdownTrigger::Hover,
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "Hover 下拉菜单"
                        }
                    },
                }
            }

            if !last_click.read().is_empty() {
                p { style: "margin-top: 16px; color: var(--adui-color-text-secondary);",
                    "最近一次点击的 key: {last_click}"
                }
            }
        }
    }
}
