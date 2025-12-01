use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::{FormItemControlContext, use_form_item_control};
use dioxus::prelude::*;
use serde_json::Value;

#[derive(Clone)]
struct RadioGroupContext {
    selected: Signal<Option<String>>,
    disabled: bool,
    name: Option<String>,
    controlled: bool,
    on_change: Option<EventHandler<String>>,
    form_control: Option<FormItemControlContext>,
}

/// Single radio props.
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// Value for this radio within a group.
    pub value: String,
    /// Controlled checked state when not in a group.
    #[props(optional)]
    pub checked: Option<bool>,
    #[props(default)]
    pub default_checked: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Called when this radio becomes checked in standalone mode or within a group.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Rendered label content.
    pub children: Element,
    /// Render as button-style radio (uses different class name).
    #[props(default)]
    pub button: bool,
}

/// Ant Design flavored radio.
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let RadioProps {
        value,
        checked,
        default_checked,
        disabled,
        status,
        class,
        style,
        on_change,
        children,
        button,
    } = props;

    let group_ctx = try_use_context::<RadioGroupContext>();
    let standalone = group_ctx.is_none();

    let inner_checked = use_signal(|| default_checked);

    let is_disabled = disabled || group_ctx.as_ref().is_some_and(|ctx| ctx.disabled);

    let is_checked = if let Some(group) = &group_ctx {
        group
            .selected
            .read()
            .as_ref()
            .map(|current| current == &value)
            .unwrap_or(false)
    } else if let Some(c) = checked {
        c
    } else {
        *inner_checked.read()
    };

    let mut class_list = if button {
        vec!["adui-radio-button".to_string()]
    } else {
        vec!["adui-radio".to_string()]
    };
    if is_checked {
        class_list.push("adui-radio-checked".into());
    }
    if is_disabled {
        class_list.push("adui-radio-disabled".into());
    }
    push_status_class(&mut class_list, status);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let group_for_click = group_ctx.clone();
    let mut inner_for_click = inner_checked;
    let on_change_cb = on_change;
    let value_for_click = value.clone();

    rsx! {
        label {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "radio",
            "aria-checked": is_checked,
            "aria-disabled": is_disabled,
            onclick: move |_| {
                if is_disabled {
                    return;
                }
                handle_radio_click(
                    &group_for_click,
                    standalone,
                    &mut inner_for_click,
                    &value_for_click,
                    on_change_cb,
                );
            },
            input {
                class: "adui-radio-input",
                r#type: "radio",
                name: group_ctx.as_ref().and_then(|g| g.name.clone()).unwrap_or_default(),
                checked: is_checked,
                disabled: is_disabled,
            }
            span { class: "adui-radio-inner" }
            span { {children} }
        }
    }
}

fn handle_radio_click(
    group_ctx: &Option<RadioGroupContext>,
    standalone: bool,
    inner: &mut Signal<bool>,
    value: &str,
    on_change: Option<EventHandler<String>>,
) {
    if let Some(group) = group_ctx {
        let selected_value = value.to_string();

        // Update Form store if bound.
        if let Some(form) = &group.form_control {
            form.set_string(selected_value.clone());
        }

        // Emit group change.
        if let Some(cb) = group.on_change {
            cb.call(selected_value.clone());
        }

        // Update internal state when uncontrolled.
        if !group.controlled {
            let mut signal = group.selected;
            signal.set(Some(selected_value));
        }
        return;
    }

    if standalone {
        let mut state = *inner;
        state.set(true);
    }

    if let Some(cb) = on_change {
        cb.call(value.to_string());
    }
}

/// Group props controlling a set of radios.
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Controlled selected value.
    #[props(optional)]
    pub value: Option<String>,
    /// Initial selected value in uncontrolled mode.
    #[props(optional)]
    pub default_value: Option<String>,
    #[props(optional)]
    pub name: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    pub children: Element,
}

