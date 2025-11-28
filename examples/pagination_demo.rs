use adui_dioxus::{App, ComponentSize, ConfigProvider, Pagination};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { PaginationDemoShell {} }
        }
    }
}

#[component]
fn PaginationDemoShell() -> Element {
    let current = use_signal(|| 1u32);
    let page_size = use_signal(|| 10u32);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Pagination demo" }
            p { "展示基础分页用法，包含页码切换与页面大小切换。" }

            div { style: "display: flex; flex-direction: column; gap: 12px; align-items: flex-start;",
                Pagination {
                    total: 100,
                    current: Some(*current.read()),
                    page_size: Some(*page_size.read()),
                    show_total: true,
                    show_size_changer: true,
                    on_change: {
                        let mut current = current;
                        let mut page_size = page_size;
                        move |(page, size)| {
                            current.set(page);
                            page_size.set(size);
                        }
                    },
                }

                p { "当前页: {current}，每页条数: {page_size}" }
            }
        }
    }
}
