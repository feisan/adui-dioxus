use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::{FormItemControlContext, use_form_item_control};
use crate::components::select_base::{
    DropdownLayer, OptionKey, SelectOption, handle_option_list_key_event, option_key_to_value,
    option_keys_to_value, toggle_option_key, use_dropdown_layer, value_to_option_key,
    value_to_option_keys,
};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use serde_json::Value;

/// Re-export of the shared option type so that callers can build option lists
/// without depending on the internal `select_base` module path.
pub use crate::components::select_base::SelectOption as PublicSelectOption;

/// Props for the Select component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Controlled value for single-select mode.
    #[props(optional)]
    pub value: Option<String>,
    /// Controlled values for multi-select mode.
    #[props(optional)]
    pub values: Option<Vec<String>>,
    /// Option list rendered in the dropdown.
    pub options: Vec<SelectOption>,
    /// When true, allow selecting multiple options.
    #[props(default)]
    pub multiple: bool,
    /// Whether to show a clear icon when there is a selection.
    #[props(default)]
    pub allow_clear: bool,
    /// Placeholder text displayed when there is no selection.
    #[props(optional)]
    pub placeholder: Option<String>,
    /// Disable user interaction.
    #[props(default)]
    pub disabled: bool,
    /// Enable simple client-side search by option label.
    #[props(default)]
    pub show_search: bool,
    /// Optional visual status applied to the wrapper.
    #[props(optional)]
    pub status: Option<ControlStatus>,
    /// Override component size; falls back to ConfigProvider when `None`.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Optional extra classes/styles for the dropdown popup.
    #[props(optional)]
    pub dropdown_class: Option<String>,
    #[props(optional)]
    pub dropdown_style: Option<String>,
    /// Change event emitted with the full set of selected keys.
    #[props(optional)]
    pub on_change: Option<EventHandler<Vec<String>>>,
}

