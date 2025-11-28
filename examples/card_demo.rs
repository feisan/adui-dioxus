use adui_dioxus::{App, Button, ButtonType, Card, CardProps, ComponentSize, ConfigProvider};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { CardDemoShell {} }
        }
    }
}

#[component]
fn CardDemoShell() -> Element {
    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Card demo" }
            p { "展示基础 Card 用法，包括标题/extra/hoverable 与 loading 占位。" }

            // 基础卡片
            h3 { style: "margin-top: 16px;", "基础卡片" }
            Card {
                title: Some(rsx!("基础信息")),
                children: rsx! {
                    p { "这里是基础卡片的正文内容，可以放任意组件。" }
                }
            }

            // 带 extra 与 hoverable 的卡片
            h3 { style: "margin-top: 24px;", "带 extra 的操作卡片" }
            Card {
                title: Some(rsx!("项目概览")),
                extra: Some(rsx!(
                    Button { r#type: ButtonType::Link, "查看详情" }
                )),
                hoverable: true,
                children: rsx! {
                    p { "卡片右上角可以放操作按钮或链接，例如“查看详情”。" }
                }
            }

            // loading 卡片
            h3 { style: "margin-top: 24px;", "加载态卡片" }
            Card {
                title: Some(rsx!("加载中的数据块")),
                loading: true,
                children: rsx! {
                    // children 会在 loading=false 时展示，这里仅占位
                    p { "加载完成后将显示的内容。" }
                }
            }
        }
    }
}
