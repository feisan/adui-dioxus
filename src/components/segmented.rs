use crate::components::config_provider::use_config;
use crate::components::form::use_form_item_control;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use serde_json::Value;

/// Segmented option with label/value/icon/tooltip.
#[derive(Clone, PartialEq)]
pub struct SegmentedOption {
    pub label: String,
    pub value: String,
    pub icon: Option<Element>,
    pub tooltip: Option<String>,
    pub disabled: bool,
}

impl SegmentedOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            icon: None,
            tooltip: None,
            disabled: false,
        }
    }
}

/// Props for segmented control (single selection).
#[derive(Props, Clone, PartialEq)]
pub struct SegmentedProps {
    pub options: Vec<SegmentedOption>,
    /// Controlled value.
    #[props(optional)]
    pub value: Option<String>,
    /// Uncontrolled initial value.
    #[props(optional)]
    pub default_value: Option<String>,
    /// Fill parent's width.
    #[props(default)]
    pub block: bool,
    /// Rounded shape.
    #[props(default)]
    pub round: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
}

#[component]
pub fn Segmented(props: SegmentedProps) -> Element {
    let SegmentedProps {
        options,
        value,
        default_value,
        block,
        round,
        disabled,
        class,
        style,
        on_change,
    } = props;

    let config = use_config();
    let form_control = use_form_item_control();
    let controlled = value.is_some();

    let inner = use_signal(|| default_value.clone());

    // Sync external changes into local state.
    {
        let form_ctx = form_control.clone();
        let prop_val = value.clone();
        let mut inner_signal = inner.clone();
        use_effect(move || {
            let next = resolve_value(prop_val.clone(), &form_ctx, &inner_signal);
            inner_signal.set(next);
        });
    }

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    let mut class_list = vec!["adui-segmented".to_string()];
    if block {
        class_list.push("adui-segmented-block".into());
    }
    if round {
        class_list.push("adui-segmented-round".into());
    }
    if is_disabled {
        class_list.push("adui-segmented-disabled".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let current_value = resolve_value(value.clone(), &form_control, &inner).unwrap_or_else(|| {
        options
            .iter()
            .find(|opt| !opt.disabled)
            .map(|opt| opt.value.clone())
            .unwrap_or_default()
    });

    let handle_key = {
        let opts = options.clone();
        let form_for_key = form_control.clone();
        let mut inner_for_key = inner.clone();
        let on_change_for_key = on_change.clone();
        let value_for_key = value.clone();
        let fallback_value = current_value.clone();
        move |evt: KeyboardEvent| {
            if is_disabled {
                return;
            }
            let current = resolve_value(value_for_key.clone(), &form_for_key, &inner_for_key)
                .unwrap_or_else(|| fallback_value.clone());
            let idx = opts.iter().position(|opt| opt.value == current);
            let len = opts.len();
            if len == 0 {
                return;
            }
            let next_idx = match evt.key() {
                Key::ArrowRight | Key::ArrowDown => idx.map(|i| (i + 1) % len).unwrap_or(0),
                Key::ArrowLeft | Key::ArrowUp => idx
                    .map(|i| if i == 0 { len - 1 } else { i - 1 })
                    .unwrap_or(0),
                _ => return,
            };
            let target = &opts[next_idx];
            if target.disabled {
                return;
            }
            apply_segmented(
                target.value.clone(),
                controlled,
                &mut inner_for_key,
                &form_for_key,
                &on_change_for_key,
            );
        }
    };
    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "tablist",
            tabindex: if is_disabled { -1 } else { 0 },
            onkeydown: handle_key,
            {options.into_iter().map(|opt| {
                let active = opt.value == current_value;
                let mut item_class = vec!["adui-segmented-item".to_string()];
                if active { item_class.push("adui-segmented-item-active".into()); }
                if opt.disabled { item_class.push("adui-segmented-item-disabled".into()); }

                let tooltip_text = opt.tooltip.clone().unwrap_or_default();

                let on_click = {
                    let value = opt.value.clone();
                    let form_for_click = form_control.clone();
                    let mut inner_for_click = inner.clone();
                    let on_change_for_click = on_change.clone();
                    move |_| {
                        if is_disabled || opt.disabled {
                            return;
                        }
                        apply_segmented(
                            value.clone(),
                            controlled,
                            &mut inner_for_click,
                            &form_for_click,
                            &on_change_for_click,
                        );
                    }
                };

                rsx! {
                    button {
                        class: "{item_class.join(\" \")}",
                        title: tooltip_text,
                        aria_pressed: active,
                        disabled: is_disabled || opt.disabled,
                        onclick: on_click,
                        if let Some(icon) = opt.icon.clone() {
                            span { class: "adui-segmented-item-icon", {icon} }
                        }
                        span { class: "adui-segmented-item-label", {opt.label.clone()} }
                    }
                }
            })}
        }
    }
}

fn resolve_value(
    value: Option<String>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    inner: &Signal<Option<String>>,
) -> Option<String> {
    value
        .or_else(|| {
            form_control
                .as_ref()
                .and_then(|ctx| value_from_form(ctx.value()))
        })
        .or_else(|| (*inner.read()).clone())
}

fn value_from_form(val: Option<Value>) -> Option<String> {
    match val {
        Some(Value::String(s)) => Some(s),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(b.to_string()),
        _ => None,
    }
}

fn apply_segmented(
    next: String,
    controlled: bool,
    inner: &mut Signal<Option<String>>,
    form_control: &Option<crate::components::form::FormItemControlContext>,
    on_change: &Option<EventHandler<String>>,
) {
    if !controlled {
        inner.set(Some(next.clone()));
    }

    if let Some(ctx) = form_control.as_ref() {
        ctx.set_value(Value::String(next.clone()));
    }

    if let Some(cb) = on_change.as_ref() {
        cb.call(next);
    }
}
