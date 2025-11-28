use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Layout, Menu, MenuItemNode, MenuMode,
    Sider,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { MenuDemoShell {} }
        }
    }
}

fn sider_items() -> Vec<MenuItemNode> {
    vec![
        MenuItemNode {
            id: "dashboard".into(),
            label: "仪表盘".into(),
            icon: None,
            disabled: false,
            children: None,
        },
        MenuItemNode {
            id: "list".into(),
            label: "列表".into(),
            icon: None,
            disabled: false,
            children: Some(vec![
                MenuItemNode::leaf("list-basic", "基础列表"),
                MenuItemNode::leaf("list-advanced", "高级列表"),
            ]),
        },
        MenuItemNode {
            id: "settings".into(),
            label: "设置".into(),
            icon: None,
            disabled: false,
            children: None,
        },
    ]
}

fn header_items() -> Vec<MenuItemNode> {
    vec![
        MenuItemNode::leaf("home", "首页"),
        MenuItemNode::leaf("docs", "文档"),
        MenuItemNode::leaf("about", "关于"),
    ]
}

#[component]
fn MenuDemoShell() -> Element {
    let mut sider_selected = use_signal(|| vec!["dashboard".to_string()]);
    let mut header_selected = use_signal(|| vec!["home".to_string()]);

    rsx! {
        Layout {
            has_sider: Some(true),
            class: None,
            style: None,
            Sider {
                width: Some(220.0),
                collapsed_width: Some(80.0),
                default_collapsed: false,
                collapsible: true,
                reverse_arrow: false,
                trigger: None,
                zero_width_trigger_style: None,
                theme: adui_dioxus::SiderTheme::Dark,
                has_border: true,
                on_collapse: None,
                class: None,
                style: None,
                div {
                    style: "margin-bottom: 12px; font-weight: 600; color: #fff;",
                    "示例导航"
                }
                Menu {
                    mode: MenuMode::Inline,
                    items: sider_items(),
                    selected_keys: Some(sider_selected.read().clone()),
                    on_select: move |key: String| {
                        sider_selected.set(vec![key]);
                    },
                }
            }
            adui_dioxus::Content {
                class: None,
                style: None,
                has_sider: None,
                children: rsx! {
                    adui_dioxus::Header {
                        class: None,
                        style: None,
                        has_sider: None,
                        children: rsx! {
                            div {
                                style: "display: flex; align-items: center; justify-content: space-between;",
                                span { style: "font-weight: 600;", "顶部菜单示例" }
                                Menu {
                                    mode: MenuMode::Horizontal,
                                    items: header_items(),
                                    selected_keys: Some(header_selected.read().clone()),
                                    on_select: move |key: String| {
                                        header_selected.set(vec![key]);
                                    },
                                }
                            }
                        }
                    }
                    div {
                        style: "padding: 16px;",
                        h2 { "Menu demo" }
                        p { "左侧 Sider 显示 inline 菜单，顶部 Header 显示 horizontal 菜单。" }
                        p { "当前侧边选中 key: {sider_selected.read().first().cloned().unwrap_or_default()}" }
                        p { "当前顶部选中 key: {header_selected.read().first().cloned().unwrap_or_default()}" }
                        div { style: "margin-top: 12px; display: flex; gap: 8px;",
                            Button {
                                r#type: ButtonType::Default,
                                onclick: move |_| sider_selected.set(vec!["settings".into()]),
                                "选中“设置”"
                            }
                            Button {
                                r#type: ButtonType::Default,
                                onclick: move |_| header_selected.set(vec!["docs".into()]),
                                "选中“文档”"
                            }
                        }
                    }
                }
            }
        }
    }
}
