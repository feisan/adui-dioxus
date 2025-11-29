use adui_dioxus::components::color_picker::ColorPicker;
use dioxus::prelude::*;

const DEMO_STYLE: &str = r#"
    .demo-column {
        display: flex;
        flex-direction: column;
        gap: 24px;
        padding: 24px;
    }
    .demo-row {
        display: flex;
        gap: 12px;
        align-items: flex-start;
    }
    .demo-label {
        min-width: 80px;
        font-weight: bold;
    }
    .demo-value {
        padding: 4px 8px;
        background: #f0f0f0;
        border-radius: 4px;
        font-family: monospace;
    }
    .adui-color-picker {
        display: inline-flex;
        flex-direction: column;
        gap: 8px;
        border: 1px solid #d9d9d9;
        border-radius: 4px;
        padding: 12px;
        background: white;
    }
    .adui-color-picker-preview {
        width: 200px;
        height: 40px;
        border: 1px solid #d9d9d9;
        border-radius: 4px;
    }
    .adui-color-picker-controls {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }
    .adui-color-picker-sat {
        position: relative;
        width: 200px;
        height: 150px;
        cursor: pointer;
    }
    .adui-color-picker-sat-white {
        position: absolute;
        inset: 0;
        background: linear-gradient(to right, white, rgba(255,255,255,0));
    }
    .adui-color-picker-sat-black {
        position: absolute;
        inset: 0;
        background: linear-gradient(to bottom, rgba(0,0,0,0), black);
    }
    .adui-color-picker-sat-handle {
        position: absolute;
        width: 12px;
        height: 12px;
        border: 2px solid white;
        border-radius: 50%;
        box-shadow: 0 0 4px rgba(0,0,0,0.5);
        transform: translate(-50%, -50%);
        pointer-events: none;
    }
    .adui-color-picker-slider {
        position: relative;
        width: 200px;
        height: 12px;
        border-radius: 6px;
        cursor: pointer;
    }
    .adui-color-picker-input-row {
        display: flex;
        gap: 8px;
    }
    .adui-color-picker-input {
        flex: 1;
        padding: 4px 8px;
        border: 1px solid #d9d9d9;
        border-radius: 4px;
        font-family: monospace;
    }
    .adui-color-picker-clear {
        padding: 4px 12px;
        border: 1px solid #d9d9d9;
        border-radius: 4px;
        background: white;
        cursor: pointer;
    }
    .adui-color-picker-clear:hover {
        background: #f0f0f0;
    }
    .adui-color-picker-disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
"#;

fn app() -> Element {
    let mut value = use_signal(|| Some("#1677ff".to_string()));
    rsx! {
        style { {DEMO_STYLE} }
        div { class: "demo-column",
            div { class: "demo-row",
                span { class: "demo-label", "基础" }
                ColorPicker {
                    default_value: Some("#1677ff".to_string()),
                    on_change: move |hex| {
                        value.set(Some(hex));
                    },
                }
                {
                    let val_str = value.read().clone().unwrap_or_else(|| "(empty)".into());
                    rsx! { span { class: "demo-value", "{val_str}" } }
                }
            }
            div { class: "demo-row",
                span { class: "demo-label", "禁用" }
                ColorPicker { value: Some("#faad14".into()), disabled: true, allow_clear: false }
            }
            div { class: "demo-row",
                span { class: "demo-label", "允许清除" }
                ColorPicker {
                    default_value: value.read().clone(),
                    allow_clear: true,
                    on_change: move |hex| {
                        value.set(Some(hex));
                    },
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(app);
}
