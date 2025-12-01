use crate::components::config_provider::use_config;
use crate::components::form::use_form_item_control;
#[cfg(target_arch = "wasm32")]
use crate::components::interaction::start_pointer;
use crate::components::interaction::{
    PointerState, as_pointer_event, end_pointer, is_active_pointer,
};
#[cfg(target_arch = "wasm32")]
use crate::components::slider_base::ratio_from_pointer_event;
#[cfg(target_arch = "wasm32")]
use crate::components::slider_base::ratio_to_value;
use crate::components::slider_base::{
    SliderMath, SliderOrientation, apply_keyboard_action, keyboard_action_for_key, snap_value,
    value_to_ratio,
};
use dioxus::events::{KeyboardEvent, PointerData};
use dioxus::prelude::*;
use serde_json::{Number, Value};
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// A labeled mark rendered along the slider track.
#[derive(Clone, PartialEq)]
pub struct SliderMark {
    pub value: f64,
    pub label: String,
}

/// Supported slider value shapes (single vs. range).
#[derive(Clone, PartialEq, Debug)]
pub enum SliderValue {
    Single(f64),
    Range(f64, f64),
}

impl SliderValue {
    pub fn as_single(&self) -> f64 {
        match *self {
            SliderValue::Single(v) => v,
            SliderValue::Range(_, end) => end,
        }
    }

    pub fn as_range(&self) -> (f64, f64) {
        match *self {
            SliderValue::Single(v) => (v, v),
            SliderValue::Range(start, end) => (start, end),
        }
    }

    fn ensure_range(self) -> Self {
        match self {
            SliderValue::Single(v) => SliderValue::Range(v, v),
            SliderValue::Range(start, end) => {
                if start <= end {
                    SliderValue::Range(start, end)
                } else {
                    SliderValue::Range(end, start)
                }
            }
        }
    }
}

