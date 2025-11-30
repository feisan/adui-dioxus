//! Select component aligned with Ant Design 6.0.
//!
//! Supports single select, multiple select, and tags mode.

use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::{FormItemControlContext, use_form_item_control};
use crate::components::icon::{Icon, IconKind};
use crate::components::select_base::{
    DropdownLayer, OptionKey, SelectOption, handle_option_list_key_event, option_key_to_value,
    option_keys_to_value, toggle_option_key, use_dropdown_layer, value_to_option_key,
    value_to_option_keys,
};
use crate::foundation::{
    ClassListExt, SelectClassNames, SelectSemantic, SelectStyles, StyleStringExt, Variant,
    variant_from_bordered,
};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use serde_json::Value;

/// Re-export of the shared option type so that callers can build option lists
/// without depending on the internal `select_base` module path.
pub use crate::components::select_base::SelectOption as PublicSelectOption;

/// Select mode determining single/multiple selection behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectMode {
    /// Single selection (default).
    #[default]
    Single,
    /// Multiple selection.
    Multiple,
    /// Tags mode - allows creating new options from input.
    Tags,
}

impl SelectMode {
    /// Whether this mode allows multiple selections.
    pub fn is_multiple(&self) -> bool {
        matches!(self, SelectMode::Multiple | SelectMode::Tags)
    }
}

/// Placement of the dropdown relative to the select trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectPlacement {
    #[default]
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl SelectPlacement {
    fn as_style(&self) -> &'static str {
        match self {
            SelectPlacement::BottomLeft => "top: 100%; left: 0;",
            SelectPlacement::BottomRight => "top: 100%; right: 0;",
            SelectPlacement::TopLeft => "bottom: 100%; left: 0;",
            SelectPlacement::TopRight => "bottom: 100%; right: 0;",
        }
    }
}

/// Props for the Select component.
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
    /// Selection mode: single, multiple, or tags.
    #[props(default)]
    pub mode: SelectMode,
    /// @deprecated Use `mode` instead. When true, allow selecting multiple options.
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
    /// Visual variant (outlined/filled/borderless).
    #[props(optional)]
    pub variant: Option<Variant>,
    /// @deprecated Use `variant="borderless"` instead.
    #[props(optional)]
    pub bordered: Option<bool>,
    /// Prefix element displayed before the selection.
    #[props(optional)]
    pub prefix: Option<Element>,
    /// Custom suffix icon (defaults to down arrow).
    #[props(optional)]
    pub suffix_icon: Option<Element>,
    /// Dropdown placement relative to the select.
    #[props(default)]
    pub placement: SelectPlacement,
    /// Whether dropdown width should match select width.
    #[props(default = true)]
    pub popup_match_select_width: bool,
    #[props(optional)]
    pub class: Option<String>,
    /// Extra class applied to root element.
    #[props(optional)]
    pub root_class_name: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Semantic class names for sub-parts.
    #[props(optional)]
    pub class_names: Option<SelectClassNames>,
    /// Semantic styles for sub-parts.
    #[props(optional)]
    pub styles: Option<SelectStyles>,
    /// Optional extra classes/styles for the dropdown popup.
    #[props(optional)]
    pub dropdown_class: Option<String>,
    #[props(optional)]
    pub dropdown_style: Option<String>,
    /// Change event emitted with the full set of selected keys.
    #[props(optional)]
    pub on_change: Option<EventHandler<Vec<String>>>,
    /// Called when dropdown visibility changes.
    #[props(optional)]
    pub on_dropdown_visible_change: Option<EventHandler<bool>>,
}

