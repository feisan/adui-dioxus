use adui_dioxus::{
    Button, Divider, Icon, IconKind, Text, TextType, Theme, ThemeMode, ThemeProvider, Timeline,
    TimelineColor, TimelineItem, TimelineMode, TitleLevel, use_theme, THEME_BASE_STYLE,
    Title as AduiTitle,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {}
        }
    }
}

#[component]
fn App() -> Element {
    let theme = use_theme();
    let mut theme_mode = use_signal(|| ThemeMode::Light);
    let mut primary_color = use_signal(|| "#1890ff".to_string());

    use_effect(move || {
        let mode_val = theme_mode();
        let color = primary_color();
        let mut next = match mode_val {
            ThemeMode::Light => Theme::light(),
            ThemeMode::Dark => Theme::dark(),
            ThemeMode::Custom => Theme::light(),
        };
        next.tokens.color_primary = color.clone();
        next.tokens.color_primary_hover = color.clone();
        next.tokens.color_primary_active = color;
        theme.set_theme(next);
    });

    rsx! {
        style { {THEME_BASE_STYLE} },
        div { style: "min-height: 100vh; padding: 24px; background: var(--adui-color-bg-container);",
            div { style: "max-width: 1200px; margin: 0 auto;",
                // Header controls
                div { style: "margin-bottom: 32px; display: flex; gap: 16px; align-items: center;",
                    Button {
                        onclick: move |_| {
                            theme_mode.set(match theme_mode() {
                                ThemeMode::Light => ThemeMode::Dark,
                                ThemeMode::Dark => ThemeMode::Light,
                                ThemeMode::Custom => ThemeMode::Light,
                            });
                        },
                        "切换主题 ({theme_mode():?})"
                    },
                    select {
                        value: "{primary_color()}",
                        onchange: move |evt| primary_color.set(evt.value()),
                        option { value: "#1890ff", "蓝色 (默认)" },
                        option { value: "#52c41a", "绿色" },
                        option { value: "#f5222d", "红色" },
                        option { value: "#fa8c16", "橙色" },
                    }
                },

                AduiTitle { level: TitleLevel::H2, "Timeline 时间轴" }
                Divider {}

                // Basic Timeline
                AduiTitle { level: TitleLevel::H3, "基础时间轴" }
                BasicTimelineDemo {}
                Divider {}

                // Color Variants
                AduiTitle { level: TitleLevel::H3, "不同颜色" }
                ColorVariantsDemo {}
                Divider {}

                // Custom Icons
                AduiTitle { level: TitleLevel::H3, "自定义图标" }
                CustomIconDemo {}
                Divider {}

                // Pending Status
                AduiTitle { level: TitleLevel::H3, "进行中状态" }
                PendingDemo {}
                Divider {}

                // Alternate Mode
                AduiTitle { level: TitleLevel::H3, "交替展示" }
                AlternateDemo {}
                Divider {}

                // Right Mode
                AduiTitle { level: TitleLevel::H3, "右侧展示" }
                RightModeDemo {}
                Divider {}

                // Reverse Order
                AduiTitle { level: TitleLevel::H3, "倒序排列" }
                ReverseDemo {}
            }
        }
    }
}

#[component]
fn BasicTimelineDemo() -> Element {
    let items = vec![
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
    ];

    rsx! {
        Timeline {
            items: items,
        }
    }
}

#[component]
fn ColorVariantsDemo() -> Element {
    let items = vec![
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
    ];

    rsx! {
        Timeline {
            items: items,
        }
    }
}

#[component]
fn CustomIconDemo() -> Element {
    let items = vec![
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
    ];

    rsx! {
        Timeline {
            items: items,
        }
    }
}

#[component]
fn PendingDemo() -> Element {
    let items = vec![
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
    ];

    rsx! {
        Timeline {
            items: items,
        }
    }
}

#[component]
fn AlternateDemo() -> Element {
    let items = vec![
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
    ];

    rsx! {
        Timeline {
            items: items,
            mode: TimelineMode::Alternate,
        }
    }
}

#[component]
fn RightModeDemo() -> Element {
    let items = vec![
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
    ];

    rsx! {
        Timeline {
            items: items,
            mode: TimelineMode::Right,
        }
    }
}

#[component]
fn ReverseDemo() -> Element {
    let items = vec![
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
    ];

    rsx! {
        div { style: "display: flex; gap: 48px;",
            div { style: "flex: 1;",
                Text { r#type: TextType::Secondary, "正序（从旧到新）：" }
                Timeline {
                    items: items.clone(),
            }
            },
            div { style: "flex: 1;",
                Text { r#type: TextType::Secondary, "倒序（从新到旧）：" }
                Timeline {
                    items: items,
                    reverse: true,
            }
            }
        }
    }
}
