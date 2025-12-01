use crate::components::config_provider::use_config;
use crate::components::form::use_form_item_control;
#[cfg(target_arch = "wasm32")]
use crate::components::interaction::as_pointer_event;
use dioxus::events::PointerData;
use dioxus::prelude::*;
use serde_json::Value;
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// HSVA color model for internal state.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Hsva {
    h: f64, // 0..360
    s: f64, // 0..1
    v: f64, // 0..1
    a: f64, // 0..1
}

/// Props for the ColorPicker (single color).
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// Controlled color string, e.g. `#RRGGBB` or `#RRGGBBAA`.
    #[props(optional)]
    pub value: Option<String>,
    /// Uncontrolled initial value.
    #[props(optional)]
    pub default_value: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub allow_clear: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Fired on every change with hex string (empty when cleared).
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Fired when interaction completes (pointer up / input blur).
    #[props(optional)]
    pub on_change_complete: Option<EventHandler<String>>,
}

#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let ColorPickerProps {
        value,
        default_value,
        disabled,
        allow_clear,
        class,
        style,
        on_change,
        on_change_complete,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();
    let controlled = value.is_some();

    let initial = resolve_color(value.clone(), &form_control, default_value.as_ref());
    let color_state = use_signal(|| initial);
    let mut text_value = use_signal(|| color_to_hex(initial.as_ref()));

    // Sync external value changes.
    {
        let form_ctx = form_control.clone();
        let prop_val = value.clone();
        let mut color_signal = color_state.clone();
        let mut text_signal = text_value.clone();
        use_effect(move || {
            let next = resolve_color(prop_val.clone(), &form_ctx, None);
            color_signal.set(next);
            text_signal.set(color_to_hex(next.as_ref()));
        });
    }

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let mut class_list = vec!["adui-color-picker".to_string()];
    if is_disabled {
        class_list.push("adui-color-picker-disabled".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let apply_color: Rc<dyn Fn(Option<Hsva>, bool)> = Rc::new({
        let on_change_cb = on_change.clone();
        let on_change_complete_cb = on_change_complete.clone();
        let form_ctx_for_apply = form_control.clone();
        let color_for_apply = color_state.clone();
        let text_for_apply = text_value.clone();
        move |next: Option<Hsva>, fire_change: bool| {
            if !controlled {
                let mut state = color_for_apply;
                state.set(next);
            }
            let hex = color_to_hex(next.as_ref());
            let mut text_state = text_for_apply;
            text_state.set(hex.clone());

            if let Some(ctx) = form_ctx_for_apply.as_ref() {
                if hex.is_empty() {
                    ctx.set_value(Value::Null);
                } else {
                    ctx.set_value(Value::String(hex.clone()));
                }
            }

            if fire_change {
                if let Some(cb) = on_change_cb.as_ref() {
                    cb.call(hex.clone());
                }
            }
            if let Some(cb) = on_change_complete_cb.as_ref() {
                cb.call(hex);
            }
        }
    });

    let apply_for_input = apply_color.clone();
    let apply_for_clear = apply_color.clone();

    // 跟踪拖动状态
    let dragging_sat = use_signal(|| false);
    let dragging_hue = use_signal(|| false);
    let dragging_alpha = use_signal(|| false);

    // Derived values for UI rendering.
    let current = color_state.read().clone();
    let base_hue = current.map(|c| c.h).unwrap_or(0.0);
    let hue_rgb = hsv_to_rgb(base_hue, 1.0, 1.0);
    let sat_cursor = current
        .map(|c| (c.s.clamp(0.0, 1.0), 1.0 - c.v.clamp(0.0, 1.0)))
        .unwrap_or((0.0, 0.0));
    let (sat_x, sat_y) = sat_cursor;
    let _alpha_value = current.map(|c| c.a).unwrap_or(1.0);
    let preview_css = color_to_css(current.as_ref());
    let hue_gradient = "linear-gradient(90deg, red 0%, #ff0 17%, #0f0 33%, #0ff 50%, #00f 67%, #f0f 83%, red 100%)";
    let alpha_gradient = format!(
        "linear-gradient(90deg, rgba({},{},{},0) 0%, rgba({},{},{},1) 100%)",
        hue_rgb.0, hue_rgb.1, hue_rgb.2, hue_rgb.0, hue_rgb.1, hue_rgb.2
    );

    let handle_sat_pointer = {
        let apply_for_sat = apply_color.clone();
        let color_signal = color_state.clone();
        let mut is_dragging = dragging_sat.clone();
        move |evt: Event<PointerData>| {
            if is_disabled {
                return;
            }

            // 只在按下或拖动时响应
            let buttons = evt.held_buttons();
            if !buttons.contains(dioxus::html::input_data::MouseButton::Primary) {
                is_dragging.set(false);
                return;
            }

            is_dragging.set(true);

            // 获取元素相对坐标
            let elem_coords = evt.element_coordinates();
            let s = (elem_coords.x / 200.0).clamp(0.0, 1.0);
            let v = 1.0 - (elem_coords.y / 150.0).clamp(0.0, 1.0);

            let current_color = color_signal.read().clone();
            let current_alpha = current_color.map(|c| c.a).unwrap_or(1.0);
            let mut next = current_color.unwrap_or(Hsva {
                h: base_hue,
                s: 1.0,
                v: 1.0,
                a: current_alpha,
            });
            next.s = s;
            next.v = v;
            apply_for_sat(Some(next), true);
        }
    };

    let handle_hue_pointer = {
        let apply_for_hue = apply_color.clone();
        let color_signal = color_state.clone();
        let mut is_dragging = dragging_hue.clone();
        move |evt: Event<PointerData>| {
            if is_disabled {
                return;
            }

            let buttons = evt.held_buttons();
            if !buttons.contains(dioxus::html::input_data::MouseButton::Primary) {
                is_dragging.set(false);
                return;
            }

            is_dragging.set(true);

            let elem_coords = evt.element_coordinates();
            let ratio = (elem_coords.x / 200.0).clamp(0.0, 1.0);
            let h = ratio * 360.0;

            let current_color = color_signal.read().clone();
            let current_alpha = current_color.map(|c| c.a).unwrap_or(1.0);
            let mut next = current_color.unwrap_or(Hsva {
                h,
                s: 1.0,
                v: 1.0,
                a: current_alpha,
            });
            next.h = h;
            apply_for_hue(Some(next), true);
        }
    };

    let handle_alpha_pointer = {
        let apply_for_alpha = apply_color.clone();
        let color_signal = color_state.clone();
        let mut is_dragging = dragging_alpha.clone();
        move |evt: Event<PointerData>| {
            if is_disabled {
                return;
            }

            let buttons = evt.held_buttons();
            if !buttons.contains(dioxus::html::input_data::MouseButton::Primary) {
                is_dragging.set(false);
                return;
            }

            is_dragging.set(true);

            let elem_coords = evt.element_coordinates();
            let ratio = (elem_coords.x / 200.0).clamp(0.0, 1.0);

            let current_color = color_signal.read().clone();
            let mut next = current_color.unwrap_or(Hsva {
                h: base_hue,
                s: 1.0,
                v: 1.0,
                a: ratio,
            });
            next.a = ratio;
            apply_for_alpha(Some(next), true);
        }
    };

    let handle_input = move |evt: Event<FormData>| {
        if is_disabled {
            return;
        }
        let text = evt.value();
        text_value.set(text.clone());
        let parsed = parse_color(&text);
        apply_for_input(parsed, true);
    };

    let handle_clear = move |_| {
        if is_disabled || !allow_clear {
            return;
        }
        apply_for_clear(None, true);
    };

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            div { class: "adui-color-picker-preview",
                style: "background:{preview_css};",
            }
            div { class: "adui-color-picker-controls",
                div { class: "adui-color-picker-sat",
                    style: "background: {hue_background(base_hue)};",
                    onpointerdown: handle_sat_pointer.clone(),
                    onpointermove: handle_sat_pointer,
                    div { class: "adui-color-picker-sat-white" }
                    div { class: "adui-color-picker-sat-black" }
                    div { class: "adui-color-picker-sat-handle",
                        style: format!("left:{:.2}%;top:{:.2}%;", sat_x * 100.0, sat_y * 100.0),
                    }
                }
                div { class: "adui-color-picker-slider",
                    style: "background:{hue_gradient};",
                    onpointerdown: handle_hue_pointer.clone(),
                    onpointermove: handle_hue_pointer,
                }
                div { class: "adui-color-picker-slider",
                    style: format!("background:{alpha_gradient};"),
                    onpointerdown: handle_alpha_pointer.clone(),
                    onpointermove: handle_alpha_pointer,
                }
                div { class: "adui-color-picker-input-row",
                    input {
                        class: "adui-color-picker-input",
                        value: "{text_value.read()}",
                        disabled: is_disabled,
                        oninput: handle_input,
                    }
                    if allow_clear {
                        button { class: "adui-color-picker-clear", disabled: is_disabled, onclick: handle_clear, "Clear" }
                    }
                }
            }
        }
    }
}

