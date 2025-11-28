use adui_dioxus::{
    App, Breadcrumb, BreadcrumbItem, ComponentSize, ConfigProvider, Content, Header, Layout, Menu,
    MenuItemNode, MenuMode, Pagination, Sider,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { LayoutNavigationDemoShell {} }
        }
    }
}

fn sider_items() -> Vec<MenuItemNode> {
    vec![
        MenuItemNode::leaf("dashboard", "仪表盘"),
        MenuItemNode {
            id: "list".into(),
            label: "列表页".into(),
            icon: None,
            disabled: false,
            children: Some(vec![
                MenuItemNode::leaf("list-basic", "基础列表"),
                MenuItemNode::leaf("list-advanced", "高级列表"),
            ]),
        },
        MenuItemNode::leaf("settings", "设置"),
    ]
}

fn breadcrumb_for(key: &str) -> Vec<BreadcrumbItem> {
    match key {
        "dashboard" => vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::new("dashboard", "仪表盘"),
        ],
        "list-basic" => vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::with_href("list", "列表页", "/list"),
            BreadcrumbItem::new("list-basic", "基础列表"),
        ],
        "list-advanced" => vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::with_href("list", "列表页", "/list"),
            BreadcrumbItem::new("list-advanced", "高级列表"),
        ],
        "settings" => vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::new("settings", "设置"),
        ],
        _ => vec![BreadcrumbItem::new("home", "首页")],
    }
}

#[component]
fn LayoutNavigationDemoShell() -> Element {
    // 当前选中的菜单 key（单选）。
    let mut sider_selected = use_signal(|| vec!["dashboard".to_string()]);
    // 当前分页状态。
    let mut current_page = use_signal(|| 1u32);
    let mut page_size = use_signal(|| 10u32);

    const TOTAL_ITEMS: u32 = 42;

    let current_key = sider_selected
        .read()
        .first()
        .cloned()
        .unwrap_or_else(|| "dashboard".to_string());
    let breadcrumb_items = breadcrumb_for(&current_key);

    let page = *current_page.read();
    let size = *page_size.read();
    let total_pages = ((TOTAL_ITEMS + size - 1) / size).max(1);
    let page_clamped = page.min(total_pages).max(1);
    let start_index = ((page_clamped - 1) * size) as usize;
    let end_index = (start_index as u32 + size).min(TOTAL_ITEMS) as usize;

    rsx! {
        Layout {
            has_sider: Some(true),
            Sider {
                width: Some(220.0),
                collapsed_width: Some(80.0),
                collapsible: true,
                theme: adui_dioxus::SiderTheme::Dark,
                has_border: true,
                div {
                    style: "margin-bottom: 12px; font-weight: 600; color: #fff;",
                    "导航"
                }
                Menu {
                    mode: MenuMode::Inline,
                    items: sider_items(),
                    selected_keys: Some(sider_selected.read().clone()),
                    on_select: {
                        let mut selected = sider_selected;
                        let mut page_sig = current_page;
                        move |key: String| {
                            selected.set(vec![key]);
                            page_sig.set(1);
                        }
                    },
                }
            }
            Layout {
                Header {
                    div {
                        style: "padding: 0 16px; display: flex; align-items: center; height: 48px; background: var(--adui-color-bg-container); border-bottom: 1px solid var(--adui-color-border);",
                        Breadcrumb {
                            items: breadcrumb_items,
                            separator: Some(" / ".to_string()),
                        }
                    }
                }
                Content {
                    div {
                        style: "padding: 16px; background: var(--adui-color-bg-base); min-height: calc(100vh - 48px);",
                        h2 { "Layout + Menu + Breadcrumb + Pagination" }
                        p { style: "color: var(--adui-color-text-secondary);", "模拟典型中后台布局：左侧导航、顶部面包屑、内容区列表 + 分页器。" }

                        // 列表区域
                        div {
                            style: "margin-top: 16px; display: flex; flex-direction: column; gap: 8px;",
                            {
                                (start_index..end_index).map(|idx| {
                                    let label = format!("列表项 #{idx}");
                                    rsx!(
                                        div {
                                            key: "{idx}",
                                            style: "padding: 8px 12px; border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border); background: var(--adui-color-bg-container);",
                                            "{label}"
                                        }
                                    )
                                })
                            }
                        }

                        // 分页器
                        div {
                            style: "margin-top: 16px;",
                            Pagination {
                                total: TOTAL_ITEMS,
                                current: Some(page_clamped),
                                page_size: Some(size),
                                show_total: true,
                                show_size_changer: true,
                                on_change: {
                                    let mut page_sig = current_page;
                                    let mut size_sig = page_size;
                                    move |(page, size)| {
                                        page_sig.set(page);
                                        size_sig.set(size);
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
