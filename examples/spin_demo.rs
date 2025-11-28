use adui_dioxus::{App, Button, ButtonType, ComponentSize, ConfigProvider, Spin, SpinSize};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { SpinDemoShell {} }
        }
    }
}

#[component]
fn SpinDemoShell() -> Element {
    let loading = use_signal(|| true);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Spin demo" }
            p { "展示基础 Spin 指示器以及嵌套加载状态。" }

            h3 { style: "margin-top: 16px;", "基础指示器" }
            div { style: "display: flex; gap: 16px; align-items: center;",
                Spin { size: Some(SpinSize::Small), tip: Some("小号".to_string()) }
                Spin { tip: Some("默认".to_string()) }
                Spin { size: Some(SpinSize::Large), tip: Some("大号".to_string()) }
            }

            h3 { style: "margin-top: 24px;", "嵌套加载" }
            div { style: "margin-bottom: 8px;",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut flag = loading;
                        move |_| {
                            let next = !*flag.read();
                            flag.set(next);
                        }
                    },
                    if *loading.read() { "停止加载" } else { "开始加载" }
                }
            }
            Spin {
                spinning: Some(*loading.read()),
                tip: Some("加载中...".to_string()),
                div {
                    style: "padding: 16px; border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border); background: var(--adui-color-bg-container); max-width: 320px;",
                    h4 { "内容卡片" }
                    p { style: "margin: 0; color: var(--adui-color-text-secondary);",
                        "当 Spin 处于加载状态时，会在内容上方显示半透明遮罩和指示器。"
                    }
                }
            }
        }
    }
}
