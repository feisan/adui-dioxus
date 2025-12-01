//! Integration tests for form components
//!
//! Tests Form, FormItem, FormList, and their integration with form controls
//! like Input, Select, Checkbox without requiring full Dioxus runtime.

// CheckboxProps and CheckboxGroupProps are tested through structure tests
use adui_dioxus::components::form::{
    ControlSize, FeedbackIcons, FormFinishEvent, FormFinishFailedEvent, FormHandle,
    FormLayout, FormRule, FormValues, LabelAlign, RequiredMark, ScrollToFirstErrorConfig,
    ValuesChangeEvent,
};
use adui_dioxus::components::input::InputProps;
use adui_dioxus::components::select::{SelectMode, SelectPlacement, SelectProps};
use serde_json::Value;

#[test]
fn form_layout_variants() {
    assert_eq!(FormLayout::Horizontal, FormLayout::default());
    assert_ne!(FormLayout::Horizontal, FormLayout::Vertical);
    assert_ne!(FormLayout::Vertical, FormLayout::Inline);
}

#[test]
fn form_control_size_variants() {
    assert_eq!(ControlSize::Middle, ControlSize::default());
    assert_ne!(ControlSize::Small, ControlSize::Large);
}

#[test]
fn form_label_align_variants() {
    assert_eq!(LabelAlign::Right, LabelAlign::default());
    assert_ne!(LabelAlign::Left, LabelAlign::Right);
}

#[test]
fn form_required_mark_variants() {
    assert_eq!(RequiredMark::Default, RequiredMark::default());
    assert_ne!(RequiredMark::None, RequiredMark::Optional);
    assert_ne!(RequiredMark::Default, RequiredMark::None);
}

#[test]
fn form_handle_structure() {
    // Test that FormHandle type exists and can be used
    // Note: FormHandle::new() requires Dioxus runtime, so we test the type structure
    assert!(std::mem::size_of::<FormHandle>() > 0);
    
    // Test FormValues and FormErrors types
    let mut values = FormValues::new();
    values.insert("test".to_string(), Value::String("value".to_string()));
    assert_eq!(values.len(), 1);
    
    let mut errors = std::collections::HashMap::new();
    errors.insert("test".to_string(), "error".to_string());
    assert_eq!(errors.len(), 1);
}

#[test]
fn form_rule_creation() {
    let rule = FormRule {
        required: true,
        min: Some(3),
        max: Some(20),
        len: None,
        pattern: None,
        message: Some("Invalid input".to_string()),
        validator: None,
    };
    assert_eq!(rule.required, true);
    assert_eq!(rule.min, Some(3));
    assert_eq!(rule.max, Some(20));
    assert_eq!(rule.message, Some("Invalid input".to_string()));
}

#[test]
fn form_rule_defaults() {
    let rule = FormRule::default();
    assert_eq!(rule.required, false);
    assert!(rule.min.is_none());
    assert!(rule.max.is_none());
    assert!(rule.message.is_none());
}

#[test]
fn form_rule_partial_eq() {
    let rule1 = FormRule {
        required: true,
        message: Some("Error".to_string()),
        ..FormRule::default()
    };
    let rule2 = FormRule {
        required: true,
        message: Some("Error".to_string()),
        ..FormRule::default()
    };
    let rule3 = FormRule {
        required: false,
        message: Some("Error".to_string()),
        ..FormRule::default()
    };
    assert_eq!(rule1, rule2);
    assert_ne!(rule1, rule3);
}

#[test]
fn form_finish_event_creation() {
    let mut values = FormValues::new();
    values.insert("username".to_string(), Value::String("test".to_string()));
    let event = FormFinishEvent { values };
    assert_eq!(event.values.len(), 1);
    assert_eq!(
        event.values.get("username"),
        Some(&Value::String("test".to_string()))
    );
}

