use adui_dioxus::{
    App, Avatar, AvatarGroup, Badge, BadgeStatus, Breadcrumb, BreadcrumbItem, Button, ButtonType,
    Card, ComponentSize, ConfigProvider, Content, Header, Layout, List, Menu, MenuItemNode,
    MenuMode, Pagination, TabItem, Tabs, Tag,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { DashboardShell {} }
        }
    }
}

fn sider_items() -> Vec<MenuItemNode> {
    vec![
        MenuItemNode::leaf("overview", "总览"),
        MenuItemNode::leaf("users", "用户"),
        MenuItemNode::leaf("settings", "设置"),
    ]
}

fn breadcrumb_for(key: &str) -> Vec<BreadcrumbItem> {
    match key {
        "overview" => vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::new("overview", "总览"),
        ],
        "users" => vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::new("users", "用户"),
        ],
        "settings" => vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::new("settings", "设置"),
        ],
        _ => vec![BreadcrumbItem::new("home", "首页")],
    }
}

#[component]
fn DashboardShell() -> Element {
    let mut sider_selected = use_signal(|| vec!["overview".to_string()]);
    let mut tab_active = use_signal(|| "overview-dashboard".to_string());
    let mut current_page = use_signal(|| 1u32);
    let page_size: u32 = 5;

    const TOTAL_USERS: u32 = 23;

    let current_menu = sider_selected
        .read()
        .first()
        .cloned()
        .unwrap_or_else(|| "overview".to_string());
    let breadcrumb_items = breadcrumb_for(&current_menu);

    let page = (*current_page.read()).max(1);
    let total_pages = ((TOTAL_USERS + page_size - 1) / page_size).max(1);
    let page_clamped = page.min(total_pages);
    let start_index = ((page_clamped - 1) * page_size) as usize;
    let end_index = (start_index as u32 + page_size).min(TOTAL_USERS) as usize;

    rsx! {
        Layout {
            has_sider: Some(true),
            // 左侧菜单
            adui_dioxus::Sider {
                width: Some(220.0),
                collapsed_width: Some(80.0),
                collapsible: true,
                theme: adui_dioxus::SiderTheme::Dark,
                has_border: true,
                div { style: "margin-bottom: 12px; font-weight: 600; color: #fff;", "仪表盘" }
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
                // 顶部 Header：Breadcrumb + 用户信息
                Header {
                    div {
                        style: "padding: 0 16px; display: flex; align-items: center; height: 56px; background: var(--adui-color-bg-container); border-bottom: 1px solid var(--adui-color-border);",
                        div { style: "flex: 1;", Breadcrumb { items: breadcrumb_items, separator: Some(" / ".to_string()) } }
                        div { style: "display: flex; align-items: center; gap: 12px;",
                            Badge {
                                count: Some(3),
                                children: Some(rsx!(
                                    Button { r#type: ButtonType::Default, "消息" }
                                )),
                            }
                            AvatarGroup {
                                children: rsx! {
                                    Avatar { children: Some(rsx!("ME")), }
                                    Avatar { children: Some(rsx!("UX")), }
                                }
                            }
                        }
                    }
                }

                Content {
                    div {
                        style: "padding: 16px; background: var(--adui-color-bg-base); min-height: calc(100vh - 56px);",
                        h2 { "仪表盘" }
                        p { style: "color: var(--adui-color-text-secondary);", "示例：组合 Tabs / Card / Tag / Badge / Avatar / List / Pagination 构成典型中后台仪表盘页面。" }

                        // 顶部统计卡片
                        div { style: "display: flex; gap: 16px; margin-top: 16px; flex-wrap: wrap;",
                            Card {
                                hoverable: true,
                                title: Some(rsx!("今日访问量")),
                                children: rsx!(
                                    h3 { "1,234" }
                                    Tag { color: Some(adui_dioxus::TagColor::Success), children: rsx!("较昨日 +10%") }
                                ),
                            }
                            Card {
                                hoverable: true,
                                title: Some(rsx!("活跃用户")),
                                children: rsx!(
                                    h3 { "567" }
                                    Tag { color: Some(adui_dioxus::TagColor::Warning), children: rsx!("较昨日 -3%") }
                                ),
                            }
                            Card {
                                hoverable: true,
                                title: Some(rsx!("待处理工单")),
                                children: rsx!(
                                    h3 { "12" }
                                    Tag { color: Some(adui_dioxus::TagColor::Error), children: rsx!("请尽快处理") }
                                ),
                            }
                        }

                        // Tabs 区域：不同视图
                        div { style: "margin-top: 24px;",
                            Tabs {
                                items: vec![
                                    TabItem::new("overview-dashboard", "概览", None),
                                    TabItem::new("user-list", "用户列表", None),
                                    TabItem::new("settings", "设置", None),
                                ],
                                active_key: Some((*tab_active.read()).clone()),
                                on_change: move |key: String| {
                                    tab_active.set(key);
                                },
                            }

                            // Tab 内容区域
                            match tab_active.read().as_str() {
                                "overview-dashboard" => rsx! {
                                    Card {
                                        title: Some(rsx!("系统说明")),
                                        children: rsx!(
                                            p { "这里可以放一些概览性的说明、图表或近期活动记录。" }
                                        ),
                                    }
                                },
                                "user-list" => rsx! {
                                    Card {
                                        title: Some(rsx!("用户列表")),
                                        children: rsx!(
                                            List {
                                                bordered: true,
                                                is_empty: Some(start_index >= end_index),
                                                pagination_total: Some(TOTAL_USERS),
                                                pagination_current: Some(page_clamped),
                                                pagination_page_size: Some(page_size),
                                                pagination_on_change: {
                                                    let mut page_sig = current_page;
                                                    move |(next, _size)| page_sig.set(next)
                                                },
                                                children: rsx! {
                                                    {(start_index..end_index).map(|idx| {
                                                        let user_name = format!("用户 #{idx}");
                                                        rsx!(
                                                            div { class: "adui-list-item",
                                                                div { style: "display: flex; align-items: center; gap: 8px;",
                                                                    Avatar { children: Some(rsx!("U")), }
                                                                    div { "{user_name}" }
                                                                    Tag { color: Some(adui_dioxus::TagColor::Success), children: rsx!("正常") }
                                                                }
                                                            }
                                                        )
                                                    })}
                                                }
                                            }
                                        ),
                                    }
                                },
                                "settings" => rsx! {
                                    Card {
                                        title: Some(rsx!("设置")),
                                        children: rsx!(
                                            p { "设置区域可以放置表单或其他配置项。" }
                                        ),
                                    }
                                },
                                _ => rsx! {},
                            }
                        }
                    }
                }
            }
        }
    }
}
