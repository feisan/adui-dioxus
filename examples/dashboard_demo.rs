//! 仪表盘场景演示
//!
//! 模拟真实的仪表盘页面，综合使用：
//! - Layout 布局
//! - Card 卡片
//! - Statistic 统计数值
//! - Table 表格
//! - Progress 进度条
//! - Tabs 标签页

use adui_dioxus::{
    Card, Content, Header, Layout, Progress, Statistic, TabItem, Table, TableColumn, Tabs,
    ThemeProvider, Title, TitleLevel,
};
use dioxus::prelude::*;
use serde_json::json;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DashboardDemo {}
        }
    }
}

#[component]
fn DashboardDemo() -> Element {
    let active_tab = use_signal(|| "1".to_string());

    let table_columns = vec![
        TableColumn::new("name", "姓名"),
        TableColumn::new("age", "年龄"),
        TableColumn::new("city", "城市"),
        TableColumn::new("status", "状态"),
    ];

    let table_data = vec![
        json!({ "name": "Alice", "age": 28, "city": "上海", "status": "活跃" }),
        json!({ "name": "Bob", "age": 35, "city": "北京", "status": "活跃" }),
        json!({ "name": "Charlie", "age": 42, "city": "深圳", "status": "离线" }),
        json!({ "name": "David", "age": 29, "city": "广州", "status": "活跃" }),
    ];

    rsx! {
        Layout {
            style: "min-height: 100vh; background: var(--adui-color-bg-base);",
            Header {
                div {
                    style: "padding: 16px 24px; background: var(--adui-color-bg-container); border-bottom: 1px solid var(--adui-color-border);",
                    Title { level: TitleLevel::H3, "仪表盘" }
                }
            }
            Content {
                div {
                    style: "padding: 24px;",
                    // 统计卡片
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 16px; margin-bottom: 24px;",
                        Card {
                            title: Some(rsx!("总用户数")),
                            Statistic {
                                value: 12345.0,
                                precision: Some(0),
                            }
                        }
                        Card {
                            title: Some(rsx!("今日访问")),
                            Statistic {
                                value: 1234.0,
                                precision: Some(0),
                            }
                        }
                        Card {
                            title: Some(rsx!("活跃用户")),
                            Statistic {
                                value: 5678.0,
                                precision: Some(0),
                            }
                        }
                        Card {
                            title: Some(rsx!("转化率")),
                            Statistic {
                                value: 12.5,
                                suffix: Some(rsx!("%")),
                                precision: Some(1),
                            }
                        }
                    }

                    // 标签页内容
                    Tabs {
                        items: vec![
                            TabItem::new("1", "概览", Some(rsx!(
                                OverviewTab {}
                            ))),
                            TabItem::new("2", "数据表格", Some(rsx!(
                                DataTableTab {
                                    columns: table_columns.clone(),
                                    data: table_data.clone(),
                                }
                            ))),
                            TabItem::new("3", "进度监控", Some(rsx!(
                                ProgressTab {}
                            ))),
                        ],
                        active_key: Some(active_tab.read().clone()),
                        on_change: {
                            let mut sig = active_tab;
                            move |key: String| {
                                sig.set(key);
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn OverviewTab() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            Card {
                title: Some(rsx!("系统状态")),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        span { "CPU 使用率" }
                        Progress {
                            percent: 65.0,
                        }
                    }
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        span { "内存使用率" }
                        Progress {
                            percent: 45.0,
                        }
                    }
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        span { "磁盘使用率" }
                        Progress {
                            percent: 80.0,
                        }
                    }
                }
            }
            Card {
                title: Some(rsx!("最近活动")),
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div { "用户 Alice 登录了系统" }
                    div { "用户 Bob 创建了新项目" }
                    div { "用户 Charlie 更新了设置" }
                }
            }
        }
    }
}

#[component]
fn DataTableTab(columns: Vec<TableColumn>, data: Vec<serde_json::Value>) -> Element {
    rsx! {
        Card {
            Table {
                columns: columns.clone(),
                data: data.clone(),
                bordered: true,
            }
        }
    }
}

#[component]
fn ProgressTab() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",
            Card {
                title: Some(rsx!("任务进度")),
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        div {
                            style: "display: flex; justify-content: space-between;",
                            span { "任务 1" }
                            span { "65%" }
                        }
                        Progress {
                            percent: 65.0,
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        div {
                            style: "display: flex; justify-content: space-between;",
                            span { "任务 2" }
                            span { "45%" }
                        }
                        Progress {
                            percent: 45.0,
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        div {
                            style: "display: flex; justify-content: space-between;",
                            span { "任务 3" }
                            span { "100%" }
                        }
                        Progress {
                            percent: 100.0,
                        }
                    }
                }
            }
        }
    }
}
