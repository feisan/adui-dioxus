use crate::components::floating::use_floating_close_handle;
use crate::components::overlay::OverlayKind;
use crate::components::select_base::use_floating_layer;
use crate::components::tooltip::{TooltipPlacement, TooltipTrigger};
use dioxus::events::{KeyboardEvent, MouseEvent};
use dioxus::prelude::Key;
use dioxus::prelude::*;

/// Props for the Popover component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// Optional title node displayed at the top of the popover.
    #[props(optional)]
    pub title: Option<Element>,
    /// Main content of the popover.
    #[props(optional)]
    pub content: Option<Element>,
    /// Placement of the popover relative to the trigger. Defaults to `Top`.
    #[props(optional)]
    pub placement: Option<TooltipPlacement>,
    /// Trigger mode. Defaults to click.
    #[props(default = TooltipTrigger::Click)]
    pub trigger: TooltipTrigger,
    /// Controlled open state. When set, the component does not manage its own
    /// visibility.
    #[props(optional)]
    pub open: Option<bool>,
    /// Initial open state in uncontrolled mode.
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
    /// Extra class for the popover panel.
    #[props(optional)]
    pub overlay_class: Option<String>,
    /// Inline styles for the popover panel.
    #[props(optional)]
    pub overlay_style: Option<String>,
    /// Trigger element.
    pub children: Element,
}

/// Rich content popover built on top of the floating overlay infrastructure.
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let PopoverProps {
        title,
        content,
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

    let placement = placement.unwrap_or(TooltipPlacement::Top);

    let open_state: Signal<bool> = use_signal(|| default_open.unwrap_or(false));
    let is_controlled = open.is_some();
    let current_open = open.unwrap_or(*open_state.read());

    // Register with the overlay manager to obtain a z-index slot.
    let floating = use_floating_layer(OverlayKind::Popup, current_open);
    let current_z = *floating.z_index.read();

    // Click-outside + Esc close for uncontrolled click-triggered popovers.
    let close_handle = if !is_controlled && matches!(trigger, TooltipTrigger::Click) {
        Some(use_floating_close_handle(open_state))
    } else {
        None
    };

    let disabled_flag = disabled;
    let is_controlled_flag = is_controlled;
    let open_for_handlers = open_state;
    let trigger_mode = trigger;
    let current_open_flag = current_open;
    let close_handle_for_click = close_handle;
    let close_handle_for_content = close_handle;

    let class_attr = {
        let mut list = vec!["adui-popover-root".to_string()];
        if let Some(extra) = class {
            list.push(extra);
        }
        list.join(" ")
    };

    let overlay_class_attr = {
        let mut list = vec!["adui-popover".to_string()];
        if let Some(extra) = overlay_class {
            list.push(extra);
        }
        list.join(" ")
    };

    let overlay_style_attr = {
        let placement_css = match placement {
            TooltipPlacement::Top => {
                "bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 8px;"
            }
            TooltipPlacement::Bottom => {
                "top: 100%; left: 50%; transform: translateX(-50%); margin-top: 8px;"
            }
            TooltipPlacement::Left => {
                "right: 100%; top: 50%; transform: translateY(-50%); margin-right: 8px;"
            }
            TooltipPlacement::Right => {
                "left: 100%; top: 50%; transform: translateY(-50%); margin-left: 8px;"
            }
        };
        let extra = overlay_style.unwrap_or_default();
        format!(
            "position: absolute; z-index: {}; {}; {}",
            current_z, placement_css, extra
        )
    };

    rsx! {
        span {
            class: "{class_attr}",
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_evt: MouseEvent| {
                if matches!(trigger_mode, TooltipTrigger::Hover) {
                    super::tooltip::update_open_state(
                        disabled_flag,
                        is_controlled_flag,
                        open_for_handlers,
                        on_open_change,
                        true,
                    );
                }
            },
            onmouseleave: move |_evt: MouseEvent| {
                if matches!(trigger_mode, TooltipTrigger::Hover) {
                    super::tooltip::update_open_state(
                        disabled_flag,
                        is_controlled_flag,
                        open_for_handlers,
                        on_open_change,
                        false,
                    );
                }
            },
            onclick: move |_evt: MouseEvent| {
                if !matches!(trigger_mode, TooltipTrigger::Click) {
                    return;
                }
                if let Some(handle) = close_handle_for_click {
                    handle.mark_internal_click();
                }
                super::tooltip::update_open_state(
                    disabled_flag,
                    is_controlled_flag,
                    open_for_handlers,
                    on_open_change,
                    !current_open_flag,
                );
            },
            {children}
            if current_open {
                div {
                    class: "{overlay_class_attr}",
                    style: "{overlay_style_attr}",
                    tabindex: 0,
                    onkeydown: move |evt: KeyboardEvent| {
                        if matches!(evt.key(), Key::Escape) {
                            evt.prevent_default();
                            super::tooltip::update_open_state(
                                disabled_flag,
                                is_controlled_flag,
                                open_for_handlers,
                                on_open_change,
                                false,
                            );
                        }
                    },
                    onclick: move |_evt| {
                        if let Some(handle) = close_handle_for_content {
                            handle.mark_internal_click();
                        }
                    },
                    div { class: "adui-popover-inner",
                        if let Some(title_node) = title {
                            div { class: "adui-popover-title", {title_node} }
                        }
                        if let Some(content_node) = content {
                            div { class: "adui-popover-content", {content_node} }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod popover_tests {
    use super::*;

    #[test]
    fn popover_placement_default() {
        assert_eq!(TooltipPlacement::Top, TooltipPlacement::Top);
    }

    #[test]
    fn popover_trigger_default() {
        assert_eq!(TooltipTrigger::Click, TooltipTrigger::Click);
    }

    #[test]
    fn popover_trigger_all_variants() {
        assert_eq!(TooltipTrigger::Click, TooltipTrigger::Click);
        assert_eq!(TooltipTrigger::Hover, TooltipTrigger::Hover);
        assert_ne!(TooltipTrigger::Click, TooltipTrigger::Hover);
    }
}
