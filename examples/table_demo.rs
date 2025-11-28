use adui_dioxus::{App, ComponentSize, ConfigProvider, Table, TableColumn};
use dioxus::prelude::*;
use serde_json::json;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { TableDemoShell {} }
        }
    }
}

#[component]
fn TableDemoShell() -> Element {
    let columns = vec![
        TableColumn::new("name", "姓名"),
        TableColumn::new("age", "年龄"),
        TableColumn::new("city", "城市"),
    ];

    let data = vec![
        json!({ "name": "Alice", "age": 28, "city": "上海" }),
        json!({ "name": "Bob", "age": 35, "city": "北京" }),
        json!({ "name": "Charlie", "age": 42, "city": "深圳" }),
    ];

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Table demo" }
            p { "展示基础表格用法，包含简单列定义与数据映射。" }

            Table {
                columns: columns,
                data: data,
                bordered: true,
                size: Some(ComponentSize::Middle),
                loading: false,
                is_empty: Some(false),
            }
        }
    }
}
