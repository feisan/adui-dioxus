use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::floating::use_floating_close_handle;
use crate::components::overlay::OverlayKind;
use crate::components::select_base::use_floating_layer;
use dioxus::events::{KeyboardEvent, MouseEvent};
use dioxus::prelude::Key;
use dioxus::prelude::*;

/// Simple menu item model for the Dropdown component.
#[derive(Clone, Debug, PartialEq)]
pub struct DropdownItem {
    pub key: String,
    pub label: String,
    pub disabled: bool,
}

impl DropdownItem {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: false,
        }
    }
}

/// Trigger mode for Dropdown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownTrigger {
    #[default]
    Click,
    Hover,
}

/// Placement of the dropdown menu relative to the trigger.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownPlacement {
    #[default]
    BottomLeft,
    BottomRight,
}

/// Props for the lightweight Dropdown component (MVP).
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    /// Menu items to display in the dropdown.
    pub items: Vec<DropdownItem>,
    /// Trigger mode. Defaults to click.
    #[props(default)]
    pub trigger: DropdownTrigger,
    /// Placement of the dropdown menu relative to the trigger.
    #[props(optional)]
    pub placement: Option<DropdownPlacement>,
    /// Controlled open state. When set, the component becomes controlled.
    #[props(optional)]
    pub open: Option<bool>,
    /// Initial open state when used in uncontrolled mode.
    #[props(optional)]
    pub default_open: Option<bool>,
    /// Called when the open state changes due to user interaction.
    #[props(optional)]
    pub on_open_change: Option<EventHandler<bool>>,
    /// Called when a menu item is clicked.
    #[props(optional)]
    pub on_click: Option<EventHandler<String>>,
    /// Disable user interaction.
    #[props(default)]
    pub disabled: bool,
    /// Extra class applied to the trigger wrapper.
    #[props(optional)]
    pub class: Option<String>,
    /// Extra class applied to the dropdown menu.
    #[props(optional)]
    pub overlay_class: Option<String>,
    /// Inline styles applied to the dropdown menu.
    #[props(optional)]
    pub overlay_style: Option<String>,
    /// Custom width for the dropdown menu in pixels (optional).
    #[props(optional)]
    pub overlay_width: Option<f32>,
    /// Trigger element (usually a Button or link).
    pub children: Element,
}