/// Ant Design flavored Select (MVP).
#[component]
pub fn Select(props: SelectProps) -> Element {
    let SelectProps {
        value,
        values,
        options,
        multiple,
        allow_clear,
        placeholder,
        disabled,
        show_search,
        status,
        size,
        class,
        style,
        dropdown_class,
        dropdown_style,
        on_change,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();

    let final_size = size.unwrap_or(config.size);

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    // Internal selection state used only when the component is not controlled
    // by Form or external props.
    let internal_selected: Signal<Vec<OptionKey>> = use_signal(Vec::new);

    let has_form = form_control.is_some();
    let prop_single = value.clone();
    let prop_multi = values.clone();
    let multiple_flag = multiple;

    // Snapshot of currently selected keys for this render.
    let selected_keys: Vec<OptionKey> = if let Some(ctx) = form_control.as_ref() {
        if multiple_flag {
            value_to_option_keys(ctx.value())
        } else {
            match value_to_option_key(ctx.value()) {
                Some(k) => vec![k],
                None => Vec::new(),
            }
        }
    } else if let Some(vs) = prop_multi {
        vs
    } else if let Some(v) = prop_single {
        vec![v]
    } else {
        internal_selected.read().clone()
    };

    let controlled_by_prop = has_form || value.is_some() || values.is_some();

    // Dropdown open/close state and active index for keyboard navigation.
    let open_state: Signal<bool> = use_signal(|| false);
    let active_index: Signal<Option<usize>> = use_signal(|| None);

    // Flag used to distinguish between internal clicks (on the select trigger
    // or dropdown) and genuine outside clicks. This allows us to install a
    // document-level click handler for "click outside to close" without
    // interfering with normal selection.
    let internal_click_flag: Signal<bool> = use_signal(|| false);

    // Document-level click handler for closing the dropdown when clicking
    // outside of the select. This is only compiled for wasm32 targets.
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
                                // Internal click: consume the flag and skip
                                // closing.
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
                    // Leak the handler for the lifetime of the page; this keeps
                    // the implementation simple and matches the typical app
                    // lifetime.
                    handler.forget();
                }
            }
        });
    }

    // Search query (when show_search = true).
    let search_query: Signal<String> = use_signal(|| String::new());

    let open_flag = *open_state.read();
    let DropdownLayer { z_index, .. } = use_dropdown_layer(open_flag);
    let current_z = *z_index.read();

    let placeholder_str = placeholder.unwrap_or_default();

    // Apply search filtering when enabled.
    let filtered_options: Vec<SelectOption> = if show_search {
        let query = search_query.read().clone();
        crate::components::select_base::filter_options_by_query(&options, &query)
    } else {
        options.clone()
    };

    // Build wrapper classes.
    let mut class_list = vec!["adui-select".to_string()];
    if multiple {
        class_list.push("adui-select-multiple".into());
    }
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

    // Helper to find the label for a given key.
    let find_label = |key: &str| -> String {
        options
            .iter()
            .find(|opt| opt.key == key)
            .map(|opt| opt.label.clone())
            .unwrap_or_else(|| key.to_string())
    };

    let display_node = if multiple {
        if selected_keys.is_empty() {
            rsx! { span { class: "adui-select-selection-placeholder", "{placeholder_str}" } }
        } else {
            rsx! {
                div { class: "adui-select-selection-overflow",
                    {selected_keys.iter().map(|k| {
                        let label = find_label(k);
                        rsx! {
                            span { class: "adui-select-selection-item", "{label}" }
                        }
                    })}
                }
            }
        }
    } else {
        if let Some(first) = selected_keys.get(0) {
            let label = find_label(first);
            rsx! { span { class: "adui-select-selection-item", "{label}" } }
        } else {
            rsx! { span { class: "adui-select-selection-placeholder", "{placeholder_str}" } }
        }
    };

    // Shared helpers for event handlers.
    let form_for_handlers = form_control.clone();
    let internal_selected_for_handlers = internal_selected;
    let on_change_cb = on_change;
    let multiple_flag = multiple;
    let controlled_flag = controlled_by_prop;

    let mut open_for_toggle = open_state;
    let is_disabled_flag = is_disabled;

    let mut search_for_input = search_query;

    let mut active_for_keydown = active_index;
    let internal_selected_for_keydown = internal_selected;
    let form_for_keydown = form_for_handlers.clone();
    let mut open_for_keydown = open_for_toggle;

    // Local copies of the internal click flag for different handlers.
    let mut internal_click_for_toggle = internal_click_flag;
    let internal_click_for_keydown = internal_click_flag;

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

    rsx! {
        div {
            class: "adui-select-root",
            style: "position: relative; display: inline-block;",
            div {
                class: "{class_attr}",
                style: "{style_attr}",
                role: "combobox",
                tabindex: 0,
                "aria-expanded": open_flag,
                "aria-disabled": is_disabled_flag,
                onclick: move |_| {
                    if is_disabled_flag {
                        return;
                    }
                    // Mark as internal click so the document-level handler does
                    // not immediately close the dropdown.
                    let mut flag = internal_click_for_toggle;
                    flag.set(true);

                    let mut open_signal = open_for_toggle;
                    let current = *open_signal.read();
                    open_signal.set(!current);
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if is_disabled_flag {
                        return;
                    }
                    use dioxus::prelude::Key;

                    let open_now = *open_for_keydown.read();
                    if !open_now {
                        match evt.key() {
                            Key::Enter | Key::ArrowDown => {
                                evt.prevent_default();
                                let mut open_signal = open_for_keydown;
                                open_signal.set(true);
                            }
                            Key::Escape => {
                                // 没有打开时按 Escape 不做任何事。
                            }
                            _ => {}
                        }
                        return;
                    }

                    if matches!(evt.key(), Key::Escape) {
                        let mut open_signal = open_for_keydown;
                        open_signal.set(false);
                        return;
                    }

                    let opts_len = filtered_options.len();
                    if opts_len == 0 {
                        return;
                    }

                    // Keyboard interactions inside the select should not be
                    // treated as outside clicks, so mark this as internal.
                    let mut flag = internal_click_for_keydown;
                    flag.set(true);

                    if let Some(idx) = handle_option_list_key_event(&evt, opts_len, &active_for_keydown) {
                        if idx < opts_len {
                            let opt = &filtered_options[idx];
                            if opt.disabled {
                                return;
                            }

                            let key = opt.key.clone();
                            let current_keys = selected_keys.clone();
                            let next_keys = if multiple_flag {
                                toggle_option_key(&current_keys, &key)
                            } else {
                                vec![key.clone()]
                            };

                            apply_selected_keys(
                                &form_for_keydown,
                                multiple_flag,
                                controlled_flag,
                                &internal_selected_for_keydown,
                                on_change_cb,
                                next_keys,
                            );

                            if !multiple_flag {
                                let mut open_signal = open_for_keydown;
                                open_signal.set(false);
                            }
                        }
                    }
                },
                div { class: "adui-select-selector", {display_node} }
                if allow_clear && !selected_keys.is_empty() && !is_disabled_flag {
                    span {
                        class: "adui-select-clear",
                        onclick: move |_| {
                            apply_selected_keys(
                                &form_for_handlers,
                                multiple_flag,
                                controlled_flag,
                                &internal_selected_for_handlers,
                                on_change_cb,
                                Vec::new(),
                            );
                        },
                        "×"
                    }
                }
            }
            if open_flag {
                div {
                    class: "{dropdown_class_attr}",
                    style: "{dropdown_style_attr}",
                    role: "listbox",
                    "aria-multiselectable": multiple_flag,
                    if show_search {
                        div { class: "adui-select-search",
                            input {
                                class: "adui-select-search-input",
                                value: "{search_for_input.read()}",
                                oninput: move |evt| {
                                    let mut signal = search_for_input;
                                    signal.set(evt.value());
                                }
                            }
                        }
                    }
                    ul { class: "adui-select-item-list",
                        {filtered_options.iter().enumerate().map(|(index, opt)| {
                            let key = opt.key.clone();
                            let label = opt.label.clone();
                            let disabled_opt = opt.disabled || is_disabled_flag;
                            let is_selected = selected_keys.contains(&key);
                            let is_active = active_index
                                .read()
                                .as_ref()
                                .map(|i| *i == index)
                                .unwrap_or(false);
                            let selected_snapshot = selected_keys.clone();
                            let form_for_click = form_control.clone();
                            let internal_selected_for_click = internal_selected;
                            let mut open_for_click = open_state;
                            let mut internal_click_for_item = internal_click_flag;

                            rsx! {
                                li {
                                    class: {
                                        let mut classes = vec!["adui-select-item".to_string()];
                                        if is_selected {
                                            classes.push("adui-select-item-option-selected".into());
                                        }
                                        if disabled_opt {
                                            classes.push("adui-select-item-option-disabled".into());
                                        }
                                        if is_active {
                                            classes.push("adui-select-item-option-active".into());
                                        }
                                        classes.join(" ")
                                    },
                                    role: "option",
                                    "aria-selected": is_selected,
                                    onclick: move |_| {
                                        if disabled_opt {
                                            return;
                                        }
                                        // Mark as internal click so the document-level
                                        // handler does not treat this as outside.
                                        let mut flag = internal_click_for_item;
                                        flag.set(true);

                                        let current_keys = selected_snapshot.clone();
                                        let next_keys = if multiple_flag {
                                            toggle_option_key(&current_keys, &key)
                                        } else {
                                            vec![key.clone()]
                                        };

                                        apply_selected_keys(
                                            &form_for_click,
                                            multiple_flag,
                                            controlled_flag,
                                            &internal_selected_for_click,
                                            on_change_cb,
                                            next_keys,
                                        );

                                        if !multiple_flag {
                                            let mut open_signal = open_for_click;
                                            open_signal.set(false);
                                        }
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

fn apply_selected_keys(
    form_control: &Option<FormItemControlContext>,
    multiple: bool,
    controlled_by_prop: bool,
    selected_signal: &Signal<Vec<OptionKey>>,
    on_change: Option<EventHandler<Vec<String>>>,
    new_keys: Vec<OptionKey>,
) {
    if let Some(ctx) = form_control {
        if multiple {
            let json = option_keys_to_value(&new_keys);
            ctx.set_value(json);
        } else if let Some(first) = new_keys.get(0) {
            let json = option_key_to_value(first);
            ctx.set_value(json);
        } else {
            ctx.set_value(Value::Null);
        }
    } else if !controlled_by_prop {
        let mut signal = *selected_signal;
        signal.set(new_keys.clone());
    }

    if let Some(cb) = on_change {
        cb.call(new_keys);
    }
}
