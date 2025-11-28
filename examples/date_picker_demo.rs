use adui_dioxus::{
    App, Button, ButtonType, ComponentSize, ConfigProvider, Content, DatePicker, DateRangeValue,
    Form, FormHandle, FormItem, Layout, RangePicker, components::form::FormRule,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { DatePickerDemoPage {} }
        }
    }
}

#[component]
fn DatePickerDemoPage() -> Element {
    let mut single_value = use_signal(|| None);
    let mut range_value = use_signal(|| DateRangeValue {
        start: None,
        end: None,
    });
    let form_handle: Signal<FormHandle> = use_signal(FormHandle::new);

    rsx! {
        Layout {
            Content {
                style: Some("padding: 16px;".into()),
                h2 { "DatePicker / RangePicker demo" }

                h3 { "Basic" }
                div { style: "display: flex; gap: 12px; align-items: center;",
                    DatePicker {
                        value: *single_value.read(),
                        on_change: move |next| {
                            single_value.set(next);
                        },
                    }
                    span { "当前值: " }
                }

                h3 { "RangePicker" }
                div { style: "display: flex; gap: 12px; align-items: center; margin-top: 8px;",
                    RangePicker {
                        value: Some(*range_value.read()),
                        on_change: move |next| {
                            range_value.set(next);
                        },
                    }
                    span {
                        "当前区间: ",
                        match (range_value.read().start, range_value.read().end) {
                            (Some(s), Some(e)) => rsx!("{s.to_ymd_string()} ~ {e.to_ymd_string()}"),
                            _ => rsx!("(none)"),
                        }
                    }
                }

                h3 { "With Form" }
                div { style: "margin-top: 16px; max-width: 420px; padding: 16px; border: 1px solid #f0f0f0; border-radius: 4px;",
                    Form {
                        form: Some(form_handle.read().clone()),
                        FormItem {
                            name: Some("start_date".into()),
                            label: Some("开始日期".into()),
                            rules: Some(vec![FormRule {
                                required: true,
                                message: Some("请选择开始日期".into()),
                                ..FormRule::default()
                            }]),
                            DatePicker {}
                        }
                        FormItem {
                            name: Some("range".into()),
                            label: Some("日期区间".into()),
                            RangePicker {}
                        }
                        Button { r#type: ButtonType::Primary, "提交 (仅示意)" }
                    }
                }
            }
        }
    }
}
