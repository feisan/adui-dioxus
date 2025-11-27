use crate::components::overlay::{OverlayKey, OverlayKind, use_overlay};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;

/// Basic modal props, targeting the most common controlled use cases.
#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    /// Whether the modal is visible.
    pub open: bool,
    /// Optional title displayed in the header.
    #[props(optional)]
    pub title: Option<String>,
    /// Custom footer content. When `None`, a minimal footer with OK/Cancel
    /// buttons can be added in the future; for now we simply omit the footer.
    #[props(optional)]
    pub footer: Option<Element>,
    /// Called when the user confirms the dialog.
    #[props(optional)]
    pub on_ok: Option<EventHandler<()>>,
    /// Called when the user cancels or dismisses the dialog.
    #[props(optional)]
    pub on_cancel: Option<EventHandler<()>>,
    /// When true, show a close button in the top-right corner.
    #[props(default = true)]
    pub closable: bool,
    /// Whether clicking the mask should trigger `on_cancel`.
    #[props(default = true)]
    pub mask_closable: bool,
    /// Remove modal content from the tree when closed.
    #[props(default)]
    pub destroy_on_close: bool,
    /// Optional fixed width for the modal content in pixels.
    #[props(optional)]
    pub width: Option<f32>,
    /// Additional CSS class on the root container.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline styles applied to the root container.
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Simple Ant Design flavored modal.
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let ModalProps {
        open,
        title,
        footer,
        on_ok,
        on_cancel,
        closable,
        mask_closable,
        destroy_on_close,
        width,
        class,
        style,
        children,
    } = props;

    // Track the overlay key associated with this modal so we can release the
    // z-index slot when it closes. When OverlayManager is not available we
    // gracefully fall back to a fixed z-index.
    let overlay = use_overlay();
    let modal_key: Signal<Option<OverlayKey>> = use_signal(|| None);
    let z_index: Signal<i32> = use_signal(|| 1000);

    {
        let overlay = overlay.clone();
        let mut key_signal = modal_key;
        let mut z_signal = z_index;
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
        });
    }

    if !open && destroy_on_close {
        return rsx! {};
    }

    let current_z = *z_index.read();
    let width_px = width.unwrap_or(520.0);

    let class_attr = class.unwrap_or_else(|| "adui-modal".to_string());
    let style_attr = style.unwrap_or_default();

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
        if matches!(evt.key(), Key::Escape) {
            evt.prevent_default();
            on_close();
        }
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
                style: "position: fixed; inset: 0; display: flex; align-items: center; justify-content: center; z-index: {current_z + 1}; {style_attr}",
                onkeydown: on_keydown,
                tabindex: 0,
                div {
                    class: "adui-modal-content",
                    style: "min-width: {width_px}px; max-width: 80vw; background: var(--adui-color-bg-container); border-radius: var(--adui-radius-lg, 8px); box-shadow: var(--adui-shadow-secondary); border: 1px solid var(--adui-color-border); overflow: hidden;",
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
                                    "×"
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
                    if let Some(footer_node) = footer {
                        div {
                            class: "adui-modal-footer",
                            style: "padding: 10px 16px; border-top: 1px solid var(--adui-color-border); text-align: right;",
                            {footer_node}
                        }
                    } else {
                        // Simple default footer with OK/Cancel buttons could be added here in the future.
                        div { }
                    }
                }
            }
        }
    }
}
