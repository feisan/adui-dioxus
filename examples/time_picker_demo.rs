use adui_dioxus::{App, ComponentSize, ConfigProvider, Content, Layout, TimePicker, TimeValue};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { TimePickerDemoPage {} }
        }
    }
}

#[component]
fn TimePickerDemoPage() -> Element {
    let mut basic_value = use_signal(|| None);
    let mut stepped_value = use_signal(|| Some(TimeValue::new(9, 30, 0)));

    rsx! {
        Layout {
            Content {
                style: Some("padding: 16px;".into()),

                h2 { "TimePicker demo" }

                h3 { "Basic" }
                div { style: "display: flex; gap: 12px; align-items: center;",
                    TimePicker {
                        value: *basic_value.read(),
                        on_change: move |next| {
                            basic_value.set(next);
                        },
                    }
                }

                h3 { "With steps" }
                div { style: "display: flex; gap: 12px; align-items: center; margin-top: 8px;",
                    TimePicker {
                        value: *stepped_value.read(),
                        hour_step: Some(2),
                        minute_step: Some(15),
                        second_step: Some(30),
                        on_change: move |next| {
                            stepped_value.set(next);
                        },
                    }
                }
            }
        }
    }
}
