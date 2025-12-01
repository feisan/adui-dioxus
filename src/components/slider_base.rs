//! Shared slider math and keyboard/pointer helpers.
//!
//! This stays UI-agnostic so multiple slider-like components (Slider, Rate, ColorPicker) can
//! share the same value math, orientation handling and accessibility behaviors.

use crate::components::number_utils::{clamp, round_with_precision};
use dioxus::prelude::Key;

/// Orientation of the slider track.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SliderOrientation {
    pub fn is_vertical(self) -> bool {
        matches!(self, SliderOrientation::Vertical)
    }
}

/// Core numeric configuration for slider-like controls.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SliderMath {
    pub min: f64,
    pub max: f64,
    /// Step granularity; when `None`, treat as continuous.
    pub step: Option<f64>,
    /// Optional decimal precision enforced on output.
    pub precision: Option<u32>,
    /// When true, visual direction is reversed (RTL / vertical top-down).
    pub reverse: bool,
    pub orientation: SliderOrientation,
}

impl Default for SliderMath {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            step: Some(1.0),
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        }
    }
}

impl SliderMath {
    /// Ensure min/max ordering is valid by swapping if needed.
    pub fn normalized(self) -> Self {
        if self.max >= self.min {
            self
        } else {
            Self {
                min: self.max,
                max: self.min,
                ..self
            }
        }
    }

    pub fn range(&self) -> f64 {
        (self.max - self.min).abs().max(f64::EPSILON)
    }
}

/// Clamp and snap a value to step/precision within min/max.
pub fn snap_value(value: f64, math: &SliderMath) -> f64 {
    let math = math.normalized();
    let clamped = clamp(
        value,
        &crate::components::number_utils::NumberRules {
            min: Some(math.min),
            max: Some(math.max),
            step: None,
            precision: None,
        },
    );

    // Snap to step relative to min when step is present.
    let snapped = if let Some(step) = math.step {
        let positive_step = step.abs();
        if positive_step <= f64::EPSILON {
            clamped
        } else {
            let steps = ((clamped - math.min) / positive_step).round();
            let candidate = math.min + steps * positive_step;
            // Ensure the snapped value doesn't exceed max
            candidate.min(math.max)
        }
    } else {
        clamped
    };

    round_with_precision(snapped, math.precision)
}

/// Convert an absolute value into a [0.0, 1.0] ratio along the track.
pub fn value_to_ratio(value: f64, math: &SliderMath) -> f64 {
    let math = math.normalized();
    let snapped = snap_value(value, &math);
    let ratio = (snapped - math.min) / math.range();
    if math.reverse { 1.0 - ratio } else { ratio }.clamp(0.0, 1.0)
}

/// Convert a [0.0, 1.0] ratio into an absolute value.
pub fn ratio_to_value(ratio: f64, math: &SliderMath) -> f64 {
    let math = math.normalized();
    let mut normalized = ratio.clamp(0.0, 1.0);
    if math.reverse {
        normalized = 1.0 - normalized;
    }
    let raw = math.min + normalized * math.range();
    snap_value(raw, &math)
}

/// Keyboard intents supported by slider-like controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardAction {
    /// Relative step (can be >1 for PageUp/PageDown).
    Step(i32),
    /// Jump to minimum value.
    ToMin,
    /// Jump to maximum value.
    ToMax,
}

/// Map a key into a keyboard action, respecting reverse direction.
pub fn keyboard_action_for_key(key: &Key, reverse: bool) -> Option<KeyboardAction> {
    let direction = |positive: KeyboardAction, negative: KeyboardAction| {
        if reverse { negative } else { positive }
    };
    match key {
        Key::ArrowRight | Key::ArrowUp => {
            Some(direction(KeyboardAction::Step(1), KeyboardAction::Step(-1)))
        }
        Key::ArrowLeft | Key::ArrowDown => {
            Some(direction(KeyboardAction::Step(-1), KeyboardAction::Step(1)))
        }
        Key::PageUp => Some(direction(
            KeyboardAction::Step(10),
            KeyboardAction::Step(-10),
        )),
        Key::PageDown => Some(direction(
            KeyboardAction::Step(-10),
            KeyboardAction::Step(10),
        )),
        Key::Home => Some(KeyboardAction::ToMin),
        Key::End => Some(KeyboardAction::ToMax),
        _ => None,
    }
}