/// Lightweight Ant Design flavored Dropdown (menu).
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let DropdownProps {
        items,
        trigger,
        placement,
        open,
        default_open,
        on_open_change,
        on_click,
        disabled,
        class,
        overlay_class,
        overlay_style,
        overlay_width,
        children,
    } = props;

    let config = use_config();
    let global_size = config.size;

    let open_state: Signal<bool> = use_signal(|| default_open.unwrap_or(false));
    let is_controlled = open.is_some();
    let current_open = open.unwrap_or(*open_state.read());

    let floating = use_floating_layer(OverlayKind::Dropdown, current_open);
    let current_z = *floating.z_index.read();

    let close_handle = if !is_controlled && matches!(trigger, DropdownTrigger::Click) {
        Some(use_floating_close_handle(open_state))
    } else {
        None
    };

    let disabled_flag = disabled;
    let is_controlled_flag = is_controlled;
    let open_for_handlers = open_state;
    let on_open_change_cb = on_open_change;
    let trigger_mode = trigger;
    let current_open_flag = current_open;
    let close_handle_for_click = close_handle;
    let close_handle_for_menu = close_handle;

    let class_attr = {
        let mut list = vec!["adui-dropdown-root".to_string()];
        if let Some(extra) = class {
            list.push(extra);
        }
        list.join(" ")
    };

    let overlay_class_attr = {
        let mut list = vec!["adui-dropdown-menu".to_string()];
        if let Some(extra) = overlay_class {
            list.push(extra);
        }
        list.join(" ")
    };

    let placement = placement.unwrap_or_default();
    let width_style = overlay_width
        .map(|w| format!("min-width: {w}px;"))
        .unwrap_or_default();

    let align_style = match placement {
        DropdownPlacement::BottomLeft => "left: 0;",
        DropdownPlacement::BottomRight => "right: 0;",
    };

    let overlay_style_attr = {
        let extra = overlay_style.unwrap_or_default();
        format!(
            "position: absolute; top: 100%; margin-top: 4px; z-index: {}; {}; {} {}",
            current_z, align_style, width_style, extra
        )
    };

    let size_class = match global_size {
        ComponentSize::Small => "adui-dropdown-sm",
        ComponentSize::Large => "adui-dropdown-lg",
        ComponentSize::Middle => "adui-dropdown-md",
    };

    let on_click_cb = on_click;

    rsx! {
        span {
            class: "{class_attr}",
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_evt: MouseEvent| {
                if matches!(trigger_mode, DropdownTrigger::Hover) {
                    crate::components::tooltip::update_open_state(
                        disabled_flag,
                        is_controlled_flag,
                        open_for_handlers,
                        on_open_change_cb,
                        true,
                    );
                }
            },
            onmouseleave: move |_evt: MouseEvent| {
                if matches!(trigger_mode, DropdownTrigger::Hover) {
                    crate::components::tooltip::update_open_state(
                        disabled_flag,
                        is_controlled_flag,
                        open_for_handlers,
                        on_open_change_cb,
                        false,
                    );
                }
            },
            onclick: move |_evt: MouseEvent| {
                if !matches!(trigger_mode, DropdownTrigger::Click) {
                    return;
                }
                if let Some(handle) = close_handle_for_click {
                    handle.mark_internal_click();
                }
                crate::components::tooltip::update_open_state(
                    disabled_flag,
                    is_controlled_flag,
                    open_for_handlers,
                    on_open_change_cb,
                    !current_open_flag,
                );
            },
            onkeydown: move |evt: KeyboardEvent| {
                if matches!(evt.key(), Key::Escape) {
                    evt.prevent_default();
                    crate::components::tooltip::update_open_state(
                        disabled_flag,
                        is_controlled_flag,
                        open_for_handlers,
                        on_open_change_cb,
                        false,
                    );
                }
            },
            {children}
            if current_open {
                div {
                    class: "{overlay_class_attr} {size_class}",
                    style: "{overlay_style_attr}",
                    onclick: move |_evt| {
                        if let Some(handle) = close_handle_for_menu {
                            handle.mark_internal_click();
                        }
                    },
                    ul {
                        class: "adui-dropdown-menu-list",
                        {items.iter().map(|item| {
                            let key = item.key.clone();
                            let label = item.label.clone();
                            let disabled_item = item.disabled || disabled_flag;
                            rsx! {
                                li {
                                    class: {
                                        let mut list = vec!["adui-dropdown-menu-item".to_string()];
                                        if disabled_item {
                                            list.push("adui-dropdown-menu-item-disabled".into());
                                        }
                                        list.join(" ")
                                    },
                                    onclick: move |_evt| {
                                        if disabled_item {
                                            return;
                                        }
                                        crate::components::tooltip::update_open_state(
                                            disabled_flag,
                                            is_controlled_flag,
                                            open_for_handlers,
                                            on_open_change_cb,
                                            false,
                                        );
                                        if let Some(cb) = on_click_cb {
                                            cb.call(key.clone());
                                        }
                                    },
                                    "{label}"
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_item_new() {
        let item = DropdownItem::new("key1", "Label 1");
        assert_eq!(item.key, "key1");
        assert_eq!(item.label, "Label 1");
        assert_eq!(item.disabled, false);
    }

    #[test]
    fn dropdown_item_new_with_strings() {
        let item = DropdownItem::new(String::from("key2"), String::from("Label 2"));
        assert_eq!(item.key, "key2");
        assert_eq!(item.label, "Label 2");
        assert_eq!(item.disabled, false);
    }

    #[test]
    fn dropdown_item_clone() {
        let item1 = DropdownItem::new("key1", "Label 1");
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn dropdown_item_partial_eq() {
        let item1 = DropdownItem::new("key1", "Label 1");
        let item2 = DropdownItem::new("key1", "Label 1");
        let item3 = DropdownItem::new("key2", "Label 2");
        assert_eq!(item1, item2);
        assert_ne!(item1, item3);
    }

    #[test]
    fn dropdown_trigger_default() {
        assert_eq!(DropdownTrigger::default(), DropdownTrigger::Click);
    }

    #[test]
    fn dropdown_trigger_all_variants() {
        assert_eq!(DropdownTrigger::Click, DropdownTrigger::Click);
        assert_eq!(DropdownTrigger::Hover, DropdownTrigger::Hover);
        assert_ne!(DropdownTrigger::Click, DropdownTrigger::Hover);
    }

    #[test]
    fn dropdown_trigger_clone() {
        let original = DropdownTrigger::Hover;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn dropdown_placement_default() {
        assert_eq!(DropdownPlacement::default(), DropdownPlacement::BottomLeft);
    }

    #[test]
    fn dropdown_placement_all_variants() {
        assert_eq!(DropdownPlacement::BottomLeft, DropdownPlacement::BottomLeft);
        assert_eq!(DropdownPlacement::BottomRight, DropdownPlacement::BottomRight);
        assert_ne!(DropdownPlacement::BottomLeft, DropdownPlacement::BottomRight);
    }

    #[test]
    fn dropdown_placement_clone() {
        let original = DropdownPlacement::BottomRight;
        let cloned = original;
        assert_eq!(original, cloned);
    }
}
