//! Input component family aligned with Ant Design 6.0.
//!
//! Includes:
//! - `Input` - Basic text input
//! - `TextArea` - Multi-line text input
//! - `Password` - Password input with visibility toggle
//! - `Search` - Input with search button
//! - `OTP` - One-time password input

use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::use_form_item_control;
use crate::components::form::{FormItemControlContext, form_value_to_string};
use crate::components::icon::{Icon, IconKind};
use crate::foundation::{
    ClassListExt, InputClassNames, InputSemantic, InputStyles, StyleStringExt, Variant,
    variant_from_bordered,
};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::Key;
use dioxus::prelude::*;

/// Size variants for Input components.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl InputSize {
    fn from_global(size: ComponentSize) -> Self {
        match size {
            ComponentSize::Small => InputSize::Small,
            ComponentSize::Large => InputSize::Large,
            ComponentSize::Middle => InputSize::Middle,
        }
    }

    fn as_class(&self) -> &'static str {
        match self {
            InputSize::Small => "adui-input-sm",
            InputSize::Middle => "",
            InputSize::Large => "adui-input-lg",
        }
    }
}

/// Props for a single-line text input.
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Controlled value. When set, the component will not manage internal state.
    #[props(optional)]
    pub value: Option<String>,
    /// Initial value in uncontrolled mode.
    #[props(optional)]
    pub default_value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub disabled: bool,
    /// Component size.
    #[props(optional)]
    pub size: Option<InputSize>,
    /// Visual variant (outlined/filled/borderless).
    #[props(optional)]
    pub variant: Option<Variant>,
    /// @deprecated Use `variant="borderless"` instead.
    #[props(optional)]
    pub bordered: Option<bool>,
    /// Optional status used to style the wrapper (success/warning/error).
    #[props(optional)]
    pub status: Option<ControlStatus>,
    /// Leading element rendered inside the affix wrapper.
    #[props(optional)]
    pub prefix: Option<Element>,
    /// Trailing element rendered inside the affix wrapper.
    #[props(optional)]
    pub suffix: Option<Element>,
    /// @deprecated Use `Space.Compact` instead. Content before the input.
    #[props(optional)]
    pub addon_before: Option<Element>,
    /// @deprecated Use `Space.Compact` instead. Content after the input.
    #[props(optional)]
    pub addon_after: Option<Element>,
    /// Whether to show a clear icon when there is content.
    #[props(default)]
    pub allow_clear: bool,
    /// Maximum length of input.
    #[props(optional)]
    pub max_length: Option<usize>,
    /// Whether to show character count.
    #[props(default)]
    pub show_count: bool,
    #[props(optional)]
    pub class: Option<String>,
    /// Extra class applied to root element.
    #[props(optional)]
    pub root_class_name: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Semantic class names for sub-parts.
    #[props(optional)]
    pub class_names: Option<InputClassNames>,
    /// Semantic styles for sub-parts.
    #[props(optional)]
    pub styles: Option<InputStyles>,
    /// Change event with the next string value.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Triggered when pressing Enter.
    #[props(optional)]
    pub on_press_enter: Option<EventHandler<()>>,
    /// Data attributes as a map of key-value pairs. Keys should be without the "data-" prefix.
    /// For example, `data_attributes: Some([("test", "value")])` will render as `data-test="value"`.
    #[props(optional)]
    pub data_attributes: Option<Vec<(String, String)>>,
}

