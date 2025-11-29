//! QRCode component demo.

use adui_dioxus::{
    App, Button, ButtonType, Card, ComponentSize, ConfigProvider, Input, QRCode, QRCodeErrorLevel,
    QRCodeStatus,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { QRCodeDemoShell {} }
        }
    }
}

#[component]
fn QRCodeDemoShell() -> Element {
    let url = use_signal(|| "https://ant.design".to_string());
    let status = use_signal(|| QRCodeStatus::Active);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "QRCode demo" }
            p { "展示 QRCode 组件的各种用法，包括基础用法、自定义样式和状态展示。" }

            // 基础用法
            h3 { style: "margin-top: 24px;", "基础用法" }
            div { style: "display: flex; gap: 16px; flex-wrap: wrap; align-items: flex-start;",
                Card {
                    children: rsx! {
                        div { style: "display: flex; flex-direction: column; gap: 12px;",
                            QRCode {
                                value: url.read().clone(),
                            }
                            Input {
                                value: url.read().clone(),
                                on_change: {
                                    let mut url = url;
                                    move |v: String| url.set(v)
                                },
                                placeholder: Some("输入 URL...".to_string()),
                            }
                        }
                    }
                }
            }

            // 不同尺寸
            h3 { style: "margin-top: 24px;", "不同尺寸" }
            div { style: "display: flex; gap: 24px; flex-wrap: wrap; align-items: flex-end;",
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        size: 80,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "80px" }
                }
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        size: 120,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "120px" }
                }
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        size: 160,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "160px (默认)" }
                }
            }

            // 自定义颜色
            h3 { style: "margin-top: 24px;", "自定义颜色" }
            div { style: "display: flex; gap: 24px; flex-wrap: wrap;",
                QRCode {
                    value: "https://ant.design".to_string(),
                    color: Some("#1677ff".to_string()),
                }
                QRCode {
                    value: "https://ant.design".to_string(),
                    color: Some("#52c41a".to_string()),
                }
                QRCode {
                    value: "https://ant.design".to_string(),
                    color: Some("#ff4d4f".to_string()),
                    bg_color: Some("#fff1f0".to_string()),
                }
            }

            // 带图标
            h3 { style: "margin-top: 24px;", "带图标" }
            p { style: "color: var(--adui-color-text-secondary); margin-bottom: 12px;",
                "在二维码中心显示图标（如 Logo）。"
            }
            div { style: "display: flex; gap: 24px; flex-wrap: wrap;",
                QRCode {
                    value: "https://ant.design".to_string(),
                    icon: Some("https://gw.alipayobjects.com/zos/rmsportal/KDpgvguMpGfqaHPjicRK.svg".to_string()),
                    icon_size: Some(40),
                }
                QRCode {
                    value: "https://ant.design".to_string(),
                    icon: Some("https://gw.alipayobjects.com/zos/rmsportal/KDpgvguMpGfqaHPjicRK.svg".to_string()),
                    icon_size: Some(60),
                    size: 200,
                    error_level: QRCodeErrorLevel::H,
                }
            }

            // 纠错等级
            h3 { style: "margin-top: 24px;", "纠错等级" }
            p { style: "color: var(--adui-color-text-secondary); margin-bottom: 12px;",
                "不同的纠错等级可以容忍不同程度的损坏。H 级别最高，适合带 Logo 的二维码。"
            }
            div { style: "display: flex; gap: 24px; flex-wrap: wrap;",
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        error_level: QRCodeErrorLevel::L,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "L (~7%)" }
                }
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        error_level: QRCodeErrorLevel::M,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "M (~15%)" }
                }
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        error_level: QRCodeErrorLevel::Q,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "Q (~25%)" }
                }
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        error_level: QRCodeErrorLevel::H,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "H (~30%)" }
                }
            }

            // 状态展示
            h3 { style: "margin-top: 24px;", "状态展示" }
            div { style: "margin-bottom: 12px; display: flex; gap: 8px; flex-wrap: wrap;",
                Button {
                    r#type: if matches!(*status.read(), QRCodeStatus::Active) { ButtonType::Primary } else { ButtonType::Default },
                    onclick: {
                        let mut status = status;
                        move |_| status.set(QRCodeStatus::Active)
                    },
                    "Active"
                }
                Button {
                    r#type: if matches!(*status.read(), QRCodeStatus::Loading) { ButtonType::Primary } else { ButtonType::Default },
                    onclick: {
                        let mut status = status;
                        move |_| status.set(QRCodeStatus::Loading)
                    },
                    "Loading"
                }
                Button {
                    r#type: if matches!(*status.read(), QRCodeStatus::Expired) { ButtonType::Primary } else { ButtonType::Default },
                    onclick: {
                        let mut status = status;
                        move |_| status.set(QRCodeStatus::Expired)
                    },
                    "Expired"
                }
                Button {
                    r#type: if matches!(*status.read(), QRCodeStatus::Scanned) { ButtonType::Primary } else { ButtonType::Default },
                    onclick: {
                        let mut status = status;
                        move |_| status.set(QRCodeStatus::Scanned)
                    },
                    "Scanned"
                }
            }
            QRCode {
                value: "https://ant.design".to_string(),
                status: *status.read(),
                on_refresh: {
                    let mut status = status;
                    move |_| status.set(QRCodeStatus::Active)
                },
            }

            // 无边框
            h3 { style: "margin-top: 24px;", "无边框" }
            div { style: "display: flex; gap: 24px; flex-wrap: wrap;",
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        bordered: true,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "有边框" }
                }
                div { style: "text-align: center;",
                    QRCode {
                        value: "https://ant.design".to_string(),
                        bordered: false,
                    }
                    p { style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);", "无边框" }
                }
            }
        }
    }
}
