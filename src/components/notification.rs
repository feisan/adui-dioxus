use crate::components::overlay::{OverlayHandle, OverlayKey, OverlayKind, OverlayMeta};
use dioxus::prelude::*;

/// Notification types aligned with message types for simplicity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

/// Placement for notification list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NotificationPlacement {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl NotificationPlacement {
    fn as_style(&self) -> &'static str {
        match self {
            NotificationPlacement::TopRight => "top: 24px; right: 24px;",
            NotificationPlacement::TopLeft => "top: 24px; left: 24px;",
            NotificationPlacement::BottomRight => "bottom: 24px; right: 24px;",
            NotificationPlacement::BottomLeft => "bottom: 24px; left: 24px;",
        }
    }
}

/// Configuration of a single notification.
#[derive(Clone, Debug, PartialEq)]
pub struct NotificationConfig {
    pub title: String,
    pub description: Option<String>,
    pub r#type: NotificationType,
    pub placement: NotificationPlacement,
    /// Auto close delay in seconds. Set to 0 for no auto-dismiss.
    pub duration: f32,
    /// Custom icon element. When None, default icon based on type is used.
    pub icon: Option<Element>,
    /// Additional CSS class.
    pub class: Option<String>,
    /// Inline styles.
    pub style: Option<String>,
    /// Callback when notification is clicked.
    pub on_click: Option<EventHandler<()>>,
    /// Unique key for this notification.
    pub key: Option<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            r#type: NotificationType::Info,
            placement: NotificationPlacement::TopRight,
            duration: 4.5,
            icon: None,
            class: None,
            style: None,
            on_click: None,
            key: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NotificationEntry {
    pub key: OverlayKey,
    pub meta: OverlayMeta,
    pub config: NotificationConfig,
}

pub type NotificationEntriesSignal = Signal<Vec<NotificationEntry>>;

pub fn use_notification_entries_provider() -> NotificationEntriesSignal {
    let signal: NotificationEntriesSignal = use_context_provider(|| Signal::new(Vec::new()));
    signal
}

pub fn use_notification_entries() -> NotificationEntriesSignal {
    use_context::<NotificationEntriesSignal>()
}

#[derive(Clone)]
pub struct NotificationApi {
    overlay: OverlayHandle,
    entries: NotificationEntriesSignal,
}

impl NotificationApi {
    pub fn new(overlay: OverlayHandle, entries: NotificationEntriesSignal) -> Self {
        Self { overlay, entries }
    }

    pub fn open(&self, config: NotificationConfig) -> OverlayKey {
        let (key, meta) = self.overlay.open(OverlayKind::Notification, false);
        let mut entries = self.entries;
        entries.write().push(NotificationEntry {
            key,
            meta,
            config: config.clone(),
        });
        schedule_notification_dismiss(key, self.entries, self.overlay.clone(), config.duration);
        key
    }

    fn open_with_type(
        &self,
        title: impl Into<String>,
        description: Option<String>,
        placement: NotificationPlacement,
        kind: NotificationType,
    ) -> OverlayKey {
        let cfg = NotificationConfig {
            title: title.into(),
            description,
            r#type: kind,
            placement,
            duration: 4.5,
            icon: None,
            class: None,
            style: None,
            on_click: None,
            key: None,
        };
        self.open(cfg)
    }

    pub fn info(&self, title: impl Into<String>, description: Option<String>) -> OverlayKey {
        self.open_with_type(
            title,
            description,
            NotificationPlacement::TopRight,
            NotificationType::Info,
        )
    }

    pub fn success(&self, title: impl Into<String>, description: Option<String>) -> OverlayKey {
        self.open_with_type(
            title,
            description,
            NotificationPlacement::TopRight,
            NotificationType::Success,
        )
    }

    pub fn warning(&self, title: impl Into<String>, description: Option<String>) -> OverlayKey {
        self.open_with_type(
            title,
            description,
            NotificationPlacement::TopRight,
            NotificationType::Warning,
        )
    }

    pub fn error(&self, title: impl Into<String>, description: Option<String>) -> OverlayKey {
        self.open_with_type(
            title,
            description,
            NotificationPlacement::TopRight,
            NotificationType::Error,
        )
    }

    pub fn close(&self, key: OverlayKey) {
        let mut entries = self.entries;
        entries.write().retain(|e| e.key != key);
        self.overlay.close(key);
    }