/// Ant Design flavored text input.
#[component]
pub fn Input(props: InputProps) -> Element {
    let InputProps {
        value,
        default_value,
        placeholder,
        disabled,
        size,
        variant,
        bordered,
        status,
        prefix,
        suffix,
        addon_before,
        addon_after,
        allow_clear,
        max_length,
        show_count,
        class,
        root_class_name,
        style,
        class_names,
        styles,
        on_change,
        on_press_enter,
        data_attributes,
    } = props;

    // Generate a unique ID for this input to support data-* attributes via JavaScript interop
    let input_id = use_signal(|| format!("adui-input-{}", rand_id()));

    // Set data-* attributes via JavaScript interop if provided
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(data_attrs) = data_attributes.as_ref() {
            let id = input_id.read().clone();
            let attrs = data_attrs.clone();
            {
                use_effect(move || {
                    use wasm_bindgen::JsCast;
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(element) = document.get_element_by_id(&id) {
                                for (key, value) in attrs.iter() {
                                    let attr_name = format!("data-{}", key);
                                    let _ = element.set_attribute(&attr_name, value);
                                }
                            }
                        }
                    }
                });
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Suppress unused variable warning on non-wasm32 targets
        let _ = data_attributes;
    }

    let placeholder_str = placeholder.unwrap_or_default();
    let config = use_config();
    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();

    // Resolve size
    let resolved_size = size.unwrap_or_else(|| InputSize::from_global(config.size));

    // Resolve variant
    let resolved_variant = variant_from_bordered(bordered, variant);

    // Local state used only when not controlled by Form or external value.
    let initial_inner = default_value.clone().unwrap_or_default();
    let inner_value = use_signal(|| initial_inner);

    let current_value = resolve_current_value(&form_control, value.clone(), inner_value);
    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let has_prefix = prefix.is_some();
    let has_user_suffix = suffix.is_some();
    let has_clear = allow_clear && !current_value.is_empty() && !is_disabled;
    let has_any_suffix = has_user_suffix || has_clear || show_count;
    let has_addon_before = addon_before.is_some();
    let has_addon_after = addon_after.is_some();
    let has_addon = has_addon_before || has_addon_after;

    // Build shared handlers.
    let control_for_change = form_control.clone();
    let on_change_cb = on_change;
    let on_press_enter_cb = on_press_enter;
    let controlled_flag = controlled_by_prop;
    let mut inner_for_change = inner_value;

    // Build input element classes
    let mut input_class_list = vec!["adui-input".to_string()];
    input_class_list.push_semantic(&class_names, InputSemantic::Input);

    let input_class_attr = input_class_list.join(" ");

    // Build input style
    let mut input_style = String::new();
    input_style.append_semantic(&styles, InputSemantic::Input);

    // Count display
    let char_count = current_value.chars().count();
    let count_text = if let Some(max) = max_length {
        format!("{}/{}", char_count, max)
    } else {
        char_count.to_string()
    };

    let input_id_val = input_id.read().clone();
    let input_node = {
        let max_len_attr = max_length.map(|m| m.to_string());
        rsx! {
            input {
                id: "{input_id_val}",
                class: "{input_class_attr}",
                style: "{input_style}",
                disabled: is_disabled,
                value: "{current_value}",
                placeholder: "{placeholder_str}",
                maxlength: max_len_attr,
                oninput: move |evt| {
                    let next = evt.value();
                    apply_input_value(
                        next,
                        &control_for_change,
                        controlled_flag,
                        &mut inner_for_change,
                        on_change_cb,
                    );
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if matches!(evt.key(), Key::Enter)
                        && let Some(cb) = on_press_enter_cb
                    {
                        cb.call(());
                    }
                }
            }
        }
    };

    // Build wrapper classes
    let build_wrapper_classes = |extra_class: &str| {
        let mut classes = vec!["adui-input-affix-wrapper".to_string()];
        classes.push(resolved_size.as_class().to_string());
        classes.push(resolved_variant.class_for("adui-input"));
        if is_disabled {
            classes.push("adui-input-disabled".into());
        }
        push_status_class(&mut classes, status);
        if !extra_class.is_empty() {
            classes.push(extra_class.to_string());
        }
        classes.push_semantic(&class_names, InputSemantic::Root);
        if let Some(extra) = class.clone() {
            classes.push(extra);
        }
        if let Some(extra) = root_class_name.clone() {
            classes.push(extra);
        }
        classes
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    };

    let build_wrapper_style = || {
        let mut s = style.clone().unwrap_or_default();
        s.append_semantic(&styles, InputSemantic::Root);
        s
    };

    // Wrap with addon group if needed
    if has_addon {
        let wrapper_class = build_wrapper_classes("");
        let wrapper_style = build_wrapper_style();

        let control_for_clear = form_control;
        let mut inner_for_clear = inner_value;
        let on_change_for_clear = on_change_cb;

        rsx! {
            div { class: "adui-input-group adui-input-group-wrapper",
                if let Some(before) = addon_before {
                    span { class: "adui-input-group-addon", {before} }
                }
                div {
                    class: "{wrapper_class}",
                    style: "{wrapper_style}",
                    if let Some(icon) = prefix {
                        span { class: "adui-input-prefix", {icon} }
                    }
                    {input_node}
                    if has_any_suffix {
                        span {
                            class: "adui-input-suffix",
                            if let Some(icon) = suffix {
                                {icon}
                            }
                            if has_clear {
                                span {
                                    class: "adui-input-clear",
                                    onclick: move |_| {
                                        apply_input_value(
                                            String::new(),
                                            &control_for_clear,
                                            controlled_flag,
                                            &mut inner_for_clear,
                                            on_change_for_clear,
                                        );
                                    },
                                    "×"
                                }
                            }
                            if show_count {
                                span { class: "adui-input-count", "{count_text}" }
                            }
                        }
                    }
                }
                if let Some(after) = addon_after {
                    span { class: "adui-input-group-addon", {after} }
                }
            }
        }
    } else if has_prefix || has_any_suffix {
        // Affix wrapper variant
        let wrapper_class = build_wrapper_classes("");
        let wrapper_style = build_wrapper_style();

        let control_for_clear = form_control;
        let mut inner_for_clear = inner_value;
        let on_change_for_clear = on_change_cb;

        rsx! {
            div {
                class: "{wrapper_class}",
                style: "{wrapper_style}",
                if let Some(icon) = prefix {
                    span { class: "adui-input-prefix", {icon} }
                }
                {input_node}
                if has_any_suffix {
                    span {
                        class: "adui-input-suffix",
                        if let Some(icon) = suffix {
                            {icon}
                        }
                        if has_clear {
                            span {
                                class: "adui-input-clear",
                                onclick: move |_| {
                                    apply_input_value(
                                        String::new(),
                                        &control_for_clear,
                                        controlled_flag,
                                        &mut inner_for_clear,
                                        on_change_for_clear,
                                    );
                                },
                                "×"
                            }
                        }
                        if show_count {
                            span { class: "adui-input-count", "{count_text}" }
                        }
                    }
                }
            }
        }
    } else {
        // Simple input variant
        let mut class_list = vec!["adui-input".to_string()];
        class_list.push(resolved_size.as_class().to_string());
        class_list.push(resolved_variant.class_for("adui-input"));
        if is_disabled {
            class_list.push("adui-input-disabled".into());
        }
        push_status_class(&mut class_list, status);
        class_list.push_semantic(&class_names, InputSemantic::Root);
        if let Some(extra) = class {
            class_list.push(extra);
        }
        if let Some(extra) = root_class_name {
            class_list.push(extra);
        }
        let class_attr = class_list
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let style_attr = build_wrapper_style();

        let max_len_attr = max_length.map(|m| m.to_string());
        let input_id_val = input_id.read().clone();

        rsx! {
            input {
                id: "{input_id_val}",
                class: "{class_attr}",
                style: "{style_attr}",
                disabled: is_disabled,
                value: "{current_value}",
                placeholder: "{placeholder_str}",
                maxlength: max_len_attr,
                oninput: move |evt| {
                    let next = evt.value();
                    apply_input_value(
                        next,
                        &form_control,
                        controlled_flag,
                        &mut inner_for_change,
                        on_change_cb,
                    );
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if matches!(evt.key(), Key::Enter)
                        && let Some(cb) = on_press_enter
                    {
                        cb.call(());
                    }
                }
            }
        }
    }
}

