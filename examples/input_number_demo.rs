use adui_dioxus::components::input_number::InputNumber;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
struct DemoProps {
    label: &'static str,
    disabled: bool,
}

fn Demo(props: DemoProps) -> Element {
    let DemoProps { label, disabled } = props;
    let mut value = use_signal(|| 1.0);
    rsx! {
        div { class: "demo-row",
            span { class: "demo-label", "{label}" }
            InputNumber {
                value: value.read().clone(),
                step: 0.5,
                min: 0.0,
                max: 5.0,
                precision: 1,
                disabled,
                on_change: move |v: Option<f64>| value.set(v.unwrap_or(0.0)),
            }
            span { class: "demo-value", "value: {value.read():.1}" }
        }
    }
}

fn app() -> Element {
    rsx! {
        div { class: "demo-grid",
            Demo { label: "基本", disabled: false }
            Demo { label: "禁用", disabled: true }
        }
    }
}

fn main() {
    dioxus::launch(app);
}
