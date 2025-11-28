use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Progress, ProgressStatus, ProgressType,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { ProgressDemoShell {} }
        }
    }
}

#[component]
fn ProgressDemoShell() -> Element {
    let percent = use_signal(|| 30.0f32);

    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Progress demo" }
            p { "展示线形和圆形进度条，以及简单的状态切换。" }

            div { style: "margin-bottom: 16px; display: flex; gap: 8px;",
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| {
                        let mut value = percent;
                        let next = (*value.read() + 10.0).min(100.0);
                        value.set(next);
                    },
                    "增加 10%"
                }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| {
                        let mut value = percent;
                        value.set(0.0);
                    },
                    "重置"
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                Progress {
                    percent: *percent.read(),
                    show_info: true,
                    r#type: ProgressType::Line,
                }
                Progress {
                    percent: *percent.read(),
                    show_info: true,
                    r#type: ProgressType::Line,
                    status: Some(ProgressStatus::Exception),
                }
                div { style: "margin-top: 16px;", "圆形进度：" }
                Progress {
                    percent: *percent.read(),
                    show_info: true,
                    r#type: ProgressType::Circle,
                }
            }
        }
    }
}
