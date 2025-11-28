use adui_dioxus::{App, Button, ButtonType, ComponentSize, ConfigProvider, Modal};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { ModalDemoShell {} }
        }
    }
}

#[component]
fn ModalDemoShell() -> Element {
    let basic_open = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Modal demo" }
            Button {
                r#type: ButtonType::Primary,
                onclick: {
                    let mut basic_open = basic_open;
                    move |_| basic_open.set(true)
                },
                "打开基础 Modal",
            }

            Modal {
                open: *basic_open.read(),
                title: Some("基础 Modal".into()),
                on_cancel: {
                    let mut basic_open = basic_open;
                    move |_| basic_open.set(false)
                },
                destroy_on_close: true,
                children: rsx! {
                    p { "这里可以放任意内容，例如表单、说明文本等。" }
                },
            }
        }
    }
}
