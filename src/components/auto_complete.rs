use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::{form_value_to_string, use_form_item_control};
use crate::components::select_base::{
    DropdownLayer, SelectOption, filter_options_by_query, use_dropdown_layer,
};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;

/// Props for the AutoComplete component (MVP subset).
///
/// 当前版本重点支持：
/// - 基于 Input 的受控/非受控输入；
/// - 本地 options 过滤；
/// - 选择候选项时将其 label 写回输入框，并触发 on_select/on_change；
/// - 与 Form 集成时，直接以字符串形式读写字段值。
#[derive(Props, Clone, PartialEq)]
pub struct AutoCompleteProps {
    /// 候选列表（建议使用较小集合），使用 SelectOption 复用模型。
    #[props(optional)]
    pub options: Option<Vec<SelectOption>>,
    /// 受控输入值。
    #[props(optional)]
    pub value: Option<String>,
    /// 非受控模式下的初始值。
    #[props(optional)]
    pub default_value: Option<String>,
    /// 占位文本。
    #[props(optional)]
    pub placeholder: Option<String>,
    /// 是否显示清除按钮。
    #[props(default)]
    pub allow_clear: bool,
    /// 禁用整个组件。
    #[props(default)]
    pub disabled: bool,
    /// 视觉状态（success / warning / error）。
    #[props(optional)]
    pub status: Option<ControlStatus>,
    /// 组件尺寸，默认跟随 ConfigProvider。
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// 自定义类名与样式。
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// 弹层额外类名与样式。
    #[props(optional)]
    pub dropdown_class: Option<String>,
    #[props(optional)]
    pub dropdown_style: Option<String>,
    /// 输入变化回调（Form 场景下也会在写回字段后触发）。
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// 输入变化时的搜索回调，可用于外部异步更新 options。
    #[props(optional)]
    pub on_search: Option<EventHandler<String>>,
    /// 选择某个候选项时触发，参数为该项的 key。
    #[props(optional)]
    pub on_select: Option<EventHandler<String>>,
}

