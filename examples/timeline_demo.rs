//! Timeline 组件演示
//!
//! 展示 Timeline 组件的基础用法和高级用法，包括：
//! - 基础时间轴
//! - 不同颜色
//! - 自定义图标
//! - 进行中状态
//! - 交替展示
//! - 右侧展示
//! - 倒序排列

use adui_dioxus::{
    Button, ButtonType, Icon, IconKind, Text, TextType, ThemeMode, ThemeProvider, Timeline,
    TimelineColor, TimelineItem, TimelineMode, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TimelineDemo {}
        }
    }
}

#[component]
fn TimelineDemo() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);

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

            // 基础时间轴
            DemoSection {
                title: "基础时间轴",
                Timeline {
                    items: vec![
                        TimelineItem::new("1")
                            .title(rsx! { Text { "创建服务" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 09:00:00" } })
                            .color(TimelineColor::Green),
                        TimelineItem::new("2")
                            .title(rsx! { Text { "初步审核" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 10:30:00" } })
                            .color(TimelineColor::Green),
                        TimelineItem::new("3")
                            .title(rsx! { Text { "技术评估" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 14:00:00" } })
                            .color(TimelineColor::Green),
                        TimelineItem::new("4")
                            .title(rsx! { Text { "最终审批" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "等待处理..." } })
                            .color(TimelineColor::Gray),
                    ],
                }
            }

            // 不同颜色
            DemoSection {
                title: "不同颜色",
                Timeline {
                    items: vec![
                        TimelineItem::new("1")
                            .title(rsx! { Text { "创建成功" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "使用绿色表示成功状态" } })
                            .color(TimelineColor::Green),
                        TimelineItem::new("2")
                            .title(rsx! { Text { "处理中" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "使用蓝色表示进行中" } })
                            .color(TimelineColor::Blue),
                        TimelineItem::new("3")
                            .title(rsx! { Text { "出现错误" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "使用红色表示错误状态" } })
                            .color(TimelineColor::Red),
                        TimelineItem::new("4")
                            .title(rsx! { Text { "已完成" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "使用灰色表示完成" } })
                            .color(TimelineColor::Gray),
                    ],
                }
            }

            // 自定义图标
            DemoSection {
                title: "自定义图标",
                Timeline {
                    items: vec![
                        TimelineItem::new("1")
                            .title(rsx! { Text { "用户注册" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "新用户完成注册" } })
                            .icon(rsx! {
                                Icon { kind: IconKind::Check, color: Some("#52c41a".to_string()) }
                            }),
                        TimelineItem::new("2")
                            .title(rsx! { Text { "信息审核" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "正在审核用户信息" } })
                            .icon(rsx! {
                                Icon { kind: IconKind::Loading, spin: true, color: Some("#1890ff".to_string()) }
                            }),
                        TimelineItem::new("3")
                            .title(rsx! { Text { "审核失败" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "信息不完整，需要补充" } })
                            .icon(rsx! {
                                Icon { kind: IconKind::Close, color: Some("#f5222d".to_string()) }
                            }),
                        TimelineItem::new("4")
                            .title(rsx! { Text { "等待重新提交" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "请补充完整信息后重新提交" } })
                            .icon(rsx! {
                                Icon { kind: IconKind::Info, color: Some("#faad14".to_string()) }
                            }),
                    ],
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 进行中状态
            DemoSection {
                title: "进行中状态",
                Timeline {
                    items: vec![
                        TimelineItem::new("1")
                            .title(rsx! { Text { "步骤一：提交申请" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 09:00:00 - 已完成" } })
                            .color(TimelineColor::Green),
                        TimelineItem::new("2")
                            .title(rsx! { Text { "步骤二：资料审核" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 10:00:00 - 已完成" } })
                            .color(TimelineColor::Green),
                        TimelineItem::new("3")
                            .title(rsx! { Text { "步骤三：现场核验" } })
                            .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 11:00:00 - 已完成" } })
                            .color(TimelineColor::Green),
                        TimelineItem::new("4")
                            .title(rsx! { Text { "步骤四：最终审批" } })
                            .content(
                                rsx! { Text { r#type: TextType::Secondary, "正在处理中，预计需要 2-3 个工作日" } },
                            )
                            .pending(true),
                    ],
                }
            }

            // 交替展示
            DemoSection {
                title: "交替展示",
                Timeline {
                    items: vec![
                        TimelineItem::new("1")
                            .title(rsx! { Text { "2024-11-29" } })
                            .content(rsx! { Text { "项目启动，召开启动会议" } }),
                        TimelineItem::new("2")
                            .title(rsx! { Text { "2024-12-05" } })
                            .content(rsx! { Text { "完成需求分析和技术方案设计" } }),
                        TimelineItem::new("3")
                            .title(rsx! { Text { "2024-12-15" } })
                            .content(rsx! { Text { "开发阶段，完成核心功能开发" } }),
                        TimelineItem::new("4")
                            .title(rsx! { Text { "2024-12-25" } })
                            .content(rsx! { Text { "测试阶段，修复 Bug 并优化性能" } }),
                        TimelineItem::new("5")
                            .title(rsx! { Text { "2025-01-05" } })
                            .content(rsx! { Text { "正式上线，开始运维阶段" } }),
                    ],
                    mode: TimelineMode::Alternate,
                }
            }

            // 右侧展示
            DemoSection {
                title: "右侧展示",
                Timeline {
                    items: vec![
                        TimelineItem::new("1")
                            .title(rsx! { Text { "版本 1.0.0" } })
                            .content(rsx! {
                                div {
                                    Text { r#type: TextType::Secondary, "2024-01-01" }
                                    br {}
                                    Text { "初始版本发布，包含基础功能" }
                                }
                            })
                            .color(TimelineColor::Green),
                        TimelineItem::new("2")
                            .title(rsx! { Text { "版本 2.0.0" } })
                            .content(rsx! {
                                div {
                                    Text { r#type: TextType::Secondary, "2024-06-01" }
                                    br {}
                                    Text { "重大更新，新增多个高级功能" }
                                }
                            })
                            .color(TimelineColor::Blue),
                        TimelineItem::new("3")
                            .title(rsx! { Text { "版本 3.0.0" } })
                            .content(rsx! {
                                div {
                                    Text { r#type: TextType::Secondary, "2024-11-29" }
                                    br {}
                                    Text { "最新版本，性能优化和 UI 改进" }
                                }
                            })
                            .color(TimelineColor::Blue),
                    ],
                    mode: TimelineMode::Right,
                }
            }

            // 倒序排列
            DemoSection {
                title: "倒序排列",
                div {
                    style: "display: flex; gap: 48px;",
                    div {
                        style: "flex: 1;",
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 12px; display: block;",
                            "正序（从旧到新）："
                        }
                        Timeline {
                            items: vec![
                                TimelineItem::new("1")
                                    .title(rsx! { Text { "最早记录" } })
                                    .content(rsx! { Text { r#type: TextType::Secondary, "2024-01-01 - 项目创建" } })
                                    .color(TimelineColor::Gray),
                                TimelineItem::new("2")
                                    .title(rsx! { Text { "中间记录" } })
                                    .content(rsx! { Text { r#type: TextType::Secondary, "2024-06-15 - 功能迭代" } })
                                    .color(TimelineColor::Blue),
                                TimelineItem::new("3")
                                    .title(rsx! { Text { "最新记录" } })
                                    .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 - 最新更新" } })
                                    .color(TimelineColor::Green),
                            ],
                        }
                    }
                    div {
                        style: "flex: 1;",
                        span {
                            style: "font-size: 14px; color: var(--adui-color-text-secondary); margin-bottom: 12px; display: block;",
                            "倒序（从新到旧）："
                        }
                        Timeline {
                            items: vec![
                                TimelineItem::new("1")
                                    .title(rsx! { Text { "最早记录" } })
                                    .content(rsx! { Text { r#type: TextType::Secondary, "2024-01-01 - 项目创建" } })
                                    .color(TimelineColor::Gray),
                                TimelineItem::new("2")
                                    .title(rsx! { Text { "中间记录" } })
                                    .content(rsx! { Text { r#type: TextType::Secondary, "2024-06-15 - 功能迭代" } })
                                    .color(TimelineColor::Blue),
                                TimelineItem::new("3")
                                    .title(rsx! { Text { "最新记录" } })
                                    .content(rsx! { Text { r#type: TextType::Secondary, "2024-11-29 - 最新更新" } })
                                    .color(TimelineColor::Green),
                            ],
                            reverse: true,
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
