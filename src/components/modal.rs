//! Modal component aligned with Ant Design 6.0.
//!
//! Features:
//! - Confirm loading state
//! - Centered positioning
//! - Customizable buttons
//! - Semantic classNames/styles

use crate::components::button::{Button, ButtonType};
use crate::components::overlay::{OverlayKey, OverlayKind, use_overlay};
use crate::foundation::{
    ClassListExt, ModalClassNames, ModalSemantic, ModalStyles, StyleStringExt,
};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

/// Modal type for static method variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalType {
    Info,
    Success,
    Error,
    Warning,
    Confirm,
}

impl ModalType {
    #[allow(dead_code)]
    fn as_class(&self) -> &'static str {
        match self {
            ModalType::Info => "adui-modal-info",
            ModalType::Success => "adui-modal-success",
            ModalType::Error => "adui-modal-error",
            ModalType::Warning => "adui-modal-warning",
            ModalType::Confirm => "adui-modal-confirm",
        }
    }
}

/// Basic modal props, targeting the most common controlled use cases.
#[derive(Props, Clone)]
pub struct ModalProps {
    /// Whether the modal is visible.
    pub open: bool,
    /// Optional title displayed in the header.
    #[props(optional)]
    pub title: Option<String>,
    /// Custom footer content. When `None`, default OK/Cancel buttons are shown.
    /// Can be an Element or a function: (originNode, extra) -> Element
    #[props(optional)]
    pub footer: Option<Element>,
    /// Custom footer render function: (originNode, extra) -> Element
    /// When provided, this takes precedence over `footer`.
    #[props(optional)]
    pub footer_render: Option<Rc<dyn Fn(Element, FooterExtra) -> Element>>,
    /// Whether to show the footer (set to false to hide default footer).
    #[props(default = true)]
    pub show_footer: bool,
    /// Called when the user confirms the dialog.
    #[props(optional)]
    pub on_ok: Option<EventHandler<()>>,
    /// Called when the user cancels or dismisses the dialog.
    #[props(optional)]
    pub on_cancel: Option<EventHandler<()>>,
    /// When true, show a close button in the top-right corner.
    /// Can also be a ClosableConfig object for advanced configuration.
    #[props(default = true)]
    pub closable: bool,
    /// Advanced closable configuration (takes precedence over `closable` boolean).
    #[props(optional)]
    pub closable_config: Option<ClosableConfig>,
    /// Whether clicking the mask should trigger `on_cancel`.
    #[props(default = true)]
    pub mask_closable: bool,
    /// Remove modal content from the tree when closed.
    #[props(default)]
    pub destroy_on_close: bool,
    /// Remove modal content from the tree when hidden (since 5.25.0).
    #[props(default)]
    pub destroy_on_hidden: bool,
    /// Force render Modal even when not visible.
    #[props(default)]
    pub force_render: bool,
    /// Optional fixed width for the modal content in pixels.
    /// Can also be a responsive width map: { xs: 300, sm: 400, ... }
    #[props(optional)]
    pub width: Option<f32>,
    /// Responsive width configuration: { breakpoint: width }
    #[props(optional)]
    pub width_responsive: Option<HashMap<String, f32>>,
    /// Whether to vertically center the modal.
    #[props(default)]
    pub centered: bool,
    /// Whether the OK button is in loading state.
    #[props(default)]
    pub confirm_loading: bool,
    /// OK button text.
    #[props(optional)]
    pub ok_text: Option<String>,
    /// Cancel button text.
    #[props(optional)]
    pub cancel_text: Option<String>,
    /// OK button type.
    #[props(optional)]
    pub ok_type: Option<ButtonType>,
    /// Whether to enable keyboard (Escape to close).
    #[props(default = true)]
    pub keyboard: bool,
    /// Custom close icon element.
    #[props(optional)]
    pub close_icon: Option<Element>,
    /// Callback after modal closes completely.
    #[props(optional)]
    pub after_close: Option<EventHandler<()>>,
    /// Callback when modal open state changes.
    #[props(optional)]
    pub after_open_change: Option<EventHandler<bool>>,
    /// Additional CSS class on the root container.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline styles applied to the root container.
    #[props(optional)]
    pub style: Option<String>,
    /// Semantic class names.
    #[props(optional)]
    pub class_names: Option<ModalClassNames>,
    /// Semantic styles.
    #[props(optional)]
    pub styles: Option<ModalStyles>,
    /// Custom container for modal rendering (selector string or "false" to disable portal).
    #[props(optional)]
    pub get_container: Option<String>,
    /// Custom z-index for the modal.
    #[props(optional)]
    pub z_index: Option<i32>,
    /// Mask configuration (style, closable, etc.).
    #[props(optional)]
    pub mask: Option<MaskConfig>,
    /// Custom modal render function: (node) -> Element
    #[props(optional)]
    pub modal_render: Option<Rc<dyn Fn(Element) -> Element>>,
    /// Mouse position for modal placement (x, y).
    #[props(optional)]
    pub mouse_position: Option<(f32, f32)>,
    /// Loading state for the entire modal (since 5.18.0).
    #[props(default)]
    pub loading: bool,
    /// OK button props.
    #[props(optional)]
    pub ok_button_props: Option<HashMap<String, String>>,
    /// Cancel button props.
    #[props(optional)]
    pub cancel_button_props: Option<HashMap<String, String>>,
    pub children: Element,
}

