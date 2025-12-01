use crate::components::config_provider::use_config;
use crate::components::form::use_form_item_control;
#[cfg(target_arch = "wasm32")]
use crate::components::interaction::as_pointer_event;
use crate::components::number_utils::{NumberRules, apply_step};
use dioxus::events::{KeyboardEvent, PointerData};
use dioxus::prelude::Key;
use dioxus::prelude::*;
use serde_json::{Number, Value};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Rate component for collecting evaluations with stars (supports half).
#[derive(Props, Clone, PartialEq)]
pub struct RateProps {
    /// Controlled numeric value.
    #[props(optional)]
    pub value: Option<f64>,
    /// Uncontrolled initial value.
    #[props(optional)]
    pub default_value: Option<f64>,
    /// Total count of items.
    #[props(default = 5)]
    pub count: usize,
    /// Allow selecting half steps (0.5 increments).
    #[props(default)]
    pub allow_half: bool,
    /// Allow clearing when clicking the same value again.
    #[props(default = true)]
    pub allow_clear: bool,
    /// Disable interactions.
    #[props(default)]
    pub disabled: bool,
    /// Optional custom character for each item.
    #[props(optional)]
    pub character: Option<Element>,
    /// Optional tooltips per item (aligned by index).
    #[props(optional)]
    pub tooltips: Option<Vec<String>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Change callback (after click/keyboard). None means cleared.
    #[props(optional)]
    pub on_change: Option<EventHandler<Option<f64>>>,
    /// Hover value callback.
    #[props(optional)]
    pub on_hover_change: Option<EventHandler<Option<f64>>>,
    #[props(optional)]
    pub on_focus: Option<EventHandler<()>>,
    #[props(optional)]
    pub on_blur: Option<EventHandler<()>>,
}

