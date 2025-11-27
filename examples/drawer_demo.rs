use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Drawer, DrawerPlacement,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { DrawerDemoShell {} }
        }
    }
}

#[component]
fn DrawerDemoShell() -> Element {
    let mut right_open = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Drawer demo" }
            Button {
                r#type: ButtonType::Primary,
                onclick: move |_| right_open.set(true),
                "打开右侧 Drawer",
            }

            Drawer {
                open: *right_open.read(),
                title: Some("右侧 Drawer".into()),
                placement: DrawerPlacement::Right,
                size: Some(320.0),
                on_close: move |_| right_open.set(false),
                destroy_on_close: true,
                children: rsx! {
                    p { "用于展示表单、详情或导航内容的侧边抽屉。" }
                },
            }
        }
    }
}
