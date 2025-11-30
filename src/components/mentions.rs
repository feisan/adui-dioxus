//! Mentions component for @-style mentions in text input.
//!
//! A textarea-like input that shows a dropdown when the user types a trigger
//! character (default `@`) to allow selecting from a list of options.

use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::control::ControlStatus;
use dioxus::prelude::*;

/// A single mention option.
#[derive(Clone, Debug, PartialEq)]
pub struct MentionOption {
    /// The value to insert when selected.
    pub value: String,
    /// The label to display in the dropdown.
    pub label: String,
    /// Whether this option is disabled.
    pub disabled: bool,
}

impl MentionOption {
    /// Create a new mention option.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Create a simple option where value equals label.
    pub fn simple(value: impl Into<String>) -> Self {
        let v = value.into();
        Self {
            value: v.clone(),
            label: v,
            disabled: false,
        }
    }

    /// Builder method to set disabled state.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Placement of the dropdown popup.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MentionPlacement {
    /// Dropdown appears above the input.
    Top,
    /// Dropdown appears below the input (default).
    #[default]
    Bottom,
}

/// Props for the Mentions component.
#[derive(Props, Clone, PartialEq)]
pub struct MentionsProps {
    /// Controlled value of the textarea.
    #[props(optional)]
    pub value: Option<String>,
    /// Default value for uncontrolled usage.
    #[props(optional)]
    pub default_value: Option<String>,
    /// Placeholder text.
    #[props(optional)]
    pub placeholder: Option<String>,
    /// Available mention options.
    #[props(default)]
    pub options: Vec<MentionOption>,
    /// Trigger character(s) that open the dropdown.
    #[props(default = vec!['@'])]
    pub prefix: Vec<char>,
    /// Character that separates mentions from other text.
    #[props(default = ' ')]
    pub split: char,
    /// Whether the component is disabled.
    #[props(default)]
    pub disabled: bool,
    /// Whether the component is read-only.
    #[props(default)]
    pub read_only: bool,
    /// Whether to show a loading indicator in the dropdown.
    #[props(default)]
    pub loading: bool,
    /// Status for validation styling.
    #[props(optional)]
    pub status: Option<ControlStatus>,
    /// Size variant.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Dropdown placement.
    #[props(default)]
    pub placement: MentionPlacement,
    /// Whether to auto-focus the input.
    #[props(default)]
    pub auto_focus: bool,
    /// Number of rows for the textarea.
    #[props(default = 1)]
    pub rows: u32,
    /// Callback when the value changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Callback when an option is selected.
    #[props(optional)]
    pub on_select: Option<EventHandler<MentionOption>>,
    /// Callback when search text changes.
    #[props(optional)]
    pub on_search: Option<EventHandler<(String, char)>>,
    /// Callback when focus changes.
    #[props(optional)]
    pub on_focus: Option<EventHandler<()>>,
    /// Callback when blur occurs.
    #[props(optional)]
    pub on_blur: Option<EventHandler<()>>,
    /// Custom filter function.
    #[props(optional)]
    pub filter_option: Option<fn(&str, &MentionOption) -> bool>,
    /// Extra class for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
}

