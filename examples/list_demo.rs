use adui_dioxus::{App, ComponentSize, ConfigProvider, Empty, List};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { ListDemoShell {} }
        }
    }
}

const ITEMS: &[&str] = &[
    "列表项 1",
    "列表项 2",
    "列表项 3",
    "列表项 4",
    "列表项 5",
    "列表项 6",
    "列表项 7",
    "列表项 8",
    "列表项 9",
    "列表项 10",
];

#[component]
fn ListDemoShell() -> Element {
    let mut current_page = use_signal(|| 1u32);
    let page_size: u32 = 4;
    let total = ITEMS.len() as u32;

    let page = *current_page.read();
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(ITEMS.len());

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "List demo" }
            p { "展示基础列表用法，包含简单列表、空列表和分页列表。" }

            // 基础无分页列表
            h3 { style: "margin-top: 16px;", "基础列表" }
            List {
                bordered: true,
                header: Some(rsx!("最近更新")),
                footer: Some(rsx!("共 3 条")),
                children: rsx! {
                    div { class: "adui-list-item", "列表项 A" }
                    div { class: "adui-list-item", "列表项 B" }
                    div { class: "adui-list-item", "列表项 C" }
                }
            }

            // 空列表示例
            h3 { style: "margin-top: 24px;", "空列表" }
            List {
                bordered: true,
                is_empty: Some(true),
                empty: Some(rsx!(Empty { description: Some("当前没有任何记录".to_string()) })),
                children: rsx! { /* 无实际项，由 is_empty 控制空态 */ }
            }

            // 分页列表
            h3 { style: "margin-top: 24px;", "分页列表" }
            List {
                bordered: true,
                header: Some(rsx!("分页示例")),
                is_empty: Some(start >= end),
                pagination_total: Some(total),
                pagination_current: Some(page),
                pagination_page_size: Some(page_size),
                pagination_on_change: {
                    let mut page_sig = current_page;
                    move |(next, _size)| page_sig.set(next)
                },
                children: rsx! {
                    {(start..end).map(|idx| {
                        let text = ITEMS[idx];
                        rsx!(
                            div { class: "adui-list-item", "{text}" }
                        )
                    })}
                }
            }
        }
    }
}
