use adui_dioxus::components::slider::{Slider, SliderMark, SliderValue};
use dioxus::prelude::*;

fn app() -> Element {
    let mut single = use_signal(|| 30.0);
    let mut range = use_signal(|| SliderValue::Range(20.0, 60.0));

    let marks = vec![
        SliderMark {
            value: 0.0,
            label: "0".into(),
        },
        SliderMark {
            value: 50.0,
            label: "50".into(),
        },
        SliderMark {
            value: 100.0,
            label: "100".into(),
        },
    ];

    rsx! {
        div { class: "demo-column",
            div { class: "demo-row",
                span { class: "demo-label", "单滑块" }
                Slider {
                    value: SliderValue::Single(*single.read()),
                    min: 0.0,
                    max: 100.0,
                    step: Some(5.0),
                    marks: marks.clone(),
                    on_change: move |v| {
                        if let SliderValue::Single(val) = v { single.set(val); }
                    },
                }
                span { class: "demo-value", "{single.read():.0}" }
            }
            div { class: "demo-row",
                span { class: "demo-label", "范围" }
                Slider {
                    value: (*range.read()).clone(),
                    range: true,
                    min: 0.0,
                    max: 100.0,
                    step: Some(1.0),
                    on_change: move |v| range.set(v),
                }
                {
                    let (a, b) = range.read().as_range();
                    rsx! { span { class: "demo-value", "{a:.0} - {b:.0}" } }
                }
            }
            div { class: "demo-row",
                span { class: "demo-label", "禁用" }
                Slider { value: SliderValue::Single(40.0), disabled: true }
            }
            div { class: "demo-row",
                span { class: "demo-label", "竖向" }
                Slider {
                    value: SliderValue::Single(50.0),
                    vertical: true,
                    style: "height:180px;",
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(app);
}
