use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Tooltip, TooltipPlacement,
    TooltipTrigger,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { TooltipDemoShell {} }
        }
    }
}

#[component]
fn TooltipDemoShell() -> Element {
    let mut controlled_open = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Tooltip demo" }
            p { "展示 Tooltip 的基础用法：hover / click 触发以及受控 open 模式。" }

            div { style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: center;",
                // Default hover tooltip
                Tooltip {
                    title: Some("默认 hover 提示".to_string()),
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "Hover me"
                        }
                    },
                }

                // Click trigger tooltip
                Tooltip {
                    title: Some("点击按钮切换 Tooltip".to_string()),
                    trigger: TooltipTrigger::Click,
                    placement: Some(TooltipPlacement::Bottom),
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "Click me"
                        }
                    },
                }

                // Controlled tooltip
                Tooltip {
                    title: Some("受控 Tooltip（由外部按钮控制 open）".to_string()),
                    trigger: TooltipTrigger::Click,
                    open: Some(*controlled_open.read()),
                    on_open_change: move |next: bool| {
                        controlled_open.set(next);
                    },
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "Controlled"
                        }
                    },
                }

                Button {
                    r#type: ButtonType::Primary,
                    onclick: move |_| controlled_open.set(false),
                    "关闭受控 Tooltip"
                }
            }
        }
    }
}
