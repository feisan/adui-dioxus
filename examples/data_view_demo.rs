use adui_dioxus::{
    App, Breadcrumb, BreadcrumbItem, Button, ButtonType, ComponentSize, ConfigProvider, Content,
    Header, Layout, List, Skeleton, Table, TableColumn,
};
use dioxus::prelude::*;
use serde_json::json;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { DataViewDemoShell {} }
        }
    }
}

fn user_breadcrumb() -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem::with_href("home", "首页", "/"),
        BreadcrumbItem::with_href("user", "用户管理", "/users"),
        BreadcrumbItem::new("list", "用户列表"),
    ]
}

fn user_data() -> Vec<serde_json::Value> {
    vec![
        json!({ "name": "Alice", "email": "alice@example.com", "role": "管理员" }),
        json!({ "name": "Bob", "email": "bob@example.com", "role": "运营" }),
        json!({ "name": "Charlie", "email": "charlie@example.com", "role": "客服" }),
        json!({ "name": "David", "email": "david@example.com", "role": "访客" }),
        json!({ "name": "Eve", "email": "eve@example.com", "role": "管理员" }),
        json!({ "name": "Frank", "email": "frank@example.com", "role": "运营" }),
        json!({ "name": "Grace", "email": "grace@example.com", "role": "客服" }),
        json!({ "name": "Heidi", "email": "heidi@example.com", "role": "访客" }),
        json!({ "name": "Ivan", "email": "ivan@example.com", "role": "运营" }),
        json!({ "name": "Judy", "email": "judy@example.com", "role": "管理员" }),
    ]
}

#[component]
fn DataViewDemoShell() -> Element {
    // 一组简单的列表页状态：loading / empty / 分页。
    let loading = use_signal(|| false);
    let empty = use_signal(|| false);
    let current_page = use_signal(|| 1u32);
    let page_size: u32 = 5;

    let all_data = user_data();
    let total = if *empty.read() {
        0
    } else {
        all_data.len() as u32
    };

    let page = (*current_page.read()).max(1);
    let total_pages = total.div_ceil(page_size);
    let page_clamped = if total == 0 { 1 } else { page.min(total_pages) };

    let start_index = if total == 0 {
        0
    } else {
        ((page_clamped - 1) * page_size) as usize
    };
    let end_index = if total == 0 {
        0
    } else {
        (start_index as u32 + page_size).min(total) as usize
    };

    let page_rows: Vec<serde_json::Value> = if total == 0 {
        Vec::new()
    } else {
        all_data[start_index..end_index].to_vec()
    };

    let columns = vec![
        TableColumn::new("name", "姓名"),
        TableColumn::new("email", "邮箱"),
        TableColumn::new("role", "角色"),
    ];

    let breadcrumb_items = user_breadcrumb();

    let loading_flag = *loading.read();
    let empty_flag = total == 0 && !loading_flag;

    rsx! {
        Layout {
            Header {
                div {
                    style: "padding: 0 16px; display: flex; align-items: center; height: 48px; background: var(--adui-color-bg-container); border-bottom: 1px solid var(--adui-color-border);",
                    Breadcrumb { items: breadcrumb_items, separator: Some(" / ".to_string()) }
                }
            }
            Content {
                div {
                    style: "padding: 16px; background: var(--adui-color-bg-base); min-height: calc(100vh - 48px);",
                    h2 { "数据视图综合示例" }
                    p { style: "color: var(--adui-color-text-secondary);", "演示 Layout + Breadcrumb + Table/List + Pagination + Empty/Spin/Skeleton 的组合，用于典型用户列表页。" }

                    // 状态控制区域
                    div { style: "margin-top: 16px; display: flex; gap: 8px;",
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let mut flag = loading;
                                move |_| {
                                    let next = !*flag.read();
                                    flag.set(next);
                                }
                            },
                            if loading_flag { "停止加载" } else { "模拟加载" }
                        }
                        Button {
                            r#type: ButtonType::Default,
                            onclick: {
                                let mut flag = empty;
                                let mut page_sig = current_page;
                                move |_| {
                                    let next = !*flag.read();
                                    flag.set(next);
                                    if next {
                                        page_sig.set(1);
                                    }
                                }
                            },
                            if empty_flag { "恢复数据" } else { "清空数据" }
                        }
                    }

                    // 当整页处于 loading 时，展示简单 Skeleton
                    if loading_flag {
                        div { style: "margin-top: 16px; max-width: 720px;",
                            Skeleton {
                                loading: Some(true),
                                active: true,
                                paragraph_rows: Some(3),
                            }
                        }
                    }

                    // 表格视图：受控分页 + Table 内部 Empty/Spin 体验。
                    div { style: "margin-top: 24px; max-width: 960px;",
                        h3 { "表格视图（Table）" }
                        Table {
                            columns: columns.clone(),
                            data: page_rows.clone(),
                            bordered: true,
                            size: Some(ComponentSize::Middle),
                            loading: loading_flag,
                            is_empty: Some(empty_flag),
                            pagination_total: Some(total),
                            pagination_current: Some(page_clamped),
                            pagination_page_size: Some(page_size),
                            pagination_on_change: {
                                let mut page_sig = current_page;
                                move |(next, _size)| page_sig.set(next)
                            },
                        }
                    }

                    // 列表视图：复用相同数据，通过 List + Empty/Spin 实现视觉一致的列表页。
                    div { style: "margin-top: 32px; max-width: 720px;",
                        h3 { "列表视图（List）" }
                        List {
                            bordered: true,
                            header: Some(rsx!("用户列表（与上方表格数据同步）")),
                            loading: loading_flag,
                            is_empty: Some(empty_flag),
                            pagination_total: Some(total),
                            pagination_current: Some(page_clamped),
                            pagination_page_size: Some(page_size),
                            pagination_on_change: {
                                let mut page_sig = current_page;
                                move |(next, _size)| page_sig.set(next)
                            },
                            children: rsx! {
                                {page_rows.iter().enumerate().map(|(idx, row)| {
                                    let name = row.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                    let email = row.get("email").and_then(|v| v.as_str()).unwrap_or("");
                                    let role = row.get("role").and_then(|v| v.as_str()).unwrap_or("");
                                    let label = format!("{name} ({role})");
                                    rsx!(
                                        div {
                                            key: "user-{idx}",
                                            class: "adui-list-item",
                                            div { "{label}" }
                                            div { style: "color: var(--adui-color-text-secondary); font-size: 12px;", "{email}" }
                                        }
                                    )
                                })}
                            }
                        }
                    }
                }
            }
        }
    }
}