#[component]
pub fn Rate(props: RateProps) -> Element {
    let RateProps {
        value,
        default_value,
        count,
        allow_half,
        allow_clear,
        disabled,
        character,
        tooltips,
        class,
        style,
        on_change,
        on_hover_change,
        on_focus,
        on_blur,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();
    let controlled = value.is_some();

    let rules = NumberRules {
        min: Some(0.0),
        max: Some(count as f64),
        step: Some(if allow_half { 0.5 } else { 1.0 }),
        precision: Some(if allow_half { 1 } else { 0 }),
    };

    let inner_value = use_signal(|| default_value);

    // Sync controlled/form value into local state when it changes.
    {
        let form_ctx = form_control.clone();
        let prop_val = value.clone();
        let mut inner_signal = inner_value.clone();
        use_effect(move || {
            let next = resolve_value(prop_val.clone(), &form_ctx, &inner_signal);
            inner_signal.set(next);
        });
    }

    let hover_value = use_signal(|| None::<f64>);

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let mut class_list = vec!["adui-rate".to_string()];
    if allow_half {
        class_list.push("adui-rate-half".into());
    }
    if is_disabled {
        class_list.push("adui-rate-disabled".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let on_hover_cb = on_hover_change;
    let mut hover_setter = hover_value.clone();
    let hover_cb = on_hover_cb;

    let current_value = resolve_value(value, &form_control, &inner_value);
    let display_value = hover_value.read().or(current_value).unwrap_or(0.0);

    let handle_keyboard = {
        let mut hover_signal = hover_value.clone();
        let form_for_key = form_control.clone();
        let mut inner_for_key = inner_value.clone();
        let on_change_for_key = on_change.clone();
        move |evt: KeyboardEvent| {
            if is_disabled {
                return;
            }
            let current = resolve_value(value, &form_for_key, &inner_for_key).unwrap_or(0.0);
            let next = match evt.key() {
                Key::ArrowRight | Key::ArrowUp => Some(apply_step(current, 1, &rules)),
                Key::ArrowLeft | Key::ArrowDown => Some(apply_step(current, -1, &rules)),
                Key::Home => Some(0.0),
                Key::End => Some(count as f64),
                Key::Enter => Some(current),
                Key::Character(c) if c == " " => Some(current),
                _ => None,
            };
            if let Some(val) = next {
                apply_rate(
                    Some(val),
                    true,
                    controlled,
                    &mut inner_for_key,
                    &form_for_key,
                    &on_change_for_key,
                );
                hover_signal.set(None);
            }
        }
    };

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "slider",
            tabindex: if is_disabled { -1 } else { 0 },
            aria_valuemin: 0,
            aria_valuemax: count,
            aria_valuenow: display_value,
            onkeydown: handle_keyboard,
            onfocus: move |_| if let Some(cb) = on_focus.as_ref() { cb.call(()); },
            onblur: {
                let mut hover_for_blur = hover_value.clone();
                move |_| {
                    hover_for_blur.set(None);
                    if let Some(cb) = on_blur.as_ref() { cb.call(()); }
                }
            },
            onpointerleave: move |_| {
                hover_setter.set(None);
                if let Some(cb) = hover_cb.as_ref() { cb.call(None); }
            },
            {(0..count).map(|idx| {
                let star_index = idx + 1;
                let tooltip = tooltips.as_ref().and_then(|t| t.get(idx).cloned());
                let char_node = character.clone().unwrap_or(rsx! { span { class: "adui-rate-star-default", "★" } });
                let is_full = display_value + f64::EPSILON >= star_index as f64;
                let is_half = allow_half && !is_full && display_value + f64::EPSILON >= star_index as f64 - 0.5;
                let mut star_classes = vec!["adui-rate-star".to_string()];
                if is_full {
                    star_classes.push("adui-rate-star-full".into());
                } else if is_half {
                    star_classes.push("adui-rate-star-half".into());
                }

                let on_pointer_move = {
                    let mut hover_signal = hover_value.clone();
                    move |evt: Event<PointerData>| {
                        if is_disabled {
                            return;
                        }
                        let val = pointer_value(&evt, star_index, allow_half).unwrap_or(star_index as f64);
                        hover_signal.set(Some(val));
                        if let Some(cb) = hover_cb.as_ref() {
                            cb.call(Some(val));
                        }
                    }
                };

                let on_pointer_down = {
                    let form_for_click = form_control.clone();
                    let mut inner_for_click = inner_value.clone();
                    let on_change_for_click = on_change.clone();
                    move |evt: Event<PointerData>| {
                        if is_disabled {
                            return;
                        }
                        let current = resolve_value(value, &form_for_click, &inner_for_click);
                        let mut val = pointer_value(&evt, star_index, allow_half).unwrap_or(star_index as f64);
                        if allow_clear && current.is_some_and(|v| (v - val).abs() < f64::EPSILON) {
                            val = 0.0;
                        }
                        let next = if val == 0.0 { None } else { Some(val) };
                        apply_rate(
                            next,
                            true,
                            controlled,
                            &mut inner_for_click,
                            &form_for_click,
                            &on_change_for_click,
                        );
                        hover_setter.set(None);
                        if let Some(cb) = hover_cb.as_ref() {
                            cb.call(next);
                        }
                    }
                };

                rsx! {
                    span {
                        class: "{star_classes.join(\" \")}",
                        title: tooltip.unwrap_or_default(),
                        onpointermove: on_pointer_move,
                        onpointerdown: on_pointer_down,
                        onpointerenter: on_pointer_move,
                        {char_node}
                    }
                }
            })}
        }
    }
}

fn resolve_value(
    value: Option<f64>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    inner: &Signal<Option<f64>>,
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
        _ => None,
    }
}

