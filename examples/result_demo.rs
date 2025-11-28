use adui_dioxus::{
    App, Button, ButtonType, Card, ComponentSize, ConfigProvider, Layout, Result, ResultStatus,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { ResultDemoShell {} }
        }
    }
}

#[component]
fn ResultDemoShell() -> Element {
    rsx! {
        Layout {
            adui_dioxus::Content {
                class: None,
                style: Some("padding: 24px; background: var(--adui-color-bg-base);".into()),
                has_sider: None,
                children: rsx! {
                    h2 { "Result demo" }
                    p { "展示基础成功/错误结果页，并组合按钮作为操作区域。" }

                    Card {
                        title: Some(rsx!("操作成功")),
                        children: rsx! {
                            Result {
                                status: Some(ResultStatus::Success),
                                title: Some(rsx!("提交成功")),
                                sub_title: Some(rsx!("您的操作已成功提交，可以在列表页查看最新状态。")),
                                extra: Some(rsx!(
                                    div { style: "display: inline-flex; gap: 8px;",
                                        Button { r#type: ButtonType::Primary, "返回列表" }
                                        Button { r#type: ButtonType::Default, "查看详情" }
                                    }
                                )),
                            }
                        },
                    }

                    Card {
                        style: Some("margin-top: 24px;".into()),
                        title: Some(rsx!("错误结果")),
                        children: rsx! {
                            Result {
                                status: Some(ResultStatus::ServerError),
                                title: Some(rsx!("服务器错误")),
                                sub_title: Some(rsx!(
                                    "请求处理过程中发生了未知错误，请稍后重试，或联系管理员。"
                                )),
                                extra: Some(rsx!(
                                    div { style: "display: inline-flex; gap: 8px;",
                                        Button { r#type: ButtonType::Primary, "重新尝试" }
                                        Button { r#type: ButtonType::Default, "返回首页" }
                                    }
                                )),
                            }
                        },
                    }
                },
            }
        }
    }
}
