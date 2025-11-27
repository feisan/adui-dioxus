use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::{
    form_value_to_bool, form_value_to_string_vec, use_form_item_control,
};
use dioxus::prelude::*;
use serde_json::Value;

#[derive(Clone)]
struct CheckboxGroupContext {
    selected: Signal<Vec<String>>,
    disabled: bool,
    controlled: bool,
    on_change: Option<EventHandler<Vec<String>>>,
}

/// Props for a single checkbox.
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Controlled checked state.
    #[props(optional)]
    pub checked: Option<bool>,
    /// Initial state in uncontrolled mode.
    #[props(default)]
    pub default_checked: bool,
    /// Visual indeterminate state (mainly for \"select all\" scenarios).
    #[props(default)]
    pub indeterminate: bool,
    #[props(default)]
    pub disabled: bool,
    /// Value used when inside a CheckboxGroup.
    #[props(optional)]
    pub value: Option<String>,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub on_change: Option<EventHandler<bool>>,
    /// Optional label content rendered to the right of the box.
    #[props(optional)]
    pub children: Element,
}

/// Ant Design flavored checkbox.
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let CheckboxProps {
        checked,
        default_checked,
        indeterminate,
        disabled,
        value,
        status,
        class,
        style,
        on_change,
        children,
    } = props;

    let form_control = use_form_item_control();
    let group_ctx = try_use_context::<CheckboxGroupContext>();

    let controlled_by_prop = checked.is_some();
    let inner_checked = use_signal(|| default_checked);

    let is_disabled = disabled
        || group_ctx.as_ref().is_some_and(|ctx| ctx.disabled)
        || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let is_checked = resolve_checked(
        &group_ctx,
        &form_control,
        checked,
        value.as_deref(),
        inner_checked,
    );

    let mut class_list = vec!["adui-checkbox".to_string()];
    if is_checked {
        class_list.push("adui-checkbox-checked".into());
    }
    if indeterminate && !is_checked {
        class_list.push("adui-checkbox-indeterminate".into());
    }
    if is_disabled {
        class_list.push("adui-checkbox-disabled".into());
    }
    push_status_class(&mut class_list, status);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    rsx! {
        label {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "checkbox",
            "aria-checked": is_checked,
            "aria-disabled": is_disabled,
            input {
                class: "adui-checkbox-input",
                r#type: "checkbox",
                checked: is_checked,
                disabled: is_disabled,
                onclick: {
                    let group_for_click = group_ctx.clone();
                    let form_for_click = form_control.clone();
                    let value_for_click = value.clone();
                    let on_change_cb = on_change;
                    let mut inner_for_click = inner_checked;
                    move |_| {
                        if is_disabled {
                            return;
                        }
                        handle_checkbox_toggle(
                            &group_for_click,
                            &form_for_click,
                            controlled_by_prop,
                            &mut inner_for_click,
                            value_for_click.as_deref(),
                            on_change_cb,
                        );
                    }
                },
            }
            span { class: "adui-checkbox-inner" }
            span { {children} }
        }
    }
}

fn resolve_checked(
    group_ctx: &Option<CheckboxGroupContext>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    prop_checked: Option<bool>,
    value: Option<&str>,
    inner: Signal<bool>,
) -> bool {
    if let Some(group) = group_ctx {
        if let Some(val) = value {
            return group.selected.read().contains(&val.to_string());
        }
    }
    if let Some(ctx) = form_control {
        return form_value_to_bool(ctx.value(), false);
    }
    if let Some(c) = prop_checked {
        return c;
    }
    *inner.read()
}

fn handle_checkbox_toggle(
    group_ctx: &Option<CheckboxGroupContext>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    controlled_by_prop: bool,
    inner: &mut Signal<bool>,
    value: Option<&str>,
    on_change: Option<EventHandler<bool>>,
) {
    // Group mode: toggle membership in selected set.
    if let Some(group) = group_ctx {
        if let Some(val) = value {
            // Limit the scope of the read borrow so we can safely write later.
            let next = {
                let current = group.selected.read();
                toggle_membership(&current, val)
            };
            if let Some(cb) = group.on_change {
                cb.call(next.clone());
            }
            if let Some(ctx) = form_control {
                let json = Value::Array(next.iter().cloned().map(Value::String).collect());
                ctx.set_value(json);
            }
            if !group.controlled {
                let mut signal = group.selected;
                signal.set(next);
            }
        }
        // Group mode does not call individual on_change; the group callback is primary.
        return;
    }

    // Simple checkbox: Form 模式下以 FormStore 为真相源，其他情况使用内部 state。
    let next = if let Some(ctx) = form_control {
        let current = form_value_to_bool(ctx.value(), false);
        let next = !current;
        ctx.set_value(Value::Bool(next));
        next
    } else {
        let current = *inner.read();
        let next = !current;
        if !controlled_by_prop {
            let mut state = *inner;
            state.set(next);
        }
        next
    };

    if let Some(cb) = on_change {
        cb.call(next);
    }
}

/// Toggle a single value within a set of string values.
/// If the value already exists it is removed, otherwise it is appended.
fn toggle_membership(current: &[String], value: &str) -> Vec<String> {
    let mut next = current.to_vec();
    if let Some(pos) = next.iter().position(|v| v == value) {
        next.remove(pos);
    } else {
        next.push(value.to_string());
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_membership_adds_when_missing_and_removes_when_present() {
        let current: Vec<String> = vec![];
        let next = toggle_membership(&current, "a");
        assert_eq!(next, vec!["a".to_string()]);

        let next2 = toggle_membership(&next, "a");
        assert!(next2.is_empty());
    }
}

/// Checkbox group props.
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxGroupProps {
    /// Controlled set of selected values.
    #[props(optional)]
    pub value: Option<Vec<String>>,
    /// Initial selection in uncontrolled mode.
    #[props(default)]
    pub default_value: Vec<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub on_change: Option<EventHandler<Vec<String>>>,
    pub children: Element,
}

/// Group container for multiple checkboxes.
#[component]
pub fn CheckboxGroup(props: CheckboxGroupProps) -> Element {
    let CheckboxGroupProps {
        value,
        default_value,
        disabled,
        class,
        style,
        on_change,
        children,
    } = props;

    let form_control = crate::components::form::use_form_item_control();

    let controlled = value.is_some();
    let selected = use_signal(|| {
        if let Some(external) = value.clone() {
            external
        } else if let Some(ctx) = form_control.as_ref() {
            form_value_to_string_vec(ctx.value())
        } else {
            default_value.clone()
        }
    });

    // Sync internal signal when controlled value or form value changes.
    {
        let mut selected_signal = selected;
        let external = value.clone();
        let form_ctx = form_control.clone();
        use_effect(move || {
            if let Some(external_value) = external.clone() {
                selected_signal.set(external_value);
            } else if let Some(ctx) = form_ctx.as_ref() {
                let next = form_value_to_string_vec(ctx.value());
                selected_signal.set(next);
            }
        });
    }

    let ctx = CheckboxGroupContext {
        selected,
        disabled,
        controlled,
        on_change,
    };
    use_context_provider(|| ctx);

    let mut class_list = vec!["adui-checkbox-group".to_string()];
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