/// Slider component supporting single and range values.
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Controlled value; use `Range(_, _)` when `range = true`.
    #[props(optional)]
    pub value: Option<SliderValue>,
    /// Default value in uncontrolled mode.
    #[props(optional)]
    pub default_value: Option<SliderValue>,
    /// Whether to render two handles.
    #[props(default)]
    pub range: bool,
    #[props(default = 0.0)]
    pub min: f64,
    #[props(default = 100.0)]
    pub max: f64,
    /// Step granularity; when `None` the slider is continuous.
    #[props(optional)]
    pub step: Option<f64>,
    /// Decimal precision used for snapping.
    #[props(optional)]
    pub precision: Option<u32>,
    /// Reverse the visual direction (RTL or top-down).
    #[props(default)]
    pub reverse: bool,
    /// Vertical orientation.
    #[props(default)]
    pub vertical: bool,
    /// Disable interactions.
    #[props(default)]
    pub disabled: bool,
    /// Render tick dots for marks.
    #[props(default)]
    pub dots: bool,
    /// Optional labeled marks along the track.
    #[props(optional)]
    pub marks: Option<Vec<SliderMark>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Fired on every value change (drag/keyboard/track click).
    #[props(optional)]
    pub on_change: Option<EventHandler<SliderValue>>,
    /// Fired when user finishes interaction (pointer up / Enter / blur).
    #[props(optional)]
    pub on_change_complete: Option<EventHandler<SliderValue>>,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let SliderProps {
        value,
        default_value,
        range,
        min,
        max,
        step,
        precision,
        reverse,
        vertical,
        disabled,
        dots,
        marks,
        class,
        style,
        on_change,
        on_change_complete,
    } = props;

    let math = SliderMath {
        min,
        max,
        step,
        precision,
        reverse,
        orientation: if vertical {
            SliderOrientation::Vertical
        } else {
            SliderOrientation::Horizontal
        },
    };

    let config = use_config();
    let form_control = use_form_item_control();
    let controlled_by_prop = value.is_some();

    let initial_value = normalize_value(
        range,
        value
            .clone()
            .or_else(|| {
                form_control
                    .as_ref()
                    .and_then(|ctx| slider_value_from_form(ctx.value(), range))
            })
            .or(default_value.clone())
            .unwrap_or_else(|| default_slider_value(range, &math)),
        &math,
    );

    let current = use_signal(|| initial_value.clone());
    let mut active_handle = use_signal(|| None::<usize>);
    let active_pointer = use_signal::<PointerState>(PointerState::default);
    // Sync to controlled/form value changes.
    {
        let mut current_signal = current.clone();
        let form_ctx = form_control.clone();
        let value_prop = value.clone();
        let default_val = default_value.clone();
        use_effect(move || {
            let next = normalize_value(
                range,
                value_prop
                    .clone()
                    .or_else(|| {
                        form_ctx
                            .as_ref()
                            .and_then(|ctx| slider_value_from_form(ctx.value(), range))
                    })
                    .or(default_val.clone())
                    .unwrap_or_else(|| default_slider_value(range, &math)),
                &math,
            );
            current_signal.set(next);
        });
    }

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let mut class_list = vec!["adui-slider".to_string()];
    if is_disabled {
        class_list.push("adui-slider-disabled".into());
    }
    if vertical {
        class_list.push("adui-slider-vertical".into());
    }
    if dots {
        class_list.push("adui-slider-dots".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let on_change_cb = on_change;
    let on_change_complete_cb = on_change_complete;
    let form_ctx_for_apply = form_control.clone();
    let current_for_apply = current.clone();

    let apply_value = move |next: SliderValue, fire_change: bool| {
        let normalized = normalize_value(range, next, &math);
        if !controlled_by_prop {
            let mut state = current_for_apply;
            state.set(normalized.clone());
        }

        if let Some(ctx) = form_ctx_for_apply.as_ref() {
            ctx.set_value(slider_value_to_form(&normalized));
        }

        if fire_change {
            if let Some(cb) = on_change_cb.as_ref() {
                cb.call(normalized.clone());
            }
        }
        normalized
    };

    let handle_pointer_move = {
        #[allow(unused_variables)]
        let current_for_move = current.clone();
        #[allow(unused_variables)]
        let active_handle_for_move = active_handle.clone();
        #[allow(unused_variables)]
        let active_pointer_for_move = active_pointer.clone();
        #[allow(unused_variables)]
        let apply_for_move = apply_value.clone();
        move |evt: Event<PointerData>| {
            #[cfg(target_arch = "wasm32")]
            {
                if is_disabled {
                    return;
                }
                let Some(pevt) = as_pointer_event(&evt) else {
                    return;
                };
                if !is_active_pointer(&active_pointer_for_move, &pevt) {
                    return;
                }
                let rect = pevt
                    .current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .map(|el| el.get_bounding_client_rect());
                let Some(rect) = rect else {
                    return;
                };
                let Some(ratio) = pointer_ratio(&pevt, &rect, &math) else {
                    return;
                };
                let target_value = ratio_to_value(ratio, &math);

                let handle_idx = active_handle_for_move.read().unwrap_or(0);
                let next = update_handle_value(
                    &current_for_move.read(),
                    handle_idx,
                    target_value,
                    range,
                    &math,
                );
                apply_for_move(next, true);
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = evt;
            }
        }
    };

    let mut pointer_state_for_up = active_pointer.clone();
    let mut active_handle_for_up = active_handle.clone();
    let current_for_up = current.clone();
    let on_change_complete_for_up = on_change_complete_cb.clone();

    let handle_pointer_up = move |evt: Event<PointerData>| {
        let Some(pevt) = as_pointer_event(&evt) else {
            return;
        };
        if !is_active_pointer(&pointer_state_for_up, &pevt) {
            return;
        }
        end_pointer(&mut pointer_state_for_up, &pevt);
        active_handle_for_up.set(None);
        if let Some(cb) = on_change_complete_for_up.as_ref() {
            cb.call(current_for_up.read().clone());
        }
    };

    let apply_for_key = apply_value.clone();

    let handle_track_pointer_down = {
        #[allow(unused_variables)]
        let apply_for_track = apply_value.clone();
        move |evt: Event<PointerData>| {
            #[cfg(target_arch = "wasm32")]
            {
                if is_disabled {
                    return;
                }
                let Some(pevt) = as_pointer_event(&evt) else {
                    return;
                };
                let rect = pevt
                    .current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                    .map(|el| el.get_bounding_client_rect());
                let Some(rect) = rect else {
                    return;
                };
                let Some(ratio) = pointer_ratio(&pevt, &rect, &math) else {
                    return;
                };
                let target_value = ratio_to_value(ratio, &math);

                let current_value = current.read();
                let handle_idx = choose_handle(&current_value, target_value);
                active_handle.set(Some(handle_idx));
                let mut pointer_for_down = active_pointer.clone();
                start_pointer(&mut pointer_for_down, &pevt);

                let next =
                    update_handle_value(&current_value, handle_idx, target_value, range, &math);
                let normalized = apply_for_track(next, true);
                if let Some(cb) = on_change_complete_cb.as_ref() {
                    cb.call(normalized);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = evt;
            }
        }
    };

    let handle_key: Rc<dyn Fn(usize, KeyboardEvent)> = Rc::new({
        let current_signal = current.clone();
        move |idx: usize, evt: KeyboardEvent| {
            if is_disabled {
                return;
            }
            if let Some(action) = keyboard_action_for_key(&evt.key(), math.reverse) {
                let current_value = current_signal.read();
                let handle_value = match *current_value {
                    SliderValue::Single(v) => v,
                    SliderValue::Range(start, end) => {
                        if idx == 0 {
                            start
                        } else {
                            end
                        }
                    }
                };
                let stepped = apply_keyboard_action(handle_value, action, &math);
                let next = update_handle_value(&current_value, idx, stepped, range, &math);
                apply_for_key(next, true);
            }
        }
    });

    let value_now = current.read().clone();
    let (ratios, handle_values): (Vec<f64>, Vec<f64>) = match value_now {
        SliderValue::Single(v) => (vec![value_to_ratio(v, &math)], vec![v]),
        SliderValue::Range(a, b) => (
            vec![value_to_ratio(a, &math), value_to_ratio(b, &math)],
            vec![a, b],
        ),
    };
    let track_range = track_range_style(&ratios, math.orientation);

    let marks_view = marks.map(|items| {
        let dots_enabled = dots;
        items
            .into_iter()
            .map(|mark| {
                let pos = value_to_ratio(mark.value, &math) * 100.0;
                rsx! {
                    div { class: "adui-slider-mark", style: match math.orientation {
                        SliderOrientation::Vertical => format!("bottom:{pos:.2}%;"),
                        SliderOrientation::Horizontal => format!("left:{pos:.2}%;"),
                    },
                        if dots_enabled {
                            span { class: "adui-slider-dot" }
                        }
                        span { class: "adui-slider-mark-label", {mark.label} }
                    }
                }
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            onpointerdown: handle_track_pointer_down,
            onpointermove: handle_pointer_move,
            onpointerup: handle_pointer_up,
            onpointercancel: handle_pointer_up,
            onpointerleave: handle_pointer_up,
            div { class: "adui-slider-rail" }
            div { class: "adui-slider-track", style: "{track_range}" }
            {handles_view(&ratios, &handle_values, range, math, is_disabled, handle_key.clone())}
            if let Some(marks) = marks_view {
                div { class: "adui-slider-marks",
                    for mark in marks {
                        {mark}
                    }
                }
            }
        }
    }
}

fn handles_view(
    ratios: &[f64],
    values: &[f64],
    range: bool,
    math: SliderMath,
    disabled: bool,
    on_key: Rc<dyn Fn(usize, KeyboardEvent)>,
) -> Element {
    let count = if range {
        ratios.len()
    } else {
        1.min(ratios.len())
    };
    let iter = ratios.iter().zip(values.iter()).take(count).enumerate();
    rsx! {
        Fragment {
            for (idx, (ratio, value_now)) in iter {
                button {
                    class: "adui-slider-handle",
                    role: "slider",
                    tabindex: 0,
                    aria_disabled: disabled,
                    aria_valuemin: math.min,
                    aria_valuemax: math.max,
                    aria_valuenow: *value_now,
                    style: "{handle_position_style(*ratio, math.orientation)}",
                    onkeydown: { let cb = on_key.clone(); move |evt| cb(idx, evt) },
                }
            }
        }
    }
}

fn track_range_style(ratios: &[f64], orientation: SliderOrientation) -> String {
    let (start, end) = if ratios.len() >= 2 {
        let a = ratios[0];
        let b = ratios[1];
        (a.min(b), a.max(b))
    } else {
        (0.0, *ratios.get(0).unwrap_or(&0.0))
    };
    let start_pct = start * 100.0;
    let length_pct = (end - start).abs() * 100.0;
    match orientation {
        SliderOrientation::Horizontal => {
            format!("left:{start_pct:.2}%;width:{length_pct:.2}%;")
        }
        SliderOrientation::Vertical => {
            format!("bottom:{start_pct:.2}%;height:{length_pct:.2}%;")
        }
    }
}

fn handle_position_style(ratio: f64, orientation: SliderOrientation) -> String {
    let pct = (ratio * 100.0).clamp(0.0, 100.0);
    match orientation {
        SliderOrientation::Horizontal => format!("left:{pct:.2}%;"),
        SliderOrientation::Vertical => format!("bottom:{pct:.2}%;"),
    }
}

fn normalize_value(range: bool, value: SliderValue, math: &SliderMath) -> SliderValue {
    let mut normalized = match value {
        SliderValue::Single(v) => SliderValue::Single(snap_value(v, math)),
        SliderValue::Range(a, b) => {
            let a = snap_value(a, math);
            let b = snap_value(b, math);
            SliderValue::Range(a.min(b), a.max(b))
        }
    };

    if range {
        normalized = normalized.ensure_range();
    } else {
        normalized = SliderValue::Single(normalized.as_single());
    }
    normalized
}

fn default_slider_value(range: bool, math: &SliderMath) -> SliderValue {
    if range {
        SliderValue::Range(math.min, math.max)
    } else {
        SliderValue::Single(math.min)
    }
}

#[allow(dead_code)]
fn choose_handle(current: &SliderValue, target: f64) -> usize {
    match current {
        SliderValue::Single(_) => 0,
        SliderValue::Range(a, b) => {
            let dist_a = (target - *a).abs();
            let dist_b = (target - *b).abs();
            if dist_a <= dist_b { 0 } else { 1 }
        }
    }
}

fn update_handle_value(
    current: &SliderValue,
    handle_idx: usize,
    target: f64,
    range: bool,
    math: &SliderMath,
) -> SliderValue {
    let next = match (range, current) {
        (false, _) => SliderValue::Single(target),
        (true, SliderValue::Range(start, end)) => {
            if handle_idx == 0 {
                SliderValue::Range(target, *end)
            } else {
                SliderValue::Range(*start, target)
            }
        }
        (true, SliderValue::Single(v)) => {
            if handle_idx == 0 {
                SliderValue::Range(target, *v)
            } else {
                SliderValue::Range(*v, target)
            }
        }
    };
    normalize_value(range, next, math)
}

fn slider_value_from_form(val: Option<Value>, range: bool) -> Option<SliderValue> {
    if range {
        match val {
            Some(Value::Array(items)) if items.len() >= 2 => {
                let first = items.get(0).and_then(|v| v.as_f64())?;
                let second = items.get(1).and_then(|v| v.as_f64())?;
                Some(SliderValue::Range(first, second))
            }
            Some(Value::Number(n)) => n.as_f64().map(|v| SliderValue::Range(v, v)),
            _ => None,
        }
    } else {
        match val {
            Some(Value::Number(n)) => n.as_f64().map(SliderValue::Single),
            Some(Value::Array(items)) if !items.is_empty() => items
                .get(0)
                .and_then(|v| v.as_f64())
                .map(SliderValue::Single),
            Some(Value::String(s)) => s.parse::<f64>().ok().map(SliderValue::Single),
            _ => None,
        }
    }
}

fn slider_value_to_form(value: &SliderValue) -> Value {
    match value {
        SliderValue::Single(v) => Number::from_f64(*v)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        SliderValue::Range(a, b) => Value::Array(vec![
            Number::from_f64(*a)
                .map(Value::Number)
                .unwrap_or(Value::Null),
            Number::from_f64(*b)
                .map(Value::Number)
                .unwrap_or(Value::Null),
        ]),
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
fn pointer_ratio(
    evt: &web_sys::PointerEvent,
    rect: &web_sys::DomRect,
    math: &SliderMath,
) -> Option<f64> {
    ratio_from_pointer_event(evt, rect, math)
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn pointer_ratio(
    _evt: &web_sys::PointerEvent,
    _rect: &web_sys::DomRect,
    _math: &SliderMath,
) -> Option<f64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_single_respects_bounds() {
        let math = SliderMath {
            min: 0.0,
            max: 10.0,
            step: Some(1.0),
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = normalize_value(false, SliderValue::Single(11.2), &math);
        assert_eq!(val.as_single(), 10.0);
    }

    #[test]
    fn normalize_single_below_min() {
        let math = SliderMath {
            min: 5.0,
            max: 10.0,
            step: None,
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = normalize_value(false, SliderValue::Single(3.0), &math);
        assert_eq!(val.as_single(), 5.0);
    }

    #[test]
    fn normalize_range_orders_and_snaps() {
        let math = SliderMath {
            min: 0.0,
            max: 5.0,
            step: Some(0.5),
            precision: Some(1),
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = normalize_value(true, SliderValue::Range(3.3, 1.0), &math);
        assert_eq!(val.as_range(), (1.0, 3.5));
    }

    #[test]
    fn normalize_range_out_of_bounds() {
        let math = SliderMath {
            min: 0.0,
            max: 10.0,
            step: Some(1.0),
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = normalize_value(true, SliderValue::Range(-5.0, 15.0), &math);
        let (start, end) = val.as_range();
        assert_eq!(start, 0.0);
        assert_eq!(end, 10.0);
    }

    #[test]
    fn normalize_single_with_step() {
        let math = SliderMath {
            min: 0.0,
            max: 10.0,
            step: Some(2.0),
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = normalize_value(false, SliderValue::Single(7.3), &math);
        assert_eq!(val.as_single(), 8.0);
    }

    #[test]
    fn normalize_single_with_precision() {
        let math = SliderMath {
            min: 0.0,
            max: 10.0,
            step: None,
            precision: Some(2),
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = normalize_value(false, SliderValue::Single(3.456789), &math);
        assert_eq!(val.as_single(), 3.46);
    }

    #[test]
    fn choose_handle_picks_nearest() {
        let val = SliderValue::Range(10.0, 20.0);
        assert_eq!(choose_handle(&val, 12.0), 0);
        assert_eq!(choose_handle(&val, 18.0), 1);
    }

    #[test]
    fn choose_handle_equal_distance() {
        let val = SliderValue::Range(10.0, 20.0);
        assert_eq!(choose_handle(&val, 15.0), 0);
    }

    #[test]
    fn choose_handle_single_value() {
        let val = SliderValue::Single(15.0);
        assert_eq!(choose_handle(&val, 20.0), 0);
    }

    #[test]
    fn slider_value_as_single() {
        let single = SliderValue::Single(5.0);
        assert_eq!(single.as_single(), 5.0);

        let range = SliderValue::Range(10.0, 20.0);
        assert_eq!(range.as_single(), 20.0);
    }

    #[test]
    fn slider_value_as_range() {
        let single = SliderValue::Single(5.0);
        assert_eq!(single.as_range(), (5.0, 5.0));

        let range = SliderValue::Range(10.0, 20.0);
        assert_eq!(range.as_range(), (10.0, 20.0));
    }

    #[test]
    fn slider_value_ensure_range() {
        let single = SliderValue::Single(5.0);
        let range = single.ensure_range();
        assert_eq!(range.as_range(), (5.0, 5.0));

        let reversed = SliderValue::Range(20.0, 10.0);
        let fixed = reversed.ensure_range();
        assert_eq!(fixed.as_range(), (10.0, 20.0));
    }

    #[test]
    fn default_slider_value_single() {
        let math = SliderMath {
            min: 0.0,
            max: 100.0,
            step: None,
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = default_slider_value(false, &math);
        assert_eq!(val.as_single(), 0.0);
    }

    #[test]
    fn default_slider_value_range() {
        let math = SliderMath {
            min: 0.0,
            max: 100.0,
            step: None,
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let val = default_slider_value(true, &math);
        assert_eq!(val.as_range(), (0.0, 100.0));
    }
}
