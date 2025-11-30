use adui_dioxus::{
    Badge, BadgeStatus, Button, ButtonSize, ButtonType, ColumnConfig, Descriptions,
    DescriptionsItem, DescriptionsLayout, DescriptionsSize, Divider, Space, SpaceDirection,
    SpaceSize, THEME_BASE_STYLE, Tag, TagColor, Text, TextType, Theme, ThemeMode, ThemeProvider,
    Title as AduiTitle, TitleLevel, use_theme,
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

                AduiTitle { level: TitleLevel::H2, "Descriptions 描述列表" }
                Divider {}

                // Basic Descriptions
                AduiTitle { level: TitleLevel::H3, "基础描述列表" }
                BasicDescriptionsDemo {}
                Divider {}

                // Bordered
                AduiTitle { level: TitleLevel::H3, "带边框" }
                BorderedDescriptionsDemo {}
                Divider {}

                // Vertical Layout
                AduiTitle { level: TitleLevel::H3, "垂直布局" }
                VerticalDescriptionsDemo {}
                Divider {}

                // Custom Column Count
                AduiTitle { level: TitleLevel::H3, "自定义列数和跨列" }
                CustomColumnDemo {}
                Divider {}

                // With Title and Extra
                AduiTitle { level: TitleLevel::H3, "带标题和额外内容" }
                TitleExtraDemo {}
                Divider {}

                // Size Variants
                AduiTitle { level: TitleLevel::H3, "不同尺寸" }
                SizeVariantsDemo {}
            }
        }
    }
}

#[component]
fn BasicDescriptionsDemo() -> Element {
    let items = vec![
        DescriptionsItem::new("name", rsx! { "用户名" }, rsx! { Text { "张三" } }),
        DescriptionsItem::new(
            "phone",
            rsx! { "手机号" },
            rsx! { Text { "138-0000-0000" } },
        ),
        DescriptionsItem::new(
            "email",
            rsx! { "邮箱" },
            rsx! { Text { "zhangsan@example.com" } },
        ),
        DescriptionsItem::new(
            "address",
            rsx! { "地址" },
            rsx! { Text { "浙江省杭州市西湖区" } },
        ),
        DescriptionsItem::new(
            "status",
            rsx! { "状态" },
            rsx! { Badge { count: None, status: Some(BadgeStatus::Success), children: Some(rsx! { Text { "正常" } }) } },
        ),
        DescriptionsItem::new(
            "created",
            rsx! { "创建时间" },
            rsx! { Text { "2024-01-01 10:00:00" } },
        ),
    ];

    rsx! {
        Descriptions {
            items: items,
        }
    }
}

