//! QRCode 组件演示
//!
//! 展示 QRCode 组件的基础用法和高级用法，包括：
//! - 基础二维码
//! - 不同尺寸
//! - 自定义颜色
//! - 带图标
//! - 纠错等级
//! - 状态展示

use adui_dioxus::{
    Button, ButtonType, Card, Input, QRCode, QRCodeErrorLevel, QRCodeStatus, ThemeMode,
    ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            QRCodeDemo {}
        }
    }
}

#[component]
fn QRCodeDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let url = use_signal(|| "https://ant.design".to_string());
    let status = use_signal(|| QRCodeStatus::Active);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",

            // 控制工具栏
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 24px; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border);",
                span { style: "font-weight: 600;", "主题控制：" }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Light,
                    "Light"
                }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Dark,
                    "Dark"
                }
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // 基础二维码
            DemoSection {
                title: "基础二维码",
                Card {
                    children: rsx! {
                        div {
                            style: "display: flex; flex-direction: column; gap: 12px;",
                            QRCode {
                                value: url.read().clone(),
                            }
                            Input {
                                value: url.read().clone(),
                                on_change: {
                                    let mut sig = url;
                                    move |v: String| sig.set(v)
                                },
                                placeholder: Some("输入 URL...".to_string()),
                            }
                        }
                    }
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap; align-items: flex-end;",
                    div {
                        style: "text-align: center;",
                        QRCode {
                            value: "https://ant.design".to_string(),
                            size: 80,
                        }
                        p {
                            style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);",
                            "80px"
                        }
                    }
                    div {
                        style: "text-align: center;",
                        QRCode {
                            value: "https://ant.design".to_string(),
                            size: 120,
                        }
                        p {
                            style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);",
                            "120px"
                        }
                    }
                    div {
                        style: "text-align: center;",
                        QRCode {
                            value: "https://ant.design".to_string(),
                            size: 160,
                        }
                        p {
                            style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);",
                            "160px (默认)"
                        }
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 自定义颜色
            DemoSection {
                title: "自定义颜色",
                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap;",
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
            }

            // 带图标
            DemoSection {
                title: "带图标",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "在二维码中心显示图标（如 Logo）："
                    }
                    div {
                        style: "display: flex; gap: 24px; flex-wrap: wrap;",
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
                }
            }

            // 纠错等级
            DemoSection {
                title: "纠错等级",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    span {
                        style: "font-size: 14px; color: var(--adui-color-text-secondary);",
                        "不同的纠错等级可以容忍不同程度的损坏。H 级别最高，适合带 Logo 的二维码。"
                    }
                    div {
                        style: "display: flex; gap: 24px; flex-wrap: wrap;",
                        for (level, label) in [
                            (QRCodeErrorLevel::L, "L (~7%)"),
                            (QRCodeErrorLevel::M, "M (~15%)"),
                            (QRCodeErrorLevel::Q, "Q (~25%)"),
                            (QRCodeErrorLevel::H, "H (~30%)"),
                        ] {
                            div {
                                style: "text-align: center;",
                                QRCode {
                                    value: "https://ant.design".to_string(),
                                    error_level: level,
                                }
                                p {
                                    style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);",
                                    {label}
                                }
                            }
                        }
                    }
                }
            }

            // 状态展示
            DemoSection {
                title: "状态展示",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    div {
                        style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        for (s, label) in [
                            (QRCodeStatus::Active, "Active"),
                            (QRCodeStatus::Loading, "Loading"),
                            (QRCodeStatus::Expired, "Expired"),
                            (QRCodeStatus::Scanned, "Scanned"),
                        ] {
                            Button {
                                r#type: if matches!(*status.read(), s) { ButtonType::Primary } else { ButtonType::Default },
                                onclick: {
                                    let mut sig = status;
                                    let s_val = s;
                                    move |_| sig.set(s_val)
                                },
                                {label}
                            }
                        }
                    }
                    QRCode {
                        value: "https://ant.design".to_string(),
                        status: *status.read(),
                        on_refresh: {
                            let mut sig = status;
                            move |_| sig.set(QRCodeStatus::Active)
                        },
                    }
                }
            }

            // 边框控制
            DemoSection {
                title: "边框控制",
                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap;",
                    div {
                        style: "text-align: center;",
                        QRCode {
                            value: "https://ant.design".to_string(),
                            bordered: true,
                        }
                        p {
                            style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);",
                            "有边框"
                        }
                    }
                    div {
                        style: "text-align: center;",
                        QRCode {
                            value: "https://ant.design".to_string(),
                            bordered: false,
                        }
                        p {
                            style: "margin-top: 8px; font-size: 12px; color: var(--adui-color-text-secondary);",
                            "无边框"
                        }
                    }
                }
            }
        }
    }
}

// 统一的demo section组件
#[derive(Props, Clone, PartialEq)]
struct DemoSectionProps {
    title: &'static str,
    children: Element,
}

#[component]
fn DemoSection(props: DemoSectionProps) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 24px; padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);",
            div {
                style: "font-weight: 600; margin-bottom: 12px; color: var(--adui-color-text); font-size: 14px;",
                {props.title}
            }
            {props.children}
        }
    }
}
