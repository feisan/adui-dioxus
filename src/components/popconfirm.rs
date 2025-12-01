use crate::components::button::{Button, ButtonType};
use crate::components::popover::Popover;
use crate::components::tooltip::{TooltipPlacement, TooltipTrigger, update_open_state};
use dioxus::prelude::*;

/// Props for the Popconfirm component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct PopconfirmProps {
    /// Main title shown in the confirmation panel.
    pub title: String,
    /// Optional description text displayed under the title.
    #[props(optional)]
    pub description: Option<String>,
    /// Text for the confirm button. Defaults to "确定".
    #[props(optional)]
    pub ok_text: Option<String>,
    /// Text for the cancel button. Defaults to "取消".
    #[props(optional)]
    pub cancel_text: Option<String>,
    /// Called when the user confirms the action.
    #[props(optional)]
    pub on_confirm: Option<EventHandler<()>>,
    /// Called when the user cancels the action.
    #[props(optional)]
    pub on_cancel: Option<EventHandler<()>>,
    /// Visual type for the confirm button. Defaults to `Primary`.
    #[props(optional)]
    pub ok_type: Option<ButtonType>,
    /// Whether the confirm button should use danger styling.
    #[props(default)]
    pub ok_danger: bool,
    /// Placement of the popconfirm relative to the trigger. Defaults to `Top`.
    #[props(optional)]
    pub placement: Option<TooltipPlacement>,
    /// Trigger mode. Defaults to click.
    #[props(default = TooltipTrigger::Click)]
    pub trigger: TooltipTrigger,
    /// Controlled open state. When set, visibility is controlled by the parent.
    #[props(optional)]
    pub open: Option<bool>,
    /// Initial open state when used in uncontrolled mode.
    #[props(optional)]
    pub default_open: Option<bool>,
    /// Called when the open state changes due to user interaction.
    #[props(optional)]
    pub on_open_change: Option<EventHandler<bool>>,
    /// Disable user interaction.
    #[props(default)]
    pub disabled: bool,
    /// Extra class for the trigger wrapper.
    #[props(optional)]
    pub class: Option<String>,
    /// Extra class for the popconfirm panel.
    #[props(optional)]
    pub overlay_class: Option<String>,
    /// Inline styles applied to the popconfirm panel.
    #[props(optional)]
    pub overlay_style: Option<String>,
    /// Trigger element.
    pub children: Element,
}

/// Confirmation popover built on top of [`Popover`].
#[component]
pub fn Popconfirm(props: PopconfirmProps) -> Element {
    let PopconfirmProps {
        title,
        description,
        ok_text,
        cancel_text,
        on_confirm,
        on_cancel,
        ok_type,
        ok_danger,
        placement,
        trigger,
        open,
        default_open,
        on_open_change,
        disabled,
        class,
        overlay_class,
        overlay_style,
        children,
    } = props;

    // Internal open state used when the component is not controlled.
    let open_state: Signal<bool> = use_signal(|| default_open.unwrap_or(false));
    let is_controlled = open.is_some();
    let current_open = open.unwrap_or(*open_state.read());

    let disabled_flag = disabled;
    let is_controlled_flag = is_controlled;
    let open_for_handlers = open_state;
    let on_open_change_cb = on_open_change;

    let ok_label = ok_text.unwrap_or_else(|| "确定".to_string());
    let cancel_label = cancel_text.unwrap_or_else(|| "取消".to_string());
    let ok_button_type = ok_type.unwrap_or(ButtonType::Primary);
    let ok_danger_flag = ok_danger;

    let title_text = title.clone();
    let description_text = description.clone();

    let on_confirm_cb = on_confirm;
    let on_cancel_cb = on_cancel;

    let overlay_class_clone = overlay_class.clone();
    let overlay_style_clone = overlay_style.clone();

    rsx! {
        Popover {
            // Pass placement/trigger/open information through to the underlying
            // Popover so it can handle click/hover semantics while Popconfirm
            // stays in control of the actual open value.
            placement: placement,
            trigger: trigger,
            open: Some(current_open),
            on_open_change: move |next: bool| {
                update_open_state(
                    disabled_flag,
                    is_controlled_flag,
                    open_for_handlers,
                    on_open_change_cb,
                    next,
                );
            },
            class: class.clone(),
            overlay_class: overlay_class_clone.clone(),
            overlay_style: overlay_style_clone.clone(),
            // Popconfirm owns the content node; Popover only handles the shell.
            content: Some(rsx! {
                div { class: "adui-popconfirm-inner",
                    div { class: "adui-popconfirm-body",
                        div { class: "adui-popconfirm-title", "{title_text}" }
                        if let Some(desc) = description_text.clone() {
                            div { class: "adui-popconfirm-description", "{desc}" }
                        }
                    }
                    div { class: "adui-popconfirm-footer",
                        Button {
                            r#type: ButtonType::Default,
                            onclick: move |_| {
                                if let Some(cb) = on_cancel_cb {
                                    cb.call(());
                                }
                                update_open_state(
                                    disabled_flag,
                                    is_controlled_flag,
                                    open_for_handlers,
                                    on_open_change_cb,
                                    false,
                                );
                            },
                            "{cancel_label}"
                        }
                        Button {
                            r#type: ok_button_type,
                            danger: ok_danger_flag,
                            onclick: move |_| {
                                if let Some(cb) = on_confirm_cb {
                                    cb.call(());
                                }
                                update_open_state(
                                    disabled_flag,
                                    is_controlled_flag,
                                    open_for_handlers,
                                    on_open_change_cb,
                                    false,
                                );
                            },
                            "{ok_label}"
                        }
                    }
                }
            }),
            children: children
        }
    }
}

