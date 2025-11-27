use serde::{Deserialize, Serialize};
use serde_json::Value;

use dioxus::{events::KeyboardEvent, prelude::*};

use crate::components::form::{
    form_value_to_radio_key,
    form_value_to_string_vec,
};
use crate::components::overlay::{use_overlay, OverlayKey, OverlayKind};

/// Shared key type used by selector-like components.
///
/// We normalise all option keys to owned `String` values so they can be
/// round-tripped through `serde_json::Value` and the Form store easily.
pub type OptionKey = String;

/// Flat option used by `Select` and `AutoComplete`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectOption {
    pub key: OptionKey,
    pub label: String,
    #[serde(default)]
    pub disabled: bool,
}

/// Tree-shaped option node shared by `TreeSelect` and `Cascader`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionNode {
    pub key: OptionKey,
    pub label: String,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub children: Vec<OptionNode>,
}

/// Alias for clarity when used by `TreeSelect`.
pub type TreeNode = OptionNode;

/// Alias for clarity when used by `Cascader`.
pub type CascaderNode = OptionNode;

// ---- Value mapping helpers -------------------------------------------------

/// Convert a `serde_json::Value` coming from Form into an optional `OptionKey`.
///
/// This delegates to `form_value_to_radio_key` so that selector components are
/// consistent with Radio/Checkbox semantics.
pub fn value_to_option_key(val: Option<Value>) -> Option<OptionKey> {
    form_value_to_radio_key(val)
}

/// Convert a single key into a JSON value suitable for storing in Form.
pub fn option_key_to_value(key: &OptionKey) -> Value {
    Value::String(key.clone())
}

/// Convert a JSON value into a vector of option keys.
///
/// This is primarily used for multi-select scenarios and internally reuses
/// `form_value_to_string_vec` (which expects an array of strings).
pub fn value_to_option_keys(val: Option<Value>) -> Vec<OptionKey> {
    form_value_to_string_vec(val)
}

/// Convert a slice of keys into a JSON array value.
pub fn option_keys_to_value(keys: &[OptionKey]) -> Value {
    let items = keys.iter().cloned().map(Value::String).collect();
    Value::Array(items)
}

/// Convert a JSON value into a path of keys (used by Cascader).
///
/// Paths are represented as a simple array of strings such as
/// `["zhejiang", "hangzhou"]`.
pub fn value_to_path(val: Option<Value>) -> Vec<OptionKey> {
    match val {
        Some(Value::Array(items)) => items
            .into_iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    }
}

/// Convert a path of keys into a JSON array value.
pub fn path_to_value(path: &[OptionKey]) -> Value {
    option_keys_to_value(path)
}

// ---- Option list helpers ---------------------------------------------------

/// Filter options by label using a case-insensitive `contains` match.
///
/// An empty query returns the original options unchanged (cloned).
pub fn filter_options_by_query(options: &[SelectOption], query: &str) -> Vec<SelectOption> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return options.to_vec();
    }
    let q = trimmed.to_lowercase();
    options
        .iter()
        .filter(|opt| opt.label.to_lowercase().contains(&q))
        .cloned()
        .collect()
}

/// Toggle a single key within a set of keys.
///
/// If the key already exists it is removed, otherwise it is appended. This is
/// the shared primitive for multi-select style components.
pub fn toggle_option_key(current: &[OptionKey], key: &str) -> Vec<OptionKey> {
    let mut next = current.to_vec();
    if let Some(pos) = next.iter().position(|v| v == key) {
        next.remove(pos);
    } else {
        next.push(key.to_string());
    }
    next
}

// ---- Dropdown layer & keyboard helpers ------------------------------------

/// Lightweight handle describing an overlay entry reserved for a dropdown.
///
/// Components using selector-style dropdowns can consume this via
/// `use_dropdown_layer` and place their popup content using the provided
/// `z_index`.
#[derive(Clone, Copy)]
pub struct DropdownLayer {
    pub key: Signal<Option<OverlayKey>>,
    pub z_index: Signal<i32>,
}

/// Hook: register/unregister a dropdown entry with the global `OverlayManager`.
///
/// - When `open` is true, ensures an overlay entry of kind `Dropdown` exists
///   and exposes its z-index via the returned `DropdownLayer`.
/// - When `open` becomes false, releases the overlay entry so that z-index
///   space can be reused.
///
/// If no overlay provider is present in the tree, this falls back to a fixed
/// z-index of 1000.
pub fn use_dropdown_layer(open: bool) -> DropdownLayer {
    let overlay = use_overlay();
    let dropdown_key: Signal<Option<OverlayKey>> = use_signal(|| None);
    let z_index: Signal<i32> = use_signal(|| 1000);

    {
        let overlay = overlay.clone();
        let mut key_signal = dropdown_key;
        let mut z_signal = z_index;
        use_effect(move || {
            if let Some(handle) = overlay.clone() {
                let current_key = *key_signal.read();
                if open {
                    if current_key.is_none() {
                        let (key, meta) = handle.open(OverlayKind::Dropdown, false);
                        z_signal.set(meta.z_index);
                        key_signal.set(Some(key));
                    }
                } else if let Some(key) = current_key {
                    handle.close(key);
                    key_signal.set(None);
                }
            }
        });
    }

    DropdownLayer {
        key: dropdown_key,
        z_index,
    }
}