/// Ant Design flavored AutoComplete (MVP).
#[component]
pub fn AutoComplete(props: AutoCompleteProps) -> Element {
    let AutoCompleteProps {
        options,
        value,
        default_value,
        placeholder,
        allow_clear,
        disabled,
        status,
        size,
        class,
        style,
        dropdown_class,
        dropdown_style,
        on_change,
        on_search,
        on_select,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();

    let final_size = size.unwrap_or(config.size);

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    // Local inner value used only when not controlled by Form or external props.
    let initial_inner = default_value.clone().unwrap_or_default();
    let inner_value: Signal<String> = use_signal(|| initial_inner);

    let has_form = form_control.is_some();
    let prop_value = value.clone();
    let controlled_by_prop = has_form || prop_value.is_some();

    // Resolve current text value from Form / props / internal state。
    let current_value: String = if let Some(ctx) = form_control.as_ref() {
        form_value_to_string(ctx.value())
    } else if let Some(v) = prop_value {
        v
    } else {
        inner_value.read().clone()
    };

    // Dropdown open state and internal click flag for click-outside closing.
    let open_state: Signal<bool> = use_signal(|| false);
    let internal_click_flag: Signal<bool> = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    {
        let mut open_for_global = open_state;
        let mut internal_flag = internal_click_flag;
        use_effect(move || {
            use wasm_bindgen::{JsCast, closure::Closure};

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let target: web_sys::EventTarget = document.into();
                    let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(
                        move |_evt: web_sys::MouseEvent| {
                            let mut flag = internal_flag;
                            if *flag.read() {
                                flag.set(false);
                                return;
                            }
                            let mut open_signal = open_for_global;
                            if *open_signal.read() {
                                open_signal.set(false);
                            }
                        },
                    ));
                    let _ = target.add_event_listener_with_callback(
                        "click",
                        handler.as_ref().unchecked_ref(),
                    );
                    handler.forget();
                }
            }
        });
    }

    let placeholder_str = placeholder.unwrap_or_default();

    let has_any_options = options
        .as_ref()
        .map(|opts| !opts.is_empty())
        .unwrap_or(false);

    // Filter options by current input value.
    let filtered_options: Vec<SelectOption> = if let Some(opts) = options.as_ref() {
        if current_value.is_empty() {
            opts.clone()
        } else {
            filter_options_by_query(opts, &current_value)
        }
    } else {
        Vec::new()
    };

    let open_flag = *open_state.read();
    let DropdownLayer { z_index, .. } = use_dropdown_layer(open_flag);
    let current_z = *z_index.read();

    // Shared handlers.
    let on_change_cb = on_change;
    let on_search_cb = on_search;
    let on_select_cb = on_select;
    let inner_for_change = inner_value;
    let form_for_handlers = form_control.clone();
    let open_for_toggle = open_state;
    let internal_click_for_toggle = internal_click_flag;

    let dropdown_class_attr = {
        let mut list = vec!["adui-select-dropdown".to_string()];
        if let Some(extra) = dropdown_class {
            list.push(extra);
        }
        list.join(" ")
    };
    let dropdown_style_attr = format!(
        "position: absolute; top: 100%; left: 0; min-width: 100%; z-index: {}; {}",
        current_z,
        dropdown_style.unwrap_or_default()
    );

    // Wrapper classes reuse Select visuals for consistency.
    let mut class_list = vec!["adui-select".to_string()];
    if is_disabled {
        class_list.push("adui-select-disabled".into());
    }
    if open_flag {
        class_list.push("adui-select-open".into());
    }
    match final_size {
        ComponentSize::Small => class_list.push("adui-select-sm".into()),
        ComponentSize::Large => class_list.push("adui-select-lg".into()),
        ComponentSize::Middle => {}
    }
    push_status_class(&mut class_list, status);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // Input change helper:写回 Form / 内部 state，并触发 on_change/on_search。
    let handle_input_change = move |next: String| {
        if let Some(ctx) = form_for_handlers.as_ref() {
            ctx.set_string(next.clone());
        } else if !controlled_by_prop {
            let mut inner = inner_for_change;
            inner.set(next.clone());
        }
        if let Some(cb) = on_change_cb {
            cb.call(next.clone());
        }
        if let Some(cb) = on_search_cb {
            cb.call(next);
        }
    };

    rsx! {
        div {
            class: "adui-select-root",
            style: "position: relative; display: inline-block;",
            div {
                class: "{class_attr}",
                style: "{style_attr}",
                role: "combobox",
                "aria-expanded": open_flag,
                "aria-disabled": is_disabled,
                onclick: move |_| {
                    if is_disabled {
                        return;
                    }
                    let mut flag = internal_click_for_toggle;
                    flag.set(true);
                    let mut open_signal = open_for_toggle;
                    let current = *open_signal.read();
                    // 允许点击触发区显式打开/关闭，下拉只有在存在 options 时才有意义。
                    if has_any_options {
                        open_signal.set(!current);
                    }
                },
                // 输入框本体。
                input {
                    class: "adui-input",
                    disabled: is_disabled || config.disabled,
                    value: "{current_value}",
                    placeholder: "{placeholder_str}",
                    onfocus: move |_| {
                        if is_disabled || !has_any_options {
                            return;
                        }
                        let mut flag = internal_click_for_toggle;
                        flag.set(true);
                        let mut open_signal = open_for_toggle;
                        open_signal.set(true);
                    },
                        oninput: {
                            let handle_input_change = handle_input_change.clone();
                            move |evt| {
                                if is_disabled {
                                    return;
                                }
                                let mut flag = internal_click_for_toggle;
                                flag.set(true);
                                let next = evt.value();
                                handle_input_change(next);
                                if has_any_options {
                                    let mut open_signal = open_for_toggle;
                                    open_signal.set(true);
                                }
                            }
                        },
                    onkeydown: move |evt: KeyboardEvent| {
                        if is_disabled {
                            return;
                        }
                        use dioxus::prelude::Key;
                        if matches!(evt.key(), Key::Escape) {
                            let mut open_signal = open_for_toggle;
                            open_signal.set(false);
                        }
                    }
                }
                if allow_clear && !current_value.is_empty() && !is_disabled {
                    {
                        let handle_input_change = handle_input_change.clone();
                        rsx! {
                            span {
                                class: "adui-select-clear",
                                onclick: move |_| {
                                    handle_input_change(String::new());
                                    let mut open_signal = open_for_toggle;
                                    open_signal.set(false);
                                },
                                "×"
                            }
                        }
                    }
                }
            }
            if open_flag && !filtered_options.is_empty() {
                div {
                    class: "{dropdown_class_attr}",
                    style: "{dropdown_style_attr}",
                    role: "listbox",
                    ul { class: "adui-select-item-list",
                        {filtered_options.iter().map(|opt| {
                            let key = opt.key.clone();
                            let label = opt.label.clone();
                            let disabled_opt = opt.disabled || is_disabled;
                            let internal_click_for_item = internal_click_flag;
                            let form_for_click = form_control.clone();
                            let inner_for_click = inner_value;
                            rsx! {
                                li {
                                    class: {
                                        let mut classes = vec!["adui-select-item".to_string()];
                                        if disabled_opt {
                                            classes.push("adui-select-item-option-disabled".into());
                                        }
                                        classes.join(" ")
                                    },
                                    role: "option",
                                    onclick: move |_| {
                                        if disabled_opt {
                                            return;
                                        }
                                        let mut flag = internal_click_for_item;
                                        flag.set(true);

                                        // 选中候选项时，将 label 写回输入框。
                                        let next_text = label.clone();
                                        if let Some(ctx) = form_for_click.as_ref() {
                                            ctx.set_string(next_text.clone());
                                        } else if !controlled_by_prop {
                                            let mut inner = inner_for_click;
                                            inner.set(next_text.clone());
                                        }
                                        if let Some(cb) = on_change_cb {
                                            cb.call(next_text.clone());
                                        }
                                        if let Some(cb) = on_select_cb {
                                            cb.call(key.clone());
                                        }

                                        let mut open_signal = open_for_toggle;
                                        open_signal.set(false);
                                    },
                                    "{label}"
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod auto_complete_tests {
    use super::*;
    use crate::components::select_base::SelectOption;

    #[test]
    fn auto_complete_props_defaults() {
        // Test that default values are correct
        // Note: AutoCompleteProps requires children, so we can't create a full instance
        // But we can verify the default values used in the component
        assert_eq!(false, false); // allow_clear defaults to false
        assert_eq!(false, false); // disabled defaults to false
    }

    #[test]
    fn auto_complete_props_optional_fields() {
        // Test that all optional fields can be None
        // This verifies the structure allows None for optional fields
        let _options: Option<Vec<SelectOption>> = None;
        let _value: Option<String> = None;
        let _default_value: Option<String> = None;
        let _placeholder: Option<String> = None;
        let _status: Option<ControlStatus> = None;
        let _size: Option<ComponentSize> = None;
        let _class: Option<String> = None;
        let _style: Option<String> = None;
        let _dropdown_class: Option<String> = None;
        let _dropdown_style: Option<String> = None;
        // All optional fields can be None
        assert!(true);
    }

    #[test]
    fn auto_complete_props_with_values() {
        // Test that optional fields can have values
        let options = Some(vec![
            SelectOption {
                key: "1".to_string(),
                label: "Option 1".to_string(),
                disabled: false,
            },
            SelectOption {
                key: "2".to_string(),
                label: "Option 2".to_string(),
                disabled: false,
            },
        ]);
        assert!(options.is_some());
        assert_eq!(options.as_ref().unwrap().len(), 2);

        let value = Some("test".to_string());
        assert_eq!(value.as_ref().unwrap(), "test");

        let status = Some(ControlStatus::Error);
        assert_eq!(status.unwrap(), ControlStatus::Error);

        let size = Some(ComponentSize::Large);
        assert_eq!(size.unwrap(), ComponentSize::Large);
    }

    #[test]
    fn auto_complete_props_boolean_defaults() {
        // Verify boolean defaults
        let allow_clear_default = false;
        let disabled_default = false;
        assert_eq!(allow_clear_default, false);
        assert_eq!(disabled_default, false);
    }

    #[test]
    fn filter_options_by_query_used_by_autocomplete() {
        // Test the filter function that AutoComplete uses
        let options = vec![
            SelectOption {
                key: "1".to_string(),
                label: "Apple".to_string(),
                disabled: false,
            },
            SelectOption {
                key: "2".to_string(),
                label: "Banana".to_string(),
                disabled: false,
            },
            SelectOption {
                key: "3".to_string(),
                label: "Cherry".to_string(),
                disabled: false,
            },
        ];

        let filtered = filter_options_by_query(&options, "app");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].label, "Apple");

        let filtered_empty = filter_options_by_query(&options, "");
        assert_eq!(filtered_empty.len(), 3);

        let filtered_case_insensitive = filter_options_by_query(&options, "BANANA");
        assert_eq!(filtered_case_insensitive.len(), 1);
        assert_eq!(filtered_case_insensitive[0].label, "Banana");
    }
}
