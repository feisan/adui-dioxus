use adui_dioxus::{
    Button, ButtonType, FloatButton, FloatButtonShape, FloatButtonType, ThemeMode, ThemeProvider,
    use_theme,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DemoPage {}
        }
    }
}

#[component]
fn DemoPage() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let show_secondary = use_signal(|| true);

    use_effect(move || {
        theme.set_mode(*mode.read());
    });

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base); color: var(--adui-color-text);",
            div {
                style: "display: flex; gap: 8px; align-items: center; margin-bottom: 12px;",
                span { "主题：" }
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
                Button {
                    r#type: ButtonType::Text,
                    onclick: {
                        let mut toggler = show_secondary;
                        move |_| {
                            let current = *toggler.read();
                            toggler.set(!current);
                        }
                    },
                    {
                        let active = *show_secondary.read();
                        let label = format!("副按钮 {}", if active { "ON" } else { "OFF" });
                        rsx!({label})
                    }
                }
            }

            div {
                style: "border: 1px dashed var(--adui-color-border); padding: 12px; border-radius: var(--adui-radius); background: var(--adui-color-bg-container); min-height: 60vh; position: relative;",
                p { "尝试点击右下角的浮动按钮。副按钮可在左侧开关控制。" }
            }

            FloatButton {
                r#type: FloatButtonType::Primary,
                shape: FloatButtonShape::Circle,
                icon: rsx!(span { "＋" }),
                tooltip: Some("快速创建".to_string()),
                onclick: move |_| {
                    println!("primary float button clicked");
                }
            }

            if *show_secondary.read() {
                FloatButton {
                    r#type: FloatButtonType::Default,
                    shape: FloatButtonShape::Square,
                    icon: rsx!(span { "?" }),
                    description: Some("帮助".to_string()),
                    tooltip: Some("查看帮助".to_string()),
                    right: 96.0,
                    bottom: 72.0,
                    onclick: move |_| {
                        println!("secondary float button clicked");
                    }
                }
            }
        }
    }
}