/// Internal helper: compute the next active index in a linear option list.
///
/// This pure function is used by keyboard handlers and is easy to test :
///
/// - `current` is the currently highlighted index (if any)
/// - `len` is the total number of options
/// - `direction` is +1 for moving forward (ArrowDown) and -1 for moving
///   backward (ArrowUp)
pub fn next_active_index(current: Option<usize>, len: usize, direction: i32) -> Option<usize> {
    if len == 0 {
        return None;
    }

    let dir = if direction >= 0 { 1 } else { -1 };

    match current {
        None => {
            if dir > 0 {
                Some(0)
            } else {
                Some(len.saturating_sub(1))
            }
        }
        Some(idx) => {
            if len == 1 {
                Some(0)
            } else if dir > 0 {
                Some((idx + 1) % len)
            } else {
                Some((idx + len - 1) % len)
            }
        }
    }
}

/// Handle a key event for an option list and update the active index.
///
/// Returns `Some(index)` when the user confirms a selection via Enter; callers
/// can then perform the corresponding option activation.
pub fn handle_option_list_key_event(
    evt: &KeyboardEvent,
    options_len: usize,
    active_index_signal: &Signal<Option<usize>>,
) -> Option<usize> {
    use dioxus::prelude::Key;

    match evt.key() {
        Key::ArrowDown => {
            let mut signal = *active_index_signal;
            let current = *signal.read();
            let next = next_active_index(current, options_len, 1);
            signal.set(next);
            None
        }
        Key::ArrowUp => {
            let mut signal = *active_index_signal;
            let current = *signal.read();
            let next = next_active_index(current, options_len, -1);
            signal.set(next);
            None
        }
        Key::Enter => {
            let current = *active_index_signal.read();
            current
        }
        Key::Escape => {
            let mut signal = *active_index_signal;
            signal.set(None);
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_to_and_from_single_key_round_trips() {
        let original = OptionKey::from("foo");
        let json = option_key_to_value(&original);
        let parsed = value_to_option_key(Some(json));
        assert_eq!(parsed, Some(original));
    }

    #[test]
    fn value_to_and_from_multiple_keys_round_trips() {
        let keys = vec!["a".to_string(), "b".to_string()];
        let json = option_keys_to_value(&keys);
        let parsed = value_to_option_keys(Some(json));
        assert_eq!(parsed, keys);
    }

    #[test]
    fn path_conversion_uses_string_array_representation() {
        let path = vec!["root".to_string(), "child".to_string()];
        let json = path_to_value(&path);
        let parsed = value_to_path(Some(json));
        assert_eq!(parsed, path);
    }

    #[test]
    fn toggle_option_key_adds_and_removes_values() {
        let current: Vec<OptionKey> = vec![];
        let next = toggle_option_key(&current, "x");
        assert_eq!(next, vec!["x".to_string()]);

        let next2 = toggle_option_key(&next, "x");
        assert!(next2.is_empty());
    }

    #[test]
    fn filter_options_by_query_matches_label_case_insensitively() {
        let options = vec![
            SelectOption { key: "1".into(), label: "Apple".into(), disabled: false },
            SelectOption { key: "2".into(), label: "Banana".into(), disabled: false },
            SelectOption { key: "3".into(), label: "Cherry".into(), disabled: false },
        ];
        let filtered = filter_options_by_query(&options, "an");
        let labels: Vec<String> = filtered.into_iter().map(|o| o.label).collect();
        assert_eq!(labels, vec!["Banana".to_string()]);
    }

    #[test]
    fn next_active_index_wraps_and_handles_empty() {
        // Empty list should always return None.
        assert_eq!(next_active_index(None, 0, 1), None);
        assert_eq!(next_active_index(Some(0), 0, 1), None);

        // First move in non-empty list.
        assert_eq!(next_active_index(None, 3, 1), Some(0));
        assert_eq!(next_active_index(None, 3, -1), Some(2));

        // Forward and backward wrapping.
        assert_eq!(next_active_index(Some(0), 3, 1), Some(1));
        assert_eq!(next_active_index(Some(2), 3, 1), Some(0));
        assert_eq!(next_active_index(Some(0), 3, -1), Some(2));
        assert_eq!(next_active_index(Some(1), 3, -1), Some(0));
    }
}
