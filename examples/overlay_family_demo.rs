use adui_dioxus::{
    App, Button, ButtonHtmlType, ButtonType, ComponentSize, ConfigProvider, Dropdown, DropdownItem,
    DropdownPlacement, Form, FormItem, FormLayout, Input, Modal, NotificationConfig,
    NotificationPlacement, NotificationType, Popconfirm, Popover, Switch, Tooltip,
    TooltipPlacement, use_form, use_message, use_notification,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { OverlayFamilyShell {} }
        }
    }
}

fn dropdown_items() -> Vec<DropdownItem> {
    vec![
        DropdownItem::new("refresh", "刷新页面"),
        DropdownItem::new("settings", "打开设置"),
        DropdownItem::new("help", "帮助中心"),
    ]
}

#[component]
fn OverlayFamilyShell() -> Element {
    let mut modal_open = use_signal(|| false);
    let form_handle = use_signal(use_form);
    let message_api = use_message();
    let notification_api = use_notification();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Overlay family integration demo" }
            p { "在同一页面集中展示 Tooltip / Popover / Popconfirm / Dropdown 与 Modal、Message、Notification 的协同行为。" }

            // 第一行：Tooltip / Popover / Popconfirm / Dropdown 基本交互
            h3 { "基础浮层组件" }
            div { style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                // Tooltip on icon-like button
                {
                    let api_for_tooltip = message_api.clone();
                    rsx! {
                        Tooltip {
                            title: Some("这是一个 Tooltip".to_string()),
                            placement: Some(TooltipPlacement::Top),
                            children: rsx! {
                                Button {
                                    r#type: ButtonType::Default,
                                    onclick: move |_| {
                                        if let Some(msg) = api_for_tooltip.clone() {
                                            msg.info("点击了 Tooltip 触发按钮");
                                        }
                                    },
                                    "Tooltip"
                                }
                            },
                        }
                    }
                }

                // Popover with rich content
                {
                    let api_for_popover = message_api.clone();
                    rsx! {
                        Popover {
                            title: Some(rsx! { b { "Popover 标题" } }),
                            content: Some(rsx! {
                                div {
                                    p { "popover 内可以包含任意内容，例如说明文本、链接或按钮。" }
                                    Button {
                                        r#type: ButtonType::Link,
                                        onclick: move |_| {
                                            if let Some(msg) = api_for_popover.clone() {
                                                msg.info("点击了 Popover 内部按钮");
                                            }
                                        },
                                        "内部操作"
                                    }
                                }
                            }),
                            children: rsx! {
                                Button {
                                    r#type: ButtonType::Default,
                                    "Popover"
                                }
                            },
                        }
                    }
                }

                // Popconfirm for dangerous action
                // Popconfirm 使用局部克隆的 message_api，避免移动原始变量。
                Popconfirm {
                    title: "确定要停用该功能吗？".to_string(),
                    description: Some("停用后用户将无法继续使用此功能。".to_string()),
                    ok_text: Some("停用".to_string()),
                    cancel_text: Some("取消".to_string()),
                    ok_type: Some(ButtonType::Primary),
                    ok_danger: true,
                    on_confirm: {
                        let api_confirm = message_api.clone();
                        move |_| {
                            if let Some(msg) = api_confirm.clone() {
                                msg.success("功能已停用");
                            }
                        }
                    },
                    on_cancel: {
                        let api_cancel = message_api.clone();
                        move |_| {
                            if let Some(msg) = api_cancel.clone() {
                                msg.info("已取消操作");
                            }
                        }
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            danger: true,
                            "Popconfirm"
                        }
                    },
                }

                // Dropdown menu
                // Dropdown 同样使用局部克隆，避免多处捕获同一个 Option<MessageApi>。
                Dropdown {
                    items: dropdown_items(),
                    placement: Some(DropdownPlacement::BottomLeft),
                    on_click: {
                        let api_dropdown = message_api.clone();
                        move |key: String| {
                            if let Some(msg) = api_dropdown.clone() {
                                msg.info(format!("选择菜单项: {key}"));
                            }
                        }
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "Dropdown"
                        }
                    },
                }
            }

            // 第二行：结合 Modal / Form / Switch，并触发 Notification
            h3 { "与 Modal / Form / Notification 联动" }
            div { style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: move |_| modal_open.set(true),
                    "在 Modal 中编辑配置",
                }
            }

            Modal {
                open: *modal_open.read(),
                title: Some("编辑配置".into()),
                on_cancel: move |_| modal_open.set(false),
                destroy_on_close: true,
                children: rsx! {
                    Form {
                        layout: FormLayout::Vertical,
                        form: Some(form_handle.read().clone()),
                        FormItem {
                            name: Some("name".into()),
                            label: Some("名称".into()),
                            Input { placeholder: Some("请输入名称".into()) }
                        }
                        FormItem {
                            name: Some("enabled".into()),
                            label: Some("启用状态".into()),
                            Switch { default_checked: true }
                        }
                        div {
                            style: "margin-top: 12px; display: flex; justify-content: flex-end; gap: 8px;",
                            Button {
                                r#type: ButtonType::Default,
                                onclick: move |_| modal_open.set(false),
                                "取消",
                            }
                            Button {
                                r#type: ButtonType::Primary,
                                html_type: ButtonHtmlType::Submit,
                                onclick: move |_| {
                                    let handle = form_handle.read().clone();
                                    let values = handle.values();
                                    if let Some(api) = notification_api.clone() {
                                        api.open(NotificationConfig {
                                            title: "配置已保存".into(),
                                            description: Some(format!("{values:?}")),
                                            r#type: NotificationType::Success,
                                            placement: NotificationPlacement::TopRight,
                                            ..NotificationConfig::default()
                                        });
                                    }
                                    modal_open.set(false);
                                },
                                "保存",
                            }
                        }
                    }
                },
            }
        }
    }
}
