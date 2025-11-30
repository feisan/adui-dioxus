use crate::components::config_provider::use_config;
use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::{FormItemControlContext, use_form_item_control};
use crate::components::number_utils::{
    NumberRules, apply_step, parse_and_normalize, round_with_precision,
};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::{Key, *};
use serde_json::{Number, Value};
/// Numeric input with step controls and basic formatting.
#[derive(Props, Clone, PartialEq)]
pub struct InputNumberProps {
    /// Controlled value. When set, internal state is ignored.
    #[props(optional)]
    pub value: Option<f64>,
    /// Uncontrolled initial value.
    #[props(optional)]
    pub default_value: Option<f64>,
    #[props(optional)]
    pub min: Option<f64>,
    #[props(optional)]
    pub max: Option<f64>,
    #[props(optional)]
    pub step: Option<f64>,
    #[props(optional)]
    pub precision: Option<u32>,
    #[props(default = true)]
    pub controls: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub prefix: Option<Element>,
    #[props(optional)]
    pub suffix: Option<Element>,
    #[props(default)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Fired whenever the numeric value changes. `None` means empty.
    #[props(optional)]
    pub on_change: Option<EventHandler<Option<f64>>>,
    /// Fired on blur or Enter to indicate the current committed value.
    #[props(optional)]
    pub on_change_complete: Option<EventHandler<Option<f64>>>,
}