/// Ant Design flavored Select.
#[allow(clippy::collapsible_if)]
#[component]
pub fn Select(props: SelectProps) -> Element {
    let SelectProps {
        value,
        values,
        options,
        mode,
        multiple,
        allow_clear,
        placeholder,
        disabled,
        show_search,
        status,
        size,
        variant,
        bordered,
        prefix,
        suffix_icon,
        placement,
        popup_match_select_width,
        class,
        root_class_name,
        style,
        class_names,
        styles,
        dropdown_class,
        dropdown_style,
        on_change,
        on_dropdown_visible_change,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();

    // Resolve if multiple selection is enabled (mode takes precedence over deprecated `multiple`)
    let is_multiple = mode.is_multiple() || multiple;

    // Resolve variant
    let resolved_variant = variant_from_bordered(bordered, variant);

    let final_size = size.unwrap_or(config.size);

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    // Internal selection state used only when the component is not controlled
    // by Form or external props.
    let internal_selected: Signal<Vec<OptionKey>> = use_signal(Vec::new);

    let has_form = form_control.is_some();
    let prop_single = value.clone();
    let prop_multi = values.clone();
    let multiple_flag = is_multiple;

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

    // Tags mode: input for creating new tags
    let tags_input: Signal<String> = use_signal(String::new);

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
        let on_visible_cb = on_dropdown_visible_change;
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
                                if let Some(cb) = on_visible_cb {
                                    cb.call(false);
                                }
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
    let search_query: Signal<String> = use_signal(String::new);

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
    if is_multiple {
        class_list.push("adui-select-multiple".into());
    }
    if matches!(mode, SelectMode::Tags) {
        class_list.push("adui-select-tags".into());
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
    class_list.push(resolved_variant.class_for("adui-select"));
    push_status_class(&mut class_list, status);
    class_list.push_semantic(&class_names, SelectSemantic::Root);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    if let Some(extra) = root_class_name {
        class_list.push(extra);
    }
    let class_attr = class_list.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ");

    let mut style_attr = style.unwrap_or_default();
    style_attr.append_semantic(&styles, SelectSemantic::Root);

    // Helper to find the label for a given key.
    let find_label = |key: &str| -> String {
        options
            .iter()
            .find(|opt| opt.key == key)
            .map(|opt| opt.label.clone())
            .unwrap_or_else(|| key.to_string())
    };

    // Clone form_control and selected_keys early for use in display_node closures
    let form_for_tags = form_control.clone();
    let selected_for_tags = selected_keys.clone();
    let selected_for_clear = selected_keys.clone();

    let display_node = if is_multiple {
        if selected_keys.is_empty() {
            rsx! { span { class: "adui-select-selection-placeholder", "{placeholder_str}" } }
        } else {
            rsx! {
                div { class: "adui-select-selection-overflow",
                    {selected_keys.iter().map(|k| {
                        let label = find_label(k);
                        let key_for_remove = k.clone();
                        let form_for_remove = form_control.clone();
                        let internal_selected_for_remove = internal_selected;
                        let selected_snapshot = selected_keys.clone();

                        rsx! {
                            span { class: "adui-select-selection-item",
                                "{label}"
                                span {
                                    class: "adui-select-selection-item-remove",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        let next_keys = selected_snapshot.iter()
                                            .filter(|k| **k != key_for_remove)
                                            .cloned()
                                            .collect();
                                        apply_selected_keys(
                                            &form_for_remove,
                                            multiple_flag,
                                            controlled_by_prop,
                                            &internal_selected_for_remove,
                                            on_change,
                                            next_keys,
                                        );
                                    },
                                    "×"
                                }
                            }
                        }
                    })}
                    if matches!(mode, SelectMode::Tags) {
                        input {
                            class: "adui-select-selection-search-input",
                            value: "{tags_input.read()}",
                            oninput: move |evt| {
                                let mut sig = tags_input;
                                sig.set(evt.value());
                            },
                            onkeydown: move |evt: KeyboardEvent| {
                                use dioxus::prelude::Key;
                                if matches!(evt.key(), Key::Enter) {
                                    let input_val = tags_input.read().trim().to_string();
                                    if !input_val.is_empty() && !selected_for_tags.contains(&input_val) {
                                        let mut next_keys = selected_for_tags.clone();
                                        next_keys.push(input_val);
                                        apply_selected_keys(
                                            &form_for_tags,
                                            multiple_flag,
                                            controlled_by_prop,
                                            &internal_selected,
                                            on_change,
                                            next_keys,
                                        );
                                        let mut sig = tags_input;
                                        sig.set(String::new());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if let Some(first) = selected_keys.first() {
        let label = find_label(first);
        rsx! { span { class: "adui-select-selection-item", "{label}" } }
    } else {
        rsx! { span { class: "adui-select-selection-placeholder", "{placeholder_str}" } }
    };

    // Shared helpers for event handlers.
    let form_for_handlers = form_control.clone();
    let internal_selected_for_handlers = internal_selected;
    let on_change_cb = on_change;

    let open_for_toggle = open_state;
    let is_disabled_flag = is_disabled;

    let search_for_input = search_query;

    let active_for_keydown = active_index;
    let internal_selected_for_keydown = internal_selected;
    let form_for_keydown = form_for_handlers.clone();
    let open_for_keydown = open_for_toggle;

    // Local copies of the internal click flag for different handlers.
    let internal_click_for_toggle = internal_click_flag;
    let internal_click_for_keydown = internal_click_flag;

    let dropdown_class_attr = {
        let mut list = vec!["adui-select-dropdown".to_string()];
        if let Some(extra) = dropdown_class {
            list.push(extra);
        }
        list.join(" ")
    };

    let min_width_style = if popup_match_select_width {
        "min-width: 100%;"
    } else {
        ""
    };

    let dropdown_style_attr = format!(
        "position: absolute; {} {} z-index: {}; {}",
        placement.as_style(),
        min_width_style,
        current_z,
        dropdown_style.unwrap_or_default()
    );

    // Default suffix icon
    let suffix_element = suffix_icon.unwrap_or_else(|| {
        rsx! {
            span { class: "adui-select-arrow",
                Icon { kind: IconKind::ArrowDown, size: 12.0 }
            }
        }
    });

    let on_visible_cb = on_dropdown_visible_change;

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
                    let next = !current;
                    open_signal.set(next);
                    if let Some(cb) = on_visible_cb {
                        cb.call(next);
                    }
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
                                if let Some(cb) = on_visible_cb {
                                    cb.call(true);
                                }
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
                        if let Some(cb) = on_visible_cb {
                            cb.call(false);
                        }
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
                                controlled_by_prop,
                                &internal_selected_for_keydown,
                                on_change_cb,
                                next_keys,
                            );

                            if !multiple_flag {
                                let mut open_signal = open_for_keydown;
                                open_signal.set(false);
                                if let Some(cb) = on_visible_cb {
                                    cb.call(false);
                                }
                            }
                        }
                    }
                },
                if let Some(prefix_el) = prefix {
                    span { class: "adui-select-prefix", {prefix_el} }
                }
                div { class: "adui-select-selector", {display_node} }
                {suffix_element}
                if allow_clear && !selected_for_clear.is_empty() && !is_disabled_flag {
                    span {
                        class: "adui-select-clear",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            apply_selected_keys(
                                &form_for_handlers,
                                multiple_flag,
                                controlled_by_prop,
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
                    onclick: move |_| {
                        // Prevent clicks inside dropdown from closing it
                        let mut flag = internal_click_flag;
                        flag.set(true);
                    },
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
                            let open_for_click = open_state;
                            let internal_click_for_item = internal_click_flag;

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
                                            controlled_by_prop,
                                            &internal_selected_for_click,
                                            on_change_cb,
                                            next_keys,
                                        );

                                        if !multiple_flag {
                                            let mut open_signal = open_for_click;
                                            open_signal.set(false);
                                            if let Some(cb) = on_visible_cb {
                                                cb.call(false);
                                            }
                                        }
                                    },
                                    "{label}"
                                    if is_selected {
                                        span { class: "adui-select-item-option-state",
                                            Icon { kind: IconKind::Check, size: 12.0 }
                                        }
                                    }
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
        } else if let Some(first) = new_keys.first() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_mode_is_multiple() {
        assert!(!SelectMode::Single.is_multiple());
        assert!(SelectMode::Multiple.is_multiple());
        assert!(SelectMode::Tags.is_multiple());
    }

    #[test]
    fn select_placement_styles() {
        assert!(SelectPlacement::BottomLeft.as_style().contains("top: 100%"));
        assert!(SelectPlacement::TopLeft.as_style().contains("bottom: 100%"));
    }
}
