use adui_dioxus::{App, Card, CardProps, ComponentSize, ConfigProvider, Tag, TagColor, TagProps};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { TagDemoShell {} }
        }
    }
}

#[component]
fn TagDemoShell() -> Element {
    let mut checked = use_signal(|| true);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Tag demo" }
            p { "展示基础 Tag 用法，包括预设颜色、可关闭与简单可选中标签。" }

            // 预设颜色标签
            h3 { style: "margin-top: 16px;", "预设颜色" }
            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                Tag { children: rsx!("Default") }
                Tag { color: Some(TagColor::Primary), children: rsx!("Primary") }
                Tag { color: Some(TagColor::Success), children: rsx!("Success") }
                Tag { color: Some(TagColor::Warning), children: rsx!("Warning") }
                Tag { color: Some(TagColor::Error), children: rsx!("Error") }
            }

            // 可关闭标签
            h3 { style: "margin-top: 24px;", "可关闭标签" }
            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                Tag { closable: true, children: rsx!("Closable") }
                Tag { color: Some(TagColor::Primary), closable: true, children: rsx!("Primary closable") }
            }

            // 可选中标签
            h3 { style: "margin-top: 24px;", "可选中标签" }
            p { style: "color: var(--adui-color-text-secondary);", "点击标签切换选中状态。" }
            div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                Tag {
                    checkable: true,
                    checked: Some(*checked.read()),
                    on_change: move |next| checked.set(next),
                    children: rsx!("Checkable")
                }
                Tag {
                    checkable: true,
                    default_checked: Some(true),
                    children: rsx!("Uncontrolled checkable")
                }
            }

            // 表格/列表中的状态标签示例（简单放在 Card 内）
            h3 { style: "margin-top: 24px;", "状态标签示例" }
            Card {
                title: Some(rsx!("订单状态")),
                children: rsx! {
                    div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        Tag { color: Some(TagColor::Success), children: rsx!("已完成") }
                        Tag { color: Some(TagColor::Warning), children: rsx!("进行中") }
                        Tag { color: Some(TagColor::Error), children: rsx!("已关闭") }
                    }
                }
            }
        }
    }
}
