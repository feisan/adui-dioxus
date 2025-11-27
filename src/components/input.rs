use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::FormItemControlContext;
use crate::components::form::use_form_item_control;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::Key;
use dioxus::prelude::*;
use serde_json::Value;

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
    /// Optional status used to style the wrapper (success/warning/error).
    #[props(optional)]
    pub status: Option<ControlStatus>,
    /// Leading element rendered inside the affix wrapper.
    #[props(optional)]
    pub prefix: Option<Element>,
    /// Trailing element rendered inside the affix wrapper.
    #[props(optional)]
    pub suffix: Option<Element>,
    /// Whether to show a clear icon when there is content.
    #[props(default)]
    pub allow_clear: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Change event with the next string value.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Triggered when pressing Enter.
    #[props(optional)]
    pub on_press_enter: Option<EventHandler<()>>,
}

/// Ant Design flavored text input.
#[component]
pub fn Input(props: InputProps) -> Element {
    let InputProps {
        value,
        default_value,
        placeholder,
        disabled,
        status,
        prefix,
        suffix,
        allow_clear,
        class,
        style,
        on_change,
        on_press_enter,
    } = props;

    let placeholder_str = placeholder.unwrap_or_default();

    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();

    // Local state used only when not controlled by Form or external value.
    let initial_inner = default_value.clone().unwrap_or_default();
    let inner_value = use_signal(|| initial_inner);

    let current_value = resolve_current_value(&form_control, value.clone(), inner_value);
    let is_disabled = disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let has_prefix = prefix.is_some();
    let has_user_suffix = suffix.is_some();
    let has_clear = allow_clear && !current_value.is_empty() && !is_disabled;
    let has_any_suffix = has_user_suffix || has_clear;

    // Build shared handlers.
    let control_for_change = form_control.clone();
    let on_change_cb = on_change;
    let on_press_enter_cb = on_press_enter;
    let controlled_flag = controlled_by_prop;
    let mut inner_for_change = inner_value;

    let input_node = rsx! {
        input {
            class: "adui-input",
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
                if matches!(evt.key(), Key::Enter) {
                    if let Some(cb) = on_press_enter_cb {
                        cb.call(());
                    }
                }
            }
        }
    };

    if has_prefix || has_any_suffix {
        // Affix wrapper variant.
        let mut wrapper_classes = vec!["adui-input-affix-wrapper".to_string()];
        push_status_class(&mut wrapper_classes, status);
        if let Some(extra) = class {
            wrapper_classes.push(extra);
        }
        let wrapper_class_attr = wrapper_classes.join(" ");
        let style_attr = style.unwrap_or_default();

        // Separate signals for clear handler so we do not move earlier captures.
        let control_for_clear = form_control;
        let mut inner_for_clear = inner_value;
        let on_change_for_clear = on_change_cb;

        rsx! {
            div {
                class: "{wrapper_class_attr}",
                style: "{style_attr}",
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
                    }
                }
            }
        }
    } else {
        // Simple input variant.
        let mut class_list = vec!["adui-input".to_string()];
        push_status_class(&mut class_list, status);
        if let Some(extra) = class {
            class_list.push(extra);
        }
        let class_attr = class_list.join(" ");
        let style_attr = style.unwrap_or_default();

        rsx! {
            input {
                class: "{class_attr}",
                style: "{style_attr}",
                disabled: is_disabled,
                value: "{current_value}",
                placeholder: "{placeholder_str}",
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
                    if matches!(evt.key(), Key::Enter) {
                        if let Some(cb) = on_press_enter {
                            cb.call(());
                        }
                    }
                }
            }
        }
    }
}

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
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
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
        status,
        class,
        style,
        on_change,
    } = props;

    let placeholder_str = placeholder.unwrap_or_default();

    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();
    let initial_inner = default_value.clone().unwrap_or_default();
    let inner_value = use_signal(|| initial_inner);

    let current_value = resolve_current_value(&form_control, value.clone(), inner_value);
    let is_disabled = disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());
    let line_rows = rows.unwrap_or(3);

    let mut class_list = vec!["adui-input".to_string(), "adui-input-textarea".to_string()];
    push_status_class(&mut class_list, status);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let control_for_change = form_control;
    let mut inner_for_change = inner_value;
    let on_change_cb = on_change;

    rsx! {
        textarea {
            class: "{class_attr}",
            style: "{style_attr}",
            disabled: is_disabled,
            rows: "{line_rows}",
            value: "{current_value}",
            placeholder: "{placeholder_str}",
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

fn form_value_to_string(val: Option<Value>) -> String {
    match val {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s,
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => {
            if b {
                "true".into()
            } else {
                "false".into()
            }
        }
        Some(Value::Array(_)) | Some(Value::Object(_)) => String::new(),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_value_to_string_handles_basic_types() {
        assert_eq!(form_value_to_string(None), "");
        assert_eq!(form_value_to_string(Some(Value::Null)), "");
        assert_eq!(
            form_value_to_string(Some(Value::String("abc".into()))),
            "abc"
        );
        assert_eq!(form_value_to_string(Some(Value::Number(42.into()))), "42");
        assert_eq!(form_value_to_string(Some(Value::Bool(true))), "true");
        assert_eq!(form_value_to_string(Some(Value::Bool(false))), "false");
    }
}