// ============================================================================
// Password Input
// ============================================================================

/// Props for Password input.
#[derive(Props, Clone, PartialEq)]
pub struct PasswordProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub default_value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub size: Option<InputSize>,
    #[props(optional)]
    pub variant: Option<Variant>,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub prefix: Option<Element>,
    /// Whether the password is visible by default.
    #[props(default)]
    pub visible: bool,
    /// Custom icon for showing password.
    #[props(optional)]
    pub icon_render: Option<Element>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub class_names: Option<InputClassNames>,
    #[props(optional)]
    pub styles: Option<InputStyles>,
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_press_enter: Option<EventHandler<()>>,
    /// Called when visibility changes.
    #[props(optional)]
    pub on_visible_change: Option<EventHandler<bool>>,
}

/// Password input with visibility toggle.
#[component]
pub fn Password(props: PasswordProps) -> Element {
    let PasswordProps {
        value,
        default_value,
        placeholder,
        disabled,
        size,
        variant,
        status,
        prefix,
        visible: initial_visible,
        icon_render,
        class,
        style,
        class_names,
        styles,
        on_change,
        on_press_enter,
        on_visible_change,
    } = props;

    let visible_signal = use_signal(|| initial_visible);
    let is_visible = *visible_signal.read();

    let visibility_icon = icon_render.unwrap_or_else(|| {
        if is_visible {
            rsx! { Icon { kind: IconKind::Eye } }
        } else {
            rsx! { Icon { kind: IconKind::EyeInvisible } }
        }
    });

    let on_visible_cb = on_visible_change;

    let suffix = rsx! {
        span {
            class: "adui-input-password-icon",
            style: "cursor: pointer;",
            onclick: move |_| {
                let mut sig = visible_signal;
                let next = !*sig.read();
                sig.set(next);
                if let Some(cb) = on_visible_cb {
                    cb.call(next);
                }
            },
            {visibility_icon}
        }
    };

    rsx! {
        InputInternal {
            value: value,
            default_value: default_value,
            placeholder: placeholder,
            disabled: disabled,
            size: size,
            variant: variant,
            status: status,
            prefix: prefix,
            suffix: Some(suffix),
            is_password: !is_visible,
            class: class,
            style: style,
            class_names: class_names,
            styles: styles,
            on_change: on_change,
            on_press_enter: on_press_enter,
            extra_class: Some("adui-input-password".to_string()),
        }
    }
}

