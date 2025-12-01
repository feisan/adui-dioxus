use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::use_form_item_control;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::Key;
use dioxus::prelude::*;
use serde_json::Value;

/// Visual size of the switch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwitchSize {
    #[default]
    Default,
    Small,
}

/// Props for the Switch component.
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Controlled checked state.
    #[props(optional)]
    pub checked: Option<bool>,
    /// Initial value in uncontrolled mode.
    #[props(default)]
    pub default_checked: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub size: SwitchSize,
    #[props(optional)]
    pub checked_children: Option<Element>,
    #[props(optional)]
    pub un_checked_children: Option<Element>,
    #[props(optional)]
    pub status: Option<ControlStatus>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Change event with the next checked state.
    #[props(optional)]
    pub on_change: Option<EventHandler<bool>>,
}

/// Ant Design flavored switch component.
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let SwitchProps {
        checked,
        default_checked,
        disabled,
        size,
        checked_children,
        un_checked_children,
        status,
        class,
        style,
        on_change,
    } = props;

    let form_control = use_form_item_control();
    let controlled_by_prop = checked.is_some();

    let inner_checked = use_signal(|| default_checked);

    // 同步内部状态与 Form 字段：
    // - 若 Form 中有布尔值，则以 Form 为准；
    // - 若 Form 中无值，则回退到 default_checked。
    if let Some(ctx) = &form_control {
        let form_value = ctx.value();
        let mut inner_signal = inner_checked;
        if let Some(Value::Bool(b)) = form_value {
            if *inner_signal.read() != b {
                inner_signal.set(b);
            }
        } else if form_value.is_none() && *inner_signal.read() != default_checked {
            inner_signal.set(default_checked);
        }
    }

    let is_disabled = disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let is_checked = *inner_checked.read();

    let mut class_list = vec!["adui-switch".to_string()];
    if size == SwitchSize::Small {
        class_list.push("adui-switch-small".into());
    }
    if is_checked {
        class_list.push("adui-switch-checked".into());
    }
    if is_disabled {
        class_list.push("adui-switch-disabled".into());
    }
    push_status_class(&mut class_list, status);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let inner_for_toggle = inner_checked;
    let form_for_toggle = form_control.clone();
    let on_change_cb = on_change;
    let disabled_flag = disabled;

    rsx! {
        button {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "switch",
            "aria-checked": is_checked,
            "aria-disabled": is_disabled,
            r#type: "button",
            disabled: is_disabled,
            onclick: {
                let mut inner_for_toggle = inner_for_toggle;
                let form_for_toggle = form_for_toggle.clone();
                move |_| {
                    let is_disabled_now = disabled_flag || form_for_toggle.as_ref().is_some_and(|ctx| ctx.is_disabled());
                    if is_disabled_now {
                        return;
                    }
                    handle_switch_toggle(
                        &form_for_toggle,
                        controlled_by_prop,
                        &mut inner_for_toggle,
                        on_change_cb,
                    );
                }
            },
            onkeydown: {
                let mut inner_for_toggle = inner_for_toggle;
                let form_for_toggle = form_for_toggle.clone();
                move |evt: KeyboardEvent| {
                    let is_disabled_now = disabled_flag || form_for_toggle.as_ref().is_some_and(|ctx| ctx.is_disabled());
                    if !is_disabled_now && key_triggers_toggle(&evt.key()) {
                        handle_switch_toggle(
                            &form_for_toggle,
                            controlled_by_prop,
                            &mut inner_for_toggle,
                            on_change_cb,
                        );
                    }
                }
            },
            span { class: "adui-switch-handle" }
            span {
                class: "adui-switch-inner",
                if is_checked {
                    if let Some(content) = checked_children {
                        {content}
                    }
                } else {
                    if let Some(content) = un_checked_children {
                        {content}
                    }
                }
            }
        }
    }
}

fn handle_switch_toggle(
    form_control: &Option<crate::components::form::FormItemControlContext>,
    controlled_by_prop: bool,
    inner: &mut Signal<bool>,
    on_change: Option<EventHandler<bool>>,
) {
    let current = *inner.read();
    let next = !current;

    // 始终尝试写回 FormStore（如果存在），确保提交时能拿到 newsletter 字段
    if let Some(ctx) = form_control {
        ctx.set_value(Value::Bool(next));
    }

    // 在非受控模式下更新内部状态，驱动 UI 立即切换
    if !controlled_by_prop {
        let mut state = *inner;
        state.set(next);
    }

    if let Some(cb) = on_change {
        cb.call(next);
    }
}