/// Apply a keyboard action to the current value.
pub fn apply_keyboard_action(current: f64, action: KeyboardAction, math: &SliderMath) -> f64 {
    match action {
        KeyboardAction::Step(delta) => {
            let step = math.step.unwrap_or(1.0).abs();
            let next = current + (delta as f64) * step;
            snap_value(next, math)
        }
        KeyboardAction::ToMin => snap_value(math.normalized().min, math),
        KeyboardAction::ToMax => snap_value(math.normalized().max, math),
    }
}

/// Convert pointer position to ratio using track bounding box.
#[cfg(target_arch = "wasm32")]
pub fn ratio_from_pointer_event(
    event: &web_sys::PointerEvent,
    rect: &web_sys::DomRect,
    math: &SliderMath,
) -> Option<f64> {
    let axis_size = if math.orientation.is_vertical() {
        rect.height()
    } else {
        rect.width()
    };
    if axis_size <= 0.0 {
        return None;
    }

    let origin = if math.orientation.is_vertical() {
        rect.y()
    } else {
        rect.x()
    };
    let position = if math.orientation.is_vertical() {
        event.client_y() as f64
    } else {
        event.client_x() as f64
    };

    let mut ratio = (position - origin) / axis_size;
    ratio = ratio.clamp(0.0, 1.0);
    if math.reverse {
        ratio = 1.0 - ratio;
    }
    Some(ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_respects_step_and_precision() {
        let math = SliderMath {
            min: 0.0,
            max: 5.0,
            step: Some(0.3),
            precision: Some(2),
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        assert_eq!(snap_value(1.11, &math), 1.2);
        assert_eq!(snap_value(5.5, &math), 5.0);
    }

    #[test]
    fn ratio_conversion_respects_reverse() {
        let math = SliderMath {
            min: 0.0,
            max: 100.0,
            step: Some(1.0),
            precision: None,
            reverse: true,
            orientation: SliderOrientation::Horizontal,
        };
        // Midpoint stays midpoint
        assert!((value_to_ratio(50.0, &math) - 0.5).abs() < 1e-6);
        // Lower value yields higher ratio when reversed
        assert!(value_to_ratio(10.0, &math) > value_to_ratio(90.0, &math));
    }

    #[test]
    fn ratio_roundtrip() {
        let math = SliderMath {
            min: -10.0,
            max: 10.0,
            step: Some(0.5),
            precision: Some(1),
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let ratio = value_to_ratio(2.5, &math);
        let back = ratio_to_value(ratio, &math);
        assert!((back - 2.5).abs() < 1e-6);
    }

    #[test]
    fn keyboard_actions_apply_steps() {
        let math = SliderMath {
            min: 0.0,
            max: 10.0,
            step: Some(1.0),
            precision: None,
            reverse: false,
            orientation: SliderOrientation::Horizontal,
        };
        let next = apply_keyboard_action(5.0, KeyboardAction::Step(1), &math);
        assert_eq!(next, 6.0);
        let prev = apply_keyboard_action(0.2, KeyboardAction::Step(-1), &math);
        assert_eq!(prev, 0.0);
        let maxed = apply_keyboard_action(3.0, KeyboardAction::ToMax, &math);
        assert_eq!(maxed, 10.0);
    }

    #[test]
    fn keyboard_action_for_key_respects_reverse() {
        let forward = keyboard_action_for_key(&Key::ArrowRight, false).unwrap();
        assert_eq!(forward, KeyboardAction::Step(1));
        let reversed = keyboard_action_for_key(&Key::ArrowRight, true).unwrap();
        assert_eq!(reversed, KeyboardAction::Step(-1));
    }
}