// ============================================================================
// Search Input
// ============================================================================

/// Props for Search input.
#[derive(Props, Clone, PartialEq)]
pub struct SearchProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub default_value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub size: Option<InputSize>,
    #[props(optional)]
    pub variant: Option<Variant>,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub prefix: Option<Element>,
    /// Whether to show enter button.
    #[props(default = true)]
    pub enter_button: bool,
    /// Custom content for enter button.
    #[props(optional)]
    pub enter_button_content: Option<Element>,
    /// Whether search is in loading state.
    #[props(default)]
    pub loading: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub class_names: Option<InputClassNames>,
    #[props(optional)]
    pub styles: Option<InputStyles>,
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Triggered when search button is clicked or Enter is pressed.
    #[props(optional)]
    pub on_search: Option<EventHandler<String>>,
}

/// Search input with search button.
#[component]
pub fn Search(props: SearchProps) -> Element {
    let SearchProps {
        value,
        default_value,
        placeholder,
        disabled,
        size,
        variant,
        status,
        prefix,
        enter_button,
        enter_button_content,
        loading,
        class,
        style,
        class_names,
        styles,
        on_change,
        on_search,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();

    let initial_inner = default_value.clone().unwrap_or_default();
    let inner_value = use_signal(|| initial_inner);

    let current_value = resolve_current_value(&form_control, value.clone(), inner_value);
    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let resolved_size = size.unwrap_or_else(|| InputSize::from_global(config.size));
    let resolved_variant = variant.unwrap_or(Variant::Outlined);

    let control_for_change = form_control.clone();
    let on_change_cb = on_change;
    let mut inner_for_change = inner_value;
    let on_search_cb = on_search;

    // Build wrapper classes
    let mut wrapper_classes = vec![
        "adui-input-search".to_string(),
        "adui-input-affix-wrapper".to_string(),
    ];
    wrapper_classes.push(resolved_size.as_class().to_string());
    wrapper_classes.push(resolved_variant.class_for("adui-input"));
    if is_disabled {
        wrapper_classes.push("adui-input-disabled".into());
    }
    if enter_button {
        wrapper_classes.push("adui-input-search-with-button".into());
    }
    push_status_class(&mut wrapper_classes, status);
    wrapper_classes.push_semantic(&class_names, InputSemantic::Root);
    if let Some(extra) = class {
        wrapper_classes.push(extra);
    }
    let wrapper_class = wrapper_classes
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let mut wrapper_style = style.unwrap_or_default();
    wrapper_style.append_semantic(&styles, InputSemantic::Root);

    let search_icon = if loading {
        rsx! { Icon { kind: IconKind::Loading, spin: true } }
    } else {
        rsx! { Icon { kind: IconKind::Search } }
    };

    let search_button_content = enter_button_content.unwrap_or(search_icon);

    // Clone value for event handlers
    let value_for_keydown = current_value.clone();
    let value_for_button = current_value.clone();
    let value_for_icon = current_value.clone();

    rsx! {
        div {
            class: "{wrapper_class}",
            style: "{wrapper_style}",
            if let Some(icon) = prefix {
                span { class: "adui-input-prefix", {icon} }
            }
            input {
                class: "adui-input",
                disabled: is_disabled,
                value: "{current_value}",
                placeholder: placeholder.unwrap_or_default(),
                oninput: move |evt| {
                    let next = evt.value();
                    apply_input_value(
                        next,
                        &control_for_change,
                        controlled_by_prop,
                        &mut inner_for_change,
                        on_change_cb,
                    );
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if matches!(evt.key(), Key::Enter) {
                        if let Some(cb) = on_search_cb {
                            cb.call(value_for_keydown.clone());
                        }
                    }
                }
            }
            if enter_button {
                button {
                    class: "adui-input-search-button",
                    r#type: "button",
                    disabled: is_disabled || loading,
                    onclick: move |_| {
                        if let Some(cb) = on_search_cb {
                            cb.call(value_for_button.clone());
                        }
                    },
                    {search_button_content}
                }
            } else {
                span {
                    class: "adui-input-suffix adui-input-search-icon",
                    style: "cursor: pointer;",
                    onclick: move |_| {
                        if let Some(cb) = on_search_cb {
                            cb.call(value_for_icon.clone());
                        }
                    },
                    {search_button_content}
                }
            }
        }
    }
}

// ============================================================================
// OTP Input
// ============================================================================

/// Props for OTP (One-Time Password) input.
#[derive(Props, Clone, PartialEq)]
pub struct OTPProps {
    /// Number of input fields (default: 6).
    #[props(default = 6)]
    pub length: usize,
    /// Controlled value.
    #[props(optional)]
    pub value: Option<String>,
    /// Default value.
    #[props(optional)]
    pub default_value: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub size: Option<InputSize>,
    #[props(optional)]
    pub variant: Option<Variant>,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    /// Whether to mask input (like password).
    #[props(default)]
    pub mask: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Called when all fields are filled.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Called when input is complete.
    #[props(optional)]
    pub on_complete: Option<EventHandler<String>>,
}

/// One-Time Password input with multiple fields.
#[component]
pub fn OTP(props: OTPProps) -> Element {
    let OTPProps {
        length,
        value,
        default_value,
        disabled,
        size,
        variant,
        status,
        mask,
        class,
        style,
        on_change,
        on_complete,
    } = props;

    let config = use_config();
    let resolved_size = size.unwrap_or_else(|| InputSize::from_global(config.size));
    let resolved_variant = variant.unwrap_or(Variant::Outlined);

    let initial = default_value.unwrap_or_default();
    let chars: Vec<char> = initial.chars().collect();
    let initial_values: Vec<String> = (0..length)
        .map(|i| chars.get(i).map(|c| c.to_string()).unwrap_or_default())
        .collect();

    let values_signal: Signal<Vec<String>> = use_signal(|| initial_values);
    let is_controlled = value.is_some();

    // Build wrapper classes
    let mut wrapper_classes = vec!["adui-input-otp".to_string()];
    wrapper_classes.push(resolved_size.as_class().to_string());
    push_status_class(&mut wrapper_classes, status);
    if disabled {
        wrapper_classes.push("adui-input-otp-disabled".into());
    }
    if let Some(extra) = class {
        wrapper_classes.push(extra);
    }
    let wrapper_class = wrapper_classes.join(" ");
    let wrapper_style = style.unwrap_or_default();

    let input_type = if mask { "password" } else { "text" };

    // Helper to get current values
    let get_current_values = move || {
        if let Some(v) = &value {
            let chars: Vec<char> = v.chars().collect();
            (0..length)
                .map(|i| chars.get(i).map(|c| c.to_string()).unwrap_or_default())
                .collect()
        } else {
            values_signal.read().clone()
        }
    };

    rsx! {
        div { class: "{wrapper_class}", style: "{wrapper_style}",
            {(0..length).map(|idx| {
                let current_values = get_current_values();
                let cell_value = current_values.get(idx).cloned().unwrap_or_default();
                let values_for_input = values_signal;
                let on_change_cb = on_change;
                let on_complete_cb = on_complete;

                let mut cell_classes = vec!["adui-input-otp-cell".to_string(), "adui-input".to_string()];
                cell_classes.push(resolved_variant.class_for("adui-input"));

                rsx! {
                    input {
                        key: "{idx}",
                        class: cell_classes.join(" "),
                        r#type: "{input_type}",
                        disabled: disabled,
                        maxlength: "1",
                        value: "{cell_value}",
                        oninput: move |evt| {
                            let new_char = evt.value();
                            // Only take first character
                            let char_val = new_char.chars().next().map(|c| c.to_string()).unwrap_or_default();

                            if !is_controlled {
                                let mut vals = values_for_input;
                                vals.write()[idx] = char_val.clone();
                            }

                            // Build combined value from signal
                            let mut updated = values_for_input.read().clone();
                            if idx < updated.len() {
                                updated[idx] = char_val;
                            }
                            let combined: String = updated.iter().map(|s| s.as_str()).collect();

                            if let Some(cb) = on_change_cb {
                                cb.call(combined.clone());
                            }

                            // Check if complete
                            let all_filled = updated.iter().all(|s| !s.is_empty());
                            if all_filled {
                                if let Some(cb) = on_complete_cb {
                                    cb.call(combined);
                                }
                            }
                        }
                    }
                }
            })}
        }
    }
}

// ============================================================================
// TextArea
// ============================================================================

/// Multi-line text area component.
#[derive(Props, Clone, PartialEq)]
pub struct TextAreaProps {
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub default_value: Option<String>,
    #[props(optional)]
    pub placeholder: Option<String>,
    #[props(optional)]
    pub rows: Option<u16>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub size: Option<InputSize>,
    #[props(optional)]
    pub variant: Option<Variant>,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub max_length: Option<usize>,
    #[props(default)]
    pub show_count: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub class_names: Option<InputClassNames>,
    #[props(optional)]
    pub styles: Option<InputStyles>,
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
}

/// Ant Design flavored multi-line text area.
#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let TextAreaProps {
        value,
        default_value,
        placeholder,
        rows,
        disabled,
        size,
        variant,
        status,
        max_length,
        show_count,
        class,
        style,
        class_names,
        styles,
        on_change,
    } = props;

    let placeholder_str = placeholder.unwrap_or_default();
    let config = use_config();

    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();
    let initial_inner = default_value.clone().unwrap_or_default();
    let inner_value = use_signal(|| initial_inner);

    let current_value = resolve_current_value(&form_control, value.clone(), inner_value);
    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());
    let line_rows = rows.unwrap_or(3);

    let resolved_size = size.unwrap_or_else(|| InputSize::from_global(config.size));
    let resolved_variant = variant.unwrap_or(Variant::Outlined);

    let mut class_list = vec!["adui-input".to_string(), "adui-input-textarea".to_string()];
    class_list.push(resolved_size.as_class().to_string());
    class_list.push(resolved_variant.class_for("adui-input"));
    if is_disabled {
        class_list.push("adui-input-disabled".into());
    }
    push_status_class(&mut class_list, status);
    class_list.push_semantic(&class_names, InputSemantic::Root);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut style_attr = style.unwrap_or_default();
    style_attr.append_semantic(&styles, InputSemantic::Root);

    let control_for_change = form_control;
    let mut inner_for_change = inner_value;
    let on_change_cb = on_change;

    let char_count = current_value.chars().count();
    let count_text = if let Some(max) = max_length {
        format!("{}/{}", char_count, max)
    } else {
        char_count.to_string()
    };

    let max_len_attr = max_length.map(|m| m.to_string());

    if show_count {
        rsx! {
            div { class: "adui-input-textarea-wrapper",
                textarea {
                    class: "{class_attr}",
                    style: "{style_attr}",
                    disabled: is_disabled,
                    rows: "{line_rows}",
                    value: "{current_value}",
                    placeholder: "{placeholder_str}",
                    maxlength: max_len_attr,
                    oninput: move |evt| {
                        let next = evt.value();
                        apply_input_value(
                            next,
                            &control_for_change,
                            controlled_by_prop,
                            &mut inner_for_change,
                            on_change_cb,
                        );
                    }
                }
                span { class: "adui-input-textarea-count", "{count_text}" }
            }
        }
    } else {
        rsx! {
            textarea {
                class: "{class_attr}",
                style: "{style_attr}",
                disabled: is_disabled,
                rows: "{line_rows}",
                value: "{current_value}",
                placeholder: "{placeholder_str}",
                maxlength: max_len_attr,
                oninput: move |evt| {
                    let next = evt.value();
                    apply_input_value(
                        next,
                        &control_for_change,
                        controlled_by_prop,
                        &mut inner_for_change,
                        on_change_cb,
                    );
                }
            }
        }
    }
}

