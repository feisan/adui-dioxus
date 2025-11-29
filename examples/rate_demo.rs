use adui_dioxus::components::rate::Rate;
use dioxus::prelude::*;

fn app() -> Element {
    let mut value = use_signal(|| Some(3.0));
    rsx! {
        div { class: "demo-column",
            div { class: "demo-row",
                span { class: "demo-label", "基础" }
                Rate {
                    value: value.read().clone(),
                    allow_half: true,
                    on_change: move |v| value.set(v),
                    on_hover_change: move |_v| {
                        // if let Some(val) = v { /* log::info!("hover {val}"); */ }
                    },
                }
                {
                    let val_opt = *value.read();
                    let display = val_opt
                        .map(|v| format!("{:.1}", v))
                        .unwrap_or_else(|| "-".into());
                    rsx! { span { class: "demo-value", "{display}" } }
                }
            }
            div { class: "demo-row",
                span { class: "demo-label", "禁用" }
                Rate { value: Some(2.0), disabled: true }
            }
        }
    }
}

fn main() {
    dioxus::launch(app);
}
