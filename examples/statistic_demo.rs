use adui_dioxus::{App, Card, Col, ComponentSize, ConfigProvider, Row, Statistic, Tag};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { StatisticDemoShell {} }
        }
    }
}

#[component]
fn StatisticDemoShell() -> Element {
    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Statistic demo" }
            p { "数字统计卡片示例，展示基础数值与前后缀组合。" }

            Row {
                Col { span: 8,
                    Card {
                        title: Some(rsx!("今日访问量")),
                        children: rsx! {
                            Statistic {
                                title: Some(rsx!("Visits")),
                                value: Some(12345.0),
                                precision: Some(0),
                                suffix: Some(rsx!("次")),
                            }
                            Tag { children: rsx!("较昨日 +8%") }
                        },
                    }
                }
                Col { span: 8,
                    Card {
                        title: Some(rsx!("转化率")),
                        children: rsx! {
                            Statistic {
                                title: Some(rsx!("Conversion")),
                                value: Some(3.1415),
                                precision: Some(2),
                                suffix: Some(rsx!("%")),
                            }
                            Tag { children: rsx!("较昨日 -1.2%") }
                        },
                    }
                }
                Col { span: 8,
                    Card {
                        title: Some(rsx!("错误率")),
                        children: rsx! {
                            Statistic {
                                title: Some(rsx!("Error rate")),
                                value: Some(0.07),
                                precision: Some(2),
                                suffix: Some(rsx!("%")),
                            }
                            Tag { children: rsx!("稳定") }
                        },
                    }
                }
            }
        }
    }
}
