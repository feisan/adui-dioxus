use adui_dioxus::{App, Button, ButtonType, ComponentSize, ConfigProvider, Skeleton};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { SkeletonDemoShell {} }
        }
    }
}

#[component]
fn SkeletonDemoShell() -> Element {
    let mut loading = use_signal(|| true);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Skeleton demo" }
            p { "展示基础骨架屏用法，以及加载完成后渲染真实内容。" }

            h3 { style: "margin-top: 16px;", "纯骨架" }
            Skeleton {}

            h3 { style: "margin-top: 24px;", "包裹真实内容" }
            div { style: "margin-bottom: 8px;",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut flag = loading;
                        move |_| {
                            let next = !*flag.read();
                            flag.set(next);
                        }
                    },
                    if *loading.read() { "加载完成" } else { "重新加载" }
                }
            }
            Skeleton {
                loading: Some(*loading.read()),
                active: true,
                paragraph_rows: Some(3),
                content: Some(rsx! {
                    div {
                        style: "padding: 16px; border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border); background: var(--adui-color-bg-container); max-width: 360px;",
                        h4 { "真实内容" }
                        p { style: "margin: 0; color: var(--adui-color-text-secondary);",
                            "当 loading=false 时，Skeleton 渲染子内容而不再显示骨架。"
                        }
                    }
                }),
            }
        }
    }
}