fn resolve_color(
    value: Option<String>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    fallback: Option<&String>,
) -> Option<Hsva> {
    value
        .or_else(|| {
            form_control
                .as_ref()
                .and_then(|ctx| value_from_form(ctx.value()))
        })
        .or_else(|| fallback.cloned())
        .and_then(|s| parse_color(&s))
}

fn value_from_form(val: Option<Value>) -> Option<String> {
    match val {
        Some(Value::String(s)) => Some(s),
        _ => None,
    }
}

fn parse_color(input: &str) -> Option<Hsva> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    if !trimmed.starts_with('#') {
        return None;
    }
    let hex = trimmed.trim_start_matches('#');
    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };
    let (h, s, v) = rgb_to_hsv(r, g, b);
    Some(Hsva {
        h,
        s,
        v,
        a: (a as f64 / 255.0).clamp(0.0, 1.0),
    })
}

fn color_to_hex(hsva: Option<&Hsva>) -> String {
    if let Some(color) = hsva {
        let (r, g, b) = hsv_to_rgb(color.h, color.s, color.v);
        if (color.a - 1.0).abs() < f64::EPSILON {
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        } else {
            let a = (color.a * 255.0).round() as u8;
            format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
        }
    } else {
        String::new()
    }
}

fn color_to_css(hsva: Option<&Hsva>) -> String {
    if let Some(color) = hsva {
        let (r, g, b) = hsv_to_rgb(color.h, color.s, color.v);
        format!("rgba({},{},{},{:.3})", r, g, b, color.a)
    } else {
        "transparent".into()
    }
}

