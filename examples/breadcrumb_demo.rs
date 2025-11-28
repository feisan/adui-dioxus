use adui_dioxus::{App, Breadcrumb, BreadcrumbItem, ComponentSize, ConfigProvider, use_message};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { BreadcrumbDemoShell {} }
        }
    }
}

fn demo_items() -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem::with_href("home", "首页", "/"),
        BreadcrumbItem::with_href("list", "列表", "/list"),
        BreadcrumbItem::new("detail", "详情"),
    ]
}

#[component]
fn BreadcrumbDemoShell() -> Element {
    let api = use_message();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Breadcrumb demo" }
            p { "展示一个典型路径：首页 / 列表 / 详情，点击前两级会触发 message 提示。" }

            Breadcrumb {
                items: demo_items(),
                separator: Some(" > ".to_string()),
                on_item_click: move |id: String| {
                    if let Some(msg) = api.clone() {
                        msg.info(format!("点击了节点: {id}"));
                    }
                },
            }

            p { style: "margin-top: 12px; color: var(--adui-color-text-secondary);",
                "当前页面视为最后一个节点“详情”，仅作为文本展示。"
            }
        }
    }
}
