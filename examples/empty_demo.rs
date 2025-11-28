use adui_dioxus::{App, Button, ButtonType, ComponentSize, ConfigProvider, Empty};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { EmptyDemoShell {} }
        }
    }
}

#[component]
fn EmptyDemoShell() -> Element {
    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Empty demo" }
            p { "展示统一的空状态组件 Empty，不同描述与附加操作按钮。" }

            div { style: "margin-top: 16px; display: flex; flex-direction: column; gap: 24px; max-width: 480px;",
                // 基础用法
                div {
                    h3 { "基础 Empty" }
                    Empty {}
                }

                // 自定义描述
                div {
                    h3 { "自定义描述" }
                    Empty { description: Some("当前没有任何记录".to_string()) }
                }

                // 带操作按钮
                div {
                    h3 { "带操作按钮" }
                    Empty {
                        description: Some("还没有创建任何项目".to_string()),
                        footer: Some(rsx! {
                            Button { r#type: ButtonType::Primary, "创建第一个项目" }
                        }),
                    }
                }
            }
        }
    }
}