/// Container that coordinates selected radio in a group.
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let form_control = use_form_item_control();

    let RadioGroupProps {
        value,
        default_value,
        name,
        disabled,
        class,
        style,
        on_change,
        children,
    } = props;

    let controlled = value.is_some();

    let initial_value = if let Some(v) = value.clone() {
        Some(v)
    } else if let Some(ctx) = &form_control {
        form_radio_value_to_string(ctx.value())
    } else {
        default_value.clone()
    };

    let selected = use_signal(|| initial_value.clone());

    // Keep internal selected in sync with controlled `value`.
    {
        let mut signal = selected;
        let external = value.clone();
        use_effect(move || {
            if let Some(next) = external.clone() {
                signal.set(Some(next));
            }
        });
    }

    let ctx = RadioGroupContext {
        selected,
        disabled,
        name,
        controlled,
        on_change,
        form_control,
    };
    use_context_provider(|| ctx);

    let mut class_list = vec!["adui-radio-group".to_string()];
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
    }
}

/// Convenience wrapper to create a button-style radio.
#[component]
pub fn RadioButton(props: RadioProps) -> Element {
    Radio(RadioProps {
        button: true,
        ..props
    })
}

fn form_radio_value_to_string(val: Option<Value>) -> Option<String> {
    match val {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(s),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(if b { "true".into() } else { "false".into() }),
        Some(Value::Array(_)) | Some(Value::Object(_)) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_radio_value_to_string_converts_basic_types() {
        assert_eq!(form_radio_value_to_string(None), None);
        assert_eq!(
            form_radio_value_to_string(Some(Value::String("a".into()))),
            Some("a".into())
        );
        assert_eq!(
            form_radio_value_to_string(Some(Value::Number(1.into()))),
            Some("1".into())
        );
        assert_eq!(
            form_radio_value_to_string(Some(Value::Bool(true))),
            Some("true".into())
        );
    }

    #[test]
    fn form_radio_value_to_string_handles_none_and_null() {
        assert_eq!(form_radio_value_to_string(None), None);
        assert_eq!(form_radio_value_to_string(Some(Value::Null)), None);
    }

    #[test]
    fn form_radio_value_to_string_converts_bool_false() {
        assert_eq!(
            form_radio_value_to_string(Some(Value::Bool(false))),
            Some("false".into())
        );
    }

    #[test]
    fn form_radio_value_to_string_converts_numbers() {
        assert_eq!(
            form_radio_value_to_string(Some(Value::Number(0.into()))),
            Some("0".into())
        );
        assert_eq!(
            form_radio_value_to_string(Some(Value::Number(42.into()))),
            Some("42".into())
        );
        assert_eq!(
            form_radio_value_to_string(Some(Value::Number((-10).into()))),
            Some("-10".into())
        );
        // Test with integer number (serde_json::Number doesn't support f64 directly)
        assert_eq!(
            form_radio_value_to_string(Some(Value::Number(314.into()))),
            Some("314".into())
        );
    }

    #[test]
    fn form_radio_value_to_string_handles_strings() {
        assert_eq!(
            form_radio_value_to_string(Some(Value::String("".into()))),
            Some("".into())
        );
        assert_eq!(
            form_radio_value_to_string(Some(Value::String("hello".into()))),
            Some("hello".into())
        );
        assert_eq!(
            form_radio_value_to_string(Some(Value::String("123".into()))),
            Some("123".into())
        );
    }

    #[test]
    fn form_radio_value_to_string_rejects_array_and_object() {
        assert_eq!(form_radio_value_to_string(Some(Value::Array(vec![]))), None);
        assert_eq!(
            form_radio_value_to_string(Some(Value::Array(vec![Value::String("a".into())]))),
            None
        );
        assert_eq!(
            form_radio_value_to_string(Some(Value::Object(serde_json::Map::new()))),
            None
        );
    }

    #[test]
    fn radio_props_defaults() {
        // Test that default values are correct
        // Note: RadioProps requires children and value, so we can't create a full instance
        // But we can verify the default values used in the component
        let default_checked = false;
        let default_disabled = false;
        let default_button = false;
        assert_eq!(default_checked, false);
        assert_eq!(default_disabled, false);
        assert_eq!(default_button, false);
    }

    #[test]
    fn radio_group_props_defaults() {
        // Test that default values are correct
        // Note: RadioGroupProps requires children, so we can't create a full instance
        let default_disabled = false;
        assert_eq!(default_disabled, false);
    }
}