fn hue_background(h: f64) -> String {
    let (r, g, b) = hsv_to_rgb(h, 1.0, 1.0);
    format!("rgb({},{},{})", r, g, b)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = match (h / 60.0).floor() as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    )
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta < f64::EPSILON {
        0.0
    } else if (max - r).abs() < f64::EPSILON {
        60.0 * (((g - b) / delta) % 6.0)
    } else if (max - g).abs() < f64::EPSILON {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let s = if max.abs() < f64::EPSILON {
        0.0
    } else {
        delta / max
    };
    (if h < 0.0 { h + 360.0 } else { h }, s, max)
}

#[cfg(test)]
mod color_picker_tests {
    use super::*;

    #[test]
    fn parse_color_empty_string() {
        assert_eq!(parse_color(""), None);
        assert_eq!(parse_color("   "), None);
    }

    #[test]
    fn parse_color_invalid_format() {
        assert_eq!(parse_color("not-a-color"), None);
        assert_eq!(parse_color("rgb(255,0,0)"), None);
        assert_eq!(parse_color("#GGG"), None);
    }

    #[test]
    fn parse_color_6_digit_hex() {
        let result = parse_color("#FF0000");
        assert!(result.is_some());
        let color = result.unwrap();
        assert!((color.h - 0.0).abs() < 1.0 || (color.h - 360.0).abs() < 1.0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn parse_color_8_digit_hex() {
        let result = parse_color("#FF000080");
        assert!(result.is_some());
        let color = result.unwrap();
        assert!((color.a - 0.5).abs() < 0.01);
    }

    #[test]
    fn color_to_hex_with_alpha() {
        let color = Hsva {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 0.5,
        };
        let hex = color_to_hex(Some(&color));
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 9); // #RRGGBBAA
    }

    #[test]
    fn color_to_hex_without_alpha() {
        let color = Hsva {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let hex = color_to_hex(Some(&color));
        assert_eq!(hex.len(), 7); // #RRGGBB
    }

    #[test]
    fn color_to_hex_none() {
        assert_eq!(color_to_hex(None), "");
    }

    #[test]
    fn color_to_css_with_color() {
        let color = Hsva {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 0.5,
        };
        let css = color_to_css(Some(&color));
        assert!(css.starts_with("rgba("));
    }

    #[test]
    fn color_to_css_none() {
        assert_eq!(color_to_css(None), "transparent");
    }

    #[test]
    fn hsv_to_rgb_red() {
        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn hsv_to_rgb_green() {
        let (r, g, b) = hsv_to_rgb(120.0, 1.0, 1.0);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);
    }

    #[test]
    fn hsv_to_rgb_blue() {
        let (r, g, b) = hsv_to_rgb(240.0, 1.0, 1.0);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }

    #[test]
    fn rgb_to_hsv_red() {
        let (h, s, v) = rgb_to_hsv(255, 0, 0);
        assert!((h - 0.0).abs() < 1.0 || (h - 360.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);
    }

    #[test]
    fn rgb_to_hsv_green() {
        let (h, s, v) = rgb_to_hsv(0, 255, 0);
        assert!((h - 120.0).abs() < 1.0);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);
    }

    #[test]
    fn rgb_to_hsv_black() {
        let (_h, _s, v) = rgb_to_hsv(0, 0, 0);
        assert!((v - 0.0).abs() < 0.01);
    }

    #[test]
    fn rgb_to_hsv_white() {
        let (_h, s, v) = rgb_to_hsv(255, 255, 255);
        assert!((s - 0.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);
    }
}