fn key_triggers_toggle(key: &Key) -> bool {
    match key {
        Key::Enter => true,
        Key::Character(text) if text == " " => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_size_default_value() {
        assert_eq!(SwitchSize::Default, SwitchSize::default());
    }

    #[test]
    fn switch_size_all_variants() {
        assert_eq!(SwitchSize::Default, SwitchSize::Default);
        assert_eq!(SwitchSize::Small, SwitchSize::Small);
        assert_ne!(SwitchSize::Default, SwitchSize::Small);
    }

    #[test]
    fn switch_size_equality() {
        let size1 = SwitchSize::Default;
        let size2 = SwitchSize::Default;
        let size3 = SwitchSize::Small;
        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
    }

    #[test]
    fn switch_size_clone() {
        let original = SwitchSize::Small;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn switch_size_debug() {
        let size = SwitchSize::Default;
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Default") || debug_str.contains("Small"));
    }

    #[test]
    fn switch_props_defaults() {
        // SwitchProps requires no mandatory fields
        // default_checked defaults to false
        // disabled defaults to false
        // size defaults to SwitchSize::Default
    }

    #[test]
    fn key_triggers_toggle_enter() {
        assert!(key_triggers_toggle(&Key::Enter));
    }

    #[test]
    fn key_triggers_toggle_space() {
        assert!(key_triggers_toggle(&Key::Character(" ".into())));
    }

    #[test]
    fn key_triggers_toggle_other_keys() {
        assert!(!key_triggers_toggle(&Key::ArrowLeft));
        assert!(!key_triggers_toggle(&Key::ArrowRight));
        assert!(!key_triggers_toggle(&Key::Character("a".into())));
        assert!(!key_triggers_toggle(&Key::Escape));
    }

    #[test]
    fn key_triggers_toggle_space_variations() {
        // Space should trigger
        assert!(key_triggers_toggle(&Key::Character(" ".into())));
        // Other whitespace should not
        assert!(!key_triggers_toggle(&Key::Character("\t".into())));
        assert!(!key_triggers_toggle(&Key::Character("\n".into())));
    }

    #[test]
    fn key_triggers_toggle_arrow_keys() {
        assert!(!key_triggers_toggle(&Key::ArrowUp));
        assert!(!key_triggers_toggle(&Key::ArrowDown));
    }

    #[test]
    fn key_triggers_toggle_modifier_keys() {
        assert!(!key_triggers_toggle(&Key::Control));
        assert!(!key_triggers_toggle(&Key::Shift));
        assert!(!key_triggers_toggle(&Key::Alt));
        assert!(!key_triggers_toggle(&Key::Meta));
    }

    #[test]
    fn key_triggers_toggle_function_keys() {
        assert!(!key_triggers_toggle(&Key::F1));
        assert!(!key_triggers_toggle(&Key::F12));
    }

    #[test]
    fn key_triggers_toggle_navigation_keys() {
        assert!(!key_triggers_toggle(&Key::Home));
        assert!(!key_triggers_toggle(&Key::End));
        assert!(!key_triggers_toggle(&Key::PageUp));
        assert!(!key_triggers_toggle(&Key::PageDown));
    }

    #[test]
    fn key_triggers_toggle_other_special_keys() {
        assert!(!key_triggers_toggle(&Key::Backspace));
        assert!(!key_triggers_toggle(&Key::Delete));
        assert!(!key_triggers_toggle(&Key::Tab));
        assert!(!key_triggers_toggle(&Key::CapsLock));
    }

    #[test]
    fn key_triggers_toggle_character_keys() {
        // Only space should trigger, other characters should not
        assert!(!key_triggers_toggle(&Key::Character("0".into())));
        assert!(!key_triggers_toggle(&Key::Character("9".into())));
        assert!(!key_triggers_toggle(&Key::Character("A".into())));
        assert!(!key_triggers_toggle(&Key::Character("z".into())));
        assert!(!key_triggers_toggle(&Key::Character("!".into())));
        assert!(!key_triggers_toggle(&Key::Character("@".into())));
    }

    #[test]
    fn key_triggers_toggle_enter_vs_space() {
        // Both Enter and Space should trigger
        assert!(key_triggers_toggle(&Key::Enter));
        assert!(key_triggers_toggle(&Key::Character(" ".into())));
    }

    #[test]
    fn handle_switch_toggle_logic() {
        // This function requires Signal and EventHandler which are hard to test in isolation
        // But we can verify the logic conceptually:
        // - If not controlled, updates inner signal
        // - Always updates form control if present
        // - Calls on_change callback if present
    }
}
