use crate::components::floating::use_floating_close_handle;
use crate::components::overlay::OverlayKind;
use crate::components::select_base::use_floating_layer;
use dioxus::events::{KeyboardEvent, MouseEvent};
use dioxus::prelude::Key;
use dioxus::prelude::*;

/// Placement of the tooltip bubble relative to the trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Trigger mode for opening/closing the tooltip.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TooltipTrigger {
    #[default]
    Hover,
    Click,
}

/// Props for the Tooltip component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Simple text title shown inside the tooltip when `content` is not set.
    #[props(optional)]
    pub title: Option<String>,
    /// Custom tooltip content node.
    #[props(optional)]
    pub content: Option<Element>,
    /// Placement of the tooltip relative to the trigger. Defaults to `Top`.
    #[props(optional)]
    pub placement: Option<TooltipPlacement>,
    /// How the tooltip is triggered. Defaults to hover.
    #[props(default)]
    pub trigger: TooltipTrigger,
    /// Controlled open state. When set, the component becomes controlled and
    /// does not manage its own visibility.
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
    /// Extra class for the tooltip bubble.
    #[props(optional)]
    pub overlay_class: Option<String>,
    /// Inline styles applied to the tooltip bubble.
    #[props(optional)]
    pub overlay_style: Option<String>,
    /// Trigger element.
    pub children: Element,
}

/// Lightweight Ant Design flavored tooltip.
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let TooltipProps {
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

    let placement = placement.unwrap_or_default();

    // Internal open state used only when the component is not controlled.
    let open_state: Signal<bool> = use_signal(|| default_open.unwrap_or(false));
    let is_controlled = open.is_some();
    let current_open = open.unwrap_or(*open_state.read());

    // Register with the overlay manager to obtain a z-index slot.
    let floating = use_floating_layer(OverlayKind::Tooltip, current_open);
    let current_z = *floating.z_index.read();

    // Click-outside + Esc close helper is only needed for uncontrolled
    // click-triggered tooltips. For controlled cases the parent is expected to
    // handle visibility.
    let close_handle = if !is_controlled && matches!(trigger, TooltipTrigger::Click) {
        Some(use_floating_close_handle(open_state))
    } else {
        None
    };

    // Snapshots for event handlers.
    let disabled_flag = disabled;
    let is_controlled_flag = is_controlled;
    let open_for_handlers = open_state;
    let trigger_mode = trigger;
    let current_open_flag = current_open;
    let close_handle_for_click = close_handle;
    let close_handle_for_content = close_handle;

    let class_attr = {
        let mut list = vec!["adui-tooltip-root".to_string()];
        if let Some(extra) = class {
            list.push(extra);
        }
        list.join(" ")
    };

    let overlay_class_attr = {
        let mut list = vec!["adui-tooltip".to_string()];
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

    let title_text = title.clone();
    let content_node = content.clone();

    rsx! {
        span {
            class: "{class_attr}",
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_evt: MouseEvent| {
                if matches!(trigger_mode, TooltipTrigger::Hover) {
                    update_open_state(
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
                    update_open_state(
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
                update_open_state(
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
                            update_open_state(
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
                    div { class: "adui-tooltip-inner",
                        if let Some(node) = content_node {
                            {node}
                        } else if let Some(text) = title_text {
                            span { "{text}" }
                        }
                    }
                }
            }
        }
    }
}

pub fn update_open_state(
    disabled: bool,
    is_controlled: bool,
    mut open_signal: Signal<bool>,
    on_open_change: Option<EventHandler<bool>>,
    next: bool,
) {
    if disabled {
        return;
    }

    if is_controlled {
        if let Some(cb) = on_open_change {
            cb.call(next);
        }
    } else {
        open_signal.set(next);
        if let Some(cb) = on_open_change {
            cb.call(next);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_placement_default() {
        assert_eq!(TooltipPlacement::default(), TooltipPlacement::Top);
    }

    #[test]
    fn tooltip_placement_all_variants() {
        assert_eq!(TooltipPlacement::Top, TooltipPlacement::Top);
        assert_eq!(TooltipPlacement::Bottom, TooltipPlacement::Bottom);
        assert_eq!(TooltipPlacement::Left, TooltipPlacement::Left);
        assert_eq!(TooltipPlacement::Right, TooltipPlacement::Right);
        assert_ne!(TooltipPlacement::Top, TooltipPlacement::Bottom);
        assert_ne!(TooltipPlacement::Top, TooltipPlacement::Left);
        assert_ne!(TooltipPlacement::Top, TooltipPlacement::Right);
        assert_ne!(TooltipPlacement::Bottom, TooltipPlacement::Left);
        assert_ne!(TooltipPlacement::Bottom, TooltipPlacement::Right);
        assert_ne!(TooltipPlacement::Left, TooltipPlacement::Right);
    }

    #[test]
    fn tooltip_placement_clone() {
        let original = TooltipPlacement::Right;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn tooltip_trigger_default() {
        assert_eq!(TooltipTrigger::default(), TooltipTrigger::Hover);
    }

    #[test]
    fn tooltip_trigger_all_variants() {
        assert_eq!(TooltipTrigger::Hover, TooltipTrigger::Hover);
        assert_eq!(TooltipTrigger::Click, TooltipTrigger::Click);
        assert_ne!(TooltipTrigger::Hover, TooltipTrigger::Click);
    }

    #[test]
    fn tooltip_trigger_clone() {
        let original = TooltipTrigger::Click;
        let cloned = original;
        assert_eq!(original, cloned);
    }
}
