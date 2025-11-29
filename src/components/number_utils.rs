//! Shared helpers for numeric controls (InputNumber, Slider, Rate, ColorPicker sliders).
//!
//! This module centralizes step/precision handling so components can stay focused on UI.

/// Normalization rules applied to numeric values.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NumberRules {
    /// Minimum allowed value.
    pub min: Option<f64>,
    /// Maximum allowed value.
    pub max: Option<f64>,
    /// Step used when incrementing/decrementing. Defaults to `1.0` when not set.
    pub step: Option<f64>,
    /// Optional decimal precision enforced on output.
    pub precision: Option<u32>,
}

impl NumberRules {
    /// Returns the effective step (>= 0) used for arithmetic.
    pub fn effective_step(&self) -> f64 {
        let step = self.step.unwrap_or(1.0);
        if step.is_sign_negative() { -step } else { step }
    }
}

/// Clamp a value into the optional min/max range.
pub fn clamp(value: f64, rules: &NumberRules) -> f64 {
    let after_min = if let Some(min) = rules.min {
        value.max(min)
    } else {
        value
    };
    if let Some(max) = rules.max {
        after_min.min(max)
    } else {
        after_min
    }
}

/// Round a value with the given precision, leaving untouched when precision is not set.
pub fn round_with_precision(value: f64, precision: Option<u32>) -> f64 {
    if let Some(p) = precision {
        if p == 0 {
            value.round()
        } else {
            let factor = 10_f64.powi(p as i32);
            (value * factor).round() / factor
        }
    } else {
        value
    }
}

/// Apply a delta in step units, clamp, then round with precision.
pub fn apply_step(value: f64, delta_steps: i32, rules: &NumberRules) -> f64 {
    let step = rules.effective_step();
    let next = value + step * (delta_steps as f64);
    let clamped = clamp(next, rules);
    round_with_precision(clamped, rules.precision)
}

/// Parse a string into a number and clamp/round it. Returns `None` on invalid input.
pub fn parse_and_normalize(input: &str, rules: &NumberRules) -> Option<f64> {
    let raw = input.trim();
    if raw.is_empty() {
        return None;
    }
    let parsed: f64 = raw.parse().ok()?;
    let clamped = clamp(parsed, rules);
    Some(round_with_precision(clamped, rules.precision))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_respects_bounds() {
        let rules = NumberRules {
            min: Some(0.0),
            max: Some(10.0),
            step: None,
            precision: None,
        };
        assert_eq!(clamp(-2.0, &rules), 0.0);
        assert_eq!(clamp(5.5, &rules), 5.5);
        assert_eq!(clamp(20.0, &rules), 10.0);
    }

    #[test]
    fn apply_step_clamps_and_rounds() {
        let rules = NumberRules {
            min: Some(0.0),
            max: Some(2.0),
            step: Some(0.3),
            precision: Some(2),
        };
        // Start from 1.0, add two steps (0.6) -> 1.6
        assert_eq!(apply_step(1.0, 2, &rules), 1.6);
        // Go below min
        assert_eq!(apply_step(0.2, -2, &rules), 0.0);
        // Go above max
        assert_eq!(apply_step(1.9, 2, &rules), 2.0);
    }

    #[test]
    fn parse_and_normalize_handles_precision() {
        let rules = NumberRules {
            min: None,
            max: None,
            step: None,
            precision: Some(1),
        };
        assert_eq!(parse_and_normalize("3.1415", &rules), Some(3.1));
        assert_eq!(parse_and_normalize("   2.05 ", &rules), Some(2.1));
        assert_eq!(parse_and_normalize("", &rules), None);
        assert_eq!(parse_and_normalize("abc", &rules), None);
    }
}
