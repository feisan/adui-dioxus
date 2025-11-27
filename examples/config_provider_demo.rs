use adui_dioxus::{Button, ButtonSize, ComponentSize, ConfigProvider, Input, ThemeProvider};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        // For this demo we wrap the whole tree in a ConfigProvider. It will
        // internally create a ThemeProvider so tokens & CSS variables are
        // still available.
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            disabled: Some(false),
            prefix_cls: Some("adui".into()),
            DemoRoot {}
        }
    }
}

#[component]
fn DemoRoot() -> Element {
    let mut global_disabled = use_signal(|| false);
    let mut global_size = use_signal(|| ComponentSize::Middle);

    let size_label = match *global_size.read() {
        ComponentSize::Small => "small",
        ComponentSize::Middle => "middle",
        ComponentSize::Large => "large",
    };

    rsx! {
        // Outer ConfigProvider driven by local signals to showcase dynamic
        // updates of size/disabled.
        ConfigProvider {
            size: Some(*global_size.read()),
            disabled: Some(*global_disabled.read()),
            div {
                style: "padding: 16px; background: var(--adui-color-bg-base); min-height: 100vh;",
                h2 { "ConfigProvider demo" }
                p { "当前全局 size: {size_label}, disabled: {global_disabled}" }

                div { style: "display: flex; gap: 8px; margin-bottom: 16px;",
                    Button {
                        size: ButtonSize::Small,
                        onclick: move |_| global_size.set(ComponentSize::Small),
                        "small",
                    }
                    Button {
                        size: ButtonSize::Middle,
                        onclick: move |_| global_size.set(ComponentSize::Middle),
                        "middle",
                    }
                    Button {
                        size: ButtonSize::Large,
                        onclick: move |_| global_size.set(ComponentSize::Large),
                        "large",
                    }
                    Button {
                        onclick: move |_| {
                            let next = !*global_disabled.read();
                            global_disabled.set(next);
                        },
                        "切换全局 disabled",
                    }
                }

                div { style: "display: flex; flex-direction: column; gap: 8px; max-width: 320px;",
                    p { "未显式指定 size / disabled 的 Button 和 Input 会继承上方 ConfigProvider 的配置。" }
                    Button { "继承全局 Config 的按钮" }
                    Input { placeholder: Some("继承 Config 的输入框".into()) }

                    p { "仍然可以在本地 props 中覆盖全局配置：" }
                    Button {
                        size: ButtonSize::Small,
                        disabled: false,
                        "本地 size = small，忽略全局 size/disabled",
                    }
                    Input {
                        disabled: false,
                        placeholder: Some("本地 disabled=false，忽略全局 disabled".into()),
                    }
                }
            }
        }
    }
}