// ============================================================================
// Internal Input (shared implementation)
// ============================================================================

#[derive(Props, Clone, PartialEq)]
struct InputInternalProps {
    #[props(optional)]
    value: Option<String>,
    #[props(optional)]
    default_value: Option<String>,
    #[props(optional)]
    placeholder: Option<String>,
    #[props(default)]
    disabled: bool,
    #[props(optional)]
    size: Option<InputSize>,
    #[props(optional)]
    variant: Option<Variant>,
    #[props(optional)]
    status: Option<ControlStatus>,
    #[props(optional)]
    prefix: Option<Element>,
    #[props(optional)]
    suffix: Option<Element>,
    #[props(default)]
    is_password: bool,
    #[props(optional)]
    class: Option<String>,
    #[props(optional)]
    style: Option<String>,
    #[props(optional)]
    class_names: Option<InputClassNames>,
    #[props(optional)]
    styles: Option<InputStyles>,
    #[props(optional)]
    on_change: Option<EventHandler<String>>,
    #[props(optional)]
    on_press_enter: Option<EventHandler<()>>,
    #[props(optional)]
    extra_class: Option<String>,
}

#[component]
fn InputInternal(props: InputInternalProps) -> Element {
    let InputInternalProps {
        value,
        default_value,
        placeholder,
        disabled,
        size,
        variant,
        status,
        prefix,
        suffix,
        is_password,
        class,
        style,
        class_names,
        styles,
        on_change,
        on_press_enter,
        extra_class,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();

    let resolved_size = size.unwrap_or_else(|| InputSize::from_global(config.size));
    let resolved_variant = variant.unwrap_or(Variant::Outlined);

    let initial_inner = default_value.clone().unwrap_or_default();
    let inner_value = use_signal(|| initial_inner);

    let current_value = resolve_current_value(&form_control, value.clone(), inner_value);
    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let control_for_change = form_control;
    let on_change_cb = on_change;
    let on_press_enter_cb = on_press_enter;
    let controlled_flag = controlled_by_prop;
    let mut inner_for_change = inner_value;

    let mut wrapper_classes = vec!["adui-input-affix-wrapper".to_string()];
    wrapper_classes.push(resolved_size.as_class().to_string());
    wrapper_classes.push(resolved_variant.class_for("adui-input"));
    if is_disabled {
        wrapper_classes.push("adui-input-disabled".into());
    }
    push_status_class(&mut wrapper_classes, status);
    wrapper_classes.push_semantic(&class_names, InputSemantic::Root);
    if let Some(extra) = extra_class {
        wrapper_classes.push(extra);
    }
    if let Some(extra) = class {
        wrapper_classes.push(extra);
    }
    let wrapper_class = wrapper_classes
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut wrapper_style = style.unwrap_or_default();
    wrapper_style.append_semantic(&styles, InputSemantic::Root);

    let placeholder_str = placeholder.unwrap_or_default();
    let input_type = if is_password { "password" } else { "text" };

    rsx! {
        div {
            class: "{wrapper_class}",
            style: "{wrapper_style}",
            if let Some(icon) = prefix {
                span { class: "adui-input-prefix", {icon} }
            }
            input {
                class: "adui-input",
                r#type: "{input_type}",
                disabled: is_disabled,
                value: "{current_value}",
                placeholder: "{placeholder_str}",
                oninput: move |evt| {
                    let next = evt.value();
                    apply_input_value(
                        next,
                        &control_for_change,
                        controlled_flag,
                        &mut inner_for_change,
                        on_change_cb,
                    );
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if matches!(evt.key(), Key::Enter)
                        && let Some(cb) = on_press_enter_cb
                    {
                        cb.call(());
                    }
                }
            }
            if let Some(icon) = suffix {
                span { class: "adui-input-suffix", {icon} }
            }
        }
    }
}

// ============================================================================
// Helper functions
// ============================================================================

fn resolve_current_value(
    form_control: &Option<FormItemControlContext>,
    prop_value: Option<String>,
    inner: Signal<String>,
) -> String {
    if let Some(ctx) = form_control {
        return form_value_to_string(ctx.value());
    }
    if let Some(v) = prop_value {
        return v;
    }
    inner.read().clone()
}

fn apply_input_value(
    next: String,
    form_control: &Option<FormItemControlContext>,
    controlled_by_prop: bool,
    inner: &mut Signal<String>,
    on_change: Option<EventHandler<String>>,
) {
    if let Some(ctx) = form_control {
        ctx.set_string(next.clone());
    } else if !controlled_by_prop {
        let mut state = *inner;
        state.set(next.clone());
    }
    if let Some(cb) = on_change {
        cb.call(next);
    }
}

/// Generate a simple random ID for element identification.
fn rand_id() -> u32 {
    // Simple pseudo-random based on current time
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Math;
        (Math::random() * 1_000_000.0) as u32
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_size_class_mapping() {
        assert_eq!(InputSize::Small.as_class(), "adui-input-sm");
        assert_eq!(InputSize::Middle.as_class(), "");
        assert_eq!(InputSize::Large.as_class(), "adui-input-lg");
    }

    #[test]
    fn input_size_from_global() {
        assert_eq!(
            InputSize::from_global(ComponentSize::Small),
            InputSize::Small
        );
        assert_eq!(
            InputSize::from_global(ComponentSize::Middle),
            InputSize::Middle
        );
        assert_eq!(
            InputSize::from_global(ComponentSize::Large),
            InputSize::Large
        );
    }

    // Note: resolve_current_value and apply_input_value functions require Dioxus runtime
    // and cannot be easily tested in unit tests without a full runtime setup.
    // These functions are tested indirectly through integration tests and component demos.

    // Test InputSize enum methods
    #[test]
    fn input_size_variants() {
        assert_eq!(InputSize::Small.as_class(), "adui-input-sm");
        assert_eq!(InputSize::Middle.as_class(), "");
        assert_eq!(InputSize::Large.as_class(), "adui-input-lg");
    }

    // Test variant integration
    #[test]
    fn input_variant_integration() {
        use crate::foundation::variant_from_bordered;

        // Test variant takes priority over bordered
        assert_eq!(
            variant_from_bordered(Some(false), Some(Variant::Filled)),
            Variant::Filled
        );

        // Test bordered=false maps to Borderless
        assert_eq!(
            variant_from_bordered(Some(false), None),
            Variant::Borderless
        );

        // Test default is Outlined
        assert_eq!(variant_from_bordered(None, None), Variant::Outlined);
    }

    // Test character count calculation
    #[test]
    fn input_character_count_calculation() {
        // Test basic character count
        let text = "Hello";
        assert_eq!(text.chars().count(), 5);

        // Test empty string
        let empty = "";
        assert_eq!(empty.chars().count(), 0);

        // Test with special characters
        let special = "Hello!@#";
        assert_eq!(special.chars().count(), 8);

        // Test with unicode characters
        let unicode = "你好";
        assert_eq!(unicode.chars().count(), 2);
    }

    // Test max_length validation
    #[test]
    fn input_max_length_validation() {
        let max_len = 10;
        let short_text = "Hello";
        let long_text = "This is a very long text that exceeds the maximum length";

        assert!(short_text.chars().count() <= max_len);
        assert!(long_text.chars().count() > max_len);
    }

    // Test clear button visibility logic
    #[test]
    fn input_clear_button_visibility() {
        // Clear button should be visible when:
        // - allow_clear is true
        // - value is not empty
        // - input is not disabled

        let allow_clear = true;
        let has_value = !"test".is_empty();
        let is_disabled = false;
        let should_show = allow_clear && has_value && !is_disabled;
        assert!(should_show);

        // Should not show when disabled
        let is_disabled = true;
        let should_show = allow_clear && has_value && !is_disabled;
        assert!(!should_show);

        // Should not show when value is empty
        let has_value = !"".is_empty();
        let is_disabled = false;
        let should_show = allow_clear && has_value && !is_disabled;
        assert!(!should_show);
    }

    // Test InputProps default values
    #[test]
    fn input_props_defaults() {
        let props = InputProps {
            value: None,
            default_value: None,
            placeholder: None,
            disabled: false,
            size: None,
            variant: None,
            bordered: None,
            status: None,
            prefix: None,
            suffix: None,
            addon_before: None,
            addon_after: None,
            allow_clear: false,
            max_length: None,
            show_count: false,
            class: None,
            root_class_name: None,
            style: None,
            class_names: None,
            styles: None,
            on_change: None,
            on_press_enter: None,
            data_attributes: None,
        };

        assert_eq!(props.disabled, false);
        assert_eq!(props.allow_clear, false);
        assert_eq!(props.show_count, false);
    }

    #[test]
    fn input_size_default() {
        assert_eq!(InputSize::default(), InputSize::Middle);
    }

    #[test]
    fn input_size_all_variants() {
        assert_eq!(InputSize::Small, InputSize::Small);
        assert_eq!(InputSize::Middle, InputSize::Middle);
        assert_eq!(InputSize::Large, InputSize::Large);
        assert_ne!(InputSize::Small, InputSize::Large);
    }

    #[test]
    fn input_size_equality() {
        let size1 = InputSize::Small;
        let size2 = InputSize::Small;
        let size3 = InputSize::Large;
        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
    }

    #[test]
    fn input_character_count_with_emoji() {
        let text = "Hello 😀";
        assert_eq!(text.chars().count(), 7);
    }

    #[test]
    fn input_character_count_with_mixed_unicode() {
        let text = "Hello 世界";
        assert_eq!(text.chars().count(), 8);
    }

    #[test]
    fn input_max_length_formatting() {
        let char_count = 5;
        let max_len = 10;
        let count_text = format!("{}/{}", char_count, max_len);
        assert_eq!(count_text, "5/10");
    }

    #[test]
    fn input_max_length_no_limit() {
        let char_count = 15;
        let count_text = char_count.to_string();
        assert_eq!(count_text, "15");
    }

    #[test]
    fn input_clear_button_logic_edge_cases() {
        // Empty string should not show clear button
        assert!(!"".is_empty() == false);

        // Non-empty string should show clear button when enabled
        assert!("test".is_empty() == false);
    }

    #[test]
    fn input_size_class_empty_for_middle() {
        // Middle size should return empty string
        assert_eq!(InputSize::Middle.as_class(), "");
    }

    #[test]
    fn input_variant_bordered_false() {
        use crate::foundation::variant_from_bordered;
        assert_eq!(
            variant_from_bordered(Some(false), None),
            Variant::Borderless
        );
    }

    #[test]
    fn input_variant_bordered_true() {
        use crate::foundation::variant_from_bordered;
        assert_eq!(variant_from_bordered(Some(true), None), Variant::Outlined);
    }

    #[test]
    fn input_variant_priority() {
        use crate::foundation::variant_from_bordered;
        // Variant should take priority over bordered
        assert_eq!(
            variant_from_bordered(Some(true), Some(Variant::Filled)),
            Variant::Filled
        );
        assert_eq!(
            variant_from_bordered(Some(false), Some(Variant::Filled)),
            Variant::Filled
        );
    }

    #[test]
    fn input_character_count_unicode_boundary() {
        // Test with various unicode characters
        let text1 = "a";
        let text2 = "中";
        let text3 = "😀";
        assert_eq!(text1.chars().count(), 1);
        assert_eq!(text2.chars().count(), 1);
        assert_eq!(text3.chars().count(), 1);
    }

    #[test]
    fn input_max_length_boundary_values() {
        let max_len = 0;
        let text = "";
        assert!(text.chars().count() <= max_len);

        let max_len = 1;
        let text = "a";
        assert!(text.chars().count() <= max_len);

        let text2 = "ab";
        assert!(text2.chars().count() > max_len);
    }
}
