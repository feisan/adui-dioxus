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
}