impl PartialEq for ModalProps {
    fn eq(&self, other: &Self) -> bool {
        // Compare all fields except function pointers
        self.open == other.open
            && self.title == other.title
            && self.footer == other.footer
            && self.show_footer == other.show_footer
            && self.closable == other.closable
            && self.mask_closable == other.mask_closable
            && self.destroy_on_close == other.destroy_on_close
            && self.destroy_on_hidden == other.destroy_on_hidden
            && self.force_render == other.force_render
            && self.width == other.width
            && self.width_responsive == other.width_responsive
            && self.centered == other.centered
            && self.confirm_loading == other.confirm_loading
            && self.ok_text == other.ok_text
            && self.cancel_text == other.cancel_text
            && self.ok_type == other.ok_type
            && self.keyboard == other.keyboard
            && self.close_icon == other.close_icon
            && self.after_close == other.after_close
            && self.after_open_change == other.after_open_change
            && self.class == other.class
            && self.style == other.style
            && self.class_names == other.class_names
            && self.styles == other.styles
            && self.get_container == other.get_container
            && self.z_index == other.z_index
            && self.mask == other.mask
            && self.mouse_position == other.mouse_position
            && self.loading == other.loading
            && self.ok_button_props == other.ok_button_props
            && self.cancel_button_props == other.cancel_button_props
            && self.closable_config == other.closable_config
        // Function pointers cannot be compared for equality
    }
}

/// Extra props for footer render function.
#[derive(Clone, Debug)]
pub struct FooterExtra {
    /// OK button component (as Element).
    pub ok_btn: Element,
    /// Cancel button component (as Element).
    pub cancel_btn: Element,
}

/// Closable configuration for advanced close button control.
#[derive(Clone)]
pub struct ClosableConfig {
    /// Whether to show the close button.
    pub show: bool,
    /// Custom close handler.
    pub on_close: Option<Rc<dyn Fn()>>,
    /// Callback after close animation completes.
    pub after_close: Option<Rc<dyn Fn()>>,
}

impl PartialEq for ClosableConfig {
    fn eq(&self, other: &Self) -> bool {
        self.show == other.show
        // Function pointers cannot be compared for equality
    }
}

impl std::fmt::Debug for ClosableConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClosableConfig")
            .field("show", &self.show)
            .field("on_close", &"<function>")
            .field("after_close", &"<function>")
            .finish()
    }
}

/// Mask configuration for modal backdrop.
#[derive(Clone, Debug, PartialEq)]
pub struct MaskConfig {
    /// Whether mask is visible.
    pub visible: bool,
    /// Whether clicking mask closes the modal.
    pub closable: bool,
    /// Custom mask style.
    pub style: Option<String>,
}