#[component]
pub fn InputNumber(props: InputNumberProps) -> Element {
    let InputNumberProps {
        value,
        default_value,
        min,
        max,
        step,
        precision,
        controls,
        disabled,
        status,
        prefix,
        suffix,
        class,
        style,
        on_change,
        on_change_complete,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();

    let rules = NumberRules {
        min,
        max,
        step,
        precision,
    };

    // Local storage for uncontrolled usage.
    let inner_value = use_signal(|| default_value);
    let draft = {
        let initial = format_value(
            resolve_current_value(value, &form_control, inner_value.clone()),
            precision,
        );
        use_signal(|| initial)
    };

    // Keep the displayed text in sync when external value/form changes.
    {
        let mut draft_signal = draft.clone();
        let inner_value_signal = inner_value.clone();
        let form_control_ctx = form_control.clone();
        use_effect(move || {
            let current =
                resolve_current_value(value, &form_control_ctx, inner_value_signal.clone());
            draft_signal.set(format_value(current, precision));
        });
    }

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let mut classes = vec!["adui-input-number".to_string()];
    if is_disabled {
        classes.push("adui-input-number-disabled".into());
    }
    push_status_class(&mut classes, status);
    if let Some(extra) = class {
        classes.push(extra);
    }
    let class_attr = classes.join(" ");
    let style_attr = style.unwrap_or_default();

    let mut draft_for_input = draft.clone();
    let draft_for_blur = draft.clone();
    let draft_for_keys = draft.clone();
    let on_change_complete_cb = on_change_complete.clone();

    let form_for_input = form_control.clone();
    let inner_for_input = inner_value.clone();
    let on_change_for_input = on_change.clone();

    let form_for_keys = form_control.clone();
    let inner_for_keys = inner_value.clone();
    let on_change_for_keys = on_change.clone();

    let form_for_blur = form_control.clone();
    let inner_for_blur = inner_value.clone();
    let on_change_for_blur = on_change.clone();

    let form_for_up = form_control.clone();
    let inner_for_up = inner_value.clone();
    let on_change_for_up = on_change.clone();
    let draft_for_up = draft.clone();

    let form_for_down = form_control.clone();
    let inner_for_down = inner_value.clone();
    let on_change_for_down = on_change.clone();
    let draft_for_down = draft.clone();

    let input_value = draft.read().clone();

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if let Some(icon) = prefix {
                span { class: "adui-input-number-prefix", {icon} }
            }
            input {
                class: "adui-input-number-input",
                r#type: "text",
                inputmode: "decimal",
                disabled: is_disabled,
                value: "{input_value}",
                oninput: move |evt| {
                    let text = evt.value();
                    *draft_for_input.write() = text.clone();

                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        apply_value(
                            None,
                            false,
                            controlled_by_prop,
                            inner_for_input.clone(),
                            &form_for_input,
                            precision,
                            draft_for_input.clone(),
                            &on_change_for_input,
                        );
                        return;
                    }

                    if let Some(parsed) = parse_and_normalize(trimmed, &rules) {
                        apply_value(
                            Some(parsed),
                            false,
                            controlled_by_prop,
                            inner_for_input.clone(),
                            &form_for_input,
                            precision,
                            draft_for_input.clone(),
                            &on_change_for_input,
                        );
                    }
                },
                onkeydown: move |evt: KeyboardEvent| {
                    match evt.key() {
                        Key::ArrowUp => {
                            let base = resolve_current_value(value, &form_for_keys, inner_for_keys.clone())
                                .or(rules.min)
                                .unwrap_or(0.0);
                            let next = apply_step(base, 1, &rules);
                            apply_value(
                                Some(next),
                                true,
                                controlled_by_prop,
                                inner_for_keys.clone(),
                                &form_for_keys,
                                precision,
                                draft_for_keys.clone(),
                                &on_change_for_keys,
                            );
                        }
                        Key::ArrowDown => {
                            let base = resolve_current_value(value, &form_for_keys, inner_for_keys.clone())
                                .or(rules.min)
                                .unwrap_or(0.0);
                            let next = apply_step(base, -1, &rules);
                            apply_value(
                                Some(next),
                                true,
                                controlled_by_prop,
                                inner_for_keys.clone(),
                                &form_for_keys,
                                precision,
                                draft_for_keys.clone(),
                                &on_change_for_keys,
                            );
                        }
                        Key::Enter => {
                            let current_text = draft.read().clone();
                            let normalized = if current_text.trim().is_empty() {
                                None
                            } else {
                                parse_and_normalize(&current_text, &rules)
                            };
                            apply_value(
                                normalized,
                                true,
                                controlled_by_prop,
                                inner_for_keys.clone(),
                                &form_for_keys,
                                precision,
                                draft_for_keys.clone(),
                                &on_change_for_keys,
                            );
                            if let Some(cb) = on_change_complete_cb.as_ref() {
                                cb.call(normalized);
                            }
                        }
                        _ => {}
                    }
                },
                onblur: move |_| {
                    let current_text = draft_for_blur.read().clone();
                    let normalized = if current_text.trim().is_empty() {
                        None
                    } else {
                        parse_and_normalize(&current_text, &rules)
                    };
                    apply_value(
                        normalized,
                        true,
                        controlled_by_prop,
                        inner_for_blur.clone(),
                        &form_for_blur,
                        precision,
                        draft_for_blur.clone(),
                        &on_change_for_blur,
                    );
                    if let Some(cb) = on_change_complete_cb.as_ref() {
                        cb.call(normalized);
                    }
                },
            }
            if let Some(icon) = suffix {
                span { class: "adui-input-number-suffix", {icon} }
            }
            if controls {
                div { class: "adui-input-number-handlers",
                    button {
                        class: "adui-input-number-handler adui-input-number-handler-up",
                        disabled: is_disabled,
                        onclick: move |_| {
                            let base = resolve_current_value(value, &form_for_up, inner_for_up.clone())
                                .or(rules.min)
                                .unwrap_or(0.0);
                            let next = apply_step(base, 1, &rules);
                            apply_value(
                                Some(next),
                                true,
                                controlled_by_prop,
                                inner_for_up.clone(),
                                &form_for_up,
                                precision,
                                draft_for_up.clone(),
                                &on_change_for_up,
                            );
                        },
                        "▲"
                    }
                    button {
                        class: "adui-input-number-handler adui-input-number-handler-down",
                        disabled: is_disabled,
                        onclick: move |_| {
                            let base = resolve_current_value(value, &form_for_down, inner_for_down.clone())
                                .or(rules.min)
                                .unwrap_or(0.0);
                            let next = apply_step(base, -1, &rules);
                            apply_value(
                                Some(next),
                                true,
                                controlled_by_prop,
                                inner_for_down.clone(),
                                &form_for_down,
                                precision,
                                draft_for_down.clone(),
                                &on_change_for_down,
                            );
                        },
                        "▼"
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_value(
    next: Option<f64>,
    normalize_display: bool,
    controlled_by_prop: bool,
    inner: Signal<Option<f64>>,
    form_control: &Option<FormItemControlContext>,
    precision: Option<u32>,
    draft: Signal<String>,
    on_change: &Option<EventHandler<Option<f64>>>,
) {
    if let Some(ctx) = form_control.as_ref() {
        let value_to_set = match next.and_then(Number::from_f64) {
            Some(num) => Value::Number(num),
            None => Value::Null,
        };
        ctx.set_value(value_to_set);
    } else if !controlled_by_prop {
        let mut state = inner;
        state.set(next);
    }

    if let Some(cb) = on_change.as_ref() {
        cb.call(next);
    }

    if normalize_display {
        let mut d = draft;
        d.set(format_value(next, precision));
    }
}

fn format_value(value: Option<f64>, precision: Option<u32>) -> String {
    match value {
        None => String::new(),
        Some(v) => {
            if let Some(p) = precision {
                format!("{:.*}", p as usize, round_with_precision(v, Some(p)))
            } else {
                let mut text = v.to_string();
                if text.contains('.') {
                    while text.ends_with('0') {
                        text.pop();
                    }
                    if text.ends_with('.') {
                        text.pop();
                    }
                }
                text
            }
        }
    }
}

fn resolve_current_value(
    value: Option<f64>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    inner: Signal<Option<f64>>,
) -> Option<f64> {
    value
        .or_else(|| {
            form_control
                .as_ref()
                .and_then(|ctx| value_from_form(ctx.value()))
        })
        .or_else(|| *inner.read())
}

fn value_from_form(val: Option<Value>) -> Option<f64> {
    match val {
        Some(Value::Number(n)) => n.as_f64(),
        Some(Value::String(s)) => s.parse::<f64>().ok(),
        Some(Value::Bool(b)) => Some(if b { 1.0 } else { 0.0 }),
        _ => None,
    }
}
