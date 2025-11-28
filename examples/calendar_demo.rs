use adui_dioxus::{
    App, Calendar, CalendarDate, CalendarMode, ComponentSize, ConfigProvider, Content, Layout,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider { size: Some(ComponentSize::Middle),
            App { CalendarDemoPage {} }
        }
    }
}

#[component]
fn CalendarDemoPage() -> Element {
    let mut selected = use_signal(|| None::<CalendarDate>);

    rsx! {
        Layout {
            Content {
                style: Some("padding: 16px;".into()),

                h2 { "Calendar demo" }

                Calendar {
                    value: *selected.read(),
                    on_select: move |date| {
                        selected.set(Some(date));
                    },
                    fullscreen: Some(false),
                    mode: Some(CalendarMode::Month),
                }
            }
        }
    }
}