/// Simple Ant Design flavored modal.
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let ModalProps {
        open,
        title,
        footer,
        show_footer,
        on_ok,
        on_cancel,
        closable,
        mask_closable,
        destroy_on_close,
        width,
        centered,
        confirm_loading,
        ok_text,
        cancel_text,
        ok_type,
        keyboard,
        close_icon,
        after_close,
        after_open_change,
        class,
        style,
        class_names,
        styles,
        children,
        ..
    } = props;

    // Track previous open state for after_open_change callback
    let prev_open: Signal<bool> = use_signal(|| open);

    // Track the overlay key associated with this modal so we can release the
    // z-index slot when it closes.
    let overlay = use_overlay();
    let modal_key: Signal<Option<OverlayKey>> = use_signal(|| None);
    let z_index: Signal<i32> = use_signal(|| 1000);

    {
        let overlay = overlay.clone();
        let mut key_signal = modal_key;
        let mut z_signal = z_index;
        let mut prev_signal = prev_open;
        use_effect(move || {
            if let Some(handle) = overlay.clone() {
                let current_key = {
                    let guard = key_signal.read();
                    *guard
                };
                if open {
                    if current_key.is_none() {
                        let (key, meta) = handle.open(OverlayKind::Modal, true);
                        z_signal.set(meta.z_index);
                        key_signal.set(Some(key));
                    }
                } else if let Some(key) = current_key {
                    handle.close(key);
                    key_signal.set(None);
                }
            }

            // Call after_open_change when open state changes
            let prev = *prev_signal.read();
            if prev != open {
                if let Some(cb) = after_open_change {
                    cb.call(open);
                }
                // Call after_close when closing
                if !open {
                    if let Some(cb) = after_close {
                        cb.call(());
                    }
                }
                prev_signal.set(open);
            }
        });
    }

    if !open && destroy_on_close {
        return rsx! {};
    }

    let current_z = *z_index.read();
    let width_px = width.unwrap_or(520.0);

    // Build classes
    let mut class_list = vec!["adui-modal".to_string()];
    if centered {
        class_list.push("adui-modal-centered".into());
    }
    class_list.push_semantic(&class_names, ModalSemantic::Root);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut style_attr = style.unwrap_or_default();
    style_attr.append_semantic(&styles, ModalSemantic::Root);

    // Handlers
    let ok_handler = on_ok;
    let cancel_handler = on_cancel;

    let on_close = move || {
        if let Some(cb) = cancel_handler {
            cb.call(());
        }
    };

    let handle_ok = move || {
        if let Some(cb) = ok_handler {
            cb.call(());
        }
    };

    let on_keydown = move |evt: KeyboardEvent| {
        if keyboard && matches!(evt.key(), Key::Escape) {
            evt.prevent_default();
            on_close();
        }
    };

    // Default button texts
    let ok_button_text = ok_text.unwrap_or_else(|| "确定".to_string());
    let cancel_button_text = cancel_text.unwrap_or_else(|| "取消".to_string());
    let ok_button_type = ok_type.unwrap_or(ButtonType::Primary);

    // Close icon
    let close_icon_element = close_icon.unwrap_or_else(|| {
        rsx! { "×" }
    });

    // Content positioning style
    let content_style = if centered {
        format!(
            "position: fixed; inset: 0; display: flex; align-items: center; justify-content: center; z-index: {}; {}",
            current_z + 1,
            style_attr
        )
    } else {
        format!(
            "position: fixed; top: 100px; left: 50%; transform: translateX(-50%); z-index: {}; {}",
            current_z + 1,
            style_attr
        )
    };

    rsx! {
        if open {
            // Mask layer
            div {
                class: "adui-modal-mask",
                style: "position: fixed; inset: 0; background: rgba(0,0,0,0.45); z-index: {current_z};",
                onclick: move |_| {
                    if mask_closable {
                        on_close();
                    }
                }
            }
            // Modal content layer
            div {
                class: "{class_attr}",
                style: "{content_style}",
                onkeydown: on_keydown,
                tabindex: 0,
                div {
                    class: "adui-modal-content",
                    style: "min-width: {width_px}px; max-width: 80vw; background: var(--adui-color-bg-container); border-radius: var(--adui-radius-lg, 8px); box-shadow: var(--adui-shadow-secondary); border: 1px solid var(--adui-color-border); overflow: hidden;",
                    onclick: move |evt| evt.stop_propagation(),
                    // Header
                    if title.is_some() || closable {
                        div {
                            class: "adui-modal-header",
                            style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; border-bottom: 1px solid var(--adui-color-border);",
                            if let Some(text) = title {
                                div { class: "adui-modal-title", "{text}" }
                            }
                            if closable {
                                button {
                                    class: "adui-modal-close",
                                    r#type: "button",
                                    style: "border: none; background: none; cursor: pointer; font-size: 16px;",
                                    onclick: move |_| on_close(),
                                    {close_icon_element}
                                }
                            }
                        }
                    }
                    // Body
                    div {
                        class: "adui-modal-body",
                        style: "padding: 16px;",
                        {children}
                    }
                    // Footer
                    if show_footer {
                        if let Some(footer_node) = footer {
                            div {
                                class: "adui-modal-footer",
                                style: "padding: 10px 16px; border-top: 1px solid var(--adui-color-border); text-align: right;",
                                {footer_node}
                            }
                        } else {
                            div {
                                class: "adui-modal-footer",
                                style: "padding: 10px 16px; border-top: 1px solid var(--adui-color-border); text-align: right; display: flex; gap: 8px; justify-content: flex-end;",
                                Button {
                                    onclick: move |_| on_close(),
                                    "{cancel_button_text}"
                                }
                                Button {
                                    r#type: ok_button_type,
                                    loading: confirm_loading,
                                    onclick: move |_| handle_ok(),
                                    "{ok_button_text}"
                                }
                            }
                        }
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
    fn modal_type_classes() {
        assert_eq!(ModalType::Info.as_class(), "adui-modal-info");
        assert_eq!(ModalType::Success.as_class(), "adui-modal-success");
        assert_eq!(ModalType::Error.as_class(), "adui-modal-error");
    }
}
