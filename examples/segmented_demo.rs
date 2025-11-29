use adui_dioxus::components::segmented::{Segmented, SegmentedOption};
use dioxus::prelude::*;

fn app() -> Element {
    let mut value = use_signal(|| Some("daily".to_string()));
    let options = vec![
        SegmentedOption {
            label: "日".into(),
            value: "daily".into(),
            icon: None,
            tooltip: Some("每日".into()),
            disabled: false,
        },
        SegmentedOption {
            label: "周".into(),
            value: "weekly".into(),
            icon: None,
            tooltip: Some("每周".into()),
            disabled: false,
        },
        SegmentedOption {
            label: "月".into(),
            value: "monthly".into(),
            icon: None,
            tooltip: Some("每月".into()),
            disabled: false,
        },
    ];

    rsx! {
        div { class: "demo-column",
            div { class: "demo-row",
                span { class: "demo-label", "基础" }
                Segmented {
                    options: options.clone(),
                    value: value.read().clone(),
                    on_change: move |v| value.set(Some(v)),
                }
            }
            div { class: "demo-row",
                span { class: "demo-label", "Block" }
                Segmented {
                    options: options.clone(),
                    value: value.read().clone(),
                    block: true,
                    on_change: move |v| value.set(Some(v)),
                }
            }
            div { class: "demo-row",
                span { class: "demo-label", "禁用" }
                Segmented { options, value: Some("daily".into()), disabled: true }
            }
        }
    }
}

fn main() {
    dioxus::launch(app);
}
