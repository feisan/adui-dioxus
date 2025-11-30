//! 系统设置场景演示
//!
//! 模拟真实的系统设置页面，综合使用：
//! - Layout 布局
//! - Tabs 标签页
//! - Form 表单
//! - Switch 开关
//! - Select 选择器
//! - Input 输入框
//! - Button 按钮

use adui_dioxus::{
    Button, ButtonType, Form, FormItem, Input, Layout, Select, SelectOption, Switch, TabItem, Tabs,
    ThemeProvider, Title, TitleLevel,
    components::form::{FormFinishEvent, FormRule},
    use_form,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            SettingsDemo {}
        }
    }
}

#[component]
fn SettingsDemo() -> Element {
    let form_handle = use_signal(use_form);
    let active_tab = use_signal(|| "1".to_string());

    let language_options = vec![
        SelectOption {
            key: "zh".into(),
            label: "简体中文".into(),
            disabled: false,
        },
        SelectOption {
            key: "en".into(),
            label: "English".into(),
            disabled: false,
        },
        SelectOption {
            key: "ja".into(),
            label: "日本語".into(),
            disabled: false,
        },
    ];

    rsx! {
        Layout {
            style: "min-height: 100vh; background: var(--adui-color-bg-base);",
            div {
                style: "padding: 24px; max-width: 1200px; margin: 0 auto;",
                Title { level: TitleLevel::H2, style: "margin-bottom: 24px;", "系统设置" }
                Tabs {
                    items: vec![
                        TabItem::new("1", "基本设置", Some(rsx!(
                            BasicSettings { form_handle: form_handle.read().clone() }
                        ))),
                        TabItem::new("2", "通知设置", Some(rsx!(
                            NotificationSettings { form_handle: form_handle.read().clone() }
                        ))),
                        TabItem::new("3", "隐私设置", Some(rsx!(
                            PrivacySettings { form_handle: form_handle.read().clone() }
                        ))),
                    ],
                    active_key: Some(active_tab.read().clone()),
                    on_change: {
                        let mut sig = active_tab;
                        move |key: String| {
                            sig.set(key);
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn BasicSettings(form_handle: adui_dioxus::FormHandle) -> Element {
    let language_options = vec![
        SelectOption {
            key: "zh".into(),
            label: "简体中文".into(),
            disabled: false,
        },
        SelectOption {
            key: "en".into(),
            label: "English".into(),
            disabled: false,
        },
    ];

    rsx! {
        div {
            style: "padding: 24px;",
            Form {
                form: Some(form_handle),
                on_finish: move |evt: FormFinishEvent| {
                    println!("Settings saved: {:?}", evt.values);
                },
                FormItem {
                    name: Some("username".into()),
                    label: Some("用户名".into()),
                    Input {
                        placeholder: Some("请输入用户名".into()),
                    }
                }
                FormItem {
                    name: Some("language".into()),
                    label: Some("语言".into()),
                    Select {
                        options: language_options.clone(),
                        placeholder: Some("请选择语言".into()),
                    }
                }
                FormItem {
                    name: Some("theme".into()),
                    label: Some("主题".into()),
                    Select {
                        options: vec![
                            SelectOption {
                                key: "light".into(),
                                label: "浅色".into(),
                                disabled: false,
                            },
                            SelectOption {
                                key: "dark".into(),
                                label: "深色".into(),
                                disabled: false,
                            },
                        ],
                        placeholder: Some("请选择主题".into()),
                    }
                }
                FormItem {
                    name: None,
                    label: None,
                    Button {
                        r#type: ButtonType::Primary,
                        "保存设置"
                    }
                }
            }
        }
    }
}

#[component]
fn NotificationSettings(form_handle: adui_dioxus::FormHandle) -> Element {
    rsx! {
        div {
            style: "padding: 24px;",
            Form {
                form: Some(form_handle),
                on_finish: move |evt: FormFinishEvent| {
                    println!("Notification settings saved: {:?}", evt.values);
                },
                FormItem {
                    name: Some("email_notification".into()),
                    label: Some("邮件通知".into()),
                    Switch {
                        default_checked: true,
                    }
                }
                FormItem {
                    name: Some("sms_notification".into()),
                    label: Some("短信通知".into()),
                    Switch {}
                }
                FormItem {
                    name: Some("push_notification".into()),
                    label: Some("推送通知".into()),
                    Switch {
                        default_checked: true,
                    }
                }
                FormItem {
                    name: None,
                    label: None,
                    Button {
                        r#type: ButtonType::Primary,
                        "保存设置"
                    }
                }
            }
        }
    }
}

#[component]
fn PrivacySettings(form_handle: adui_dioxus::FormHandle) -> Element {
    rsx! {
        div {
            style: "padding: 24px;",
            Form {
                form: Some(form_handle),
                on_finish: move |evt: FormFinishEvent| {
                    println!("Privacy settings saved: {:?}", evt.values);
                },
                FormItem {
                    name: Some("profile_visibility".into()),
                    label: Some("个人资料可见性".into()),
                    Select {
                        options: vec![
                            SelectOption {
                                key: "public".into(),
                                label: "公开".into(),
                                disabled: false,
                            },
                            SelectOption {
                                key: "private".into(),
                                label: "私密".into(),
                                disabled: false,
                            },
                        ],
                        placeholder: Some("请选择可见性".into()),
                    }
                }
                FormItem {
                    name: Some("data_collection".into()),
                    label: Some("数据收集".into()),
                    Switch {}
                }
                FormItem {
                    name: Some("analytics".into()),
                    label: Some("分析统计".into()),
                    Switch {
                        default_checked: true,
                    }
                }
                FormItem {
                    name: None,
                    label: None,
                    Button {
                        r#type: ButtonType::Primary,
                        "保存设置"
                    }
                }
            }
        }
    }
}