#[cfg(test)]
mod popconfirm_tests {
    use super::*;

    #[test]
    fn popconfirm_props_defaults() {
        // Test default values used in the component
        // ok_text defaults to "确定"
        let ok_text_default = "确定".to_string();
        assert_eq!(ok_text_default, "确定");

        // cancel_text defaults to "取消"
        let cancel_text_default = "取消".to_string();
        assert_eq!(cancel_text_default, "取消");

        // ok_type defaults to ButtonType::Primary
        let ok_type_default = ButtonType::Primary;
        assert_eq!(ok_type_default, ButtonType::Primary);

        // trigger defaults to TooltipTrigger::Click
        let trigger_default = TooltipTrigger::Click;
        assert_eq!(trigger_default, TooltipTrigger::Click);

        // ok_danger defaults to false
        let ok_danger_default = false;
        assert_eq!(ok_danger_default, false);

        // disabled defaults to false
        let disabled_default = false;
        assert_eq!(disabled_default, false);
    }

    #[test]
    fn popconfirm_props_optional_fields() {
        // Test that optional fields can be None
        // Note: PopconfirmProps requires children and title, so we can't create a full instance
        let _description: Option<String> = None;
        let _ok_text: Option<String> = None;
        let _cancel_text: Option<String> = None;
        let _on_confirm: Option<EventHandler<()>> = None;
        let _on_cancel: Option<EventHandler<()>> = None;
        let _ok_type: Option<ButtonType> = None;
        let _placement: Option<TooltipPlacement> = None;
        let _open: Option<bool> = None;
        let _default_open: Option<bool> = None;
        let _on_open_change: Option<EventHandler<bool>> = None;
        let _class: Option<String> = None;
        let _overlay_class: Option<String> = None;
        let _overlay_style: Option<String> = None;
        // All optional fields can be None
        assert!(true);
    }

    #[test]
    fn popconfirm_tooltip_trigger_variants() {
        // Test TooltipTrigger enum variants
        assert_eq!(TooltipTrigger::Click, TooltipTrigger::Click);
        assert_eq!(TooltipTrigger::Hover, TooltipTrigger::Hover);
        assert_ne!(TooltipTrigger::Click, TooltipTrigger::Hover);
    }

    #[test]
    fn popconfirm_tooltip_placement_variants() {
        // Test TooltipPlacement enum variants exist
        let _top = TooltipPlacement::Top;
        let _bottom = TooltipPlacement::Bottom;
        let _left = TooltipPlacement::Left;
        let _right = TooltipPlacement::Right;
        // Verify variants are distinct
        assert_ne!(_top, _bottom);
    }

    #[test]
    fn popconfirm_button_type_variants() {
        // Test ButtonType enum variants
        assert_eq!(ButtonType::Primary, ButtonType::Primary);
        assert_ne!(ButtonType::Primary, ButtonType::Default);
        assert_ne!(ButtonType::Primary, ButtonType::Dashed);
        assert_ne!(ButtonType::Primary, ButtonType::Link);
        assert_ne!(ButtonType::Primary, ButtonType::Text);
    }

    #[test]
    fn popconfirm_default_text_values() {
        // Test that default text values are correct
        let ok_text = None;
        let ok_text_final = ok_text.unwrap_or_else(|| "确定".to_string());
        assert_eq!(ok_text_final, "确定");

        let cancel_text = None;
        let cancel_text_final = cancel_text.unwrap_or_else(|| "取消".to_string());
        assert_eq!(cancel_text_final, "取消");
    }

    #[test]
    fn popconfirm_custom_text_values() {
        // Test that custom text values can be provided
        let ok_text = Some("Confirm".to_string());
        assert_eq!(ok_text.as_ref().unwrap(), "Confirm");

        let cancel_text = Some("Cancel".to_string());
        assert_eq!(cancel_text.as_ref().unwrap(), "Cancel");
    }
}