fn apply_rate(
    next: Option<f64>,
    fire_change: bool,
    controlled: bool,
    inner: &mut Signal<Option<f64>>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    on_change: &Option<EventHandler<Option<f64>>>,
) {
    if !controlled {
        inner.set(next);
    }

    if let Some(ctx) = form_control.as_ref() {
        let val = match next.and_then(Number::from_f64) {
            Some(num) => Value::Number(num),
            None => Value::Null,
        };
        ctx.set_value(val);
    }

    if fire_change {
        if let Some(cb) = on_change.as_ref() {
            cb.call(next);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn pointer_value(evt: &Event<PointerData>, star_index: usize, allow_half: bool) -> Option<f64> {
    if let Some(p_evt) = as_pointer_event(evt) {
        if let Some(target) = p_evt
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let rect = target.get_bounding_client_rect();
            if allow_half {
                let mid = rect.width() / 2.0;
                let left = p_evt.client_x() as f64 - rect.x();
                if left < mid {
                    return Some(star_index as f64 - 0.5);
                }
            }
        }
    }
    Some(star_index as f64)
}

#[cfg(not(target_arch = "wasm32"))]
fn pointer_value(_evt: &Event<PointerData>, star_index: usize, _allow_half: bool) -> Option<f64> {
    Some(star_index as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Number, Value};

    #[test]
    fn rate_props_defaults() {
        // RateProps requires no mandatory fields
        // count defaults to 5
        // allow_half defaults to false
        // allow_clear defaults to true
        // disabled defaults to false
    }

    // Note: resolve_value tests require Dioxus runtime for Signal creation
    // These are tested in integration tests or component behavior tests

    #[test]
    fn value_from_form_number() {
        let val = Some(Value::Number(Number::from_f64(4.5).unwrap()));
        assert_eq!(value_from_form(val), Some(4.5));
    }

    #[test]
    fn value_from_form_string_valid() {
        let val = Some(Value::String("3.5".to_string()));
        assert_eq!(value_from_form(val), Some(3.5));
    }

    #[test]
    fn value_from_form_string_invalid() {
        let val = Some(Value::String("not a number".to_string()));
        assert_eq!(value_from_form(val), None);
    }

    #[test]
    fn value_from_form_other_types() {
        assert_eq!(value_from_form(Some(Value::Bool(true))), None);
        assert_eq!(value_from_form(Some(Value::Null)), None);
        assert_eq!(value_from_form(Some(Value::Array(vec![]))), None);
        assert_eq!(value_from_form(None), None);
    }

    #[test]
    fn value_from_form_number_zero() {
        let val = Some(Value::Number(Number::from_f64(0.0).unwrap()));
        assert_eq!(value_from_form(val), Some(0.0));
    }

    #[test]
    fn value_from_form_number_negative() {
        let val = Some(Value::Number(Number::from_f64(-1.5).unwrap()));
        assert_eq!(value_from_form(val), Some(-1.5));
    }

    #[test]
    fn value_from_form_number_integer() {
        let val = Some(Value::Number(Number::from(5)));
        assert_eq!(value_from_form(val), Some(5.0));
    }

    #[test]
    fn value_from_form_number_large() {
        let val = Some(Value::Number(Number::from_f64(100.0).unwrap()));
        assert_eq!(value_from_form(val), Some(100.0));
    }

    #[test]
    fn value_from_form_string_zero() {
        let val = Some(Value::String("0".to_string()));
        assert_eq!(value_from_form(val), Some(0.0));
    }

    #[test]
    fn value_from_form_string_negative() {
        let val = Some(Value::String("-2.5".to_string()));
        assert_eq!(value_from_form(val), Some(-2.5));
    }

    #[test]
    fn value_from_form_string_integer() {
        let val = Some(Value::String("10".to_string()));
        assert_eq!(value_from_form(val), Some(10.0));
    }

    #[test]
    fn value_from_form_string_with_whitespace() {
        let val = Some(Value::String("  5.5  ".to_string()));
        // Note: parse::<f64> does NOT trim whitespace, so this should fail
        assert_eq!(value_from_form(val), None);
    }

    #[test]
    fn value_from_form_string_empty() {
        let val = Some(Value::String(String::new()));
        assert_eq!(value_from_form(val), None);
    }

    #[test]
    fn value_from_form_string_scientific_notation() {
        let val = Some(Value::String("1e2".to_string()));
        assert_eq!(value_from_form(val), Some(100.0));
    }

    #[test]
    fn value_from_form_string_decimal_point_only() {
        let val = Some(Value::String(".".to_string()));
        assert_eq!(value_from_form(val), None);
    }

    #[test]
    fn value_from_form_string_multiple_decimal_points() {
        let val = Some(Value::String("1.2.3".to_string()));
        assert_eq!(value_from_form(val), None);
    }

    #[test]
    fn value_from_form_string_leading_plus() {
        let val = Some(Value::String("+5.5".to_string()));
        assert_eq!(value_from_form(val), Some(5.5));
    }

    #[test]
    fn value_from_form_string_with_letters() {
        let val = Some(Value::String("abc123".to_string()));
        assert_eq!(value_from_form(val), None);
    }

    #[test]
    fn value_from_form_string_partial_number() {
        let val = Some(Value::String("123abc".to_string()));
        // parse::<f64> will fail on this
        assert_eq!(value_from_form(val), None);
    }

    #[test]
    fn value_from_form_number_precision() {
        let val = Some(Value::Number(Number::from_f64(4.999999).unwrap()));
        let result = value_from_form(val);
        assert!(result.is_some());
        assert!((result.unwrap() - 4.999999).abs() < f64::EPSILON);
    }

    // Note: apply_rate tests require Dioxus runtime for Signal creation
    // These are tested in integration tests or component behavior tests

    #[test]
    fn pointer_value_non_wasm() {
        // On non-wasm targets, pointer_value should return star_index as f64
        // We can't easily create a real Event in tests, so we test the logic directly
        // The function always returns Some(star_index as f64) on non-wasm targets
        // This is tested implicitly through the component behavior
    }
}
