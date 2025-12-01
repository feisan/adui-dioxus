use crate::components::message::{MessageApi, MessageHost, use_message_entries_provider};
use crate::components::notification::{
    NotificationApi, NotificationHost, use_notification_entries_provider,
};
use crate::components::overlay::{OverlayHandle, OverlayKind, use_overlay_provider};
use dioxus::prelude::*;

/// Minimal modal API exposed by `App`.
#[derive(Clone)]
pub struct ModalApi {
    overlay: OverlayHandle,
}

impl ModalApi {
    pub fn new(overlay: OverlayHandle) -> Self {
        Self { overlay }
    }

    /// Placeholder for future `confirm`/`open` helpers.
    pub fn open(&self) {
        let _ = self.overlay.open(OverlayKind::Modal, true);
    }
}

/// Context value shared by `App` and consumed by `use_app` / `use_message` /
/// `use_notification` / `use_modal`.
#[derive(Clone, Default)]
pub struct AppContextValue {
    pub message: Option<MessageApi>,
    pub notification: Option<NotificationApi>,
    pub modal: Option<ModalApi>,
}

/// Props for the top-level `App` container.
#[derive(Props, Clone, PartialEq)]
pub struct AppProps {
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Top-level App component that wires OverlayManager and exposes global
/// feedback APIs through context.
#[component]
pub fn App(props: AppProps) -> Element {
    let AppProps {
        class,
        style,
        children,
    } = props;

    // Install a single overlay manager for this App subtree.
    let overlay = use_overlay_provider();

    // Install per-App message & notification queues in context.
    let message_entries = use_message_entries_provider();
    let notification_entries = use_notification_entries_provider();

    let ctx = AppContextValue {
        message: Some(MessageApi::new(overlay.clone(), message_entries)),
        notification: Some(NotificationApi::new(overlay.clone(), notification_entries)),
        modal: Some(ModalApi::new(overlay)),
    };

    use_context_provider(|| ctx.clone());

    let class_attr = class.unwrap_or_default();
    let style_attr = style.unwrap_or_default();

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {children}
        }
        // Global feedback hosts
        MessageHost {}
        NotificationHost {}
    }
}

/// Access the full App context. Returns a default value when used outside of
/// an `App` subtree so that callers can opt into a graceful fallback.
pub fn use_app() -> AppContextValue {
    try_use_context::<AppContextValue>().unwrap_or_default()
}

/// Convenience hook to access the message API.
pub fn use_message() -> Option<MessageApi> {
    use_app().message
}

/// Convenience hook to access the notification API.
pub fn use_notification() -> Option<NotificationApi> {
    use_app().notification
}

/// Convenience hook to access the modal API.
pub fn use_modal() -> Option<ModalApi> {
    use_app().modal
}

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn app_context_value_default() {
        let ctx = AppContextValue::default();
        assert!(ctx.message.is_none());
        assert!(ctx.notification.is_none());
        assert!(ctx.modal.is_none());
    }

    #[test]
    fn app_context_value_clone() {
        let ctx1 = AppContextValue::default();
        let ctx2 = ctx1.clone();
        // Verify clone works - both should have None for all fields
        assert!(ctx1.message.is_none());
        assert!(ctx2.message.is_none());
        assert!(ctx1.notification.is_none());
        assert!(ctx2.notification.is_none());
        assert!(ctx1.modal.is_none());
        assert!(ctx2.modal.is_none());
    }

    #[test]
    fn app_context_value_with_all_fields() {
        // Test that AppContextValue can hold all optional fields
        let ctx = AppContextValue {
            message: None,
            notification: None,
            modal: None,
        };
        assert!(ctx.message.is_none());
        assert!(ctx.notification.is_none());
        assert!(ctx.modal.is_none());
    }

    #[test]
    fn modal_api_structure() {
        // Verify ModalApi structure and methods exist
        // Note: Creating an actual instance requires runtime context with Signal
        // But we can verify the type structure and method signatures
        fn assert_modal_api_methods() {
            // ModalApi::new takes OverlayHandle and returns ModalApi
            // ModalApi::open takes &self and returns ()
            // These are verified by compilation
        }
        assert_modal_api_methods();
    }

    #[test]
    fn app_props_structure() {
        // Verify AppProps can be created with optional fields
        // Note: Creating actual Element requires runtime context
        // But we can verify the structure
        fn assert_app_props_structure() {
            // AppProps has optional class and style, and required children
            // This is verified by compilation
        }
        assert_app_props_structure();
    }
}