/// Mentions component for @-style input.
#[component]
pub fn Mentions(props: MentionsProps) -> Element {
    let MentionsProps {
        value,
        default_value,
        placeholder,
        options,
        prefix,
        split,
        disabled,
        read_only,
        loading,
        status,
        size,
        placement,
        auto_focus: _auto_focus,
        rows,
        on_change,
        on_select,
        on_search,
        on_focus,
        on_blur,
        filter_option,
        class,
        style,
    } = props;

    let config = use_config();
    let final_size = size.unwrap_or(config.size);

    // Internal value state
    let mut internal_value: Signal<String> =
        use_signal(|| default_value.clone().unwrap_or_default());
    let current_value = value
        .clone()
        .unwrap_or_else(|| internal_value.read().clone());

    // Dropdown state
    let mut dropdown_open: Signal<bool> = use_signal(|| false);
    let mut active_index: Signal<usize> = use_signal(|| 0);

    // Current search state
    let mut search_text: Signal<String> = use_signal(String::new);
    let mut trigger_char: Signal<Option<char>> = use_signal(|| None);
    let mut trigger_start: Signal<usize> = use_signal(|| 0);

    // Filter options based on search (clone to avoid lifetime issues)
    let filter_fn = filter_option.unwrap_or(default_filter);
    let search_val = search_text.read().clone();
    let filtered_options: Vec<MentionOption> = if *dropdown_open.read() {
        options
            .iter()
            .filter(|opt| filter_fn(&search_val, opt))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };

    // Build class list
    let mut class_list = vec!["adui-mentions".to_string()];
    if disabled {
        class_list.push("adui-mentions-disabled".into());
    }
    if *dropdown_open.read() {
        class_list.push("adui-mentions-open".into());
    }
    match final_size {
        ComponentSize::Small => class_list.push("adui-mentions-sm".into()),
        ComponentSize::Large => class_list.push("adui-mentions-lg".into()),
        ComponentSize::Middle => {}
    }
    if let Some(s) = status {
        match s {
            ControlStatus::Error => class_list.push("adui-control-status-error".into()),
            ControlStatus::Warning => class_list.push("adui-control-status-warning".into()),
            ControlStatus::Success => class_list.push("adui-control-status-success".into()),
            ControlStatus::Default => {}
        }
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // Dropdown placement class
    let dropdown_class = match placement {
        MentionPlacement::Top => "adui-mentions-dropdown adui-mentions-dropdown-top",
        MentionPlacement::Bottom => "adui-mentions-dropdown adui-mentions-dropdown-bottom",
    };

    // Handle input change
    let handle_input = {
        let prefix_chars = prefix.clone();
        let on_change = on_change.clone();
        let on_search = on_search.clone();

        move |evt: Event<FormData>| {
            let new_value = evt.value();
            internal_value.set(new_value.clone());

            if let Some(handler) = &on_change {
                handler.call(new_value.clone());
            }

            // Check for trigger character
            let chars: Vec<char> = new_value.chars().collect();
            let len = chars.len();

            // Find the last trigger character
            let mut found_trigger: Option<(usize, char)> = None;
            for i in (0..len).rev() {
                if prefix_chars.contains(&chars[i]) {
                    // Check if this is a valid trigger position (start or after space)
                    if i == 0 || chars[i - 1].is_whitespace() {
                        found_trigger = Some((i, chars[i]));
                        break;
                    }
                }
                // If we hit a space, stop looking
                if chars[i].is_whitespace() {
                    break;
                }
            }

            if let Some((idx, ch)) = found_trigger {
                let search_str: String = chars[idx + 1..].iter().collect();
                if !search_str.contains(char::is_whitespace) {
                    dropdown_open.set(true);
                    trigger_char.set(Some(ch));
                    trigger_start.set(idx);
                    search_text.set(search_str.clone());
                    active_index.set(0);

                    if let Some(handler) = &on_search {
                        handler.call((search_str, ch));
                    }
                    return;
                }
            }

            // Close dropdown if no valid trigger
            dropdown_open.set(false);
            trigger_char.set(None);
            search_text.set(String::new());
        }
    };

    // Handle option selection
    let handle_select = {
        let on_select = on_select.clone();
        let on_change = on_change.clone();
        let current = current_value.clone();

        move |option: MentionOption| {
            if option.disabled {
                return;
            }

            // Insert the selected value
            let chars: Vec<char> = current.chars().collect();
            let trigger_idx = *trigger_start.read();

            let mut new_value = String::new();
            for (i, ch) in chars.iter().enumerate() {
                if i < trigger_idx {
                    new_value.push(*ch);
                } else if i == trigger_idx {
                    new_value.push(*ch); // Keep trigger char
                    new_value.push_str(&option.value);
                    new_value.push(split);
                    break;
                }
            }

            internal_value.set(new_value.clone());
            dropdown_open.set(false);

            if let Some(handler) = &on_select {
                handler.call(option);
            }
            if let Some(handler) = &on_change {
                handler.call(new_value);
            }
        }
    };

    // Handle keyboard navigation
    let handle_keydown = {
        let filtered_count = filtered_options.len();
        let filtered = filtered_options.clone();
        let handle_select = handle_select.clone();

        move |evt: Event<KeyboardData>| {
            if !*dropdown_open.read() {
                return;
            }

            match evt.key() {
                Key::ArrowDown => {
                    evt.stop_propagation();
                    let curr = *active_index.read();
                    if curr + 1 < filtered_count {
                        active_index.set(curr + 1);
                    } else {
                        active_index.set(0);
                    }
                }
                Key::ArrowUp => {
                    evt.stop_propagation();
                    let curr = *active_index.read();
                    if curr > 0 {
                        active_index.set(curr - 1);
                    } else if filtered_count > 0 {
                        active_index.set(filtered_count - 1);
                    }
                }
                Key::Enter => {
                    evt.stop_propagation();
                    let curr = *active_index.read();
                    if curr < filtered.len() {
                        let option = filtered[curr].clone();
                        let mut sel = handle_select.clone();
                        sel(option);
                    }
                }
                Key::Escape => {
                    evt.stop_propagation();
                    dropdown_open.set(false);
                }
                _ => {}
            }
        }
    };

    // Focus/blur handlers
    let handle_focus = {
        let on_focus = on_focus.clone();
        move |_| {
            if let Some(handler) = &on_focus {
                handler.call(());
            }
        }
    };

    let handle_blur = {
        let on_blur = on_blur.clone();
        move |_| {
            // Delay closing to allow click on dropdown
            dropdown_open.set(false);
            if let Some(handler) = &on_blur {
                handler.call(());
            }
        }
    };

    let active_idx = *active_index.read();

    rsx! {
        div { class: "adui-mentions-root",
            textarea {
                class: "{class_attr}",
                style: "{style_attr}",
                rows: "{rows}",
                placeholder: placeholder.clone().unwrap_or_default(),
                disabled: disabled,
                readonly: read_only,
                value: "{current_value}",
                oninput: handle_input,
                onkeydown: handle_keydown,
                onfocus: handle_focus,
                onblur: handle_blur,
            }

            // Dropdown
            if *dropdown_open.read() && (!filtered_options.is_empty() || loading) {
                div { class: "{dropdown_class}",
                    if loading {
                        div { class: "adui-mentions-dropdown-loading", "Loading..." }
                    } else {
                        ul { class: "adui-mentions-dropdown-list",
                            for (i, option) in filtered_options.iter().enumerate() {
                                li {
                                    key: "{option.value}",
                                    class: {
                                        let mut c = vec!["adui-mentions-dropdown-item".to_string()];
                                        if i == active_idx {
                                            c.push("adui-mentions-dropdown-item-active".into());
                                        }
                                        if option.disabled {
                                            c.push("adui-mentions-dropdown-item-disabled".into());
                                        }
                                        c.join(" ")
                                    },
                                    // Use onmousedown instead of onclick because mousedown fires
                                    // before blur, allowing selection before dropdown closes
                                    onmousedown: {
                                        let opt = option.clone();
                                        let mut handler = handle_select.clone();
                                        move |evt: MouseEvent| {
                                            evt.prevent_default();
                                            handler(opt.clone());
                                        }
                                    },
                                    "{option.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Default filter function for options.
fn default_filter(query: &str, option: &MentionOption) -> bool {
    if query.is_empty() {
        return true;
    }
    let query_lower = query.to_lowercase();
    option.label.to_lowercase().contains(&query_lower)
        || option.value.to_lowercase().contains(&query_lower)
}

/// Extract all mentions from a text value.
pub fn get_mentions(value: &str, prefix: &[char]) -> Vec<MentionEntity> {
    let mut mentions = Vec::new();
    let chars: Vec<char> = value.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if prefix.contains(&chars[i]) {
            // Check if valid trigger position
            if i == 0 || chars[i - 1].is_whitespace() {
                let trigger = chars[i];
                let start = i + 1;
                let mut end = start;

                // Find end of mention (next space or end of string)
                while end < len && !chars[end].is_whitespace() {
                    end += 1;
                }

                if end > start {
                    let value: String = chars[start..end].iter().collect();
                    mentions.push(MentionEntity {
                        prefix: trigger,
                        value,
                    });
                }
                i = end;
                continue;
            }
        }
        i += 1;
    }

    mentions
}

/// Represents a parsed mention entity.
#[derive(Clone, Debug, PartialEq)]
pub struct MentionEntity {
    /// The trigger character used.
    pub prefix: char,
    /// The mention value (without the prefix).
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mention_option_builder() {
        let opt = MentionOption::new("john", "John Doe").with_disabled(true);
        assert_eq!(opt.value, "john");
        assert_eq!(opt.label, "John Doe");
        assert!(opt.disabled);
    }

    #[test]
    fn mention_option_simple() {
        let opt = MentionOption::simple("alice");
        assert_eq!(opt.value, "alice");
        assert_eq!(opt.label, "alice");
        assert!(!opt.disabled);
    }

    #[test]
    fn default_filter_matches() {
        let opt = MentionOption::new("john", "John Doe");
        assert!(default_filter("", &opt));
        assert!(default_filter("john", &opt));
        assert!(default_filter("DOE", &opt));
        assert!(!default_filter("xyz", &opt));
    }

    #[test]
    fn get_mentions_parses_correctly() {
        let text = "@john hello @alice how are you";
        let mentions = get_mentions(text, &['@']);
        assert_eq!(mentions.len(), 2);
        assert_eq!(mentions[0].prefix, '@');
        assert_eq!(mentions[0].value, "john");
        assert_eq!(mentions[1].prefix, '@');
        assert_eq!(mentions[1].value, "alice");
    }

    #[test]
    fn get_mentions_ignores_mid_word() {
        let text = "email@example.com";
        let mentions = get_mentions(text, &['@']);
        assert!(mentions.is_empty());
    }

    #[test]
    fn get_mentions_multiple_prefixes() {
        let text = "@user #tag";
        let mentions = get_mentions(text, &['@', '#']);
        assert_eq!(mentions.len(), 2);
        assert_eq!(mentions[0].prefix, '@');
        assert_eq!(mentions[0].value, "user");
        assert_eq!(mentions[1].prefix, '#');
        assert_eq!(mentions[1].value, "tag");
    }
}