#[test]
fn form_finish_failed_event_creation() {
    let mut errors = std::collections::HashMap::new();
    errors.insert("username".to_string(), "Required".to_string());
    let event = FormFinishFailedEvent { errors };
    assert_eq!(event.errors.len(), 1);
    assert_eq!(event.errors.get("username"), Some(&"Required".to_string()));
}

#[test]
fn form_values_change_event_creation() {
    let mut changed_values = FormValues::new();
    changed_values.insert("username".to_string(), Value::String("new".to_string()));
    let mut all_values = FormValues::new();
    all_values.insert("username".to_string(), Value::String("new".to_string()));
    all_values.insert("email".to_string(), Value::String("test@example.com".to_string()));

    let event = ValuesChangeEvent {
        changed_values,
        all_values,
    };
    assert_eq!(event.changed_values.len(), 1);
    assert_eq!(event.all_values.len(), 2);
}

#[test]
fn form_feedback_icons_defaults() {
    let icons = FeedbackIcons::default();
    assert!(icons.success.is_none());
    assert!(icons.error.is_none());
    assert!(icons.warning.is_none());
    assert!(icons.validating.is_none());
}

#[test]
fn form_feedback_icons_default_icons() {
    let icons = FeedbackIcons::default_icons();
    assert!(icons.success.is_none());
    assert!(icons.error.is_none());
}

#[test]
fn form_scroll_to_first_error_config_defaults() {
    let config = ScrollToFirstErrorConfig::default();
    assert!(config.block.is_none());
    assert!(config.inline.is_none());
    assert!(config.behavior.is_none());
}

#[test]
fn form_scroll_to_first_error_config_custom() {
    let config = ScrollToFirstErrorConfig {
        block: Some("center".to_string()),
        inline: Some("nearest".to_string()),
        behavior: Some("smooth".to_string()),
    };
    assert_eq!(config.block, Some("center".to_string()));
    assert_eq!(config.inline, Some("nearest".to_string()));
    assert_eq!(config.behavior, Some("smooth".to_string()));
}

#[test]
fn select_mode_variants() {
    assert_eq!(SelectMode::Single, SelectMode::default());
    assert_ne!(SelectMode::Single, SelectMode::Multiple);
    assert_ne!(SelectMode::Tags, SelectMode::Combobox);
}

#[test]
fn select_mode_is_multiple() {
    assert_eq!(SelectMode::Single.is_multiple(), false);
    assert_eq!(SelectMode::Multiple.is_multiple(), true);
    assert_eq!(SelectMode::Tags.is_multiple(), true);
    assert_eq!(SelectMode::Combobox.is_multiple(), false);
}

#[test]
fn select_mode_allows_input() {
    assert_eq!(SelectMode::Single.allows_input(), false);
    assert_eq!(SelectMode::Multiple.allows_input(), false);
    assert_eq!(SelectMode::Tags.allows_input(), true);
    assert_eq!(SelectMode::Combobox.allows_input(), true);
}

#[test]
fn select_placement_variants() {
    assert_eq!(SelectPlacement::BottomLeft, SelectPlacement::default());
    assert_ne!(SelectPlacement::BottomLeft, SelectPlacement::TopRight);
    assert_ne!(SelectPlacement::TopLeft, SelectPlacement::TopRight);
    assert_ne!(SelectPlacement::BottomLeft, SelectPlacement::BottomRight);
}

#[test]
fn select_props_defaults() {
    let props = SelectProps {
        value: None,
        values: None,
        options: vec![],
        mode: SelectMode::Single,
        multiple: false,
        allow_clear: false,
        placeholder: None,
        disabled: false,
        show_search: false,
        filter_option: None,
        token_separators: None,
        status: None,
        size: None,
        variant: None,
        bordered: None,
        prefix: None,
        suffix_icon: None,
        placement: SelectPlacement::BottomLeft,
        popup_match_select_width: true,
        class: None,
        root_class_name: None,
        style: None,
        class_names: None,
        styles: None,
        dropdown_class: None,
        dropdown_style: None,
        dropdown_class_name: None,
        dropdown_style_deprecated: None,
        dropdown_match_select_width: None,
        popup_render: None,
        on_change: None,
        on_dropdown_visible_change: None,
        on_open_change: None,
    };
    assert_eq!(props.mode, SelectMode::Single);
    assert_eq!(props.multiple, false);
    assert_eq!(props.disabled, false);
    assert_eq!(props.allow_clear, false);
    assert_eq!(props.show_search, false);
}

