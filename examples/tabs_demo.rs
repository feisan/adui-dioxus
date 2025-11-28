use adui_dioxus::{App, ComponentSize, ConfigProvider, TabItem, Tabs};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { TabsDemoShell {} }
        }
    }
}

#[component]
fn TabsDemoShell() -> Element {
    let items = vec![
        TabItem::new(
            "basic",
            "基础信息",
            Some(rsx!(
                div {
                    style: "padding: 8px 0;",
                    "这里是基础信息内容，例如用户姓名、邮箱等。"
                }
            )),
        ),
        TabItem::new(
            "security",
            "安全设置",
            Some(rsx!(
                div {
                    style: "padding: 8px 0;",
                    "这里是安全设置内容，例如密码修改、两步验证。"
                }
            )),
        ),
        TabItem::new(
            "notification",
            "通知偏好",
            Some(rsx!(
                div {
                    style: "padding: 8px 0;",
                    "这里是通知偏好内容，例如站内信、邮件通知策略。"
                }
            )),
        ),
    ];

    let mut active = use_signal(|| "basic".to_string());

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Tabs demo" }
            p { "展示基础 Tabs 用法，包括非受控模式和受控模式。" }

            h3 { style: "margin-top: 16px;", "非受控 Tabs" }
            Tabs {
                items: items.clone(),
            }

            h3 { style: "margin-top: 24px;", "受控 Tabs" }
            p { style: "color: var(--adui-color-text-secondary);", "通过外部 Signal 控制当前激活的 Tab。" }
            Tabs {
                items: items,
                active_key: Some((*active.read()).clone()),
                on_change: move |key: String| {
                    active.set(key);
                },
            }
        }
    }
}