    pub fn destroy(&self) {
        let mut entries = self.entries;
        let current: Vec<_> = entries.read().iter().map(|e| e.key).collect();
        entries.write().clear();
        for k in current {
            self.overlay.close(k);
        }
    }
}

#[component]
pub fn NotificationHost() -> Element {
    let entries_signal = use_notification_entries();
    let entries = entries_signal.read().clone();

    if entries.is_empty() {
        return rsx! {};
    }

    let top_right: Vec<_> = entries
        .iter()
        .filter(|e| e.config.placement == NotificationPlacement::TopRight)
        .cloned()
        .collect();
    let bottom_right: Vec<_> = entries
        .iter()
        .filter(|e| e.config.placement == NotificationPlacement::BottomRight)
        .cloned()
        .collect();

    rsx! {
        // topRight container
        if !top_right.is_empty() {
            div {
                class: "adui-notification-root adui-notification-top-right",
                style: "position: fixed; top: 24px; inset-inline-end: 0; z-index: 1000; display: flex; flex-direction: column; gap: 8px; padding-inline-end: 24px;",
                {top_right.iter().map(|entry| {
                    let key = entry.key;
                    let z = entry.meta.z_index;
                    let title = entry.config.title.clone();
                    let desc = entry.config.description.clone();
                    let kind_class = match entry.config.r#type {
                        NotificationType::Info => "adui-notification-info",
                        NotificationType::Success => "adui-notification-success",
                        NotificationType::Warning => "adui-notification-warning",
                        NotificationType::Error => "adui-notification-error",
                    };
                    rsx! {
                        div {
                            key: "notice-{key:?}",
                            class: "adui-notification {kind_class}",
                            style: "pointer-events: auto; z-index: {z}; min-width: 280px; max-width: 480px; padding: 12px 16px; border-radius: 4px; background: var(--adui-color-bg-container); box-shadow: var(--adui-shadow); color: var(--adui-color-text); border: 1px solid var(--adui-color-border);",
                            div {
                                style: "font-weight: 500; margin-bottom: 4px;",
                                "{title}"
                            }
                            if let Some(text) = desc {
                                div { style: "font-size: 13px; color: var(--adui-color-text-secondary);", "{text}" }
                            }
                            button {
                                style: "margin-left: 8px; background: none; border: none; cursor: pointer; color: var(--adui-color-text-secondary); float: right;",
                                onclick: move |_| {
                                    let mut entries = entries_signal;
                                    entries.write().retain(|e| e.key != key);
                                },
                                "×"
                            }
                        }
                    }
                })}
            }
        }

        // bottomRight container
        if !bottom_right.is_empty() {
            div {
                class: "adui-notification-root adui-notification-bottom-right",
                style: "position: fixed; bottom: 24px; inset-inline-end: 0; z-index: 1000; display: flex; flex-direction: column; gap: 8px; padding-inline-end: 24px;",
                {bottom_right.iter().map(|entry| {
                    let key = entry.key;
                    let z = entry.meta.z_index;
                    let title = entry.config.title.clone();
                    let desc = entry.config.description.clone();
                    let kind_class = match entry.config.r#type {
                        NotificationType::Info => "adui-notification-info",
                        NotificationType::Success => "adui-notification-success",
                        NotificationType::Warning => "adui-notification-warning",
                        NotificationType::Error => "adui-notification-error",
                    };
                    rsx! {
                        div {
                            key: "notice-bottom-{key:?}",
                            class: "adui-notification {kind_class}",
                            style: "pointer-events: auto; z-index: {z}; min-width: 280px; max-width: 480px; padding: 12px 16px; border-radius: 4px; background: var(--adui-color-bg-container); box-shadow: var(--adui-shadow); color: var(--adui-color-text); border: 1px solid var(--adui-color-border);",
                            div {
                                style: "font-weight: 500; margin-bottom: 4px;",
                                "{title}"
                            }
                            if let Some(text) = desc {
                                div { style: "font-size: 13px; color: var(--adui-color-text-secondary);", "{text}" }
                            }
                            button {
                                style: "margin-left: 8px; background: none; border: none; cursor: pointer; color: var(--adui-color-text-secondary); float: right;",
                                onclick: move |_| {
                                    let mut entries = entries_signal;
                                    entries.write().retain(|e| e.key != key);
                                },
                                "×"
                            }
                        }
                    }
                })}
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn schedule_notification_dismiss(
    key: OverlayKey,
    entries: NotificationEntriesSignal,
    overlay: OverlayHandle,
    duration_secs: f32,
) {
    use wasm_bindgen::{JsCast, closure::Closure};

    if duration_secs <= 0.0 {
        return;
    }

    if let Some(window) = web_sys::window() {
        let delay_ms = (duration_secs * 1000.0) as i32;
        let mut entries_signal = entries;
        let overlay_clone = overlay.clone();
        let callback = Closure::once(move || {
            entries_signal.write().retain(|e| e.key != key);
            overlay_clone.close(key);
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            delay_ms,
        );
        callback.forget();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn schedule_notification_dismiss(
    _key: OverlayKey,
    _entries: NotificationEntriesSignal,
    _overlay: OverlayHandle,
    _duration_secs: f32,
) {
}