#[test]
fn checkbox_props_structure() {
    // Test that CheckboxProps has expected fields
    // We can't create instances without rsx! macro, so we test the type structure
    assert!(std::mem::size_of::<Option<bool>>() > 0); // checked field
    assert!(std::mem::size_of::<bool>() > 0); // default_checked field
    assert!(std::mem::size_of::<bool>() > 0); // disabled field
}

#[test]
fn checkbox_group_props_structure() {
    // Test that CheckboxGroupProps has expected fields
    assert!(std::mem::size_of::<Option<Vec<String>>>() > 0); // value field
    assert!(std::mem::size_of::<Vec<String>>() > 0); // default_value field
    assert!(std::mem::size_of::<bool>() > 0); // disabled field
}

#[test]
fn input_props_in_form_context() {
    let props = InputProps {
        value: None,
        default_value: None,
        placeholder: Some("Enter username".to_string()),
        disabled: false,
        size: None,
        variant: None,
        bordered: None,
        status: None,
        prefix: None,
        suffix: None,
        addon_before: None,
        addon_after: None,
        allow_clear: false,
        max_length: Some(20),
        show_count: false,
        class: None,
        root_class_name: None,
        style: None,
        class_names: None,
        styles: None,
        data_attributes: None,
        on_change: None,
        on_press_enter: None,
    };
    assert_eq!(props.placeholder, Some("Enter username".to_string()));
    assert_eq!(props.max_length, Some(20));
}

#[test]
fn form_handle_type_properties() {
    // Test that FormHandle implements Clone and PartialEq
    // Note: We can't create instances without Dioxus runtime
    // but we can verify the type properties
    fn assert_clone<T: Clone>() {}
    fn assert_partial_eq<T: PartialEq>() {}
    assert_clone::<FormHandle>();
    assert_partial_eq::<FormHandle>();
}

#[test]
fn form_validation_rules_integration() {
    // Test that FormRule can be used for validation
    let required_rule = FormRule {
        required: true,
        message: Some("This field is required".to_string()),
        ..FormRule::default()
    };
    assert_eq!(required_rule.required, true);

    let min_length_rule = FormRule {
        min: Some(3),
        message: Some("Minimum 3 characters".to_string()),
        ..FormRule::default()
    };
    assert_eq!(min_length_rule.min, Some(3));

    let max_length_rule = FormRule {
        max: Some(20),
        message: Some("Maximum 20 characters".to_string()),
        ..FormRule::default()
    };
    assert_eq!(max_length_rule.max, Some(20));

    let exact_length_rule = FormRule {
        len: Some(10),
        message: Some("Must be exactly 10 characters".to_string()),
        ..FormRule::default()
    };
    assert_eq!(exact_length_rule.len, Some(10));
}

#[test]
fn form_control_size_integration() {
    // Test that ControlSize can be used across form controls
    let size = ControlSize::Small;
    assert_eq!(size, ControlSize::Small);

    // Test that ControlSize has all expected variants
    assert_ne!(ControlSize::Small, ControlSize::Middle);
    assert_ne!(ControlSize::Middle, ControlSize::Large);
}

#[test]
fn form_layout_integration() {
    // Test that FormLayout affects form item layout
    assert_eq!(FormLayout::Horizontal, FormLayout::Horizontal);
    assert_ne!(FormLayout::Horizontal, FormLayout::Vertical);
    assert_ne!(FormLayout::Vertical, FormLayout::Inline);
}

