//! Descriptions 组件演示
//!
//! 展示 Descriptions 组件的基础用法和高级用法，包括：
//! - 基础描述列表
//! - 带边框
//! - 垂直布局
//! - 自定义列数和跨列
//! - 带标题和额外内容
//! - 不同尺寸

use adui_dioxus::{
    Badge, BadgeStatus, Button, ButtonSize, ButtonType, ColumnConfig, Descriptions,
    DescriptionsItem, DescriptionsLayout, DescriptionsSize, Space, SpaceDirection, SpaceSize, Tag,
    TagColor, Text, TextType, ThemeMode, ThemeProvider, Title, TitleLevel, use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DescriptionsDemo {}
        }
    }
}

#[component]
fn DescriptionsDemo() -> Element {
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

            // 基础描述列表
            DemoSection {
                title: "基础描述列表",
                Descriptions {
                    items: vec![
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
                    ],
                }
            }

            // 带边框
            DemoSection {
                title: "带边框",
                Descriptions {
                    items: vec![
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
                    ],
                    bordered: true,
                }
            }

            // 垂直布局
            DemoSection {
                title: "垂直布局",
                Descriptions {
                    items: vec![
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
                    ],
                    layout: DescriptionsLayout::Vertical,
                    bordered: true,
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 自定义列数和跨列
            DemoSection {
                title: "自定义列数和跨列",
                Descriptions {
                    items: vec![
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
                    ],
                    column: ColumnConfig::Simple(3),
                    bordered: true,
                }
            }

            // 带标题和额外内容
            DemoSection {
                title: "带标题和额外内容",
                Descriptions {
                    items: vec![
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
                    ],
                    title: Some(rsx! { Title { level: TitleLevel::H4, "用户信息" } }),
                    extra: Some(rsx! {
                        Space {
                            Button { size: ButtonSize::Small, r#type: ButtonType::Primary, "编辑" }
                            Button { size: ButtonSize::Small, "更多操作" }
                        }
                    }),
                    bordered: true,
                }
            }

            // 不同尺寸
            DemoSection {
                title: "不同尺寸",
                Space {
                    direction: SpaceDirection::Vertical,
                    size: SpaceSize::Large,
                    gap: Some(24.0),
                    div {
                        Text { r#type: TextType::Secondary, "Small 尺寸：" }
                        Descriptions {
                            items: vec![
                                DescriptionsItem::new("field1", rsx! { "字段 1" }, rsx! { Text { "值 1" } }),
                                DescriptionsItem::new("field2", rsx! { "字段 2" }, rsx! { Text { "值 2" } }),
                                DescriptionsItem::new("field3", rsx! { "字段 3" }, rsx! { Text { "值 3" } }),
                            ],
                            size: Some(DescriptionsSize::Small),
                            bordered: true,
                        }
                    }
                    div {
                        Text { r#type: TextType::Secondary, "Middle 尺寸（默认）：" }
                        Descriptions {
                            items: vec![
                                DescriptionsItem::new("field1", rsx! { "字段 1" }, rsx! { Text { "值 1" } }),
                                DescriptionsItem::new("field2", rsx! { "字段 2" }, rsx! { Text { "值 2" } }),
                                DescriptionsItem::new("field3", rsx! { "字段 3" }, rsx! { Text { "值 3" } }),
                            ],
                            size: Some(DescriptionsSize::Middle),
                            bordered: true,
                        }
                    }
                    div {
                        Text { r#type: TextType::Secondary, "Large 尺寸：" }
                        Descriptions {
                            items: vec![
                                DescriptionsItem::new("field1", rsx! { "字段 1" }, rsx! { Text { "值 1" } }),
                                DescriptionsItem::new("field2", rsx! { "字段 2" }, rsx! { Text { "值 2" } }),
                                DescriptionsItem::new("field3", rsx! { "字段 3" }, rsx! { Text { "值 3" } }),
                            ],
                            size: Some(DescriptionsSize::Large),
                            bordered: true,
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
