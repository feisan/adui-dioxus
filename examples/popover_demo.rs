use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Popover, TooltipPlacement,
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
            App { PopoverDemoShell {} }
        }
    }
}

#[component]
fn PopoverDemoShell() -> Element {
    let mut controlled_open = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Popover demo" }
            p { "展示 Popover 的基础用法：点击触发、hover 触发以及受控 open 模式。" }

            div { style: "display: flex; flex-wrap: wrap; gap: 16px; align-items: flex-start;",
                // 默认点击触发
                Popover {
                    title: Some(rsx! { b { "标题" } }),
                    content: Some(rsx! { p { "这是一个简单的 Popover 内容区域，可以放任意元素。" } }),
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "Click Popover"
                        }
                    },
                }

                // Hover 触发的 Popover
                Popover {
                    title: Some(rsx! { b { "Hover Popover" } }),
                    content: Some(rsx! { p { "鼠标悬停时展示的 Popover。" } }),
                    trigger: TooltipTrigger::Hover,
                    placement: Some(TooltipPlacement::Right),
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Default,
                            "Hover me"
                        }
                    },
                }

                // 受控 Popover
                Popover {
                    title: Some(rsx! { b { "受控 Popover" } }),
                    content: Some(rsx! { p { "由外部按钮完全控制 open 状态。" } }),
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
                    "关闭受控 Popover"
                }
            }
        }
    }
}