#[component]
fn BorderedDescriptionsDemo() -> Element {
    let items = vec![
        DescriptionsItem::new(
            "product",
            rsx! { "产品名称" },
            rsx! { Text { strong: true, "Ant Design Pro" } },
        ),
        DescriptionsItem::new("version", rsx! { "版本" }, rsx! { Text { "6.0.0" } }),
        DescriptionsItem::new(
            "billing",
            rsx! { "计费模式" },
            rsx! { Tag { color: Some(TagColor::Primary), "按量付费" } },
        ),
        DescriptionsItem::new("time", rsx! { "创建时间" }, rsx! { Text { "2024-11-29" } }),
        DescriptionsItem::new("usage", rsx! { "使用量" }, rsx! { Text { "1,234,567 次" } }),
        DescriptionsItem::new(
            "remarks",
            rsx! { "备注" },
            rsx! { Text { r#type: TextType::Secondary, "这是一个备注信息。可以包含较长的文本内容。" } },
        ),
    ];

    rsx! {
        Descriptions {
            items: items,
            bordered: true,
        }
    }
}

#[component]
fn VerticalDescriptionsDemo() -> Element {
    let items = vec![
        DescriptionsItem::new("title", rsx! { "标题" }, rsx! { Text { "订单详情" } }),
        DescriptionsItem::new(
            "order",
            rsx! { "订单号" },
            rsx! { Text { "20241129-0001" } },
        ),
        DescriptionsItem::new(
            "amount",
            rsx! { "金额" },
            rsx! { Text { strong: true, "¥1,234.56" } },
        ),
        DescriptionsItem::new(
            "status",
            rsx! { "状态" },
            rsx! { Tag { color: Some(TagColor::Success), "已完成" } },
        ),
    ];

    rsx! {
        Descriptions {
            items: items,
            layout: DescriptionsLayout::Vertical,
            bordered: true,
        }
    }
}

#[component]
fn CustomColumnDemo() -> Element {
    let items = vec![
        DescriptionsItem::new(
            "name",
            rsx! { "姓名" },
            rsx! { Text { "李四" } },
        ),
        DescriptionsItem::new(
            "phone",
            rsx! { "电话" },
            rsx! { Text { "139-0000-0000" } },
        ),
        DescriptionsItem::new(
            "email",
            rsx! { "邮箱" },
            rsx! { Text { "lisi@example.com" } },
        )
        .span(2),
        DescriptionsItem::new(
            "address",
            rsx! { "地址" },
            rsx! { Text { "上海市浦东新区陆家嘴环路 1000 号" } },
        )
        .span(3),
        DescriptionsItem::new(
            "remarks",
            rsx! { "备注" },
            rsx! { Text { r#type: TextType::Secondary, "这是一个跨 3 列的备注字段，可以容纳更多内容。" } },
        )
        .span(3),
    ];

    rsx! {
        Descriptions {
            items: items,
            column: ColumnConfig::Simple(3),
            bordered: true,
        }
    }
}

#[component]
fn TitleExtraDemo() -> Element {
    let items = vec![
        DescriptionsItem::new("name", rsx! { "用户名" }, rsx! { Text { "王五" } }),
        DescriptionsItem::new(
            "phone",
            rsx! { "手机号" },
            rsx! { Text { "150-0000-0000" } },
        ),
        DescriptionsItem::new(
            "email",
            rsx! { "邮箱" },
            rsx! { Text { "wangwu@example.com" } },
        ),
        DescriptionsItem::new(
            "role",
            rsx! { "角色" },
            rsx! { Tag { color: Some(TagColor::Primary), "管理员" } },
        ),
        DescriptionsItem::new("department", rsx! { "部门" }, rsx! { Text { "技术部" } }),
        DescriptionsItem::new(
            "joined",
            rsx! { "入职时间" },
            rsx! { Text { "2023-05-15" } },
        ),
    ];

    rsx! {
        Descriptions {
            items: items,
            title: Some(rsx! { AduiTitle { level: TitleLevel::H4, "用户信息" } }),
            extra: Some(rsx! {
                Space {
                    Button { size: ButtonSize::Small, r#type: ButtonType::Primary, "编辑" }
                    Button { size: ButtonSize::Small, "更多操作" }
            }
            }),
            bordered: true,
        }
    }
}

#[component]
fn SizeVariantsDemo() -> Element {
    let items = vec![
        DescriptionsItem::new("field1", rsx! { "字段 1" }, rsx! { Text { "值 1" } }),
        DescriptionsItem::new("field2", rsx! { "字段 2" }, rsx! { Text { "值 2" } }),
        DescriptionsItem::new("field3", rsx! { "字段 3" }, rsx! { Text { "值 3" } }),
    ];

    rsx! {
        Space { direction: SpaceDirection::Vertical, size: SpaceSize::Large, gap: Some(24.0),
            div {
                Text { r#type: TextType::Secondary, "Small 尺寸：" }
                Descriptions {
                    items: items.clone(),
                    size: Some(DescriptionsSize::Small),
                    bordered: true,
            }
            },
            div {
                Text { r#type: TextType::Secondary, "Middle 尺寸（默认）：" }
                Descriptions {
                    items: items.clone(),
                    size: Some(DescriptionsSize::Middle),
                    bordered: true,
            }
            },
            div {
                Text { r#type: TextType::Secondary, "Large 尺寸：" }
                Descriptions {
                    items: items,
                    size: Some(DescriptionsSize::Large),
                    bordered: true,
            }
            }
        }
    }
}
